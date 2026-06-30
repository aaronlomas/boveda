use crate::state::AppState;
use boveda_core::auth::TotpSetupPayload;
use tauri::State;

#[tauri::command]
pub async fn totp_is_enabled(state: State<'_, AppState>) -> Result<bool, String> {
    state.cmd_totp_is_enabled().await
}

#[tauri::command]
pub async fn totp_setup(state: State<'_, AppState>) -> Result<TotpSetupPayload, String> {
    state.cmd_totp_setup().await
}

#[tauri::command]
pub async fn totp_verify_setup(code: String, state: State<'_, AppState>) -> Result<bool, String> {
    state.cmd_totp_verify_setup(&code).await
}

#[tauri::command]
pub async fn totp_check(code: String, state: State<'_, AppState>) -> Result<bool, String> {
    state.cmd_totp_check(&code).await
}

#[tauri::command]
pub async fn totp_recovery_check(code: String, state: State<'_, AppState>) -> Result<bool, String> {
    state.cmd_totp_recovery_check(&code).await
}

#[tauri::command]
pub async fn totp_disable(state: State<'_, AppState>) -> Result<(), String> {
    state.cmd_totp_disable().await
}

#[tauri::command]
pub fn is_vault_locked(state: State<'_, AppState>) -> bool {
    state.is_locked()
}

#[tauri::command]
pub fn remote_session_active() -> bool {
    boveda_core::security::environment_check()
}
