use tauri::State;
use crate::state::AppState;
use boveda_core::vault::ImportStrategy;

// ─── User Preferences ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_preference(key: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
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
) -> Result<(), String> {
    state.cmd_import_secure_package(&src_path, password, strategy).await
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

/// Importa una base de datos externa: cierra el pool, copia archivos y reinicia la app.
/// La lógica de archivos vive en boveda-core; solo `app.restart()` permanece aquí.
#[tauri::command]
pub async fn import_db(
    src_path: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    state.cmd_prepare_import_db(&src_path).await?;
    app.restart();
}