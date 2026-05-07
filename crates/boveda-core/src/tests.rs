#[cfg(test)]
mod tests {
    use crate::vault::BovedaEngine;
    use crate::crypto::secret::SecretString;
    use crate::error::BovedaError;
    use std::path::PathBuf;

    async fn setup_engine() -> BovedaEngine {
        // Use an in-memory database for testing
        let engine = BovedaEngine::open_unencrypted(&PathBuf::from(":memory:")).await.unwrap();
        crate::storage::init_db(&engine.db).await.unwrap();
        engine
    }

    #[tokio::test]
    async fn test_locking_guards() {
        let engine = setup_engine().await;
        // Initially locked
        assert!(engine.is_locked());

        let res = engine.get_accounts().await;
        assert!(matches!(res, Err(BovedaError::VaultLocked)));

        let res = engine.add_account(
            SecretString::from("site"),
            SecretString::from("user"),
            SecretString::from("pass"),
            None
        ).await;
        assert!(matches!(res, Err(BovedaError::VaultLocked)));
    }

    #[tokio::test]
    async fn test_validation_limits() {
        // Validation tests
    }
}
