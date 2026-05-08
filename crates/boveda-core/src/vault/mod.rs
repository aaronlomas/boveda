pub mod validation;

use std::sync::{Arc, Mutex};

use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use base64::Engine;
use std::str::FromStr;
use std::path::PathBuf;
use crate::crypto::secret::{SecretKey, SecretString};
use crate::crypto;
use crate::storage;
use crate::error::{BovedaError, BovedaResult};

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
    master_key: Arc<Mutex<Option<MasterKey>>>,
}

impl BovedaEngine {
    /// Returns true if the vault database or its salt file exists.
    pub fn is_initialized(db_path: &PathBuf) -> bool {
        let salt_path = db_path.with_file_name("vault.salt");
        salt_path.exists() || db_path.exists()
    }

    pub fn is_locked(&self) -> bool {
        self.master_key.lock().unwrap().is_none()
    }

    /// High-level method to unlock the vault.
    /// Handles salt detection, migration, key derivation, and database opening.
    pub async fn unlock(db_path: &PathBuf, password: &SecretString) -> BovedaResult<Self> {
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
        let key = crypto::derive_key(password, &salt).map_err(|e| BovedaError::CryptoError(e.to_string()))?;

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
    async fn unlock_legacy_migration(db_path: &PathBuf, password: &SecretString) -> BovedaResult<Self> {
        let unencrypted_engine = Self::open_unencrypted(db_path).await?;
        let meta = storage::get_vault_meta(&unencrypted_engine.db).await?
            .ok_or_else(|| BovedaError::Other("Legacy vault has no metadata".to_string()))?;
        
        let (legacy_salt, challenge_opt) = meta;
        let key = crypto::derive_key(password, &legacy_salt).map_err(|e| BovedaError::CryptoError(e.to_string()))?;
        
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
    pub async fn open_unencrypted(db_path: &PathBuf) -> BovedaResult<Self> {
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

    /// Safe helper to format the master key into a hex string for SQLCipher PRAGMA
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
        SecretString::new(String::from_utf8(pragma).expect("Valid ASCII"))
    }

    /// Opens the database utilizing SQLCipher with the derived key.
    pub async fn open_encrypted(db_path: &PathBuf, key: &SecretKey) -> BovedaResult<Self> {
        let url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());
        let mut options = SqliteConnectOptions::from_str(&url)?;
        
        // Send the PRAGMA key right upon connecting
        let pragma_key_secret = Self::generate_pragma_key(key);
        
        options = options.pragma("key", pragma_key_secret.as_str().to_string());

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
        accounts.sort_by(|a, b| a.site.as_str().to_lowercase().cmp(&b.site.as_str().to_lowercase()));
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
            let s = crypto::encrypt(&site, key).map_err(|e| BovedaError::CryptoError(e.to_string()))?;
            let u = crypto::encrypt(&username, key).map_err(|e| BovedaError::CryptoError(e.to_string()))?;
            let p = crypto::encrypt(&password, key).map_err(|e| BovedaError::CryptoError(e.to_string()))?;
            let n = notes.as_ref().map(|n| crypto::encrypt(n, key).map_err(|e| BovedaError::CryptoError(e.to_string()))).transpose()?;
            Ok::<_, BovedaError>((s, u, p, n))
        })??;

