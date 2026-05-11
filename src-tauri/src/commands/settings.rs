use tauri::State;
use crate::state::AppState;
use boveda_core::crypto::secret::SecretString;
use boveda_core::vault::ImportStrategy;

// ─── User Preferences ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_preference(key: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    engine.get_preference(&key)
        .await
        .map_err(|e: boveda_core::BovedaError| e.to_string())
}

#[tauri::command]
pub async fn set_preference(key: String, value: String, state: State<'_, AppState>) -> Result<(), String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    engine.set_preference(&key, &value)
        .await
        .map_err(|e: boveda_core::BovedaError| e.to_string())
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

/// Copy the vault database and its salt to a user-chosen path (backup/export).
#[tauri::command]
pub async fn export_db(dest_path: String) -> Result<(), String> {
    let db_path = vault_db_path();
    let salt_path = db_path.with_file_name("vault.salt");
    
    let dest_db = std::path::Path::new(&dest_path);
    let dest_salt = dest_db.with_extension(format!("{}.salt", dest_db.extension().and_then(|e| e.to_str()).unwrap_or("bvda")));

    // Copy DB
    std::fs::copy(&db_path, dest_db).map_err(|e| e.to_string())?;
    // Copy Salt
    if salt_path.exists() {
        std::fs::copy(&salt_path, &dest_salt).map_err(|e| e.to_string())?;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(dest_db, std::fs::Permissions::from_mode(0o600));
        if dest_salt.exists() {
            let _ = std::fs::set_permissions(&dest_salt, std::fs::Permissions::from_mode(0o600));
        }
    }

    Ok(())
}

/// Exports the vault into a secure, encrypted JSON package (.bvda.pack).
#[tauri::command]
pub async fn export_secure_package(
    dest_path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };

    let secret_pass = SecretString::new(password);
    let package_json = engine.export_vault(&secret_pass)
        .await
        .map_err(|e| format!("Export failed: {}", e))?;

    std::fs::write(&dest_path, package_json)
        .map_err(|e| format!("Failed to write export file: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dest_path, std::fs::Permissions::from_mode(0o600));
    }

    Ok(())
}

/// Imports a secure package (.bvda.pack) into the current vault.
#[tauri::command]
pub async fn import_secure_package(
    src_path: String,
    password: String,
    strategy: ImportStrategy,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };

    let package_json = std::fs::read_to_string(&src_path)
        .map_err(|e| format!("Failed to read import file: {}", e))?;

    let secret_pass = SecretString::new(password);
    engine.import_vault(&package_json, &secret_pass, strategy)
        .await
        .map_err(|e| format!("Import failed: {}", e))?;

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
    let engine = {
        let mut engine_lock = state.engine.lock().unwrap();
        engine_lock.take()
    };
    if let Some(e) = engine {
        e.close().await;
    }
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Remove SQLite WAL / SHM files to avoid corruption
    let _ = std::fs::remove_file(db_path.with_extension("bvda-wal"));
    let _ = std::fs::remove_file(db_path.with_extension("bvda-shm"));
    
    // Handle Salt: look for source_file.salt
    let src_salt = src.with_extension(format!("{}.salt", src.extension().and_then(|e| e.to_str()).unwrap_or("bvda")));
    let dest_salt = db_path.with_file_name("vault.salt");
    
    if src_salt.exists() {
        std::fs::copy(&src_salt, &dest_salt).map_err(|e| format!("Error al copiar el salt: {}", e))?;
    } else {
        // If importing a non-sqlcipher or legacy db, we might not have a salt file yet
        let _ = std::fs::remove_file(&dest_salt);
    }

    std::fs::copy(&src_path, &db_path)
        .map_err(|e| format!("Error al copiar el archivo: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&db_path, std::fs::Permissions::from_mode(0o600));
        if dest_salt.exists() {
            let _ = std::fs::set_permissions(&dest_salt, std::fs::Permissions::from_mode(0o600));
        }
    }

    app.restart();
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

fn vault_db_path() -> std::path::PathBuf {
    dirs_next::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("boveda")
        .join("vault.bvda")
}