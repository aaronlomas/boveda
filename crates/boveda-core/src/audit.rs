use crate::storage;
use sqlx::SqlitePool;
use serde::Serialize;
use crate::error::BovedaResult;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum AuditAction {
    VaultUnlocked,
    VaultLocked,
    SecretCreated,
    SecretRead,
    SecretUpdated,
    SecretDeleted,
    VaultExported,
    VaultImported,
    MasterKeyChanged,
    FailedLoginAttempt,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VaultUnlocked => "VAULT_UNLOCKED",
            Self::VaultLocked => "VAULT_LOCKED",
            Self::SecretCreated => "SECRET_CREATED",
            Self::SecretRead => "SECRET_READ",
            Self::SecretUpdated => "SECRET_UPDATED",
            Self::SecretDeleted => "SECRET_DELETED",
            Self::VaultExported => "VAULT_EXPORTED",
            Self::VaultImported => "VAULT_IMPORTED",
            Self::MasterKeyChanged => "MASTER_KEY_CHANGED",
            Self::FailedLoginAttempt => "FAILED_LOGIN_ATTEMPT",
        }
    }
}

pub async fn log_event(
    pool: &SqlitePool,
    action: AuditAction,
    metadata: Option<String>,
) -> BovedaResult<()> {
    storage::add_audit_log(pool, action.as_str(), metadata.as_deref()).await
}
