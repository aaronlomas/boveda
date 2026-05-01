use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

/// A row from the `accounts` table as returned to the frontend.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct AccountRow {
    pub id: String,
    pub site: String,
    pub username: String,
    pub encrypted_password: String,
    pub encrypted_notes: Option<String>,
    pub favicon_url: Option<String>,
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
                  encrypted_notes, favicon_url, created_at, updated_at
           FROM accounts ORDER BY site ASC"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Delete an account by ID.
pub async fn delete_account(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM accounts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
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
