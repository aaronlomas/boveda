use tauri::State;
use crate::state::AppState;
use boveda_core::auth::TotpSetupPayload;

#[tauri::command]
pub async fn totp_is_enabled(state: State<'_, AppState>) -> Result<bool, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    engine.is_totp_enabled()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn totp_setup(state: State<'_, AppState>) -> Result<TotpSetupPayload, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    engine.setup_totp()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn totp_verify_setup(code: String, state: State<'_, AppState>) -> Result<bool, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    engine.verify_totp(&code)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn totp_check(code: String, state: State<'_, AppState>) -> Result<bool, String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    let valid = engine.verify_totp(&code)
        .await
        .map_err(|e| e.to_string())?;

    if !valid {
        return Err("Código TOTP inválido".to_string());
    }

    Ok(true)
}

#[tauri::command]
pub async fn totp_disable(state: State<'_, AppState>) -> Result<(), String> {
    let engine = {
        let engine_lock = state.engine.lock().unwrap();
        engine_lock.as_ref().cloned().ok_or("Vault is locked")?
    };
    
    engine.disable_totp()
        .await
        .map_err(|e| e.to_string())
}
