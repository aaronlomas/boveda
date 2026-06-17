//! # Boveda Core — Command Facade
//!
//! [`AppState`]. framework-agnostico
//!
//! ```rust,ignore
//! #[tauri::command]
//! pub async fn unlock_vault(password: SecretString, state: State<'_, AppState>) -> Result<String, String> {
//!     state.cmd_unlock_vault(password).await
//! }
//! ```

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::crypto;
use crate::crypto::secret::SecretString;
use crate::vault::BovedaEngine;

mod account;
mod group;
mod pin;
mod export_import;
mod audit_log;
pub use audit_log::AuditLogView;
mod preferences;
mod utilities;
mod totp;
mod document;
mod shell;

// ─── AppState ─────────────────────────────────────────────────────────────────

/// shared global state among all command handlers.
#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Mutex<Option<BovedaEngine>>>,
    pub session_verified: Arc<Mutex<bool>>,
    pub db_path: PathBuf,
}

impl AppState {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            engine: Arc::new(Mutex::new(None)),
            session_verified: Arc::new(Mutex::new(false)),
            db_path,
        }
    }

    /// Returns the engine if the session is verified.
    fn get_engine(&self) -> Result<BovedaEngine, String> {
        let session_ok = *self.session_verified.lock().unwrap_or_else(|e| e.into_inner());
        if !session_ok {
            return Err("Session not verified. TOTP authentication required.".to_string());
        }
        let lock = self.engine.lock()
            .map_err(|e| format!("Vault lock poisoned: {}. Please restart the application.", e))?;
        lock.as_ref().cloned().ok_or_else(|| "Vault is locked".to_string())
    }

    /// Devuelve el engine sin requerir verificación de sesión.
    fn get_engine_unverified(&self) -> Result<BovedaEngine, String> {
        let lock = self.engine.lock()
            .map_err(|e| format!("Vault lock poisoned: {}. Please restart the application.", e))?;
        lock.as_ref().cloned().ok_or_else(|| "Vault is locked".to_string())
    }

    fn app_data_dir() -> PathBuf {
        dirs_next::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("boveda")
    }

    pub fn vault_db_path() -> PathBuf {
        Self::app_data_dir().join("vault.bvda")
    }

    // Vault lifecycle ---------------------------------------------------------

    pub fn cmd_is_vault_initialized(&self) -> bool {
        BovedaEngine::is_initialized(&self.db_path)
    }

    /// Unlocks the vault and returns either `"totp_required"` or `"unlocked"`.
    pub async fn cmd_unlock_vault(&self, password: SecretString) -> Result<String, String> {
        let lock_file = Self::app_data_dir().join(".vault_unlock_lock");
        let failed_unlock_str = std::fs::read_to_string(&lock_file)
            .ok()
            .and_then(|content| content.lines().next().map(|s| s.to_string()))
            .unwrap_or_else(|| "0:0".to_string());
        
        let parts: Vec<&str> = failed_unlock_str.split(':').collect();
        let failed_attempts = parts.first().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        let last_fail_ts = parts.get(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        
        if failed_attempts >= 10 {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            
            if now - last_fail_ts < 3600 {
                return Err(format!(
                    "Too many failed unlock attempts. Please try again in {} seconds.",
                    3600 - (now - last_fail_ts)
                ));
            }
        }
        
        let engine = match BovedaEngine::unlock(&self.db_path, &password).await {
            Ok(e) => {
                let _ = std::fs::remove_file(&lock_file);
                e
            }
            Err(e) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64;
                let new_attempts = (failed_attempts + 1).to_string();
                let _ = std::fs::write(&lock_file, format!("{}:{}", new_attempts, now));
                eprintln!("[SECURITY] Failed Bóveda unlock attempt: {}", e);
                return Err(e.to_string());
            }
        };

        let is_totp = engine.is_totp_enabled().await.unwrap_or(false);

        let mut engine_lock = self.engine.lock()
            .map_err(|e| format!("Vault lock poisoned: {}", e))?;
        *engine_lock = Some(engine);

        *self.session_verified.lock().unwrap() = !is_totp;

        if is_totp {
            Ok("totp_required".to_string())
        } else {
            Ok("unlocked".to_string())
        }
    }

    pub fn cmd_lock_vault(&self) {
        if let Ok(mut lock) = self.engine.lock() {
            if let Some(engine) = lock.as_ref() {
                engine.lock();
            }
            *lock = None;
        } else {
            eprintln!("[WARNING] Failed to acquire lock when locking vault. The engine may be in an inconsistent state.");
        }
        if let Ok(mut session) = self.session_verified.lock() {
            *session = false;
        }
    }

    /// Returns true if the vault is locked.
    ///
    /// Checks both the outer engine presence in AppState and the inner master key
    /// inside BovedaEngine, so that background guards that clear only the key
    /// are correctly reflected.
    pub fn is_locked(&self) -> bool {
        self.engine
            .lock()
            .map(|guard| match guard.as_ref() {
                None => true,
                Some(engine) => engine.is_locked(),
            })
            .unwrap_or(true)
    }

    /// Deletes the vault files if the password is correct.
    pub async fn cmd_delete_vault(&self, password: SecretString) -> Result<(), String> {
        let engine = BovedaEngine::unlock(&self.db_path, &password).await.map_err(|e| e.to_string())?;
        engine.close().await;
        self.cmd_lock_vault();

        let salt_path = self.db_path.with_file_name("vault.salt");
        let _ = std::fs::remove_file(&self.db_path);
        let _ = std::fs::remove_file(&salt_path);

        Ok(())
    }

    /// Generates a random password.
    pub fn cmd_generate_password(length: usize, use_symbols: bool) -> Result<String, String> {
        let len = length.clamp(8, 128);
        crypto::generate_password(len, use_symbols)
            .map(|s| s.as_str().to_string())
            .map_err(|e| e.to_string())
    }

    /// Decrypts a single secret field.
    pub async fn cmd_decrypt_secret(&self, ciphertext: &str) -> Result<String, String> {
        let engine = self.get_engine()?;
        let _ = engine.log_audit(crate::audit::AuditAction::SecretAccess, Some(ciphertext)).await;

        engine
            .decrypt_secret(ciphertext)
            .map(|s: SecretString| s.as_str().to_string())
            .map_err(|e: crate::BovedaError| e.to_string())
    }
}
