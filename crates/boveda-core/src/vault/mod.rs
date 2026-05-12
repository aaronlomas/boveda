pub mod validation;
pub mod totp;
pub mod export;

use std::sync::{Arc, Mutex};

use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use std::str::FromStr;
use std::path::Path;
use crate::crypto::secret::{SecretKey, SecretString};
use crate::crypto;
use crate::storage;
use crate::error::{BovedaError, BovedaResult};

#[derive(Debug, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ImportStrategy {
    Merge,
    Replace,
}

/// Wrapper around the 256-bit AES-GCM master key.
/// Allocates key on the heap, and uses mlock/VirtualLock to prevent swapping to disk.
pub struct MasterKey(SecretKey);

impl MasterKey {
    pub fn new(key: SecretKey) -> Self {
        #[cfg(unix)]
        unsafe {
            let ptr = key.as_bytes().as_ptr() as *const libc::c_void;
            if libc::mlock(ptr, std::mem::size_of::<[u8; 32]>()) != 0 {
                eprintln!("Warning: Failed to mlock master key memory");
            }
        }
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualLock;
            let ptr = key.as_bytes().as_ptr() as *const std::ffi::c_void;
            if VirtualLock(ptr, std::mem::size_of::<[u8; 32]>()) == 0 {
                eprintln!("Warning: Failed to VirtualLock master key memory");
            }
        }
        Self(key)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl Drop for MasterKey {
    fn drop(&mut self) {
        #[cfg(unix)]
        unsafe {
            let ptr = self.0.as_bytes().as_ptr() as *const libc::c_void;
            libc::munlock(ptr, std::mem::size_of::<[u8; 32]>());
        }
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualUnlock;
            let ptr = self.0.as_bytes().as_ptr() as *const std::ffi::c_void;
            VirtualUnlock(ptr, std::mem::size_of::<[u8; 32]>());
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

impl BovedaEngine {
    /// Returns true if the vault database or its salt file exists.
    pub fn is_initialized(db_path: &Path) -> bool {
        let salt_path = db_path.with_file_name("vault.salt");
        salt_path.exists() || db_path.exists()
    }

    pub fn is_locked(&self) -> bool {
        self.master_key.lock().unwrap().is_none()
    }

    /// High-level method to unlock the vault.
    /// Handles salt detection, migration, key derivation, and database opening.
    pub async fn unlock(db_path: &Path, password: &SecretString) -> BovedaResult<Self> {
        let salt_path = db_path.with_file_name("vault.salt");
        let mut is_legacy_migration = false;

        let salt = if salt_path.exists() {
            std::fs::read(&salt_path).map_err(|e| BovedaError::IoError(format!("Error al leer salt: {}", e)))?
        } else if db_path.exists() {
            // Unencrypted database exists, need migration
            is_legacy_migration = true;
            vec![] // placeholder
        } else {
            // First time initialization: generate new salt
            use rand::RngCore;
            let mut new_salt = vec![0u8; 32];
            rand::rngs::OsRng.fill_bytes(&mut new_salt);
            std::fs::write(&salt_path, &new_salt).map_err(|e| BovedaError::IoError(format!("Error al escribir salt: {}", e)))?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(&salt_path, std::fs::Permissions::from_mode(0o600));
            }
            new_salt
        };

        if is_legacy_migration {
            return Self::unlock_legacy_migration(db_path, password).await;
        }

        // Normal path: Derive key from salt and password
        let key = crypto::derive_key(password, &salt)?;

        let engine = Self::open_encrypted(db_path, &key).await
            .map_err(|_| BovedaError::InvalidPassword)?;

        // Verify the key by initializing schema (if key is wrong, this will fail)
        storage::init_db(&engine.db).await.map_err(|_| BovedaError::InvalidPassword)?;

        // Store the master key in the engine
        {
            let mut key_lock = engine.master_key.lock().unwrap();
            *key_lock = Some(MasterKey::new(key));
        }

        Ok(engine)
    }

    /// Internal helper to handle legacy migration flow.
    async fn unlock_legacy_migration(db_path: &Path, password: &SecretString) -> BovedaResult<Self> {
        let unencrypted_engine = Self::open_unencrypted(db_path).await?;
        let meta = storage::get_vault_meta(&unencrypted_engine.db).await?
            .ok_or_else(|| BovedaError::MigrationError("Legacy vault has no metadata".to_string()))?;
        
        let (legacy_salt, challenge_opt) = meta;
        let key = crypto::derive_key(password, &legacy_salt)?;
        
        // Verification logic
        let mut verified = false;
        if let Some(challenge) = challenge_opt {
            if let Ok(dec) = crypto::decrypt(&challenge, &key) {
                if dec == "boveda_auth" { verified = true; }
            }
        } else {
            // Fallback: try to decrypt first account
            let accounts = storage::get_accounts(&unencrypted_engine.db).await.unwrap_or_default();
            if let Some(acc) = accounts.first() {
                if crypto::decrypt(&acc.encrypted_password, &key).is_err() {
                    return Err(BovedaError::InvalidPassword);
                }
                verified = true;
            } else {
                // No accounts and no challenge? Assume verified for migration
                verified = true;
            }
        }

        if !verified {
            return Err(BovedaError::InvalidPassword);
        }

        // Perform migration
        storage::migrate_to_sqlcipher(&unencrypted_engine.db, &key, db_path).await?;

        // Open newly encrypted database
        let engine = Self::open_encrypted(db_path, &key).await?;
        {
            let mut key_lock = engine.master_key.lock().unwrap();
            *key_lock = Some(MasterKey::new(key));
        }

        Ok(engine)
    }

    /// Opens the database without a key (useful for initial migration check).
    pub async fn open_unencrypted(db_path: &Path) -> BovedaResult<Self> {
        let url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;
        Ok(Self {
            db: pool,
            master_key: Arc::new(Mutex::new(None)),
        })
    }

    /// Safe helper to format the master key into a hex string for SQLCipher PRAGMA.
    fn generate_pragma_key(key: &SecretKey) -> SecretString {
        const HEX_CHARS: &[u8] = b"0123456789abcdef";
        let mut pragma = Vec::with_capacity(64 + 4);
        pragma.extend_from_slice(b"\"x'");
        for &byte in key.as_bytes() {
            pragma.push(HEX_CHARS[(byte >> 4) as usize]);
            pragma.push(HEX_CHARS[(byte & 0x0f) as usize]);
        }
        pragma.push(b'\'');
        pragma.push(b'"');
        
        let s = String::from_utf8(pragma).expect("Valid ASCII");
        SecretString::new(s)
    }

    /// Opens the database utilizing SQLCipher with the derived key.
    pub async fn open_encrypted(db_path: &Path, key: &SecretKey) -> BovedaResult<Self> {
        let url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());
        let mut options = SqliteConnectOptions::from_str(&url)?;
        
        // Send the PRAGMA key right upon connecting
        let pragma_key = Self::generate_pragma_key(key);
        
        options = options.pragma("key", pragma_key.as_str().to_string());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self {
            db: pool,
            master_key: Arc::new(Mutex::new(None)),
        })
    }

