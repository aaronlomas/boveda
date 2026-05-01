use tauri::State;
use crate::state::AppState;
use crate::db;

// ─── User Preferences ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_preference(key: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    db::get_preference(&state.db, &key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_preference(key: String, value: String, state: State<'_, AppState>) -> Result<(), String> {
    db::set_preference(&state.db, &key, &value)
        .await
        .map_err(|e| e.to_string())
}

// ─── Background Image ─────────────────────────────────────────────────────────

/// Copy an image file from `src_path` into the app data directory as `background.<ext>`.
/// Returns the stored filename (e.g. "background.png").
#[tauri::command]
pub async fn import_background_image(src_path: String) -> Result<String, String> {
    let src = std::path::Path::new(&src_path);
    if !src.exists() {
        return Err("El archivo de imagen no existe.".to_string());
    }

    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg")
        .to_lowercase();

    let allowed = ["jpg", "jpeg", "png", "webp", "gif"];
    if !allowed.contains(&ext.as_str()) {
        return Err("Formato de imagen no soportado. Usa JPG, PNG, WEBP o GIF.".to_string());
    }

    let dest_dir = dirs_next::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("boveda");

    // Remove any previous background file before saving the new one
    for old_ext in &allowed {
        let _ = std::fs::remove_file(dest_dir.join(format!("background.{}", old_ext)));
    }

    let dest_filename = format!("background.{}", ext);
    let dest = dest_dir.join(&dest_filename);
    std::fs::copy(&src_path, &dest)
        .map_err(|e| format!("Error al copiar imagen: {}", e))?;

    Ok(dest_filename)
}

/// Read a background image from the app data directory and return it as a data URL.
/// This avoids exposing raw filesystem paths to the frontend.
#[tauri::command]
pub fn get_background_data_url(filename: String) -> Result<String, String> {
    let path = dirs_next::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("boveda")
        .join(&filename);

    let bytes = std::fs::read(&path)
        .map_err(|e| format!("Cannot read background image: {}", e))?;

    let ext = std::path::Path::new(&filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpeg")
        .to_lowercase();

    let mime = match ext.as_str() {
        "png"  => "image/png",
        "gif"  => "image/gif",
        "webp" => "image/webp",
        _      => "image/jpeg",
    };

    use base64::Engine;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime, encoded))
}

// ─── Filesystem Utilities ─────────────────────────────────────────────────────

/// Returns the path to the app's data directory (useful for debugging).
#[tauri::command]
pub fn get_data_dir() -> String {
    dirs_next::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("boveda")
        .to_string_lossy()
        .to_string()
}

/// Copy the vault database to a user-chosen path (backup/export).
#[tauri::command]
pub async fn export_db(dest_path: String) -> Result<(), String> {
    let db_path = vault_db_path();
    std::fs::copy(&db_path, &dest_path).map_err(|e| e.to_string())?;
    Ok(())
}

/// Replace the current vault database with a user-supplied file, then restart the app.
#[tauri::command]
pub async fn import_db(
    src_path: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let src = std::path::Path::new(&src_path);
    if !src.exists() {
        return Err("El archivo de origen no existe".to_string());
    }

    let db_path = vault_db_path();
    if src == db_path {
        return Err(
            "No puedes importar el mismo archivo que la app está usando. Usa 'Exportar' para respaldos."
                .to_string(),
        );
    }

    // Close the pool to release the file lock before overwriting
    state.db.close().await;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Remove SQLite WAL / SHM files to avoid corruption
    let _ = std::fs::remove_file(db_path.with_extension("db-wal"));
    let _ = std::fs::remove_file(db_path.with_extension("db-shm"));

    std::fs::copy(&src_path, &db_path)
        .map_err(|e| format!("Error al copiar el archivo: {}", e))?;

    app.restart();
    // Restarting the app ends the process, no need to return Ok(())
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

fn vault_db_path() -> std::path::PathBuf {
    dirs_next::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("boveda")
        .join("vault.db")
}
