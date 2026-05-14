pub mod models;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, SqliteConnection, Executor};
use uuid::Uuid;
use base64::Engine;
use crate::error::BovedaResult;

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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct PinRow {
    pub id: String,
    pub name: String,
    pub encrypted_pin: String,
    pub encrypted_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// 🏗️  Schema Management
// ─────────────────────────────────────────────────────────────────────────────

/// Initialize the SQLite database schema.
pub async fn init_db(pool: &SqlitePool) -> BovedaResult<()> {
    sqlx::query(
        r"
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
            group_name          TEXT,
            created_at          TEXT NOT NULL,
            updated_at          TEXT NOT NULL
        );

        -- Index for fast sorting and searching by site
        CREATE INDEX IF NOT EXISTS idx_accounts_site ON accounts(site);
        -- Index for fast filtering by group
        CREATE INDEX IF NOT EXISTS idx_accounts_group ON accounts(group_name);

        CREATE TABLE IF NOT EXISTS pins (
            id              TEXT PRIMARY KEY,
            name            TEXT NOT NULL,
            encrypted_pin   TEXT NOT NULL,
            encrypted_notes TEXT,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
        );
        ",
    )
    .execute(pool)
    .await?;

    // Audit Log table
    sqlx::query(
        r"CREATE TABLE IF NOT EXISTS audit_log (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            action      TEXT NOT NULL,
            metadata    TEXT,
            created_at  TEXT NOT NULL
        );",
    )
    .execute(pool)
    .await?;

    // Preferences table
    sqlx::query(
        r"CREATE TABLE IF NOT EXISTS preferences (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
    )
    .execute(pool)
    .await?;

    // Handle legacy schema updates gracefully
    let _ = sqlx::query("ALTER TABLE vault_meta ADD COLUMN challenge_text TEXT").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE accounts ADD COLUMN group_name TEXT").execute(pool).await;
    // Note: If columns already exist, ALTER TABLE fails safely in this context

    Ok(())
}

