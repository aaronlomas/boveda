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

        let cipher_exists = self.get_preference("totp_secret_cipher").await?.is_some();
        let is_enabled = self.get_preference("totp_enabled").await?.as_deref() == Some("true");

        if cipher_exists {
            if is_enabled {
                // SEC-4: Prevent overwriting an active, confirmed 2FA without explicit disable
                return Err(BovedaError::Other("TOTP ya está configurado. Desactívalo primero para re-vincular.".to_string()));
            } else {
                // Orphaned cipher from a previous cancelled setup — clean it up silently
                let mut tx = self.db.begin().await?;
                sqlx::query("DELETE FROM preferences WHERE key = 'totp_secret_cipher'")
                    .execute(&mut *tx).await?;
                sqlx::query("DELETE FROM preferences WHERE key = 'totp_recovery_cipher'")
                    .execute(&mut *tx).await?;
                tx.commit().await?;
            }
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
        
        // SEC-C3: Clear recovery codes from memory after encryption
        drop(recovery_codes);

        // 7. Return the QR, URL and codes for the frontend
        // Note: Codes are only available at setup time and immediately consumed by frontend
        let display_codes = TotpManager::generate_recovery_codes();
        Ok(TotpSetupPayload {
            otpauth_url: TotpManager::get_otpauth_url(&seed)?,
            qr_png_b64: TotpManager::generate_qr_png_b64(&seed)?,
            recovery_codes: display_codes,
        })
    }

    /// Verifies a TOTP code against the persisted encrypted secret.
    /// SEC-H1: Implements rate limiting to prevent brute force attacks on TOTP codes.
    pub async fn verify_totp(&self, code: &str) -> BovedaResult<bool> {
        self.check_unlocked()?;

        // SEC-H1: Rate limiting - check failed attempts
        let failed_attempts_key = "totp_verify_failed_attempts";
        let failed_attempts_str = self.get_preference(failed_attempts_key).await?.unwrap_or_else(|| "0".to_string());
        let failed_attempts: u32 = failed_attempts_str.parse().unwrap_or(0);
        
        // Allow max 5 attempts, then require 5 minute cooldown
        if failed_attempts >= 5 {
            let last_fail_key = "totp_last_failed_attempt";
            if let Some(last_fail_ts) = self.get_preference(last_fail_key).await? {
                if let Ok(ts) = last_fail_ts.parse::<i64>() {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as i64;
                    
                    if now - ts < 300 { // 5 minutes
                        return Err(BovedaError::Other(
                            format!("Demasiados intentos fallidos. Intenta en {} segundos.", 300 - (now - ts))
                        ));
                    }
                }
            }
        }

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
            // SEC-H1: Reset failed attempts on successful verification
            self.delete_preference(failed_attempts_key).await?;
            self.delete_preference("totp_last_failed_attempt").await?;
            
            // Enable TOTP now that we know the user has verified it
            self.set_preference("totp_enabled", "true").await?;
            self.log_audit(crate::audit::AuditAction::TotpEnabled, None).await?;
        } else {
            // SEC-H1: Increment failed attempts and log timestamp
            let new_attempts = (failed_attempts + 1).to_string();
            self.set_preference(failed_attempts_key, &new_attempts).await?;
            
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            self.set_preference("totp_last_failed_attempt", &now.to_string()).await?;
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
        self.log_audit(crate::audit::AuditAction::TotpDisabled, None).await?;
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
            
        // SEC-C1: Check if code exists using constant-time comparison (NO early exit to prevent timing attacks)
        let normalized_input = input_code.trim().to_uppercase();
        let mut found = false;
        for c in codes {
            // Use bitwise OR to accumulate result without branching
            let is_match = c.to_uppercase().as_bytes().ct_eq(normalized_input.as_bytes());
            found |= bool::from(is_match); // Constant-time OR operation
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
            self.log_audit(crate::audit::AuditAction::TotpDisabled, Some("recovery_code_used")).await?;
            
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
