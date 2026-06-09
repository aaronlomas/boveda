use crate::storage;
use sqlx::SqlitePool;
use serde::Serialize;
use crate::error::BovedaResult;


#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    VaultUnlock,
    VaultLock,
    VaultExport,
    VaultImport,
    AccountCreate,
    AccountDelete,
    AccountGroupUpdate,
    SecretAccess,
    TotpEnabled,
    TotpDisabled,
    MasterKeyChanged,
    FailedLoginAttempt,
    PinCreate,
    PinDelete,
    DocumentCreate,
    DocumentUpdate,
    DocumentDelete,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::VaultUnlock => "vault_unlock",
            Self::VaultLock => "vault_lock",
            Self::VaultExport => "vault_export",
            Self::VaultImport => "vault_import",
            Self::AccountCreate => "account_create",
            Self::AccountDelete => "account_delete",
            Self::AccountGroupUpdate => "account_group_update",
            Self::SecretAccess => "secret_access",
            Self::TotpEnabled => "totp_enabled",
            Self::TotpDisabled => "totp_disabled",
            Self::MasterKeyChanged => "master_key_changed",
            Self::FailedLoginAttempt => "failed_login_attempt",
            Self::PinCreate => "pin_create",
            Self::PinDelete => "pin_delete",
            Self::DocumentCreate => "document_create",
            Self::DocumentUpdate => "document_update",
            Self::DocumentDelete => "document_delete",
        }
    }
}

/// logs an audit event in the database
pub async fn log_event(
    pool: &SqlitePool,
    action: AuditAction,
    metadata: Option<&str>,
) -> BovedaResult<()> {
    storage::add_audit_log(pool, action.as_str(), metadata).await
}
