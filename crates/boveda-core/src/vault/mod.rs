use std::sync::{Arc, Mutex};

use sqlx::SqlitePool;
use crate::crypto::secret::SecretKey;
use crate::storage;
use crate::error::BovedaResult;

pub mod account;
pub mod document;
pub mod group;
pub mod pin;
pub mod validation;
pub mod totp;
pub mod export;
pub mod authentication;
pub mod export_import;

// ─────────────────────────────────────────────────────────────────────────────
// Core Structures
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ImportStrategy {
    Merge,
    Replace,
}

/// ChaCha20-Poly1305 master key wrapper.
///
/// The inner `SecretKey` is heap-allocated inside a `Box` so that its address
/// remains stable and can be pinned with `mlock`/`VirtualLock`.
pub struct MasterKey(Box<SecretKey>);

impl MasterKey {
    pub fn new(key: SecretKey) -> Self {
        let boxed = Box::new(key);

        #[cfg(unix)]
        unsafe {
            let ptr = boxed.as_bytes().as_ptr() as *const libc::c_void;
            let _ = libc::mlock(ptr, 32);
        }
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualLock;
            let ptr = boxed.as_bytes().as_ptr() as *const std::ffi::c_void;
            let _ = VirtualLock(ptr, 32);
        }
        Self(boxed)
    }

    /// Returns a reference to the confined key without copying any bytes.
    pub fn as_secret_key(&self) -> &SecretKey {
        &self.0
    }
}

impl Drop for MasterKey {
    fn drop(&mut self) {
        #[cfg(unix)]
        unsafe {
            let ptr = self.0.as_bytes().as_ptr() as *const libc::c_void;
            let _ = libc::munlock(ptr, 32);
        }
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualUnlock;
            let ptr = self.0.as_bytes().as_ptr() as *const std::ffi::c_void;
            let _ = VirtualUnlock(ptr, 32);
        }
        // SecretKey::drop handles zeroization via its own Drop impl.
    }
}

/// SQLite-backed vault engine with encrypted master key.
#[derive(Clone)]
pub struct BovedaEngine {
    pub(crate) db: SqlitePool,
    pub(crate) master_key: Arc<Mutex<Option<MasterKey>>>,
}

// Preferences & Settings
// ─────────────────────────────────────────────────────────────────────────────

impl BovedaEngine {
    pub async fn get_preference(&self, key: &str) -> BovedaResult<Option<String>> {
        self.check_unlocked()?;
        storage::get_preference(&self.db, key).await
    }

    pub async fn set_preference(&self, key: &str, value: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(key, "Preference", validation::MAX_PREF_KEY_LEN, true)?;
        validation::validate_string(value, "Preference value", validation::MAX_PREF_VALUE_LEN, false)?;
        storage::set_preference(&self.db, key, value).await
    }

    pub async fn delete_preference(&self, key: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(key, "Preference", validation::MAX_PREF_KEY_LEN, true)?;
        storage::delete_preference(&self.db, key).await
    }
}