        storage::add_account(
            &self.db,
            &enc_site,
            &enc_username,
            &enc_password,
            enc_notes.as_deref(),
            None,
        ).await.map_err(|e| BovedaError::DatabaseError(e.to_string()))
    }

    /// Decrypts a single ciphertext on-demand.
    pub fn decrypt_secret(&self, ciphertext: &str) -> BovedaResult<SecretString> {
        self.with_key(|key| crypto::decrypt(ciphertext, key).map_err(|e| BovedaError::CryptoError(e.to_string())))?
    }

    /// Deletes an account by ID.
    pub async fn delete_account(&self, id: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        storage::delete_account(&self.db, id).await.map_err(|e| BovedaError::DatabaseError(e.to_string()))
    }

    // ─── Group Management ─────────────────────────────────────────────────────

    pub async fn update_account_group(&self, id: &str, group_name: Option<&str>) -> BovedaResult<()> {
        self.check_unlocked()?;
        if let Some(name) = group_name {
            validation::validate_string(name, "Grupo", validation::MAX_GROUP_NAME_LEN)?;
        }
        storage::update_account_group(&self.db, id, group_name).await.map_err(|e| BovedaError::DatabaseError(e.to_string()))
    }

    pub async fn rename_group(&self, old_name: &str, new_name: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(new_name, "Grupo", validation::MAX_GROUP_NAME_LEN)?;

        // Use a transaction for atomic update
        let mut tx = self.db.begin().await?;

        storage::rename_group_tx(&mut *tx, old_name, new_name).await?;

        // Update the groups list in preferences
        let raw = storage::get_preference_tx(&mut *tx, "groups").await?;
        let mut groups: Vec<String> = raw
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
            
        if let Some(pos) = groups.iter().position(|g| g == old_name) {
            groups[pos] = new_name.to_string();
        }
        
        let serialized = serde_json::to_string(&groups)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
        storage::set_preference_tx(&mut *tx, "groups", &serialized).await?;
        
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
        storage::delete_group_tx(&mut *tx, name).await?;

        // Update the groups list in preferences
        let raw = storage::get_preference_tx(&mut *tx, "groups").await?;
        let mut groups: Vec<String> = raw
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
            
        groups.retain(|g| g != name);
        let serialized = serde_json::to_string(&groups)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
        storage::set_preference_tx(&mut *tx, "groups", &serialized).await?;
        
        tx.commit().await?;
        Ok(())
    }

    // ─── Preferences ───────────────────────────────────────────────────────────

    pub async fn get_preference(&self, key: &str) -> BovedaResult<Option<String>> {
        self.check_unlocked()?;
        storage::get_preference(&self.db, key).await.map_err(|e| BovedaError::DatabaseError(e.to_string()))
    }

    pub async fn set_preference(&self, key: &str, value: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(key, "Preferencia", validation::MAX_PREF_KEY_LEN)?;
        validation::validate_string(value, "Valor de preferencia", validation::MAX_PREF_VALUE_LEN)?;
        storage::set_preference(&self.db, key, value).await.map_err(|e| BovedaError::DatabaseError(e.to_string()))
    }

    // ─── TOTP (2FA) Management ─────────────────────────────────────────────────

    /// Initializes 2FA for the vault.
    /// Generates a new seed, encrypts it with the master key, and saves it.
    pub async fn setup_totp(&self) -> BovedaResult<crate::auth::TotpSetupPayload> {
        self.check_unlocked()?;

        // 1. Generate a new seed (20 bytes random)
        let seed = crate::auth::TotpManager::generate_secret();

        // 2. Encrypt the seed using the master key
        // We encode the seed bytes as base64 string first to use the existing encrypt helper
        let seed_b64 = base64::engine::general_purpose::STANDARD.encode(seed.as_bytes());
        let encrypted_seed = self.with_key(|key| {
            crate::crypto::encrypt(&crate::crypto::secret::SecretString::from(seed_b64), key)
                .map_err(|e| BovedaError::CryptoError(e.to_string()))
        })??;

        // 3. Persist the encrypted seed (but don't enable yet)
        self.set_preference("totp_secret_cipher", &encrypted_seed).await?;

        // 4. Return the QR and URL for the frontend
        Ok(crate::auth::TotpSetupPayload {
            otpauth_url: crate::auth::TotpManager::get_otpauth_url(&seed),
            qr_png_b64: crate::auth::TotpManager::generate_qr_png_b64(&seed),
        })
    }

    /// Verifies a TOTP code against the persisted encrypted secret.
    pub async fn verify_totp(&self, code: &str) -> BovedaResult<bool> {
        self.check_unlocked()?;

        let cipher = self.get_preference("totp_secret_cipher").await?
            .ok_or_else(|| BovedaError::Other("TOTP no está configurado".to_string()))?;

        // Decrypt the seed (Base64 string)
        let seed_b64 = self.with_key(|key| {
            crate::crypto::decrypt(&cipher, key)
                .map_err(|e| BovedaError::CryptoError(e.to_string()))
        })??;

        // Decode base64 to raw bytes
        let seed_bytes = base64::engine::general_purpose::STANDARD.decode(seed_b64.as_str())
            .map_err(|e: base64::DecodeError| BovedaError::CryptoError(e.to_string()))?;
        
        let seed = crate::crypto::secret::SecretBytes::new(seed_bytes);
        let valid = crate::auth::TotpManager::verify(&seed, code);
        
        if valid {
            // Enable TOTP now that we know the user has verified it
            self.set_preference("totp_enabled", "true").await?;
        }

        Ok(valid)
    }

    /// Disables 2FA by removing the encrypted seed from storage.
    pub async fn disable_totp(&self) -> BovedaResult<()> {
        self.check_unlocked()?;

        let mut tx = self.db.begin().await?;
        
        // Remove both keys to ensure permanent destruction
        sqlx::query("DELETE FROM preferences WHERE key = 'totp_secret_cipher'")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM preferences WHERE key = 'totp_enabled'")
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    /// Returns true if TOTP 2FA is currently enabled.
    pub async fn is_totp_enabled(&self) -> BovedaResult<bool> {
        let enabled = self.get_preference("totp_enabled").await?;
        Ok(enabled.as_deref() == Some("true"))
    }

    // ─── Connection Management ─────────────────────────────────────────────────

    pub async fn close(&self) {
        self.db.close().await;
    }
}
