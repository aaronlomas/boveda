pub mod crypto;
pub mod storage;
pub mod vault;
pub mod error;

#[cfg(test)]
mod tests;

pub use vault::BovedaEngine;
pub use vault::MasterKey;
pub use storage::models::Account;
pub use crypto::secret::{SecretKey, SecretString};
pub use error::{BovedaError, BovedaResult};

/// Applies OS-level anti-forensic protections to the current process.
/// Disables core dumps on Linux to prevent memory from being written to disk upon crash.
pub fn harden_process() {
    #[cfg(target_os = "linux")]
    unsafe {
        if libc::prctl(libc::PR_SET_DUMPABLE, 0) != 0 {
            eprintln!("Warning: Failed to disable core dumps via PR_SET_DUMPABLE");
        }
    }
}
