use tauri::State;
use crate::state::AppState;
use boveda_core::{BovedaEngine, SecretString, Account};
use boveda_core::crypto;


/// Returns true if the vault DB has already been initialized (has a salt).
#[tauri::command]
pub async fn is_vault_initialized(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(BovedaEngine::is_initialized(&state.db_path))
}

#[tauri::command]
pub async fn unlock_vault(password: SecretString, state: State<'_, AppState>) -> Result<String, String> {
    let engine = BovedaEngine::unlock(&state.db_path, &password)
        .await
        .map_err(|e| e.to_string())?;

    let is_totp = engine.is_totp_enabled().await.unwrap_or(false);

    let mut engine_lock = state.engine.lock().unwrap();
    *engine_lock = Some(engine);
    
    if is_totp {
        Ok("totp_required".to_string())
    } else {
        Ok("unlocked".to_string())
    }
}

#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>) -> Result<(), String> {
    let mut engine_lock = state.engine.lock().unwrap();
    *engine_lock = None;
    Ok(())
}

#[tauri::command]
pub async fn add_account(
    site: SecretString,
    username: SecretString,
    password: SecretString,
    notes: SecretString,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    let notes_opt = if notes.as_str().is_empty() { None } else { Some(notes) };
    
    engine.add_account(site, username, password, notes_opt)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_accounts(state: State<'_, AppState>) -> Result<Vec<Account>, String> {
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
        .map(|s: SecretString| s.as_str().to_string())
        .map_err(|e: boveda_core::BovedaError| e.to_string())
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
    
    engine.rename_group(&old_name, &new_name)
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
    
    engine.delete_group(&name)
        .await
        .map_err(|e| e.to_string())
}
