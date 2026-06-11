impl super::AppState {
    pub async fn cmd_get_preference(&self, key: &str) -> Result<Option<String>, String> {
        let engine = self.get_engine()?;
        engine
            .get_preference(key)
            .await
            .map_err(|e: crate::BovedaError| e.to_string())
    }

    pub async fn cmd_set_preference(&self, key: &str, value: Option<String>) -> Result<(), String> {
        let engine = self.get_engine()?;
        match value {
            Some(v) => engine
                .set_preference(key, &v)
                .await
                .map_err(|e: crate::BovedaError| e.to_string()),
            None => engine
                .delete_preference(key)
                .await
                .map_err(|e: crate::BovedaError| e.to_string()),
        }
    }
}
