use crate::crypto::secret::SecretBytes;
use crate::error::BovedaResult;
use totp_rs::{Algorithm, TOTP};
use serde::{Serialize, Deserialize};
use base64::Engine;
use rand::RngCore;
use rand::rngs::OsRng;
use zeroize::Zeroizing;

#[derive(Debug, Serialize, Deserialize)]
pub struct TotpSetupPayload {
    pub otpauth_url: String,
    pub qr_png_b64: String,
    pub recovery_codes: Vec<String>,
}

pub struct TotpManager;

impl TotpManager {
    /// Generates a new 20-byte random secret for TOTP.
    pub fn generate_secret() -> SecretBytes {
        let mut bytes = vec![0u8; 20];
        OsRng.fill_bytes(&mut bytes);
        SecretBytes::new(bytes)
    }

    /// Creates a TOTP instance from the given secret bytes.
    fn create_totp(secret: &SecretBytes) -> BovedaResult<TOTP> {
        let secret_bytes = secret.as_bytes();
        let seed = Zeroizing::new(secret_bytes.to_vec());
        
        TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            seed.to_vec(),
            Some("Bóveda".to_string()),
            "vault".to_string(),
        ).map_err(|e| crate::error::BovedaError::CryptoError(format!("TOTP init error: {e}")))
    }

    /// Generates the otpauth URL for QR codes.
    pub fn get_otpauth_url(secret: &SecretBytes) -> BovedaResult<String> {
        let totp = Self::create_totp(secret)?;
        Ok(totp.get_url())
    }

    /// Generates the QR code PNG as a base64 string.
    pub fn generate_qr_png_b64(secret: &SecretBytes) -> BovedaResult<String> {
        let url = Self::get_otpauth_url(secret)?;
        let code = qrcode::QrCode::new(url.as_bytes())
            .map_err(|e| crate::error::BovedaError::CryptoError(format!("QR generation error: {e}")))?;
            
        let image = code.render::<image::Luma<u8>>().build();
        
        let mut png_bytes = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        image.write_with_encoder(encoder)
            .map_err(|e| crate::error::BovedaError::CryptoError(format!("PNG encoding error: {e}")))?;
        
        Ok(base64::engine::general_purpose::STANDARD.encode(&png_bytes))
    }

    /// Verifies the 6-digit TOTP code.
    pub fn verify(secret: &SecretBytes, code: &str) -> BovedaResult<bool> {
        let totp = Self::create_totp(secret)?;
        Ok(totp.check_current(code).unwrap_or(false))
    }

    /// Generates 10 random recovery codes (12 chars each).
    pub fn generate_recovery_codes() -> Vec<String> {
        let mut rng = OsRng;
        let charset = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"; // Removed ambiguous chars (O, 0, I, 1)
        
        (0..10).map(|_| {
            let mut code = String::with_capacity(14);
            for i in 0..12 {
                if i > 0 && i % 4 == 0 {
                    code.push('-');
                }
                use rand::Rng;
                let idx = rng.gen_range(0..charset.len());
                code.push(charset.chars().nth(idx).unwrap());
            }
            code
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        let s1 = TotpManager::generate_secret();
        let s2 = TotpManager::generate_secret();
        assert_eq!(s1.as_bytes().len(), 20);
        assert_ne!(s1.as_bytes(), s2.as_bytes());
    }

    #[test]
    fn test_generate_recovery_codes() {
        let codes = TotpManager::generate_recovery_codes();
        assert_eq!(codes.len(), 10);
        for code in codes {
            assert_eq!(code.len(), 14); // 12 chars + 2 dashes
            assert!(code.contains('-'));
        }
    }

    #[test]
    fn test_get_otpauth_url() {
        let secret = TotpManager::generate_secret();
        let url = TotpManager::get_otpauth_url(&secret).unwrap();
        assert!(url.contains("otpauth://totp/"));
        assert!(url.contains("secret="));
    }

    #[test]
    fn test_generate_qr_png_b64() {
        let secret = TotpManager::generate_secret();
        let b64 = TotpManager::generate_qr_png_b64(&secret).unwrap();
        assert!(!b64.is_empty());
        // Verify it's valid base64
        base64::engine::general_purpose::STANDARD.decode(b64).unwrap();
    }

    #[test]
    fn test_totp_verify_sanity() {
        let secret = TotpManager::generate_secret();
        // Since we can't easily predict the current code without a time library,
        // we just check that a random code fails.
        assert!(!TotpManager::verify(&secret, "000000").unwrap());
    }
}
