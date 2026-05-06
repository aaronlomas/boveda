use std::sync::{Arc, Mutex};
use zeroize::Zeroize;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use std::str::FromStr;
use std::path::PathBuf;
use crate::secret::SecretString;
use crate::{crypto, db};
use anyhow::{anyhow, Result};

/// Wrapper around the 256-bit AES-GCM master key.
/// Allocates key on the heap, and uses mlock/VirtualLock to prevent swapping to disk.
pub struct MasterKey(Box<[u8; 32]>);

impl MasterKey {
    pub fn new(key: [u8; 32]) -> Self {
        let boxed = Box::new(key);
        #[cfg(unix)]
        unsafe {
            let ptr = boxed.as_ptr() as *const libc::c_void;
            if libc::mlock(ptr, std::mem::size_of::<[u8; 32]>()) != 0 {
                eprintln!("Warning: Failed to mlock master key memory");
            }
        }
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualLock;
            let ptr = boxed.as_ptr() as *const std::ffi::c_void;
            if VirtualLock(ptr, std::mem::size_of::<[u8; 32]>()) == 0 {
                eprintln!("Warning: Failed to VirtualLock master key memory");
            }
        }
        Self(boxed)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Drop for MasterKey {
    fn drop(&mut self) {
        self.0.zeroize();
        #[cfg(unix)]
        unsafe {
            let ptr = self.0.as_ptr() as *const libc::c_void;
            libc::munlock(ptr, std::mem::size_of::<[u8; 32]>());
        }
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualUnlock;
            let ptr = self.0.as_ptr() as *const std::ffi::c_void;
            VirtualUnlock(ptr, std::mem::size_of::<[u8; 32]>());
        }
    }
}

/// Core engine holding the database connection and the unlocked master key.
#[derive(Clone)]
pub struct BovedaEngine {
    /// SQLite connection pool.
    pub db: SqlitePool,
    /// The derived master key, present only when the vault is unlocked.
    pub master_key: Arc<Mutex<Option<MasterKey>>>,
}

impl BovedaEngine {
    /// Returns true if the vault database or its salt file exists.
    pub fn is_initialized(db_path: &PathBuf) -> bool {
        let salt_path = db_path.with_file_name("vault.salt");
        salt_path.exists() || db_path.exists()
    }

    /// High-level method to unlock the vault.
    /// Handles salt detection, migration, key derivation, and database opening.
    pub async fn unlock(db_path: &PathBuf, password: &SecretString) -> Result<Self> {
        let salt_path = db_path.with_file_name("vault.salt");
        let mut is_legacy_migration = false;

        let salt = if salt_path.exists() {
            std::fs::read(&salt_path).map_err(|e| anyhow!("Failed to read salt: {}", e))?
        } else if db_path.exists() {
            // Unencrypted database exists, need migration
            is_legacy_migration = true;
            vec![] // placeholder
        } else {
            // First time initialization: generate new salt
            use rand::RngCore;
            let mut new_salt = vec![0u8; 32];
            rand::rngs::OsRng.fill_bytes(&mut new_salt);
            std::fs::write(&salt_path, &new_salt).map_err(|e| anyhow!("Failed to write salt: {}", e))?;
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
        let mut hex_key_str = String::with_capacity(64);
        for b in key.iter() {
            use std::fmt::Write;
            write!(&mut hex_key_str, "{:02x}", b).unwrap();
        }
        let hex_key = SecretString::from(hex_key_str);

        let engine = Self::open_encrypted(db_path, &hex_key).await
            .map_err(|_| anyhow!("Contraseña incorrecta o archivo dañado"))?;

        // Verify the key by initializing schema (if key is wrong, this will fail)
        db::init_db(&engine.db).await.map_err(|_| anyhow!("Contraseña incorrecta"))?;

        // Store the master key in the engine
        {
            let mut key_lock = engine.master_key.lock().unwrap();
            *key_lock = Some(MasterKey::new(*key));
        }

        Ok(engine)
    }

    /// Internal helper to handle legacy migration flow.
    async fn unlock_legacy_migration(db_path: &PathBuf, password: &SecretString) -> Result<Self> {
        let unencrypted_engine = Self::open_unencrypted(db_path).await?;
        let meta = db::get_vault_meta(&unencrypted_engine.db).await?
            .ok_or_else(|| anyhow!("Legacy vault has no metadata"))?;
        
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
            let accounts = db::get_accounts(&unencrypted_engine.db).await.unwrap_or_default();
            if let Some(acc) = accounts.first() {
                if crypto::decrypt(&acc.encrypted_password, &key).is_err() {
                    return Err(anyhow!("Contraseña incorrecta"));
                }
                verified = true;
            } else {
                // No accounts and no challenge? Assume verified for migration
                verified = true;
            }
        }

        if !verified {
            return Err(anyhow!("Contraseña incorrecta"));
        }

        let mut hex_key_str = String::with_capacity(64);
        for b in key.iter() {
            use std::fmt::Write;
            write!(&mut hex_key_str, "{:02x}", b).unwrap();
        }
        let hex_key = SecretString::from(hex_key_str);

        // Perform migration
        db::migrate_to_sqlcipher(&unencrypted_engine.db, &hex_key, db_path).await?;

        // Open newly encrypted database
        let engine = Self::open_encrypted(db_path, &hex_key).await?;
        {
            let mut key_lock = engine.master_key.lock().unwrap();
            *key_lock = Some(MasterKey::new(*key));
        }

        Ok(engine)
    }

