use tauri::State;
use crate::state::{AppState, MasterKey};
use crate::{crypto, db};

/// Returns true if the vault DB has already been initialized (has a salt).
#[tauri::command]
pub async fn is_vault_initialized(state: State<'_, AppState>) -> Result<bool, String> {
    db::is_initialized(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn unlock_vault(password: String, state: State<'_, AppState>) -> Result<bool, String> {
    let meta = db::get_vault_meta(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    let (salt, challenge_opt) = match meta {
        Some((s, c)) => (s, c),
        None => {
            // First time initialization — create salt + challenge
            use rand::RngCore;
            let mut salt = vec![0u8; 32];
            rand::rngs::OsRng.fill_bytes(&mut salt);

            let key = crypto::derive_key(&password, &salt).map_err(|e| e.to_string())?;
            let challenge =
                crypto::encrypt("boveda_auth", &key).map_err(|e| e.to_string())?;

            db::create_vault_meta(&state.db, &salt, &challenge)
                .await
                .map_err(|e| e.to_string())?;

            let mut lock = state.master_key.lock().unwrap();
            *lock = Some(MasterKey(key));
            return Ok(true);
        }
    };

    let key = crypto::derive_key(&password, &salt).map_err(|e| e.to_string())?;

    match challenge_opt {
        Some(challenge) => {
            let dec = crypto::decrypt(&challenge, &key)
                .map_err(|_| "Contraseña incorrecta".to_string())?;
            if dec != "boveda_auth" {
                return Err("Contraseña incorrecta".to_string());
            }
        }
        None => {
            // Legacy vault: validate by attempting to decrypt the first account
            let accounts = db::get_accounts(&state.db)
                .await
                .map_err(|e| e.to_string())?;
            if let Some(acc) = accounts.first() {
                if crypto::decrypt(&acc.encrypted_password, &key).is_err() {
                    return Err("Contraseña incorrecta".to_string());
                }
            }
            // Migrate: store challenge text for future logins
            let challenge = crypto::encrypt("boveda_auth", &key).map_err(|e| e.to_string())?;
            db::update_challenge_text(&state.db, &challenge)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    let mut lock = state.master_key.lock().unwrap();
    *lock = Some(MasterKey(key));
    Ok(true)
}

#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>) -> Result<(), String> {
    let mut lock = state.master_key.lock().unwrap();
    *lock = None; // MasterKey::drop() will zeroize the key bytes automatically
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
    let key = {
        let lock = state.master_key.lock().unwrap();
        lock.as_ref()
            .map(|mk| mk.0)
            .ok_or_else(|| "Vault is locked".to_string())?
    };

    let enc_password = crypto::encrypt(&password, &key).map_err(|e| e.to_string())?;
    let enc_notes = if notes.is_empty() {
        None
    } else {
        Some(crypto::encrypt(&notes, &key).map_err(|e| e.to_string())?)
    };

    let favicon_url = build_favicon_url(&site);

    db::add_account(
        &state.db,
        &site,
        &username,
        &enc_password,
        enc_notes.as_deref(),
        favicon_url.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_accounts(state: State<'_, AppState>) -> Result<Vec<crate::Account>, String> {
    let key = {
        let lock = state.master_key.lock().unwrap();
        lock.as_ref()
            .map(|mk| mk.0)
            .ok_or_else(|| "Vault is locked".to_string())?
    };

    let rows = db::get_accounts(&state.db).await.map_err(|e| e.to_string())?;

    let mut accounts = Vec::with_capacity(rows.len());
    for row in rows {
        let password = crypto::decrypt(&row.encrypted_password, &key)
            .unwrap_or_else(|_| "[Error de descifrado]".to_string());
        let notes = match &row.encrypted_notes {
            Some(enc) => crypto::decrypt(enc, &key)
                .unwrap_or_else(|_| "[Error de descifrado]".to_string()),
            None => String::new(),
        };
        accounts.push(crate::Account {
            id: row.id,
            site: row.site,
            username: row.username,
            password,
            notes,
            favicon_url: row.favicon_url,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }
    Ok(accounts)
}

#[tauri::command]
pub async fn delete_account(id: String, state: State<'_, AppState>) -> Result<(), String> {
    db::delete_account(&state.db, &id)
        .await
        .map_err(|e| e.to_string())
}

/// Generate a random password (never stored, purely in-memory).
#[tauri::command]
pub fn generate_password(length: usize, use_symbols: bool) -> String {
    let len = length.clamp(8, 128);
    crypto::generate_password(len, use_symbols)
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Build a Google favicon URL from a site string.
/// Strips scheme and path, returning only the domain part.
fn build_favicon_url(site: &str) -> Option<String> {
    let domain = site
        .trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()?;
    if domain.is_empty() {
        return None;
    }
    Some(format!(
        "https://www.google.com/s2/favicons?domain={domain}&sz=64"
    ))
}
