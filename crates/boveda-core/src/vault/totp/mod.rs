use crate::vault::BovedaEngine;
use crate::error::{BovedaError, BovedaResult};
use crate::crypto;
use crate::crypto::secret::SecretString;
use crate::auth::{TotpManager, TotpSetupPayload};
use base64::Engine;
use subtle::ConstantTimeEq;

impl BovedaEngine {
    // ─── TOTP (2FA) Management ─────────────────────────────────────────────────

    /// Initializes 2FA for the vault.
    /// Generates a new seed, encrypts it with the master key, and saves it.
    pub async fn setup_totp(&self) -> BovedaResult<TotpSetupPayload> {
        self.check_unlocked()?;

        // SEC-4: Prevent overwriting existing setup without explicit disable
        if self.get_preference("totp_secret_cipher").await?.is_some() {
            return Err(BovedaError::Other("TOTP ya está configurado. Desactívalo primero para re-vincular.".to_string()));
        }

        // 1. Generate a new seed (20 bytes random)
        let seed = TotpManager::generate_secret();

        // 2. Encrypt the seed using the master key
        let seed_b64 = base64::engine::general_purpose::STANDARD.encode(seed.as_bytes());
        let encrypted_seed = self.with_key(|key| {
            crypto::encrypt(&SecretString::from(seed_b64), key)
                .map_err(|e| BovedaError::CryptoError(e.to_string()))
        })??;

        // 4. Generate recovery codes
        let recovery_codes = TotpManager::generate_recovery_codes();
        
        // 5. Encrypt recovery codes
        let recovery_json = serde_json::to_string(&recovery_codes)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
            
        let encrypted_recovery = self.with_key(|key| {
            crypto::encrypt(&SecretString::from(recovery_json), key)
                .map_err(|e| BovedaError::CryptoError(e.to_string()))
        })??;
        
        // 6. Persist encrypted data
        self.set_preference("totp_secret_cipher", &encrypted_seed).await?;
        self.set_preference("totp_recovery_cipher", &encrypted_recovery).await?;

        // 7. Return the QR, URL and codes for the frontend
        Ok(TotpSetupPayload {
            otpauth_url: TotpManager::get_otpauth_url(&seed)?,
            qr_png_b64: TotpManager::generate_qr_png_b64(&seed)?,
            recovery_codes,
        })
    }

    /// Verifies a TOTP code against the persisted encrypted secret.
    pub async fn verify_totp(&self, code: &str) -> BovedaResult<bool> {
        self.check_unlocked()?;

        let cipher = self.get_preference("totp_secret_cipher").await?
            .ok_or_else(|| BovedaError::Other("TOTP no está configurado".to_string()))?;

        // Decrypt the seed (Base64 string)
        let seed_b64 = self.with_key(|key| {
            crypto::decrypt(&cipher, key)
                .map_err(|e| BovedaError::CryptoError(e.to_string()))
        })??;

        // Decode base64 to raw bytes
        let seed_bytes = base64::engine::general_purpose::STANDARD.decode(seed_b64.as_str())
            .map_err(|e: base64::DecodeError| BovedaError::CryptoError(e.to_string()))?;
        
        let seed = crate::crypto::secret::SecretBytes::new(seed_bytes);
        let valid = TotpManager::verify(&seed, code)?;
        
        if valid {
            // Enable TOTP now that we know the user has verified it
            self.set_preference("totp_enabled", "true").await?;
        }

        Ok(valid)
    }

    /// Disables 2FA by removing the encrypted seed and recovery codes from storage.
    pub async fn disable_totp(&self) -> BovedaResult<()> {
        self.check_unlocked()?;

        let mut tx = self.db.begin().await?;
        
        // Remove all keys to ensure permanent destruction
        sqlx::query("DELETE FROM preferences WHERE key = 'totp_secret_cipher'")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM preferences WHERE key = 'totp_enabled'")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM preferences WHERE key = 'totp_recovery_cipher'")
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    /// Verifies a recovery code and, if valid, completely disables 2FA to allow for a fresh re-linking.
    pub async fn verify_totp_recovery(&self, input_code: &str) -> BovedaResult<bool> {
        self.check_unlocked()?;
        
        let cipher = self.get_preference("totp_recovery_cipher").await?
            .ok_or_else(|| BovedaError::Other("Códigos de recuperación no configurados".to_string()))?;
            
        // Decrypt the recovery list
        let recovery_json = self.with_key(|key| {
            crypto::decrypt(&cipher, key)
                .map_err(|e| BovedaError::CryptoError(e.to_string()))
        })??;
        
        let codes: Vec<String> = serde_json::from_str(recovery_json.as_str())
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
            
        // Check if code exists (constant-time check for each code)
        let normalized_input = input_code.trim().to_uppercase();
        let mut found = false;
        for c in codes {
            if c.to_uppercase().as_bytes().ct_eq(normalized_input.as_bytes()).into() {
                found = true;
                break; // SEC: Early exit found
            }
        }

        if found {
            // Valid recovery code! Reset 2FA entirely.
            let mut tx = self.db.begin().await?;
            
            sqlx::query("DELETE FROM preferences WHERE key = 'totp_secret_cipher'")
                .execute(&mut *tx)
                .await?;
            sqlx::query("DELETE FROM preferences WHERE key = 'totp_enabled'")
                .execute(&mut *tx)
                .await?;
            sqlx::query("DELETE FROM preferences WHERE key = 'totp_recovery_cipher'")
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns true if TOTP 2FA is currently enabled.
    pub async fn is_totp_enabled(&self) -> BovedaResult<bool> {
        let enabled = self.get_preference("totp_enabled").await?;
        Ok(enabled.as_deref() == Some("true"))
    }
}
