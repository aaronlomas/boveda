use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::secret::SecretString;

/// A row from the `accounts` table as returned to the frontend.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct AccountRow {
    pub id: String,
    pub site: String,
    pub username: String,
    pub encrypted_password: String,
    pub encrypted_notes: Option<String>,
    pub favicon_url: Option<String>,
    pub group_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Initialize the SQLite database schema.
pub async fn init_db(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS vault_meta (
            id          INTEGER PRIMARY KEY,
            salt        TEXT NOT NULL,
            challenge_text TEXT,
            created_at  TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS accounts (
            id                  TEXT PRIMARY KEY,
            site                TEXT NOT NULL,
            username            TEXT NOT NULL,
            encrypted_password  TEXT NOT NULL,
            encrypted_notes     TEXT,
            favicon_url         TEXT,
            created_at          TEXT NOT NULL,
            updated_at          TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Attempt to add challenge_text column for backward compatibility with existing databases
    let _ = sqlx::query("ALTER TABLE vault_meta ADD COLUMN challenge_text TEXT")
        .execute(pool)
        .await;

    // Attempt to add group_name column for backward compatibility with existing databases
    let _ = sqlx::query("ALTER TABLE accounts ADD COLUMN group_name TEXT")
        .execute(pool)
        .await;

    // Preferences table for user settings (theme, background, etc.)
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS preferences (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Retrieve the persisted Argon2id salt and challenge text.
pub async fn get_vault_meta(pool: &SqlitePool) -> Result<Option<(Vec<u8>, Option<String>)>> {
    let row: Option<(String, Option<String>)> =
        sqlx::query_as("SELECT salt, challenge_text FROM vault_meta WHERE id = 1")
            .fetch_optional(pool)
            .await?;

    if let Some((encoded, challenge)) = row {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&encoded)?;
        Ok(Some((bytes, challenge)))
    } else {
        Ok(None)
    }
}

/// Create the initial vault metadata with salt and challenge text.
pub async fn create_vault_meta(pool: &SqlitePool, salt: &[u8], challenge_text: &str) -> Result<()> {
    let encoded = base64::engine::general_purpose::STANDARD.encode(salt);
    let now = Utc::now().to_rfc3339();

    sqlx::query("INSERT INTO vault_meta (id, salt, challenge_text, created_at) VALUES (1, ?, ?, ?)")
        .bind(&encoded)
        .bind(challenge_text)
        .bind(&now)
        .execute(pool)
        .await?;

    Ok(())
}

/// Update the challenge text for legacy vaults.
pub async fn update_challenge_text(pool: &SqlitePool, challenge_text: &str) -> Result<()> {
    sqlx::query("UPDATE vault_meta SET challenge_text = ? WHERE id = 1")
        .bind(challenge_text)
        .execute(pool)
        .await?;
    Ok(())
}

/// Check whether the vault has been initialized (salt exists).
pub async fn is_initialized(pool: &SqlitePool) -> Result<bool> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vault_meta")
        .fetch_one(pool)
        .await?;
    Ok(count.0 > 0)
}

/// Insert a new account into the database.
pub async fn add_account(
    pool: &SqlitePool,
    site: &str,
    username: &str,
    encrypted_password: &str,
    encrypted_notes: Option<&str>,
    favicon_url: Option<&str>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"INSERT INTO accounts
           (id, site, username, encrypted_password, encrypted_notes, favicon_url, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(site)
    .bind(username)
    .bind(encrypted_password)
    .bind(encrypted_notes)
    .bind(favicon_url)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(id)
}

/// Fetch all account rows (still encrypted — decryption happens in Tauri commands).
pub async fn get_accounts(pool: &SqlitePool) -> Result<Vec<AccountRow>> {
    let rows = sqlx::query_as::<_, AccountRow>(
        r#"SELECT id, site, username, encrypted_password,
                  encrypted_notes, favicon_url, group_name, created_at, updated_at
           FROM accounts ORDER BY site ASC"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Update site and username (used for migrating plaintext to ciphertext).
/// Also clears the favicon_url as it's no longer persisted.
pub async fn update_account_metadata(
    pool: &SqlitePool,
    id: &str,
    encrypted_site: &str,
    encrypted_username: &str,
) -> Result<()> {
    sqlx::query(
        "UPDATE accounts SET site = ?, username = ?, favicon_url = NULL, updated_at = ? WHERE id = ?"
    )
    .bind(encrypted_site)
    .bind(encrypted_username)
    .bind(Utc::now().to_rfc3339())
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete an account by ID.
pub async fn delete_account(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM accounts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Assign (or clear) an account's group.
pub async fn update_account_group(
    pool: &SqlitePool,
    id: &str,
    group_name: Option<&str>,
) -> Result<()> {
    sqlx::query("UPDATE accounts SET group_name = ?, updated_at = ? WHERE id = ?")
        .bind(group_name)
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Rename all accounts belonging to `old_name` to `new_name`.
pub async fn rename_group(pool: &SqlitePool, old_name: &str, new_name: &str) -> Result<()> {
    sqlx::query(
        "UPDATE accounts SET group_name = ?, updated_at = ? WHERE group_name = ?",
    )
    .bind(new_name)
    .bind(Utc::now().to_rfc3339())
    .bind(old_name)
    .execute(pool)
    .await?;
    Ok(())
}

/// Remove group association from all accounts belonging to `name`.
pub async fn delete_group(pool: &SqlitePool, name: &str) -> Result<()> {
    sqlx::query(
        "UPDATE accounts SET group_name = NULL, updated_at = ? WHERE group_name = ?",
    )
    .bind(Utc::now().to_rfc3339())
    .bind(name)
    .execute(pool)
    .await?;
    Ok(())
}

/// Count how many accounts belong to a given group.
pub async fn count_accounts_in_group(pool: &SqlitePool, group_name: &str) -> Result<i64> {
    let row: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM accounts WHERE group_name = ?")
            .bind(group_name)
            .fetch_one(pool)
            .await?;
    Ok(row.0)
}

// Re-export base64 for db.rs internal use
use base64::Engine;

/// Get a user preference by key.
pub async fn get_preference(pool: &SqlitePool, key: &str) -> Result<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM preferences WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|(v,)| v))
}

/// Set (upsert) a user preference.
pub async fn set_preference(pool: &SqlitePool, key: &str, value: &str) -> Result<()> {
    sqlx::query(
        "INSERT INTO preferences (key, value) VALUES (?, ?)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

// ─── TEMPORAL MIGRATION LOGIC ──────────────────────────────────────────────────
// This function will be removed once the migration to SQLCipher is complete.

/// Migrate a plain SQLite database to SQLCipher
pub async fn migrate_to_sqlcipher(
    unencrypted_pool: &SqlitePool,
    key: &[u8],
    db_path: &std::path::Path,
) -> Result<()> {
    // 1. Get salt from vault_meta
    let salt_row: Option<(String,)> = sqlx::query_as("SELECT salt FROM vault_meta WHERE id = 1")
        .fetch_optional(unencrypted_pool)
        .await?;
        
    let salt_bytes = if let Some((encoded,)) = salt_row {
        base64::engine::general_purpose::STANDARD.decode(&encoded)?
    } else {
        return Err(anyhow::anyhow!("No salt found in vault_meta"));
    };


    // 2. Export database
    let encrypted_path = db_path.with_file_name("vault_encrypted.bvda");
    if encrypted_path.exists() {
        std::fs::remove_file(&encrypted_path)?;
    }

    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let path_str = encrypted_path.to_string_lossy();
    let mut attach_query = Vec::with_capacity(128 + path_str.len());
    attach_query.extend_from_slice(b"ATTACH DATABASE '");
    attach_query.extend_from_slice(path_str.as_bytes());
    attach_query.extend_from_slice(b"' AS encrypted KEY \"x'");
    for &byte in key {
        attach_query.push(HEX_CHARS[(byte >> 4) as usize]);
        attach_query.push(HEX_CHARS[(byte & 0x0f) as usize]);
    }
    attach_query.extend_from_slice(b"'\"");
    let attach_query_secret = SecretString::new(String::from_utf8(attach_query).expect("Valid string"));
    
    let mut conn = unencrypted_pool.acquire().await?;
    
    use sqlx::Executor;
    conn.execute(attach_query_secret.as_str()).await?;
    conn.execute("SELECT sqlcipher_export('encrypted')").await?;
    conn.execute("DETACH DATABASE encrypted").await?;
    
    drop(conn); // Release connection back to pool before closing it

    // 3. Close the pool
    unencrypted_pool.close().await;

    // 4. Cleanup
    std::fs::rename(&encrypted_path, db_path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(db_path, std::fs::Permissions::from_mode(0o600));
    }

    // 5. Save salt to vault.salt ONLY after successful migration
    let salt_path = db_path.with_file_name("vault.salt");
    std::fs::write(&salt_path, &salt_bytes).map_err(|e| anyhow::anyhow!("Error writing salt: {e}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&salt_path, std::fs::Permissions::from_mode(0o600));
    }
    
    Ok(())
}
