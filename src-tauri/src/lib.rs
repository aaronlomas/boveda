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

mod state;

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
        
        let old_db = dir.join("vault.db");
        let new_db = dir.join("vault.bvda");

        // Transparent migration: if the old DB exists but the new one doesn't, rename it.
        if old_db.exists() && !new_db.exists() {
            if let Err(e) = std::fs::rename(&old_db, &new_db) {
                eprintln!("Warning: Failed to migrate vault.db to vault.bvda: {}", e);
                // Fallback to old_db if rename fails due to permissions, though rare.
                old_db
            } else {
                new_db
            }
        } else {
            new_db
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(state::AppState::new(db_path))
        .invoke_handler(tauri::generate_handler![
            // ── Vault & accounts ──────────────────────────────────────────
            commands::vault::is_vault_initialized,
            commands::vault::unlock_vault,
            commands::vault::lock_vault,
            commands::vault::add_account,
            commands::vault::get_accounts,
            commands::vault::delete_account,
            commands::vault::generate_password,
            commands::vault::decrypt_secret,
            // ── Groups ────────────────────────────────────────────────────
            commands::vault::update_account_group,
            commands::vault::rename_group,
            commands::vault::delete_group,
            // ── Preferences & storage ─────────────────────────────────────
            commands::settings::get_preference,
            commands::settings::set_preference,
            commands::settings::import_background_image,
            commands::settings::get_background_data_url,
            commands::settings::get_data_dir,
            commands::settings::export_db,
            commands::settings::import_db,
            // ── Security ──────────────────────────────────────────────────
            commands::security::totp_is_enabled,
            commands::security::totp_setup,
            commands::security::totp_verify_setup,
            commands::security::totp_check,
            commands::security::totp_recovery_check,
            commands::security::totp_disable,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}