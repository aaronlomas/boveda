use crate::crypto::secret::SecretBytes;
use totp_rs::{Algorithm, TOTP};
use serde::{Serialize, Deserialize};
use base64::Engine;
use rand::RngCore;

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
        rand::thread_rng().fill_bytes(&mut bytes);
        SecretBytes::new(bytes)
    }

    /// Creates a TOTP instance from the given secret bytes.
    fn create_totp(secret: &SecretBytes) -> TOTP {
        let secret_bytes = secret.as_bytes();
        
        TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes.to_vec(),
            Some("Bóveda".to_string()),
            "vault".to_string(),
        ).unwrap()
    }

    /// Generates the otpauth URL for QR codes.
    pub fn get_otpauth_url(secret: &SecretBytes) -> String {
        let totp = Self::create_totp(secret);
        totp.get_url()
    }

    /// Generates the QR code PNG as a base64 string.
    pub fn generate_qr_png_b64(secret: &SecretBytes) -> String {
        let url = Self::get_otpauth_url(secret);
        let code = qrcode::QrCode::new(url.as_bytes()).unwrap();
        let image = code.render::<image::Luma<u8>>().build();
        
        let mut png_bytes = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        image.write_with_encoder(encoder).unwrap();
        
        base64::engine::general_purpose::STANDARD.encode(&png_bytes)
    }

    /// Verifies the 6-digit TOTP code.
    pub fn verify(secret: &SecretBytes, code: &str) -> bool {
        let totp = Self::create_totp(secret);
        totp.check_current(code).unwrap_or(false)
    }

    /// Generates 10 random recovery codes (12 chars each).
    pub fn generate_recovery_codes() -> Vec<String> {
        let mut rng = rand::thread_rng();
        let charset = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"; // Removed ambiguous chars (O, 0, I, 1)
        
        (0..10).map(|_| {
            let mut code = String::with_capacity(14);
            for i in 0..12 {
                if i > 0 && i % 4 == 0 {
                    code.push('-');
                }
                let idx = (rng.next_u32() as usize) % charset.len();
                code.push(charset.chars().nth(idx).unwrap());
            }
            code
        }).collect()
    }
}
