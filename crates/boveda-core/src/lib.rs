pub mod crypto;
pub mod db;
pub mod models;
pub mod engine;
pub mod secret;

pub use engine::{BovedaEngine, MasterKey};
pub use models::Account;
pub use secret::{SecretBytes, SecretString};

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
