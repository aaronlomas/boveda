//! # Shell Query Interface
//!
//! Note: The engine does not recognize terminals or UIs. It only returns `Vec<String>`.

use crate::commands::AppState;
use crate::crypto;
use crate::storage::AUDIT_LOG_MAX_ROWS;

impl AppState {
    /// Processes a plain text command and returns response lines.
    /// This method is the only entry point for the Bóveda CLI.
    pub async fn cmd_query_shell(&self, input: &str) -> Vec<String> {
        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();
        let args = parts.get(1).copied().unwrap_or("").trim();

        match command.as_str() {
            "help" => vec![
                "┌─ boveda-core CLI ─────────────────────────────────────────┐".into(),
                "│  crypto       Show active cryptographic parameters        │".into(),
                "│  status       Show vault lock state and session info      │".into(),
                "│  audit        Show last 10 audit log entries              │".into(),
                "│  audit -n <N> Show last N audit log entries               │".into(),
                "│  audit -rm    Clear all audit log entries                 │".into(),
                "│  clear        Clear terminal output                       │".into(),
                "│  help         Show this help message                      │".into(),
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

            "audit" => match args {
                "-rm" => match self.cmd_clear_audit_logs().await {
                    Ok(()) => vec![
                        "── Audit Log ────────────────────────────────────────────────".into(),
                        "  All audit log entries have been removed.".into(),
                        format!("  Retention cap: {} rows (automatic).", AUDIT_LOG_MAX_ROWS),
                        "────────────────────────────────────────────────────────────".into(),
                    ],
                    Err(e) => vec![format!("audit -rm: error: {}", e)],
                },

                flag if flag.starts_with("-n ") => {
                    let n_str = flag.trim_start_matches("-n ").trim();
                    match n_str.parse::<i64>() {
                        Ok(n) if n > 0 => self.shell_show_audit_logs(n).await,
                        _ => vec![
                            format!("audit -n: invalid argument '{}' — expected a positive integer.", n_str),
                            "  Usage: audit -n <N>".into(),
                        ],
                    }
                }

                "" => self.shell_show_audit_logs(10).await,

                unknown => vec![
                    format!("audit: unknown flag '{}'", unknown),
                    "  Usage: audit | audit -n <N> | audit -rm".into(),
                ],
            },

            "" => vec![],

            unknown => vec![
                format!("boveda-core: command not found: '{}'", unknown),
                "  Type 'help' to see available commands.".into(),
            ],
        }
    }

    /// Renders the last `limit` audit log entries as terminal output lines.
    async fn shell_show_audit_logs(&self, limit: i64) -> Vec<String> {
        match self.cmd_get_audit_logs(limit).await {
            Ok(logs) => {
                let mut output = vec![
                    format!("── Audit Log (last {}) ──────────────────────────────────────", limit),
                ];
                if logs.is_empty() {
                    output.push("  No audit log entries found.".into());
                } else {
                    for log in logs {
                        let meta = log.metadata.unwrap_or_else(|| "N/A".into());
                        output.push(format!("  [{}] {} — {}", log.created_at, log.action, meta));
                    }
                }
                output.push(format!(
                    "  Retention cap: {} rows.",
                    AUDIT_LOG_MAX_ROWS
                ));
                output.push("────────────────────────────────────────────────────────────".into());
                output
            }
            Err(e) => vec![format!("audit: error fetching entries: {}", e)],
        }
    }
}
