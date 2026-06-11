use tauri::{State, Emitter};
use crate::state::AppState;
use boveda_core::SecretString;

#[tauri::command]
pub fn is_vault_initialized(state: State<'_, AppState>) -> bool {
    state.cmd_is_vault_initialized()
}

#[tauri::command]
pub async fn unlock_vault(
    password: SecretString,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    match state.cmd_unlock_vault(password).await {
        Ok(result) => {
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "vault_unlock" }));
            Ok(result)
        }
        Err(e) => {
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "failed_login_attempt" }));
            Err(e)
        }
    }
}

#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    state.cmd_lock_vault();
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "vault_lock", "trigger": "manual" }));
    Ok(())
}

#[tauri::command]
pub async fn delete_vault(password: SecretString, state: State<'_, AppState>) -> Result<(), String> {
    state.cmd_delete_vault(password).await
}

#[tauri::command]
pub async fn add_account(
    site: SecretString,
    username: SecretString,
    password: SecretString,
    recovery_code: SecretString,
    notes: SecretString,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.cmd_add_account(site, username, password, recovery_code, notes).await
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
pub fn generate_password(length: usize, use_symbols: bool) -> Result<String, String> {
    AppState::cmd_generate_password(length, use_symbols)
}

/// Descifra un campo secreto individual bajo demanda.
#[tauri::command]
pub async fn decrypt_secret(
    ciphertext: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let result = state.cmd_decrypt_secret(&ciphertext).await;
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "secret_access" }));
    result
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

// ─── PIN commands ─────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn add_pin(
    name: SecretString,
    pin: SecretString,
    notes: SecretString,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.cmd_add_pin(name, pin, notes).await
}

#[tauri::command]
pub async fn get_pins(state: State<'_, AppState>) -> Result<Vec<boveda_core::Pin>, String> {
    state.cmd_get_pins().await
}

#[tauri::command]
pub async fn delete_pin(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.cmd_delete_pin(&id).await
}

// ─── Document commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn add_document(
    title: SecretString,
    description: Option<SecretString>,
    content: SecretString,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.cmd_add_document(title, description, content).await
}

#[tauri::command]
pub async fn get_documents(state: State<'_, AppState>) -> Result<Vec<boveda_core::Document>, String> {
    state.cmd_get_documents().await
}

#[tauri::command]
pub async fn update_document(
    id: String,
    title: SecretString,
    description: Option<SecretString>,
    content: SecretString,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.cmd_update_document(&id, title, description, content).await
}

#[tauri::command]
pub async fn delete_document(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.cmd_delete_document(&id).await
}

#[tauri::command]
pub async fn decrypt_document_content(
    encrypted_content: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.cmd_decrypt_document_content(&encrypted_content).await
}

#[derive(serde::Serialize)]
pub struct CryptoParams {
    pub argon2_m_cost: u32,
    pub argon2_t_cost: u32,
    pub argon2_p_cost: u32,
    pub nonce_len: usize,
    pub tag_len: usize,
}

#[tauri::command]
pub fn get_crypto_params() -> CryptoParams {
    CryptoParams {
        argon2_m_cost: boveda_core::crypto::ARGON2_M_COST,
        argon2_t_cost: boveda_core::crypto::ARGON2_T_COST,
        argon2_p_cost: boveda_core::crypto::ARGON2_P_COST,
        nonce_len: boveda_core::crypto::NONCE_LEN,
        tag_len: boveda_core::crypto::TAG_LEN,
    }
}
