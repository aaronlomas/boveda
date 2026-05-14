use tauri::State;
use crate::state::AppState;
use boveda_core::SecretString;

#[tauri::command]
pub fn is_vault_initialized(state: State<'_, AppState>) -> bool {
    state.cmd_is_vault_initialized()
}

#[tauri::command]
pub async fn unlock_vault(password: SecretString, state: State<'_, AppState>) -> Result<String, String> {
    state.cmd_unlock_vault(password).await
}

#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>) -> Result<(), String> {
    state.cmd_lock_vault();
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
    state.cmd_add_account(site, username, password, notes).await
}

#[tauri::command]
pub async fn get_accounts(state: State<'_, AppState>) -> Result<Vec<boveda_core::Account>, String> {
    state.cmd_get_accounts().await
}

#[tauri::command]
pub async fn delete_account(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.cmd_delete_account(&id).await
}

/// Genera una contraseña aleatoria (nunca se almacena, puramente en memoria).
#[tauri::command]
pub fn generate_password(length: usize, use_symbols: bool) -> String {
    AppState::cmd_generate_password(length, use_symbols)
}

/// Descifra un campo secreto individual bajo demanda.
#[tauri::command]
pub async fn decrypt_secret(ciphertext: String, state: State<'_, AppState>) -> Result<String, String> {
    state.cmd_decrypt_secret(&ciphertext).await
}

// ─── Group commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn update_account_group(
    id: String,
    group_name: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.cmd_update_account_group(&id, group_name.as_deref()).await
}

#[tauri::command]
pub async fn rename_group(
    old_name: String,
    new_name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.cmd_rename_group(&old_name, &new_name).await
}

#[tauri::command]
pub async fn delete_group(name: String, state: State<'_, AppState>) -> Result<(), String> {
    state.cmd_delete_group(&name).await
}
