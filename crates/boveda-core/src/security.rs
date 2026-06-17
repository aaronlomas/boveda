#[cfg(target_os = "linux")]
use std::env;

/// Checks if the current session is a remote connection.
/// Returns `true` if a remote session is detected (e.g. SSH, RDP, etc.).
pub fn environment_check() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            // GetSystemMetrics(SM_REMOTESESSION)
            // SM_REMOTESESSION = 0x1000
            use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_REMOTESESSION};
            if GetSystemMetrics(SM_REMOTESESSION) != 0 {
                return true;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 1. Check common SSH/Remote environment variables
        let remote_vars = ["SSH_CLIENT", "SSH_CONNECTION", "SSH_TTY", "REMOTEHOST"];
        for var in remote_vars.iter() {
            if env::var(var).is_ok() {
                return true;
            }
        }

        // 2. Check systemd-logind via dlsym to avoid hard dependency on libsystemd.so
        unsafe {
            let handle = libc::dlopen(c"libsystemd.so.0".as_ptr() as *const _, libc::RTLD_LAZY);
            if !handle.is_null() {
                let func = libc::dlsym(handle, c"sd_session_is_remote".as_ptr() as *const _);
                if !func.is_null() {
                    let sd_session_is_remote: extern "C" fn(*const libc::c_char) -> libc::c_int = std::mem::transmute(func);
                    // Pass NULL to check the current session
                    let result = sd_session_is_remote(std::ptr::null());
                    libc::dlclose(handle);
                    if result > 0 {
                        return true;
                    }
                } else {
                    libc::dlclose(handle);
                }
            }
        }

        // 3. Detect remote desktop software processes (AnyDesk, TeamViewer, VNC, etc.)
        if let Ok(entries) = std::fs::read_dir("/proc") {
            let remote_procs = ["anydesk", "teamviewer", "rustdesk", "xrdp", "vnc", "remmina"];
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let file_name = entry.file_name();
                        let file_name_str = file_name.to_string_lossy();
                        if file_name_str.chars().all(|c| c.is_ascii_digit()) {
                            let mut comm_path = entry.path();
                            comm_path.push("comm");
                            if let Ok(comm) = std::fs::read_to_string(comm_path) {
                                let comm_lower = comm.trim().to_lowercase();
                                for rp in &remote_procs {
                                    if comm_lower.contains(rp) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    false
}
