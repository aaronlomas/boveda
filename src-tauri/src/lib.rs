//! # Bóveda — Password Manager Backend
//!
//! This crate wires together the Tauri application.
//! The actual logic lives in focused modules:
//!
//! - [`crypto`]   — AES-256-GCM encryption + Argon2id key derivation
//! - [`db`]       — SQLite schema, queries, and migrations
//! - [`state`]    — Global `AppState` shared via Tauri's managed state
//! - [`commands::vault`]    — Tauri commands for vault unlock/lock and accounts
//! - [`commands::settings`] — Tauri commands for preferences, DB import/export, backgrounds

mod commands;
mod crypto;
mod db;
mod state;

use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use state::AppState;

// ─── Shared Data Types ───────────────────────────────────────────────────────
//
// These structs are serialized to JSON and sent to the Svelte frontend via
// Tauri's IPC bridge.

/// A decrypted account entry as returned to the frontend.
/// Passwords and notes are decrypted in memory before serialization —
/// they are never stored in plaintext on disk.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub site: String,
    pub username: String,
    pub password: String,
    pub notes: String,
    pub favicon_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ─── App Entry Point ─────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Resolve the database path in the OS-appropriate data directory:
    //   Linux:   ~/.local/share/boveda/vault.db
    //   macOS:   ~/Library/Application Support/boveda/vault.db
    //   Windows: %APPDATA%\boveda\vault.db
    let db_path = {
        let dir = dirs_next::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("boveda");
        std::fs::create_dir_all(&dir).ok();
        dir.join("vault.db")
    };

    // Initialize the async runtime and open the database connection pool
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    let pool = rt
        .block_on(async {
            let url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await?;
            db::init_db(&pool).await?;
            Ok::<_, anyhow::Error>(pool)
        })
        .expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(AppState::new(pool))
        .invoke_handler(tauri::generate_handler![
            // ── Vault & accounts ──────────────────────────────────────────
            commands::vault::is_vault_initialized,
            commands::vault::unlock_vault,
            commands::vault::lock_vault,
            commands::vault::add_account,
            commands::vault::get_accounts,
            commands::vault::delete_account,
            commands::vault::generate_password,
            // ── Preferences & storage ─────────────────────────────────────
            commands::settings::get_preference,
            commands::settings::set_preference,
            commands::settings::import_background_image,
            commands::settings::get_background_data_url,
            commands::settings::get_data_dir,
            commands::settings::export_db,
            commands::settings::import_db,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}