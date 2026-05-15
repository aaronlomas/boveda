#[cfg(test)]
use crate::vault::BovedaEngine;
#[cfg(test)]
use crate::crypto::secret::{SecretString, SecretKey};
#[cfg(test)]
use crate::error::BovedaError;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
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
        None,
        None
    ).await;
    assert!(matches!(res, Err(BovedaError::VaultLocked)));
}

#[tokio::test]
async fn test_validation_limits() {
    let engine = setup_engine().await;
    // Simulate unlocking
    {
        let mut lock = engine.master_key.lock().expect("Test lock poisoned");
        *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
    }

    // Test site too long
    let long_site = "a".repeat(300);
    let res = engine.add_account(
        SecretString::from(long_site),
        SecretString::from("user"),
        SecretString::from("pass"),
        None,
        None
    ).await;
    assert!(res.is_err());
    
    // Test notes too long
    let long_notes = "a".repeat(20000);
    let res = engine.add_account(
        SecretString::from("site"),
        SecretString::from("user"),
        SecretString::from("pass"),
        None,
        Some(SecretString::from(long_notes))
    ).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_group_management() {
    let engine = setup_engine().await;
    {
        let mut lock = engine.master_key.lock().expect("Test lock poisoned");
        *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
    }

    let id = engine.add_account(
        SecretString::from("s"), SecretString::from("u"), SecretString::from("p"), None, None
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
        let mut lock = engine.master_key.lock().expect("Test lock poisoned");
        *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
    }

    engine.set_preference("lang", "es").await.unwrap();
    assert_eq!(engine.get_preference("lang").await.unwrap(), Some("es".to_string()));
}

#[tokio::test]
async fn test_totp_flow() {
    let engine = setup_engine().await;
    {
        let mut lock = engine.master_key.lock().expect("Test lock poisoned");
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

#[tokio::test]
async fn test_pagination() {
    let engine = setup_engine().await;
    {
        let mut lock = engine.master_key.lock().expect("Test lock poisoned");
        *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
    }

    // Add 5 accounts
    for i in 1..=5 {
        engine.add_account(
            SecretString::from(format!("site{}", i)),
            SecretString::from("user"),
            SecretString::from("pass"),
            None,
            None
        ).await.unwrap();
    }

    // Get page 1 (2 items)
    let page1 = engine.get_accounts_paged(2, 0).await.unwrap();
    assert_eq!(page1.len(), 2);
    
    // Get page 2 (2 items)
    let page2 = engine.get_accounts_paged(2, 2).await.unwrap();
    assert_eq!(page2.len(), 2);
    
    // Ensure page 1 and page 2 are different
    for p1 in &page1 {
        for p2 in &page2 {
            assert_ne!(p1.id, p2.id);
        }
    }

    // Get page 3 (remaining 1 item)
    let page3 = engine.get_accounts_paged(2, 4).await.unwrap();
    assert_eq!(page3.len(), 1);
}

#[tokio::test]
async fn test_deduplication() {
    let engine = setup_engine().await;
    {
        let mut lock = engine.master_key.lock().expect("Test lock poisoned");
        *lock = Some(crate::vault::MasterKey::new(SecretKey::new([0u8; 32])));
    }

    let site = SecretString::from("unique.com");
    let user = SecretString::from("admin");
    let pass = SecretString::from("secret");

    // Add once
    engine.add_account(site.clone(), user.clone(), pass.clone(), None, None).await.unwrap();
    
    // Create an export package manually (simplified)
    use crate::vault::export::*;
    let payload = ExportPayload {
        accounts: vec![
            ExportAccount {
                site: site.clone(),
                username: user.clone(),
                password: pass.clone(),
                recovery_code: None,
                notes: None,
                group_name: None,
            }
        ],
        pins: vec![],
        preferences: vec![],
        timestamp: "2024".to_string(),
    };
    
    let export_pass = SecretString::from("export");
    let package = ExportPackage::encrypt(&payload, &export_pass).unwrap();
    let package_json = serde_json::to_string(&package).unwrap();

    // Import with Merge strategy
    engine.import_vault(&package_json, &export_pass, crate::vault::ImportStrategy::Merge).await.unwrap();

    // Verify it's NOT duplicated
    let accounts = engine.get_accounts().await.unwrap();
    assert_eq!(accounts.len(), 1);
}

#[test]
fn test_master_key_memory_protection() {
    // Just verify it can be created and dropped without panic
    let key = SecretKey::new([0u8; 32]);
    let mk = crate::vault::MasterKey::new(key);
    drop(mk);
}
