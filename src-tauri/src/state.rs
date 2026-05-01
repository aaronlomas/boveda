use std::sync::Mutex;
use zeroize::Zeroize;

/// Wrapper around the 256-bit AES-GCM master key.
/// Implements Zeroize so the key bytes are wiped from memory when dropped.
pub struct MasterKey(pub [u8; 32]);

impl Drop for MasterKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

/// Global application state shared through Tauri's managed state.
pub struct AppState {
    /// SQLite connection pool.
    pub db: sqlx::SqlitePool,
    /// The derived master key, present only when the vault is unlocked.
    pub master_key: Mutex<Option<MasterKey>>,
}

impl AppState {
    pub fn new(db: sqlx::SqlitePool) -> Self {
        Self {
            db,
            master_key: Mutex::new(None),
        }
    }
}
