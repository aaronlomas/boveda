use crate::crypto::secret::SecretString;
use uuid::Uuid;

impl super::AppState {
    pub async fn cmd_add_account(
        &self,
        site: SecretString,
        username: SecretString,
        password: SecretString,
        recovery_code: SecretString,
        notes: SecretString,
    ) -> Result<String, String> {
        let engine = self.get_engine()?;
        let recovery_opt = if recovery_code.as_str().is_empty() {
            None
        } else {
            Some(recovery_code)
        };
        let notes_opt = if notes.as_str().is_empty() { None } else { Some(notes) };
        engine
            .add_account(site, username, password, recovery_opt, notes_opt)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_get_accounts(&self) -> Result<Vec<crate::storage::models::Account>, String> {
        let engine = self.get_engine()?;
        engine.get_accounts().await.map_err(|e| e.to_string())
    }

    pub async fn cmd_delete_account(&self, id: &str) -> Result<(), String> {
        Uuid::parse_str(id)
            .map_err(|_| format!("Invalid account ID: '{}'. Must be a valid UUID.", id))?;

        let engine = self.get_engine()?;
        engine.delete_account(id).await.map_err(|e| e.to_string())
    }
}
