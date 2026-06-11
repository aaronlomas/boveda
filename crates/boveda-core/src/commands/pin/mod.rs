use crate::crypto::secret::SecretString;
use uuid::Uuid;

impl super::AppState {
    pub async fn cmd_add_pin(
        &self,
        name: SecretString,
        pin: SecretString,
        notes: SecretString,
    ) -> Result<String, String> {
        let engine = self.get_engine()?;
        let notes_opt = if notes.as_str().is_empty() { None } else { Some(notes) };
        engine
            .add_pin(name, pin, notes_opt)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_get_pins(&self) -> Result<Vec<crate::storage::models::Pin>, String> {
        let engine = self.get_engine()?;
        // SEC: Log access to decrypted pin values
        let _ = engine
            .log_audit(crate::audit::AuditAction::SecretAccess, Some("pin_values"))
            .await;
        engine.get_pins().await.map_err(|e| e.to_string())
    }

    pub async fn cmd_delete_pin(&self, id: &str) -> Result<(), String> {
        Uuid::parse_str(id)
            .map_err(|_| format!("Invalid PIN ID: '{}'. Must be a valid UUID.", id))?;

        let engine = self.get_engine()?;
        engine.delete_pin(id).await.map_err(|e| e.to_string())
    }
}
