use serde::{Deserialize, Serialize};
use crate::crypto::secret::{SecretString, SecretBytes};
use crate::error::BovedaResult;
use crate::crypto;

#[derive(Serialize, Deserialize)]
pub struct ExportPackage {
    pub version: u32,
    pub salt: String, // base64
    pub nonce: String, // base64
    pub payload: String, // encrypted JSON ciphertext (base64)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExportAccount {
    pub site: String,
    pub username: String,
    pub password: String,
    pub recovery_code: Option<String>,
    pub notes: Option<String>,
    pub group_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExportPin {
    pub name: String,
    pub pin: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ExportPayload {
    pub accounts: Vec<ExportAccount>,
    #[serde(default)]
    pub pins: Vec<ExportPin>,
    pub preferences: Vec<(String, String)>,
    pub timestamp: String,
}

impl ExportPackage {
    /// Creates a new encrypted export package from the given payload and password.
    pub fn encrypt(payload: &ExportPayload, password: &SecretString) -> BovedaResult<Self> {
        use rand::RngCore;
        use base64::Engine;

        // 1. Serialize payload to binary (Zeroize-ready)
        let json_data = serde_json::to_vec(payload)
            .map_err(|e| crate::error::BovedaError::SerializationError(e.to_string()))?;
        let secret_json = SecretBytes::new(json_data);

        // 2. Generate new salt for export (independent from master vault)
        let mut salt = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut salt);

        // 3. Derive export key
        let key = crypto::derive_key(password, &salt)?;

        // 4. Encrypt
        let (ciphertext, nonce) = crypto::encrypt_raw(secret_json.as_bytes(), &key)?;

        Ok(Self {
            version: 1,
            salt: base64::engine::general_purpose::STANDARD.encode(salt),
            nonce: base64::engine::general_purpose::STANDARD.encode(nonce),
            payload: base64::engine::general_purpose::STANDARD.encode(ciphertext),
        })
    }

    /// Decrypts an export package into a payload.
    pub fn decrypt(&self, password: &SecretString) -> BovedaResult<ExportPayload> {
        use base64::Engine;

        let salt = base64::engine::general_purpose::STANDARD.decode(&self.salt)?;
        let nonce = base64::engine::general_purpose::STANDARD.decode(&self.nonce)?;
        let ciphertext = base64::engine::general_purpose::STANDARD.decode(&self.payload)?;

        // 1. Derive key
        let key = crypto::derive_key(password, &salt)?;

        // 2. Decrypt
        let secret_plaintext = crypto::decrypt_raw(&ciphertext, &nonce, &key)?;

        // 3. Deserialize
        let payload: ExportPayload = serde_json::from_slice(secret_plaintext.as_bytes())
            .map_err(|e| crate::error::BovedaError::SerializationError(e.to_string()))?;

        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_export_package_roundtrip() {
        let password = SecretString::new("export_pass".to_string());
        
        let payload = ExportPayload {
            accounts: vec![
                ExportAccount {
                    site: "site.com".to_string(),
                    username: "user".to_string(),
                    password: "pass".to_string(),
                    recovery_code: Some("rec-123".to_string()),
                    notes: None,
                    group_name: Some("Work".to_string()),
                }
            ],
            pins: vec![],
            preferences: vec![("theme".to_string(), "dark".to_string())],
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        // 1. Encrypt
        let package = ExportPackage::encrypt(&payload, &password).expect("Encryption failed");
        
        // Ensure some properties of the package
        assert_eq!(package.version, 1);
        assert!(!package.payload.is_empty());
        assert!(!package.salt.is_empty());

        // 2. Decrypt
        let decrypted = package.decrypt(&password).expect("Decryption failed");
        
        // 3. Verify
        assert_eq!(decrypted.accounts.len(), 1);
        assert_eq!(decrypted.accounts[0].site.as_str(), "site.com");
        assert_eq!(decrypted.preferences[0].1, "dark");
    }

    #[test]
    fn test_export_package_wrong_password() {
        let password = SecretString::new("pass1".to_string());
        let wrong_password = SecretString::new("pass2".to_string());
        
        let payload = ExportPayload {
            accounts: vec![],
            pins: vec![],
            preferences: vec![],
            timestamp: "2024".to_string(),
        };

        let package = ExportPackage::encrypt(&payload, &password).unwrap();
        let result = package.decrypt(&wrong_password);
        
        assert!(result.is_err(), "Decryption should fail with wrong password");
    }
}