/// Retrieve the persisted Argon2id salt and challenge text.
pub async fn get_vault_meta(pool: &SqlitePool) -> BovedaResult<Option<(Vec<u8>, Option<String>)>> {
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

// ─────────────────────────────────────────────────────────────────────────────
// 📁 Account Persistence
// ─────────────────────────────────────────────────────────────────────────────

/// Insert a new account into the database.
pub async fn add_account(
    pool: &SqlitePool,
    site: &str,
    username: &str,
    encrypted_password: &str,
    encrypted_notes: Option<&str>,
    favicon_url: Option<&str>,
) -> BovedaResult<String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r"INSERT INTO accounts
           (id, site, username, encrypted_password, encrypted_notes, favicon_url, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
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

/// Fetch all account rows.
pub async fn get_accounts(pool: &SqlitePool) -> BovedaResult<Vec<AccountRow>> {
    let rows = sqlx::query_as::<_, AccountRow>(
        r"SELECT id, site, username, encrypted_password,
                  encrypted_notes, favicon_url, group_name, created_at, updated_at
           FROM accounts ORDER BY site ASC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Fetch a page of account rows.
pub async fn get_accounts_paged(pool: &SqlitePool, limit: i64, offset: i64) -> BovedaResult<Vec<AccountRow>> {
    let rows = sqlx::query_as::<_, AccountRow>(
        r"SELECT id, site, username, encrypted_password,
                  encrypted_notes, favicon_url, group_name, created_at, updated_at
           FROM accounts ORDER BY site ASC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Delete an account by ID.
pub async fn delete_account(pool: &SqlitePool, id: &str) -> BovedaResult<()> {
    sqlx::query("DELETE FROM accounts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// 🔒 PIN Persistence
// ─────────────────────────────────────────────────────────────────────────────

pub async fn add_pin(
    pool: &SqlitePool,
    name: &str,
    encrypted_pin: &str,
    encrypted_notes: Option<&str>,
) -> BovedaResult<String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r"INSERT INTO pins
           (id, name, encrypted_pin, encrypted_notes, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(name)
    .bind(encrypted_pin)
    .bind(encrypted_notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(id)
}

pub async fn get_pins(pool: &SqlitePool) -> BovedaResult<Vec<PinRow>> {
    let rows = sqlx::query_as::<_, PinRow>(
        r"SELECT id, name, encrypted_pin, encrypted_notes, created_at, updated_at
           FROM pins ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn delete_pin(pool: &SqlitePool, id: &str) -> BovedaResult<()> {
    sqlx::query("DELETE FROM pins WHERE id = ?")
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
) -> BovedaResult<()> {
    sqlx::query("UPDATE accounts SET group_name = ?, updated_at = ? WHERE id = ?")
        .bind(group_name)
        .bind(Utc::now().to_rfc3339())
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Rename all accounts belonging to `old_name` to `new_name` (Transactional).
pub async fn rename_group_tx(conn: &mut SqliteConnection, old_name: &str, new_name: &str) -> BovedaResult<()> {
    sqlx::query(
        "UPDATE accounts SET group_name = ?, updated_at = ? WHERE group_name = ?",
    )
    .bind(new_name)
    .bind(Utc::now().to_rfc3339())
    .bind(old_name)
    .execute(conn)
    .await?;
    Ok(())
}

/// Remove group association from all accounts belonging to `name` (Transactional).
pub async fn delete_group_tx(conn: &mut SqliteConnection, name: &str) -> BovedaResult<()> {
    sqlx::query(
        "UPDATE accounts SET group_name = NULL, updated_at = ? WHERE group_name = ?",
    )
    .bind(Utc::now().to_rfc3339())
    .bind(name)
    .execute(conn)
    .await?;
    Ok(())
}

/// Count how many accounts belong to a given group.
pub async fn count_accounts_in_group(pool: &SqlitePool, group_name: &str) -> BovedaResult<i64> {
    let row: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM accounts WHERE group_name = ?")
            .bind(group_name)
            .fetch_one(pool)
            .await?;
    Ok(row.0)
}

/// Get a user preference by key.
pub async fn get_preference(pool: &SqlitePool, key: &str) -> BovedaResult<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM preferences WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|(v,)| v))
}

pub async fn get_all_preferences(pool: &SqlitePool) -> BovedaResult<Vec<(String, String)>> {
    let rows: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM preferences")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn get_preference_tx(conn: &mut SqliteConnection, key: &str) -> BovedaResult<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM preferences WHERE key = ?")
        .bind(key)
        .fetch_optional(conn)
        .await?;
    Ok(row.map(|(v,)| v))
}

/// Set (upsert) a user preference.
pub async fn set_preference(pool: &SqlitePool, key: &str, value: &str) -> BovedaResult<()> {
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

pub async fn set_preference_tx(conn: &mut SqliteConnection, key: &str, value: &str) -> BovedaResult<()> {
    sqlx::query(
        "INSERT INTO preferences (key, value) VALUES (?, ?)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
    )
    .bind(key)
    .bind(value)
    .execute(conn)
    .await?;
    Ok(())
}

/// Delete a user preference by key.
pub async fn delete_preference(pool: &SqlitePool, key: &str) -> BovedaResult<()> {
    sqlx::query("DELETE FROM preferences WHERE key = ?")
        .bind(key)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_preference_tx(conn: &mut SqliteConnection, key: &str) -> BovedaResult<()> {
    sqlx::query("DELETE FROM preferences WHERE key = ?")
        .bind(key)
        .execute(conn)
        .await?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// ⚖️ Audit Logging
// ─────────────────────────────────────────────────────────────────────────────

/// Add a new entry to the audit log.
pub async fn add_audit_log(
    pool: &SqlitePool,
    action: &str,
    metadata: Option<&str>,
) -> BovedaResult<()> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("INSERT INTO audit_log (action, metadata, created_at) VALUES (?, ?, ?)")
        .bind(action)
        .bind(metadata)
        .bind(&now)
        .execute(pool)
        .await?;
    Ok(())
}

/// Fetch recent audit logs.
pub async fn get_audit_logs(pool: &SqlitePool, limit: i64) -> BovedaResult<Vec<(i64, String, Option<String>, String)>> {
    let rows: Vec<(i64, String, Option<String>, String)> = 
        sqlx::query_as("SELECT id, action, metadata, created_at FROM audit_log ORDER BY id DESC LIMIT ?")
            .bind(limit)
            .fetch_all(pool)
            .await?;
    Ok(rows)
}

// ─── TEMPORAL MIGRATION LOGIC ──────────────────────────────────────────────────

pub async fn migrate_to_sqlcipher(
    unencrypted_pool: &SqlitePool,
    key: &crate::crypto::secret::SecretKey,
    db_path: &std::path::Path,
) -> BovedaResult<()> {
    use base64::Engine;
    
    let salt_row: Option<(String,)> = sqlx::query_as("SELECT salt FROM vault_meta WHERE id = 1")
        .fetch_optional(unencrypted_pool)
        .await?;
        
    let salt_bytes = if let Some((encoded,)) = salt_row {
        base64::engine::general_purpose::STANDARD.decode(&encoded)?
    } else {
        return Err(crate::error::BovedaError::NotFound("No salt found in vault_meta".to_string()));
    };

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
    for &byte in key.as_bytes() {
        attach_query.push(HEX_CHARS[(byte >> 4) as usize]);
        attach_query.push(HEX_CHARS[(byte & 0x0f) as usize]);
    }
    attach_query.extend_from_slice(b"'\"");
    let attach_query_str = String::from_utf8(attach_query).map_err(|e| crate::error::BovedaError::DecodeError(e.to_string()))?;
    let attach_query_zero = zeroize::Zeroizing::new(attach_query_str);
    
    let mut conn = unencrypted_pool.acquire().await?;
    
    conn.execute(attach_query_zero.as_str()).await?;
    conn.execute("SELECT sqlcipher_export('encrypted')").await?;
    conn.execute("DETACH DATABASE encrypted").await?;
    
    drop(conn);
    unencrypted_pool.close().await;

    std::fs::rename(&encrypted_path, db_path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(db_path, std::fs::Permissions::from_mode(0o600));
    }

    let salt_path = db_path.with_file_name("vault.salt");
    std::fs::write(&salt_path, &salt_bytes)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&salt_path, std::fs::Permissions::from_mode(0o600));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        init_db(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_db_init() {
        let pool = setup_db().await;
        // Check if tables exist
        let tables: Vec<(String,)> = sqlx::query_as("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch_all(&pool)
            .await
            .unwrap();
        let table_names: Vec<String> = tables.into_iter().map(|(n,)| n).collect();
        assert!(table_names.contains(&"vault_meta".to_string()));
        assert!(table_names.contains(&"accounts".to_string()));
        assert!(table_names.contains(&"preferences".to_string()));
    }

    #[tokio::test]
    async fn test_crud_accounts() {
        let pool = setup_db().await;
        
        let id = add_account(&pool, "site.com", "user", "pass", Some("notes"), Some("favicon")).await.unwrap();
        let accounts = get_accounts(&pool).await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].id, id);
        assert_eq!(accounts[0].site, "site.com");

        update_account_group(&pool, &id, Some("Work")).await.unwrap();
        let accounts = get_accounts(&pool).await.unwrap();
        assert_eq!(accounts[0].group_name, Some("Work".to_string()));

        delete_account(&pool, &id).await.unwrap();
        let accounts = get_accounts(&pool).await.unwrap();
        assert_eq!(accounts.len(), 0);
    }

    #[tokio::test]
    async fn test_preferences() {
        let pool = setup_db().await;
        
        set_preference(&pool, "theme", "dark").await.unwrap();
        let val = get_preference(&pool, "theme").await.unwrap();
        assert_eq!(val, Some("dark".to_string()));

        set_preference(&pool, "theme", "light").await.unwrap();
        let val = get_preference(&pool, "theme").await.unwrap();
        assert_eq!(val, Some("light".to_string()));

        let non_existent = get_preference(&pool, "non_existent").await.unwrap();
        assert_eq!(non_existent, None);
    }

    #[tokio::test]
    async fn test_group_operations_tx() {
        let pool = setup_db().await;
        add_account(&pool, "s1", "u", "p", None, None).await.unwrap();
        let id2 = add_account(&pool, "s2", "u", "p", None, None).await.unwrap();
        update_account_group(&pool, &id2, Some("G1")).await.unwrap();

        let mut tx = pool.begin().await.unwrap();
        rename_group_tx(&mut tx, "G1", "G2").await.unwrap();
        tx.commit().await.unwrap();

        let count = count_accounts_in_group(&pool, "G2").await.unwrap();
        assert_eq!(count, 1);

        let mut tx = pool.begin().await.unwrap();
        delete_group_tx(&mut tx, "G2").await.unwrap();
        tx.commit().await.unwrap();

        let count = count_accounts_in_group(&pool, "G2").await.unwrap();
        assert_eq!(count, 0);
    }
}
