//! # Shell Query Interface
//!
//! Note: The engine does not recognize terminals or UIs. It only returns `Vec<String>`.

use crate::commands::AppState;
use crate::crypto;

impl AppState {
    /// Processes a plain text command and returns response lines.
    /// This method is the only entry point for the Bóveda CLI.
    pub fn cmd_query_shell(&self, input: &str) -> Vec<String> {
        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();
        let _args = parts.get(1).copied().unwrap_or("");

        match command.as_str() {
            "help" => vec![
                "┌─ boveda-core CLI ─────────────────────────────────────────┐".into(),
                "│  crypto    Show active cryptographic parameters           │".into(),
                "│  status    Show vault lock state and session info         │".into(),
                "│  audit     Show last 10 audit log entries (async)         │".into(),
                "│  clear     Clear terminal output                          │".into(),
                "│  help      Show this help message                         │".into(),
                "└───────────────────────────────────────────────────────────┘".into(),
            ],

            "crypto" => vec![
                "── Cryptographic Parameters ────────────────────────────────".into(),
                format!("  KDF Algorithm    : Argon2id (RFC 9106)"),
                format!("  Memory Cost (m)  : {} KiB  ({} MiB)", crypto::ARGON2_M_COST, crypto::ARGON2_M_COST / 1024),
                format!("  Time Cost   (t)  : {} iterations", crypto::ARGON2_T_COST),
                format!("  Parallelism (p)  : {} lanes", crypto::ARGON2_P_COST),
                format!("  Output Length    : 32 bytes (256-bit key)"),
                "".into(),
                format!("  AEAD Cipher      : ChaCha20-Poly1305 (RFC 8439)"),
                format!("  Nonce Length     : {} bytes (96-bit random)", crypto::NONCE_LEN),
                format!("  Auth Tag Length  : {} bytes (128-bit MAC)", crypto::TAG_LEN),
                format!("  Key Length       : 32 bytes (256-bit)"),
                "────────────────────────────────────────────────────────────".into(),
            ],

            "status" => {
                let locked = self.is_locked();
                let session_ok = *self.session_verified.lock().unwrap_or_else(|e| e.into_inner());
                let vault_state = if locked { "LOCKED" } else { "UNLOCKED" };
                let session_state = if session_ok { "VERIFIED" } else if !locked { "PENDING_TOTP" } else { "N/A" };

                vec![
                    "── Bóveda Status ────────────────────────────────────────────".into(),
                    format!("  Bóveda State      : {}", vault_state),
                    format!("  Session          : {}", session_state),
                    format!("  DB Path          : {}", self.db_path.display()),
                    "────────────────────────────────────────────────────────────".into(),
                ]
            }

            "" => vec![],

            unknown => vec![
                format!("boveda-core: command not found: '{}'", unknown),
                "  Type 'help' to see available commands.".into(),
            ],
        }
    }
}
