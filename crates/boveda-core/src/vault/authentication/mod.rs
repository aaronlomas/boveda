// ─────────────────────────────────────────────────────────────────────────────
// 🔒 Lifecycle & Authentication
// ─────────────────────────────────────────────────────────────────────────────

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::str::FromStr;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use crate::crypto::secret::{SecretKey, SecretString};
use crate::crypto;
use crate::storage;
use crate::error::{BovedaError, BovedaResult};
use super::{BovedaEngine, MasterKey};

impl BovedaEngine {
    /// Returns true if the vault database or its salt file exists.
    pub fn is_initialized(db_path: &Path) -> bool {
        let salt_path = db_path.with_file_name("vault.salt");
        salt_path.exists() || db_path.exists()
    }

    pub fn is_locked(&self) -> bool {
        // SEC-H4: Handle potential lock poisoning gracefully
        self.master_key.lock()
            .map(|l| l.is_none())
            .unwrap_or_else(|e| {
                eprintln!("[WARNING] Master key lock is poisoned: {}. Vault is considered locked.", e);
                true // Safer default: consider vault locked if we can't acquire the lock
            })
    }

    /// Unlocks the vault. Derives key from salt and password, then opens encrypted database.
    /// On first use (no salt exists), generates and saves a new 32-byte salt.
    pub async fn unlock(db_path: &Path, password: &SecretString) -> BovedaResult<Self> {
        let salt_path = db_path.with_file_name("vault.salt");

        let salt = if salt_path.exists() {
            std::fs::read(&salt_path).map_err(|e| BovedaError::IoError(format!("Error al leer salt: {}", e)))?
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

        // Derive key from salt and password
        let key = crypto::derive_key(password, &salt)?;

        let engine = Self::open_encrypted(db_path, &key).await
            .map_err(|_| BovedaError::InvalidPassword)?;

        // Verify the key by initializing schema (if key is wrong, this will fail)
        storage::init_db(&engine.db).await.map_err(|_| BovedaError::InvalidPassword)?;

        // Store the master key in the engine
        {
            let mut key_lock = engine.master_key.lock()
                .map_err(|_| BovedaError::Other("Vault lock poisoned".to_string()))?;
            *key_lock = Some(MasterKey::new(key));
        }

        // Check remote connection setting
        if let Ok(Some(pref)) = engine.get_preference("security.block_remote").await {
            if pref == "true" {
                if crate::security::environment_check() {
                    let _ = engine.log_audit(crate::audit::AuditAction::VaultUnlock, Some("Blocked: Remote Session Detected")).await;
                    engine.lock(); // Lock it back immediately
                    return Err(BovedaError::RemoteSessionDetected);
                } else {
                    // Spawn a background task to monitor for remote sessions actively
                    let master_key_clone = Arc::clone(&engine.master_key);
                    tokio::spawn(async move {
                        loop {
                            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                            
                            // Stop monitoring if the vault is already locked
                            let is_locked = {
                                if let Ok(lock) = master_key_clone.lock() {
                                    lock.is_none()
                                } else {
                                    true // Poisoned, consider locked
                                }
                            };
                            
                            if is_locked {
                                break;
                            }
                            
                            // If a remote session is detected while unlocked, lock immediately
                            if crate::security::environment_check() {
                                if let Ok(mut lock) = master_key_clone.lock() {
                                    *lock = None;
                                }
                                break;
                            }
                        }
                    });
                }
            }
        }

        // SOC2: Log successful unlock
        let _ = engine.log_audit(crate::audit::AuditAction::VaultUnlock, Some("Success")).await;

        Ok(engine)
    }

    /// Opens an **unencrypted** SQLite database. **FOR TESTING ONLY**.
    /// Should not be used in production. Exists to support unit tests with in-memory databases.
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
        
        // SEC-C4: `pragma_key` only contains safe hexadecimal characters inside `x'...'`.
        // This is safe from SQL injection, even though sqlx concatenates PRAGMA values.
        //
        // SEC-C5: All KDF/cipher parameters are pinned explicitly to avoid relying on
        // SQLCipher compiled-in defaults, which may vary across versions and platforms.
        // Values follow SQLCipher 4.x best-practice recommendations:
        //   - cipher_kdf_algorithm  : PBKDF2-HMAC-SHA512 (stronger than default SHA1)
        //   - cipher_hmac_algorithm : HMAC-SHA512 (integrity of each page)
        //   - cipher_page_size      : 4096 bytes  (performance + security balance)
        //   - kdf_iter              : 256 000     (OWASP minimum for PBKDF2-SHA512)
        options = options
            .pragma("key", pragma_key.as_str().to_string())
            .pragma("cipher_use_hmac", "ON")
            .pragma("cipher_plaintext_header_size", "0")
            .pragma("cipher_kdf_algorithm", "PBKDF2_HMAC_SHA512")
            .pragma("cipher_hmac_algorithm", "HMAC_SHA512")
            .pragma("cipher_page_size", "4096")
            .pragma("kdf_iter", "256000")
            .pragma("secure_delete", "ON")
            .pragma("journal_mode", "WAL");

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
        if let Ok(mut lock) = self.master_key.lock() {
            *lock = None;
        }
    }

    /// Logs an action to the persistent audit log.
    pub async fn log_audit(&self, action: crate::audit::AuditAction, metadata: Option<&str>) -> BovedaResult<()> {
        crate::storage::add_audit_log(&self.db, action.as_str(), metadata).await
    }

    /// Internal helper to check if unlocked and return error if not.
    pub(crate) fn check_unlocked(&self) -> BovedaResult<()> {
        if self.is_locked() {
            Err(BovedaError::VaultLocked)
        } else {
            Ok(())
        }
    }

    /// Internal helper to execute a closure with the master key if unlocked.
    pub(crate) fn with_key<F, R>(&self, f: F) -> BovedaResult<R>
    where
        F: FnOnce(&SecretKey) -> R,
    {
        let lock = self.master_key.lock()
            .map_err(|_| BovedaError::Other("Vault lock poisoned".to_string()))?;
            
        lock.as_ref()
            .map(|mk| {
                // Temporary reconstruct SecretKey from the safe boxed memory
                let sk = SecretKey::new(*mk.as_bytes());
                f(&sk)
            })
            .ok_or(BovedaError::VaultLocked)
    }
}