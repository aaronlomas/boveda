pub mod validation;
pub mod totp;
pub mod export;
pub mod authentication;
pub mod export_import;

use std::sync::{Arc, Mutex};

use sqlx::SqlitePool;
use crate::crypto::secret::{SecretKey, SecretString};
use crate::crypto;
use crate::storage;
use crate::error::{BovedaError, BovedaResult};
// ─────────────────────────────────────────────────────────────────────────────
// 🏗️  Core Structures
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ImportStrategy {
    Merge,
    Replace,
}
/// Wrapper around the 256-bit ChaCha20-Poly1305 master key.
/// Allocates key on the heap, and uses mlock/VirtualLock to prevent swapping to disk.
pub struct MasterKey(Box<[u8; 32]>);

impl MasterKey {
    pub fn new(key: SecretKey) -> Self {
        let boxed_key = Box::new(*key.as_bytes());
        
        #[cfg(unix)]
        unsafe {
            let ptr = boxed_key.as_ptr() as *const libc::c_void;
            if libc::mlock(ptr, 32) != 0 {
                eprintln!("Warning: Failed to mlock master key memory. Ensure CAP_IPC_LOCK is set for better security.");
            }
        }
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualLock;
            let ptr = boxed_key.as_ptr() as *const std::ffi::c_void;
            if VirtualLock(ptr, 32) == 0 {
                eprintln!("Warning: Failed to VirtualLock master key memory. Ensure sufficient process quotas.");
            }
        }
        Self(boxed_key)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Drop for MasterKey {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.0.zeroize();
        
        #[cfg(unix)]
        unsafe {
            let ptr = self.0.as_ptr() as *const libc::c_void;
            libc::munlock(ptr, 32);
        }
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualUnlock;
            let ptr = self.0.as_ptr() as *const std::ffi::c_void;
            VirtualUnlock(ptr, 32);
        }
    }
}

/// Core engine holding the database connection and the unlocked master key.
#[derive(Clone)]
pub struct BovedaEngine {
    /// SQLite connection pool.
    pub(crate) db: SqlitePool,
    /// The derived master key, present only when the vault is unlocked.
    pub(crate) master_key: Arc<Mutex<Option<MasterKey>>>,
}


// ─────────────────────────────────────────────────────────────────────────────
// 📁 Account Management
// ─────────────────────────────────────────────────────────────────────────────

impl BovedaEngine {
    /// Retrieves and decrypts all accounts in the vault.
    pub async fn get_accounts(&self) -> BovedaResult<Vec<crate::storage::models::Account>> {
        self.check_unlocked()?;
        let rows = storage::get_accounts(&self.db).await?;
        self.decrypt_rows(rows)
    }

    /// Retrieves and decrypts a page of accounts.
    pub async fn get_accounts_paged(&self, limit: i64, offset: i64) -> BovedaResult<Vec<crate::storage::models::Account>> {
        self.check_unlocked()?;
        let rows = storage::get_accounts_paged(&self.db, limit, offset).await?;
        self.decrypt_rows(rows)
    }

