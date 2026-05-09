#[cfg(test)]
mod tests {
    use crate::vault::BovedaEngine;
    use crate::crypto::secret::{SecretString, SecretKey};
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
        let engine = setup_engine().await;
        // Simulate unlocking (manually set key for testing purposes if possible, 
        // but here we just use open_unencrypted and then we need to set the key)
        {
            let mut lock = engine.master_key.lock().unwrap();
            *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
        }

        // Test site too long
        let long_site = "a".repeat(300);
        let res = engine.add_account(
            SecretString::from(long_site),
            SecretString::from("user"),
            SecretString::from("pass"),
            None
        ).await;
        assert!(res.is_err());
        
        // Test notes too long
        let long_notes = "a".repeat(20000);
        let res = engine.add_account(
            SecretString::from("site"),
            SecretString::from("user"),
            SecretString::from("pass"),
            Some(SecretString::from(long_notes))
        ).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_group_management() {
        let engine = setup_engine().await;
        {
            let mut lock = engine.master_key.lock().unwrap();
            *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
        }

        let id = engine.add_account(
            SecretString::from("s"), SecretString::from("u"), SecretString::from("p"), None
        ).await.unwrap();

        engine.update_account_group(&id, Some("Work")).await.unwrap();
        let accounts = engine.get_accounts().await.unwrap();
        assert_eq!(accounts[0].group_name, Some("Work".to_string()));

        engine.rename_group("Work", "Jobs").await.unwrap();
        let accounts = engine.get_accounts().await.unwrap();
        assert_eq!(accounts[0].group_name, Some("Jobs".to_string()));

        // Cannot delete group if accounts are in it
        let res = engine.delete_group("Jobs").await;
        assert!(res.is_err());

        engine.update_account_group(&id, None).await.unwrap();
        engine.delete_group("Jobs").await.unwrap();
    }

    #[tokio::test]
    async fn test_preferences_flow() {
        let engine = setup_engine().await;
        {
            let mut lock = engine.master_key.lock().unwrap();
            *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
        }

        engine.set_preference("lang", "es").await.unwrap();
        assert_eq!(engine.get_preference("lang").await.unwrap(), Some("es".to_string()));
    }

    #[tokio::test]
    async fn test_totp_flow() {
        let engine = setup_engine().await;
        {
            let mut lock = engine.master_key.lock().unwrap();
            *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
        }

        // 1. Setup TOTP
        let setup = engine.setup_totp().await.unwrap();
        assert!(!setup.otpauth_url.is_empty());
        assert!(!setup.qr_png_b64.is_empty());
        assert_eq!(setup.recovery_codes.len(), 10);

        // 2. Initially not enabled (until verified)
        assert!(!engine.is_totp_enabled().await.unwrap());

        // 3. Verify with recovery code
        let valid_recovery = &setup.recovery_codes[0];
        assert!(engine.verify_totp_recovery(valid_recovery).await.unwrap());
        
        // 4. After recovery, it should be disabled/reset
        assert!(!engine.is_totp_enabled().await.unwrap());
        assert!(engine.get_preference("totp_secret_cipher").await.unwrap().is_none());

        // 5. Setup again and test wrong code
        engine.setup_totp().await.unwrap();
        assert!(!engine.verify_totp("000000").await.unwrap());
        
        // 6. Test invalid recovery
        let res = engine.verify_totp_recovery("INVALID").await.unwrap();
        assert!(!res);
    }

    #[tokio::test]
    async fn test_full_unlock_flow_with_files() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join(format!("test_vault_{}.bvda", uuid::Uuid::new_v4()));
        let salt_path = db_path.with_file_name("vault.salt");
        let password = SecretString::from("correct_password");

        // 1. Initial unlock (should create salt and db)
        let engine = BovedaEngine::unlock(&db_path, &password).await.unwrap();
        assert!(salt_path.exists());
        assert!(!engine.is_locked());
        engine.close().await;

        // 2. Unlock again with correct password
        let engine = BovedaEngine::unlock(&db_path, &password).await.unwrap();
        assert!(!engine.is_locked());
        engine.close().await;

        // 3. Unlock with wrong password
        let wrong_password = SecretString::from("wrong");
        let res = BovedaEngine::unlock(&db_path, &wrong_password).await;
        assert!(res.is_err());

        // Cleanup
        let _ = std::fs::remove_file(&db_path);
        let _ = std::fs::remove_file(&salt_path);
    }

    #[test]
    fn test_master_key_memory_protection() {
        // Just verify it can be created and dropped without panic
        let key = SecretKey::new([0u8; 32]);
        let mk = crate::vault::MasterKey::new(key);
        drop(mk);
    }
}
