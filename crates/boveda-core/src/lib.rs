pub mod crypto;
pub mod storage;
pub mod vault;
pub mod error;
pub mod auth;
pub mod audit;
pub mod commands;

#[cfg(test)]
mod tests;

pub use vault::BovedaEngine;
pub use vault::MasterKey;
pub use storage::models::{Account, Pin};
pub use crypto::secret::{SecretKey, SecretString};
pub use error::{BovedaError, BovedaResult};
pub use commands::AppState;

/// Applies OS-level anti-forensic protections to the current process.
pub fn harden_process() {
    #[cfg(target_os = "linux")]
    unsafe {
        if libc::prctl(libc::PR_SET_DUMPABLE, 0) != 0 {
            eprintln!("Warning: Failed to disable core dumps via PR_SET_DUMPABLE");
        }
    }

    #[cfg(target_os = "windows")]
    unsafe {
        // SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX
        // Prevents the system from displaying critical-error-handler and Windows Error Reporting dialogs.
        use windows_sys::Win32::System::Diagnostics::{SetErrorMode, SEM_FAILCRITICALERRORS, SEM_NOGPFAULTERRORBOX};
        SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);
    }
}
