use crate::state::AppState;
use boveda_core::vault::ImportStrategy;
use tauri::{Emitter, State};

// ─── User Preferences ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_preference(
    key: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    state.cmd_get_preference(&key).await
}

#[tauri::command]
pub async fn set_preference(
    key: String,
    value: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.cmd_set_preference(&key, value).await
}

// ─── Background Image ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn import_background_image(src_path: String) -> Result<String, String> {
    AppState::cmd_import_background_image(&src_path).await
}

#[tauri::command]
pub fn get_background_data_url(filename: String) -> Result<String, String> {
    AppState::cmd_get_background_data_url(&filename)
}

// ─── Filesystem Utilities ─────────────────────────────────────────────────────

#[tauri::command]
pub fn get_data_dir() -> String {
    AppState::cmd_get_data_dir()
}

#[tauri::command]
pub async fn read_external_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_db(dest_path: String) -> Result<(), String> {
    AppState::cmd_export_db(&dest_path).await
}

#[tauri::command]
pub async fn export_secure_package(
    dest_path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.cmd_export_secure_package(&dest_path, password).await
}

#[tauri::command]
pub async fn import_secure_package(
    src_path: String,
    password: String,
    strategy: ImportStrategy,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let path = std::path::Path::new(&src_path);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if ext != "pack" {
        let _ = app.emit(
            "boveda://audit",
            serde_json::json!({
                "action": "custom",
                "category": "ERROR",
                "msg": "Import rejected: file is not a valid .bvda.pack package."
            }),
        );
        return Err("Invalid file type. Expected a .bvda.pack package.".to_string());
    }

    let _ = app.emit(
        "boveda://audit",
        serde_json::json!({
            "action": "custom",
            "category": "IPC",
            "msg": "Importing encrypted .bvda.pack package..."
        }),
    );

    match state
        .cmd_import_secure_package(&src_path, password, strategy)
        .await
    {
        Ok(()) => {
            let _ = app.emit(
                "boveda://audit",
                serde_json::json!({
                    "action": "custom",
                    "category": "SUCCESS",
                    "msg": "Secure package imported successfully."
                }),
            );
            Ok(())
        }
        Err(e) => {
            let msg = if e.contains("Incorrect password") || e.contains("CryptoError") || e.contains("DecodeError") {
                "Import failed: invalid password or corrupted package.".to_string()
            } else if e.contains("SerializationError") || e.contains("format") {
                "Import failed: package structure is invalid or corrupted.".to_string()
            } else {
                "Import failed: unexpected error reading the package.".to_string()
            };
            let _ = app.emit(
                "boveda://audit",
                serde_json::json!({
                    "action": "custom",
                    "category": "ERROR",
                    "msg": msg
                }),
            );
            Err(e)
        }
    }
}

#[derive(serde::Serialize)]
pub struct AppInfo {
    pub app_version: String,
    pub core_version: String,
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        core_version: boveda_core::VERSION.to_string(),
    }
}

#[tauri::command]
pub fn get_os_username() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "guest".to_string())
}

#[tauri::command]
pub async fn import_db(
    src_path: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let path = std::path::Path::new(&src_path);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if ext != "bvda" {
        let _ = app.emit(
            "boveda://audit",
            serde_json::json!({
                "action": "custom",
                "category": "ERROR",
                "msg": "Import rejected: file is not a valid .bvda database."
            }),
        );
        return Err("Invalid file type. Expected a .bvda database file.".to_string());
    }

    let _ = app.emit(
        "boveda://audit",
        serde_json::json!({
            "action": "custom",
            "category": "IPC",
            "msg": "Preparing to import .bvda database..."
        }),
    );

    match state.cmd_prepare_import_db(&src_path).await {
        Ok(()) => {
            let _ = app.emit(
                "boveda://audit",
                serde_json::json!({
                    "action": "custom",
                    "category": "SUCCESS",
                    "msg": "Database replaced. Restarting vault..."
                }),
            );
            app.restart();
        }
        Err(e) => {
            let msg = if e.contains("does not exist") || e.contains("not found") {
                "Import failed: source file not found.".to_string()
            } else if e.contains("same file") {
                "Import rejected: cannot replace the active vault with itself.".to_string()
            } else {
                "Import failed: could not replace the database file.".to_string()
            };
            let _ = app.emit(
                "boveda://audit",
                serde_json::json!({
                    "action": "custom",
                    "category": "ERROR",
                    "msg": msg
                }),
            );
            Err(e)
        }
    }
}
