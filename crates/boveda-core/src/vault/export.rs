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

#[derive(Serialize, Deserialize)]
pub struct ExportPayload {
    pub accounts: Vec<crate::storage::AccountRow>,
    pub preferences: Vec<(String, String)>,
    pub timestamp: String,
}

impl ExportPackage {
    /// Creates a new encrypted export package from the given payload and password.
    pub fn encrypt(payload: &ExportPayload, password: &SecretString) -> BovedaResult<Self> {
        use rand::RngCore;
        use base64::Engine;

        // 1. Serialize payload to JSON string (Intermediate)
        let json_data = serde_json::to_string(payload)
            .map_err(|e| crate::error::BovedaError::SerializationError(e.to_string()))?;
        let secret_json = SecretString::new(json_data);

        // 2. Generate new salt for export (independent from master vault)
        let mut salt = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut salt);

        // 3. Derive export key
        let key = crypto::derive_key(password, &salt)?;

        // 4. Encrypt
        let (ciphertext, nonce) = crypto::encrypt_raw(secret_json.as_str().as_bytes(), &key)?;

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
        let plaintext_bytes = crypto::decrypt_raw(&ciphertext, &nonce, &key)?;
        let secret_plaintext = SecretBytes::new(plaintext_bytes);

        // 3. Deserialize
        let payload: ExportPayload = serde_json::from_slice(secret_plaintext.as_bytes())
            .map_err(|e| crate::error::BovedaError::SerializationError(e.to_string()))?;

        Ok(payload)
    }
}
