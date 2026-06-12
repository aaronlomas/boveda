//! # Bóveda — Password Manager Backend
//!
//! Este crate conecta la aplicación Tauri. La lógica real vive en `boveda-core`:
//!
//! - [`boveda_core::commands`] — Facade de comandos framework-agnostico
//! - [`boveda_core::crypto`]  — Cifrado ChaCha20-Poly1305 + Argon2id
//! - [`boveda_core::vault`]   — Motor principal del baúl
//! - [`state`]                — Re-exporta `AppState` desde boveda-core

mod commands;
mod state;


// ─── App Entry Point ─────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Resuelve la ruta de la base de datos en el directorio de datos del SO:
    //   Linux:   ~/.local/share/boveda/vault.bvda
    //   macOS:   ~/Library/Application Support/boveda/vault.bvda
    //   Windows: %APPDATA%\boveda\vault.bvda
    let db_path = {
        let dir = dirs_next::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("boveda");
        std::fs::create_dir_all(&dir).ok();

        let old_db = dir.join("vault.db");
        let new_db = dir.join("vault.bvda");

        // Migración transparente: si existe la DB antigua y no la nueva, renombrarla.
        if old_db.exists() && !new_db.exists() {
            if let Err(e) = std::fs::rename(&old_db, &new_db) {
                eprintln!("Warning: Failed to migrate vault.db to vault.bvda: {}", e);
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
            commands::vault::delete_vault,
            commands::vault::add_account,
            commands::vault::get_accounts,
            commands::vault::delete_account,
            commands::vault::generate_password,
            commands::vault::decrypt_secret,
            // ── Groups ────────────────────────────────────────────────────
            commands::vault::update_account_group,
            commands::vault::rename_group,
            commands::vault::delete_group,
            // ── PINs ──────────────────────────────────────────────────────
            commands::vault::add_pin,
            commands::vault::get_pins,
            commands::vault::delete_pin,
            // ── Documents ─────────────────────────────────────────────────
            commands::vault::add_document,
            commands::vault::get_documents,
            commands::vault::update_document,
            commands::vault::delete_document,
            commands::vault::decrypt_document_content,
            commands::vault::get_crypto_params,
            // ── Preferences & storage ─────────────────────────────────────
            commands::settings::get_preference,
            commands::settings::set_preference,
            commands::settings::import_background_image,
            commands::settings::get_background_data_url,
            commands::settings::get_data_dir,
            commands::settings::read_external_file,
            commands::settings::get_app_info,
            commands::settings::get_os_username,
            commands::settings::export_db,
            commands::settings::import_db,
            commands::settings::export_secure_package,
            commands::settings::import_secure_package,
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