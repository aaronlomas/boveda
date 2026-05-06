use tauri::State;
use crate::state::AppState;
use boveda_core::{BovedaEngine, crypto, db, SecretString};
use zeroize::Zeroize;

/// Returns true if the vault DB has already been initialized (has a salt).
#[tauri::command]
pub async fn is_vault_initialized(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(BovedaEngine::is_initialized(&state.db_path))
}

#[tauri::command]
pub async fn unlock_vault(mut password: String, state: State<'_, AppState>) -> Result<bool, String> {
    let password_secret = SecretString::from(password.clone());
    
    let engine = BovedaEngine::unlock(&state.db_path, &password_secret)
        .await
        .map_err(|e| e.to_string())?;

    password.zeroize();

    let mut engine_lock = state.engine.lock().unwrap();
    *engine_lock = Some(engine);
    
    Ok(true)
}

#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>) -> Result<(), String> {
    let mut engine_lock = state.engine.lock().unwrap();
    *engine_lock = None;
    Ok(())
}

#[tauri::command]
pub async fn add_account(
    site: String,
    username: String,
    password: String,
    notes: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    let notes_opt = if notes.is_empty() { None } else { Some(notes.as_str()) };
    
    engine.add_account(&site, &username, &password, notes_opt)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_accounts(state: State<'_, AppState>) -> Result<Vec<boveda_core::Account>, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    engine.get_accounts()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_account(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    engine.delete_account(&id)
        .await
        .map_err(|e| e.to_string())
}

/// Generate a random password (never stored, purely in-memory).
#[tauri::command]
pub fn generate_password(length: usize, use_symbols: bool) -> String {
    let len = length.clamp(8, 128);
    crypto::generate_password(len, use_symbols).as_str().to_string()
}

/// Decrypt a single piece of secret data on-demand.
#[tauri::command]
pub fn decrypt_secret(ciphertext: String, state: State<'_, AppState>) -> Result<String, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    engine.decrypt_secret(&ciphertext)
        .map(|s| s.as_str().to_string())
        .map_err(|e| e.to_string())
}

// ─── Group commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn update_account_group(
    id: String,
    group_name: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    engine.update_account_group(&id, group_name.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rename_group(
    old_name: String,
    new_name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    let trimmed = new_name.trim().to_string();
    if trimmed.is_empty() {
        return Err("El nombre del grupo no puede estar vacío.".to_string());
    }
    
    engine.rename_group(&old_name, &trimmed)
        .await
        .map_err(|e| e.to_string())?;

    // Update the groups list in preferences (this is still a backend responsibility but could also be moved to engine)
    let raw = db::get_preference(&engine.db, "groups")
        .await
        .map_err(|e| e.to_string())?;
    
    let mut groups: Vec<String> = raw
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
        
    if let Some(pos) = groups.iter().position(|g| g == &old_name) {
        groups[pos] = trimmed;
    }
    
    let serialized = serde_json::to_string(&groups).map_err(|e| e.to_string())?;
    db::set_preference(&engine.db, "groups", &serialized)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_group(
    name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    let count = db::count_accounts_in_group(&engine.db, &name)
        .await
        .map_err(|e| e.to_string())?;
        
    if count > 0 {
        return Err(format!(
            "El grupo \"{}\" tiene {} cuenta(s) asignada(s). Mueve las cuentas antes de eliminarlo.",
            name, count
        ));
    }
    
    engine.delete_group(&name)
        .await
        .map_err(|e| e.to_string())?;
        
    let raw = db::get_preference(&engine.db, "groups")
        .await
        .map_err(|e| e.to_string())?;
        
    let mut groups: Vec<String> = raw
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
        
    groups.retain(|g| g != &name);
    let serialized = serde_json::to_string(&groups).map_err(|e| e.to_string())?;
    db::set_preference(&engine.db, "groups", &serialized)
        .await
        .map_err(|e| e.to_string())
}

