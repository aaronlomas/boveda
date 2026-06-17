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
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "clear_log" }));
    let t0 = std::time::Instant::now();
    let m_cost = boveda_core::crypto::ARGON2_M_COST;
    let t_cost = boveda_core::crypto::ARGON2_T_COST;
    let p_cost = boveda_core::crypto::ARGON2_P_COST;
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "KDF", "msg": format!("Invoking Argon2id (m_cost: {}, t_cost: {}, parallelism: {})", m_cost, t_cost, p_cost) }));

    match state.cmd_unlock_vault(password).await {
        Ok(result) => {
            let elapsed = t0.elapsed().as_millis();
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "SUCCESS", "msg": format!("Vault unlocked successfully [{}ms]", elapsed) }));
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "vault_unlock" }));
            Ok(result)
        }
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("Remote session detected") {
                let _ = app.emit("boveda://audit", serde_json::json!({
                    "action": "custom",
                    "category": "SEC",
                    "msg": "Vault unlock blocked: remote session detected (AnyDesk/VNC/RDP)"
                }));
                let _ = app.emit("boveda://audit", serde_json::json!({ "action": "remote_blocked" }));
            } else {
                let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "ERROR", "msg": "Vault unlock failed. Check your master password." }));
                let _ = app.emit("boveda://audit", serde_json::json!({ "action": "failed_login_attempt" }));
            }
            Err(e)
        }
    }
}

#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "clear_log" }));
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "MEM", "msg": "Clearing vault from memory. Locking..." }));
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
pub async fn get_accounts(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<Vec<boveda_core::Account>, String> {
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "clear_log" }));
    let t0 = std::time::Instant::now();
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "DB", "msg": "Querying local SQLCipher store for accounts..." }));
    let res = state.cmd_get_accounts().await;
    match &res {
        Ok(_) => {
            let elapsed = t0.elapsed().as_millis();
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "SUCCESS", "msg": format!("Accounts payload decrypted and loaded [{}ms]", elapsed) }));
        }
        Err(_) => {
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "ERROR", "msg": "Failed to load accounts payload." }));
        }
    }
    res
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
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "clear_log" }));
    let t0 = std::time::Instant::now();
    let req_id = if ciphertext.len() > 8 { &ciphertext[..8] } else { "42" };
    let nonce_len = boveda_core::crypto::NONCE_LEN;
    let tag_len = boveda_core::crypto::TAG_LEN;

    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "IPC", "msg": format!("Audited command received: GetSecret(id: {}...)", req_id) }));
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "CIPHER", "msg": "Fetching entry payload..." }));
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "CIPHER", "msg": "Decrypting with ChaCha20-Poly1305 (AEAD)" }));
    let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "CIPHER", "msg": format!("Extracting Nonce ({} bytes) and Auth Tag ({} bytes)", nonce_len, tag_len) }));

    let result = state.cmd_decrypt_secret(&ciphertext).await;
    match result {
        Ok(cleartext) => {
            let elapsed = t0.elapsed().as_millis();
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "SUCCESS", "msg": "MAC validation passed. Integrity verified." }));
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "CIPHER", "msg": format!("Payload decrypted. Integrity verified [{}ms]", elapsed) }));
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "secret_access" }));
            Ok(cleartext)
        }
        Err(e) => {
            let _ = app.emit("boveda://audit", serde_json::json!({ "action": "custom", "category": "ERROR", "msg": format!("Decryption failed: {}", e) }));
            Err(e)
        }
    }
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
