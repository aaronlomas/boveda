use crate::storage;
use crate::storage::AUDIT_LOG_MAX_ROWS;

pub struct AuditLogView {
    pub id: i64,
    pub action: String,
    pub metadata: Option<String>,
    pub created_at: String,
}

impl super::AppState {
    pub async fn cmd_get_audit_logs(&self, limit: i64) -> Result<Vec<crate::commands::AuditLogView>, String> {
        let engine = self.get_engine()?;
        let logs = storage::get_audit_logs(&engine.db, limit)
            .await
            .map_err(|e| e.to_string())?;

        let view = logs
            .into_iter()
            .map(|(id, action, meta, created_at)| crate::commands::AuditLogView {
                id,
                action,
                metadata: meta,
                created_at,
            })
            .collect();

        Ok(view)
    }

    /// Deletes all audit log entries (manual reset).
    pub async fn cmd_clear_audit_logs(&self) -> Result<(), String> {
        let engine = self.get_engine()?;
        storage::clear_audit_log(&engine.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Returns the configured maximum number of retained audit log rows.
    pub fn cmd_get_audit_log_limit(&self) -> i64 {
        AUDIT_LOG_MAX_ROWS
    }
}