    /// Opens the database without a key (useful for initial migration check).
    pub async fn open_unencrypted(db_path: &PathBuf) -> Result<Self> {
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

    /// Opens the database utilizing SQLCipher with the derived key.
    pub async fn open_encrypted(db_path: &PathBuf, hex_key: &SecretString) -> Result<Self> {
        let url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());
        let mut options = SqliteConnectOptions::from_str(&url)?;
        
        // Send the PRAGMA key right upon connecting
        let pragma_key = format!("\"x'{}'\"", hex_key.as_str());
        let pragma_key_secret = SecretString::new(pragma_key);
        
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

    /// Internal helper to execute a closure with the master key if unlocked.
    fn with_key<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&[u8; 32]) -> R,
    {
        let lock = self.master_key.lock().unwrap();
        lock.as_ref()
            .map(|mk| f(mk.as_bytes()))
            .ok_or_else(|| anyhow!("Vault is locked"))
    }

    /// Retrieves and decrypts all accounts.
    pub async fn get_accounts(&self) -> Result<Vec<crate::models::Account>> {
        let rows = db::get_accounts(&self.db).await?;
        
        let mut accounts = Vec::with_capacity(rows.len());
        for row in rows {
            let (dec_site, dec_username) = self.with_key(|key| {
                let s = crypto::decrypt(&row.site, key).unwrap_or_else(|_| SecretString::from(row.site.clone()));
                let u = crypto::decrypt(&row.username, key).unwrap_or_else(|_| SecretString::from(row.username.clone()));
                (s, u)
            })?;
            
            accounts.push(crate::models::Account {
                id: row.id,
                site: dec_site.as_str().to_string(),
                username: dec_username.as_str().to_string(),
                password_cipher: row.encrypted_password,
                notes_cipher: row.encrypted_notes,
                favicon_url: None, // Google Favicon fetch disabled for privacy
                group_name: row.group_name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        // Sort by site
        accounts.sort_by(|a, b| a.site.to_lowercase().cmp(&b.site.to_lowercase()));
        Ok(accounts)
    }

    /// Adds a new account, encrypting sensitive fields automatically.
    pub async fn add_account(
        &self,
        site: &str,
        username: &str,
        password: &str,
        notes: Option<&str>,
    ) -> Result<String> {
        let (enc_site, enc_username, enc_password, enc_notes) = self.with_key(|key| {
            let s = crypto::encrypt(&SecretString::from(site), key)?;
            let u = crypto::encrypt(&SecretString::from(username), key)?;
            let p = crypto::encrypt(&SecretString::from(password), key)?;
            let n = notes.map(|n| crypto::encrypt(&SecretString::from(n), key)).transpose()?;
            Ok::<_, anyhow::Error>((s, u, p, n))
        })??;

        db::add_account(
            &self.db,
            &enc_site,
            &enc_username,
            &enc_password,
            enc_notes.as_deref(),
            None,
        ).await
    }

    /// Decrypts a single ciphertext on-demand.
    pub fn decrypt_secret(&self, ciphertext: &str) -> Result<SecretString> {
        self.with_key(|key| crypto::decrypt(ciphertext, key))?
    }

    /// Deletes an account by ID.
    pub async fn delete_account(&self, id: &str) -> Result<()> {
        db::delete_account(&self.db, id).await
    }

    // ─── Group Management ─────────────────────────────────────────────────────

    pub async fn update_account_group(&self, id: &str, group_name: Option<&str>) -> Result<()> {
        db::update_account_group(&self.db, id, group_name).await
    }

    pub async fn rename_group(&self, old_name: &str, new_name: &str) -> Result<()> {
        db::rename_group(&self.db, old_name, new_name).await
    }

    pub async fn delete_group(&self, name: &str) -> Result<()> {
        db::delete_group(&self.db, name).await
    }
}



