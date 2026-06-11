impl super::AppState {
    pub async fn cmd_update_account_group(
        &self,
        id: &str,
        group_name: Option<&str>,
    ) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine
            .update_account_group(id, group_name)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_rename_group(&self, old_name: &str, new_name: &str) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine
            .rename_group(old_name, new_name)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_delete_group(&self, name: &str) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine.delete_group(name).await.map_err(|e| e.to_string())
    }
}