    /// Helper to decrypt a batch of account rows.
    fn decrypt_rows(&self, rows: Vec<crate::storage::AccountRow>) -> BovedaResult<Vec<crate::storage::models::Account>> {
        let mut accounts = Vec::with_capacity(rows.len());
        for row in rows {
            let (dec_site, dec_username) = self.with_key(|key| {
                // SEC-D1: Propagate decryption errors instead of silently falling back to
                // the raw ciphertext. Returning encrypted blobs as plaintext would leak
                // confusing data and mask corruption / wrong-key conditions.
                let s = crypto::decrypt(&row.site, key)?;
                let u = crypto::decrypt(&row.username, key)?;
                Ok::<_, BovedaError>((s, u))
            })??;
            
            accounts.push(crate::storage::models::Account {
                id: row.id,
                site: dec_site.as_str().to_string(),
                username: dec_username.as_str().to_string(),
                password_cipher: row.encrypted_password,
                recovery_code_cipher: row.encrypted_recovery_code,
                notes_cipher: row.encrypted_notes,
                favicon_url: row.favicon_url,
                group_name: row.group_name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        // Sort by site (Note: Storage already sorts by encrypted site, but decrypted sort is better)
        accounts.sort_by_key(|a| a.site.to_lowercase());
        Ok(accounts)
    }

    /// Adds a new account, encrypting sensitive fields automatically.
    pub async fn add_account(
        &self,
        site: SecretString,
        username: SecretString,
        password: SecretString,
        recovery_code: Option<SecretString>,
        notes: Option<SecretString>,
    ) -> BovedaResult<String> {
        self.check_unlocked()?;
        
        // Validation
        validation::validate_string(site.as_str(), "Sitio", validation::MAX_SITE_LEN, true)?;
        validation::validate_string(username.as_str(), "Usuario", validation::MAX_USERNAME_LEN, true)?;
        validation::validate_string(password.as_str(), "Contraseña", validation::MAX_PASSWORD_LEN, true)?;
        if let Some(rc) = &recovery_code {
            validation::validate_string(rc.as_str(), "Código de recuperación", validation::MAX_PASSWORD_LEN, false)?;
        }
        if let Some(n) = &notes {
            validation::validate_string(n.as_str(), "Notas", validation::MAX_NOTES_LEN, false)?;
        }

        let (enc_site, enc_username, enc_password, enc_recovery_code, enc_notes) = self.with_key(|key| {
            let s = crypto::encrypt(&site, key)?;
            let u = crypto::encrypt(&username, key)?;
            let p = crypto::encrypt(&password, key)?;
            let rc = recovery_code.as_ref().map(|rc| crypto::encrypt(rc, key)).transpose()?;
            let n = notes.as_ref().map(|n| crypto::encrypt(n, key)).transpose()?;
            Ok::<_, BovedaError>((s, u, p, rc, n))
        })??;

        let id = storage::add_account(
            &self.db,
            &enc_site,
            &enc_username,
            &enc_password,
            enc_recovery_code.as_deref(),
            enc_notes.as_deref(),
            None,
        ).await?;

        self.log_audit(crate::audit::AuditAction::AccountCreate, Some(&id)).await?;
        Ok(id)
    }

    /// Decrypts a single ciphertext on-demand.
    pub fn decrypt_secret(&self, ciphertext: &str) -> BovedaResult<SecretString> {
        // SOC2: El acceso se loguea en la capa de Comandos (Facade) mediante cmd_reveal_password
        self.with_key(|key| crypto::decrypt(ciphertext, key))?
    }

    /// Deletes an account by ID.
    pub async fn delete_account(&self, id: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::AccountDelete, Some(id)).await?;
        storage::delete_account(&self.db, id).await
    }

// ─────────────────────────────────────────────────────────────────────────────
// 🔒 PIN Management
// ─────────────────────────────────────────────────────────────────────────────

    pub async fn get_pins(&self) -> BovedaResult<Vec<crate::storage::models::Pin>> {
        self.check_unlocked()?;
        let rows = storage::get_pins(&self.db).await?;
        let mut pins = Vec::with_capacity(rows.len());
        for row in rows {
            let dec_name = self.with_key(|key| {
                // SEC-D1: Propagate decryption errors — do not return the ciphertext blob.
                crypto::decrypt(&row.name, key)
            })??;
            
            pins.push(crate::storage::models::Pin {
                id: row.id,
                name: dec_name.as_str().to_string(),
                encrypted_pin: row.encrypted_pin,
                encrypted_notes: row.encrypted_notes,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        pins.sort_by_key(|p| p.name.to_lowercase());
        Ok(pins)
    }

    pub async fn add_pin(
        &self,
        name: SecretString,
        pin: SecretString,
        notes: Option<SecretString>,
    ) -> BovedaResult<String> {
        self.check_unlocked()?;
        
        validation::validate_string(name.as_str(), "Nombre", validation::MAX_PIN_NAME_LEN, true)?;
        validation::validate_string(pin.as_str(), "PIN", validation::MAX_PIN_LEN, true)?;
        if let Some(n) = &notes {
            validation::validate_string(n.as_str(), "Notas", validation::MAX_NOTES_LEN, false)?;
        }

        let (enc_name, enc_pin, enc_notes) = self.with_key(|key| {
            let n = crypto::encrypt(&name, key)?;
            let p = crypto::encrypt(&pin, key)?;
            let note = notes.as_ref().map(|n| crypto::encrypt(n, key)).transpose()?;
            Ok::<_, BovedaError>((n, p, note))
        })??;

        let id = storage::add_pin(
            &self.db,
            &enc_name,
            &enc_pin,
            enc_notes.as_deref(),
        ).await?;

        self.log_audit(crate::audit::AuditAction::PinCreate, Some(&id)).await?;
        Ok(id)
    }

    pub async fn delete_pin(&self, id: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::PinDelete, Some(id)).await?;
        storage::delete_pin(&self.db, id).await
    }

// ─────────────────────────────────────────────────────────────────────────────
// 👥 Group Management
// ─────────────────────────────────────────────────────────────────────────────

    pub async fn update_account_group(&self, id: &str, group_name: Option<&str>) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::AccountGroupUpdate, Some(id)).await?;
        if let Some(name) = group_name {
            validation::validate_string(name, "Grupo", validation::MAX_GROUP_NAME_LEN, true)?;
        }
        storage::update_account_group(&self.db, id, group_name).await
    }

    pub async fn rename_group(&self, old_name: &str, new_name: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(new_name, "Grupo", validation::MAX_GROUP_NAME_LEN, true)?;

        // Use a transaction for atomic update
        let mut tx = self.db.begin().await?;

        storage::rename_group_tx(&mut tx, old_name, new_name).await?;

        // Update the groups list in preferences
        let raw = storage::get_preference_tx(&mut tx, "groups").await?;
        let mut groups: Vec<String> = raw
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
            
        if let Some(pos) = groups.iter().position(|g| g == old_name) {
            groups[pos] = new_name.to_string();
        }
        
        let serialized = serde_json::to_string(&groups)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
        storage::set_preference_tx(&mut tx, "groups", &serialized).await?;
        
        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_group(&self, name: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        let count = storage::count_accounts_in_group(&self.db, name).await?;
        if count > 0 {
            return Err(BovedaError::Other(format!(
                "El grupo \"{}\" tiene {} cuenta(s) asignada(s). Mueve las cuentas antes de eliminarlo.",
                name, count
            )));
        }

        let mut tx = self.db.begin().await?;
        storage::delete_group_tx(&mut tx, name).await?;

        // Update the groups list in preferences
        let raw = storage::get_preference_tx(&mut tx, "groups").await?;
        let mut groups: Vec<String> = raw
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
            
        groups.retain(|g| g != name);
        let serialized = serde_json::to_string(&groups)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
        storage::set_preference_tx(&mut tx, "groups", &serialized).await?;
        
        tx.commit().await?;
        Ok(())
    }

// Preferences & Settings--------------------------------------------------------------

    pub async fn get_preference(&self, key: &str) -> BovedaResult<Option<String>> {
        self.check_unlocked()?;
        storage::get_preference(&self.db, key).await
    }

    pub async fn set_preference(&self, key: &str, value: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(key, "Preferencia", validation::MAX_PREF_KEY_LEN, true)?;
        validation::validate_string(value, "Valor de preferencia", validation::MAX_PREF_VALUE_LEN, false)?;
        storage::set_preference(&self.db, key, value).await
    }

    pub async fn delete_preference(&self, key: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(key, "Preferencia", validation::MAX_PREF_KEY_LEN, true)?;
        storage::delete_preference(&self.db, key).await
    }
}

// Document Management----------------------------------------------------------------

impl BovedaEngine {
    /// Retrieves and decrypts all document headers (title + metadata, NOT content).
    pub async fn get_documents(&self) -> BovedaResult<Vec<crate::storage::models::Document>> {
        self.check_unlocked()?;
        let rows = storage::get_documents(&self.db).await?;
        let mut docs = Vec::with_capacity(rows.len());
        for row in rows {
            let dec_title = self.with_key(|key| {
                // SEC-D1: Propagate decryption errors — do not return the ciphertext blob.
                crypto::decrypt(&row.title, key)
            })??;
            docs.push(crate::storage::models::Document {
                id: row.id,
                title: dec_title.as_str().to_string(),
                encrypted_description: row.encrypted_description,
                encrypted_content: row.encrypted_content,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        Ok(docs)
    }

    /// Adds a new encrypted document/note.
    pub async fn add_document(
        &self,
        title: SecretString,
        description: Option<SecretString>,
        content: SecretString,
    ) -> BovedaResult<String> {
        self.check_unlocked()?;

        validation::validate_string(title.as_str(), "Título", 256, true)?;
        if let Some(d) = &description {
            validation::validate_string(d.as_str(), "Descripción", 1024, false)?;
        }
        validation::validate_string(content.as_str(), "Contenido", 1_048_576, false)?;

        let (enc_title, enc_description, enc_content) = self.with_key(|key| {
            let t = crypto::encrypt(&title, key)?;
            let d = description
                .as_ref()
                .map(|d| crypto::encrypt(d, key))
                .transpose()?;
            let c = crypto::encrypt(&content, key)?;
            Ok::<_, BovedaError>((t, d, c))
        })??;

        let id = storage::add_document(
            &self.db,
            &enc_title,
            enc_description.as_deref(),
            &enc_content,
        )
        .await?;

        self.log_audit(crate::audit::AuditAction::DocumentCreate, Some(&id)).await?;
        Ok(id)
    }

    /// Updates an existing encrypted document/note.
    pub async fn update_document(
        &self,
        id: &str,
        title: SecretString,
        description: Option<SecretString>,
        content: SecretString,
    ) -> BovedaResult<()> {
        self.check_unlocked()?;

        validation::validate_string(title.as_str(), "Título", 256, true)?;
        if let Some(d) = &description {
            validation::validate_string(d.as_str(), "Descripción", 1024, false)?;
        }
        validation::validate_string(content.as_str(), "Contenido", 1_048_576, false)?;

        let (enc_title, enc_description, enc_content) = self.with_key(|key| {
            let t = crypto::encrypt(&title, key)?;
            let d = description
                .as_ref()
                .map(|d| crypto::encrypt(d, key))
                .transpose()?;
            let c = crypto::encrypt(&content, key)?;
            Ok::<_, BovedaError>((t, d, c))
        })??;

        storage::update_document(
            &self.db,
            id,
            &enc_title,
            enc_description.as_deref(),
            &enc_content,
        )
        .await?;

        self.log_audit(crate::audit::AuditAction::DocumentUpdate, Some(id)).await?;
        Ok(())
    }

    /// Deletes a document by ID.
    pub async fn delete_document(&self, id: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::DocumentDelete, Some(id)).await?;
        storage::delete_document(&self.db, id).await
    }

    /// Decrypts the content of a single document on demand.
    pub fn decrypt_document_content(&self, encrypted_content: &str) -> BovedaResult<String> {
        self.with_key(|key| {
            crypto::decrypt(encrypted_content, key)
                .map(|s| s.as_str().to_string())
        })?
    }
}