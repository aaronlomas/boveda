use crate::error::{BovedaError, BovedaResult};
use crate::storage;
use crate::vault::validation;

impl super::BovedaEngine {
    pub async fn update_account_group(&self, id: &str, group_name: Option<&str>) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::AccountGroupUpdate, Some(id)).await?;
        if let Some(name) = group_name {
            validation::validate_string(name, "Grupo", validation::MAX_GROUP_NAME_LEN, true)?;
        }
        storage::update_account_group(&self.db, id, group_name).await
    }

    pub async fn rename_group(&self, old_name: &str, new_name: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        validation::validate_string(new_name, "Grupo", validation::MAX_GROUP_NAME_LEN, true)?;

        let mut tx = self.db.begin().await?;

        storage::rename_group_tx(&mut tx, old_name, new_name).await?;

        let raw = storage::get_preference_tx(&mut tx, "groups").await?;
        let mut groups: Vec<String> = raw
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
            
        if let Some(pos) = groups.iter().position(|g| g == old_name) {
            groups[pos] = new_name.to_string();
        }
        
        let serialized = serde_json::to_string(&groups)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
        storage::set_preference_tx(&mut tx, "groups", &serialized).await?;
        
        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_group(&self, name: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        let count = storage::count_accounts_in_group(&self.db, name).await?;
        if count > 0 {
            return Err(BovedaError::Other(format!(
                "El grupo \"{}\" tiene {} cuenta(s) asignada(s). Mueve las cuentas antes de eliminarlo.",
                name, count
            )));
        }

        let mut tx = self.db.begin().await?;
        storage::delete_group_tx(&mut tx, name).await?;

        let raw = storage::get_preference_tx(&mut tx, "groups").await?;
        let mut groups: Vec<String> = raw
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
            
        groups.retain(|g| g != name);
        let serialized = serde_json::to_string(&groups)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
        storage::set_preference_tx(&mut tx, "groups", &serialized).await?;
        
        tx.commit().await?;
        Ok(())
    }
}
