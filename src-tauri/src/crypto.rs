use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use anyhow::{anyhow, Result};
use rand::Rng;

/// Derive a 32-byte key from `password` and `salt` using Argon2id.
/// Params: t=3 iterations, m=65536 KiB, p=4 lanes — OWASP recommended.
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let params = Params::new(
        65536, // memory (KiB)
        3,     // iterations
        4,     // parallelism
        Some(32),
    )
    .map_err(|e| anyhow!("Argon2 params error: {e}"))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("Argon2 KDF error: {e}"))?;
    Ok(key)
}

/// Encrypt `plaintext` with AES-256-GCM using `key`.
/// Returns Base64(nonce || ciphertext_with_tag).
pub fn encrypt(plaintext: &str, key: &[u8; 32]) -> Result<String> {
    let aes_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(aes_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit random nonce

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| anyhow!("AES-GCM encrypt error: {e}"))?;

    // Concat nonce (12 bytes) + ciphertext+tag
    let mut payload = nonce.to_vec();
    payload.extend_from_slice(&ciphertext);

    Ok(B64.encode(payload))
}

/// Decrypt a Base64-encoded AES-256-GCM blob produced by `encrypt`.
pub fn decrypt(encoded: &str, key: &[u8; 32]) -> Result<String> {
    let payload = B64.decode(encoded)?;
    if payload.len() < 12 {
        return Err(anyhow!("Ciphertext too short"));
    }

    let (nonce_bytes, ciphertext) = payload.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let aes_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(aes_key);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("AES-GCM decrypt error: {e}"))?;

    String::from_utf8(plaintext).map_err(|e| anyhow!("UTF-8 error: {e}"))
}

/// Generate a cryptographically random password.
pub fn generate_password(length: usize, use_symbols: bool) -> String {
    let mut rng = OsRng;

    let lowercase = b"abcdefghijklmnopqrstuvwxyz";
    let uppercase = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = b"0123456789";
    let symbols = b"!@#$%^&*()-_=+[]{}|;:,.<>?";

    let mut charset: Vec<u8> = Vec::new();
    charset.extend_from_slice(lowercase);
    charset.extend_from_slice(uppercase);
    charset.extend_from_slice(digits);
    if use_symbols {
        charset.extend_from_slice(symbols);
    }

    // Ensure at least one char from each category
    let mut password: Vec<u8> = vec![
        lowercase[rng.gen_range(0..lowercase.len())],
        uppercase[rng.gen_range(0..uppercase.len())],
        digits[rng.gen_range(0..digits.len())],
    ];

    if use_symbols {
        password.push(symbols[rng.gen_range(0..symbols.len())]);
    }

    // Fill the rest
    while password.len() < length {
        password.push(charset[rng.gen_range(0..charset.len())]);
    }

    // Shuffle via Fisher-Yates
    for i in (1..password.len()).rev() {
        let j = rng.gen_range(0..=i);
        password.swap(i, j);
    }

    String::from_utf8(password).expect("All chars are valid ASCII")
}