    /// Locks the vault by clearing the master key from memory.
    pub fn lock(&self) {
        let mut lock = self.master_key.lock().unwrap();
        *lock = None;
    }

    /// Internal helper to check if unlocked and return error if not.
    fn check_unlocked(&self) -> BovedaResult<()> {
        if self.is_locked() {
            Err(BovedaError::VaultLocked)
        } else {
            Ok(())
        }
    }

    /// Internal helper to execute a closure with the master key if unlocked.
    fn with_key<F, R>(&self, f: F) -> BovedaResult<R>
    where
        F: FnOnce(&SecretKey) -> R,
    {
        let lock = self.master_key.lock().unwrap();
        lock.as_ref()
            .map(|mk| f(&mk.0))
            .ok_or(BovedaError::VaultLocked)
    }

    /// Retrieves and decrypts all accounts.
    pub async fn get_accounts(&self) -> BovedaResult<Vec<crate::storage::models::Account>> {
        self.check_unlocked()?;
        let rows = storage::get_accounts(&self.db).await?;
        
        let mut accounts = Vec::with_capacity(rows.len());
        for row in rows {
            let (dec_site, dec_username) = self.with_key(|key| {
                let s = crypto::decrypt(&row.site, key).unwrap_or_else(|_| SecretString::from(row.site.clone()));
                let u = crypto::decrypt(&row.username, key).unwrap_or_else(|_| SecretString::from(row.username.clone()));
                (s, u)
            })?;
            
            accounts.push(crate::storage::models::Account {
                id: row.id,
                site: dec_site,
                username: dec_username,
                password_cipher: row.encrypted_password,
                notes_cipher: row.encrypted_notes,
                favicon_url: None,
                group_name: row.group_name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        // Sort by site
        accounts.sort_by_key(|a| a.site.as_str().to_lowercase());
        Ok(accounts)
    }

    /// Adds a new account, encrypting sensitive fields automatically.
    pub async fn add_account(
        &self,
        site: SecretString,
        username: SecretString,
        password: SecretString,
        notes: Option<SecretString>,
    ) -> BovedaResult<String> {
        self.check_unlocked()?;
        
        // Validation
        validation::validate_string(site.as_str(), "Sitio", validation::MAX_SITE_LEN)?;
        validation::validate_string(username.as_str(), "Usuario", validation::MAX_USERNAME_LEN)?;
        validation::validate_string(password.as_str(), "Contraseña", validation::MAX_PASSWORD_LEN)?;
        if let Some(n) = &notes {
            validation::validate_string(n.as_str(), "Notas", validation::MAX_NOTES_LEN)?;
        }

        let (enc_site, enc_username, enc_password, enc_notes) = self.with_key(|key| {
            let s = crypto::encrypt(&site, key)?;
            let u = crypto::encrypt(&username, key)?;
            let p = crypto::encrypt(&password, key)?;
            let n = notes.as_ref().map(|n| crypto::encrypt(n, key)).transpose()?;
            Ok::<_, BovedaError>((s, u, p, n))
        })??;

        storage::add_account(
            &self.db,
            &enc_site,
            &enc_username,
            &enc_password,
            enc_notes.as_deref(),
            None,
        ).await
    }

    /// Decrypts a single ciphertext on-demand.
    pub fn decrypt_secret(&self, ciphertext: &str) -> BovedaResult<SecretString> {
        self.with_key(|key| crypto::decrypt(ciphertext, key))?
    }

    /// Deletes an account by ID.
    pub async fn delete_account(&self, id: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        storage::delete_account(&self.db, id).await
    }

    // ─── Group Management ─────────────────────────────────────────────────────

    pub async fn update_account_group(&self, id: &str, group_name: Option<&str>) -> BovedaResult<()> {
        self.check_unlocked()?;
        if let Some(name) = group_name {
            validation::validate_string(name, "Grupo", validation::MAX_GROUP_NAME_LEN)?;
        }
        storage::update_account_group(&self.db, id, group_name).await
    }

    pub async fn rename_group(&self, old_name: &str, new_name: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(new_name, "Grupo", validation::MAX_GROUP_NAME_LEN)?;

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

    // ─── Preferences ───────────────────────────────────────────────────────────

    pub async fn get_preference(&self, key: &str) -> BovedaResult<Option<String>> {
        self.check_unlocked()?;
        storage::get_preference(&self.db, key).await
    }

    pub async fn set_preference(&self, key: &str, value: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(key, "Preferencia", validation::MAX_PREF_KEY_LEN)?;
        validation::validate_string(value, "Valor de preferencia", validation::MAX_PREF_VALUE_LEN)?;
        storage::set_preference(&self.db, key, value).await
    }

    pub async fn delete_preference(&self, key: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(key, "Preferencia", validation::MAX_PREF_KEY_LEN)?;
        storage::delete_preference(&self.db, key).await
    }

    /// Exports the entire vault into a secure, encrypted package.
    pub async fn export_vault(&self, export_password: &SecretString) -> BovedaResult<String> {
        self.check_unlocked()?;
        
        // 1. Get all accounts (DECRYPTED)
        let accounts = self.get_accounts().await?;
        let mut export_accounts = Vec::with_capacity(accounts.len());
        
        for acc in accounts {
            // Decrypt password and notes
            let password = self.decrypt_secret(&acc.password_cipher)?;
            let notes = acc.notes_cipher.as_ref()
                .map(|c| self.decrypt_secret(c))
                .transpose()?;
                
            export_accounts.push(export::ExportAccount {
                site: acc.site,
                username: acc.username,
                password,
                notes,
                group_name: acc.group_name,
            });
        }
        
        // 2. Get all preferences
        let preferences = storage::get_all_preferences(&self.db).await?;
        
        let payload = export::ExportPayload {
            accounts: export_accounts,
            preferences,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        // 3. Encrypt into package
        let package = export::ExportPackage::encrypt(&payload, export_password)?;
        
        // 4. Serialize package to JSON
        serde_json::to_string(&package)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))
    }

    /// Imports a secure package into the current vault using the specified strategy.
    pub async fn import_vault(&self, package_json: &str, export_password: &SecretString, strategy: ImportStrategy) -> BovedaResult<()> {
        self.check_unlocked()?;

        // 1. Parse and decrypt package
        let package: export::ExportPackage = serde_json::from_str(package_json)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
        
        let payload = package.decrypt(export_password)?;

        // 2. Apply strategy
        if matches!(strategy, ImportStrategy::Replace) {
            // Clear current accounts
            sqlx::query("DELETE FROM accounts").execute(&self.db).await?;
            // Note: Preferences are overwritten anyway by set_preference later, 
            // but we might want to clear them too if we want a full replacement.
            // For now, let's just clear accounts as that's what "duplicates" refers to.
        }

        // 3. Insert accounts
        // We use add_account which handles encryption with the CURRENT master key.
        for acc in payload.accounts {
            let id = self.add_account(acc.site, acc.username, acc.password, acc.notes).await?;
            if let Some(group) = acc.group_name {
                let _ = self.update_account_group(&id, Some(&group)).await;
            }
        }

        // 3. Apply preferences (Optional merge)
        for (key, value) in payload.preferences {
            let _ = self.set_preference(&key, &value).await;
        }

        Ok(())
    }

    // ─── Connection Management ─────────────────────────────────────────────────

    pub async fn close(&self) {
        self.db.close().await;
    }
}