use crate::storage;
use sqlx::SqlitePool;
use serde::Serialize;
use crate::error::BovedaResult;

/// Representa una acción auditable en el sistema.
/// Usamos un enum para garantizar la consistencia y evitar errores tipográficos.
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
        }
    }
}

/// Registra un evento de auditoría en la base de datos.
pub async fn log_event(
    pool: &SqlitePool,
    action: AuditAction,
    metadata: Option<&str>,
) -> BovedaResult<()> {
    storage::add_audit_log(pool, action.as_str(), metadata).await
}
