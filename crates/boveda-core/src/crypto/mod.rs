pub mod secret;

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use anyhow::{anyhow, Result};
use rand::Rng;
use zeroize::Zeroize;
use self::secret::{SecretBytes, SecretString};

/// Derive a 32-byte key from `password` and `salt` using Argon2id.
/// Returns a Boxed array to prevent leaving copies on the stack.
/// Params: t=3 iterations, m=65536 KiB, p=4 lanes — OWASP recommended.
pub fn derive_key(password: &SecretString, salt: &[u8]) -> Result<SecretBytes> {
    let params = Params::new(
        65536, // memory (KiB)
        3,     // iterations
        4,     // parallelism
        Some(32),
    )
    .map_err(|e| anyhow!("Argon2 params error: {e}"))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = SecretBytes::new(vec![0u8; 32]);
    argon2
        .hash_password_into(password.as_str().as_bytes(), salt, key.as_mut_bytes())
        .map_err(|e| {
            // key will be automatically zeroized upon drop if error occurs
            anyhow!("Argon2 KDF error: {e}")
        })?;
    Ok(key)
}

/// Encrypt `plaintext` with ChaCha20-Poly1305 using `key`.
/// Returns Base64(nonce || ciphertext_with_tag).
pub fn encrypt(plaintext: &SecretString, key: &[u8; 32]) -> Result<String> {
    let chacha_key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(chacha_key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bit random nonce

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_str().as_bytes())
        .map_err(|e| anyhow!("ChaCha20Poly1305 encrypt error: {e}"))?;

    // Concat nonce (12 bytes) + ciphertext+tag
    let mut payload = nonce.to_vec();
    payload.extend_from_slice(&ciphertext);

    Ok(B64.encode(payload))
}

/// Decrypt a Base64-encoded ChaCha20-Poly1305 blob produced by `encrypt`.
/// Returns a `SecretString` to ensure the plaintext is zeroized on drop.
pub fn decrypt(encoded: &str, key: &[u8; 32]) -> Result<SecretString> {
    let payload = B64.decode(encoded)?;
    if payload.len() < 12 {
        return Err(anyhow!("Ciphertext too short"));
    }

    let (nonce_bytes, ciphertext) = payload.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let chacha_key = Key::from_slice(key);
    let cipher = ChaCha20Poly1305::new(chacha_key);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("ChaCha20Poly1305 decrypt error: {e}"))?;

    let result = String::from_utf8(plaintext).map_err(|e| {
        let mut err_bytes = e.into_bytes();
        err_bytes.zeroize();
        anyhow!("UTF-8 error")
    })?;
    
    Ok(SecretString::new(result))
}

/// Generate a cryptographically random password.
/// Returns a SecretString.
pub fn generate_password(length: usize, use_symbols: bool) -> SecretString {
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

    let result = String::from_utf8(password).expect("All chars are valid ASCII");
    SecretString::new(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_consistency() {
        let password = SecretString::new("super_secret_password".to_string());
        let salt = b"random_salt_123456";
        let key1 = derive_key(&password, salt).unwrap();
        let key2 = derive_key(&password, salt).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_key_uniqueness() {
        let salt = b"random_salt_123456";
        let key1 = derive_key(&SecretString::new("password_1".to_string()), salt).unwrap();
        let key2 = derive_key(&SecretString::new("password_2".to_string()), salt).unwrap();
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_encrypt_decrypt_success() {
        let key = [42u8; 32];
        let plaintext = SecretString::new("Hello, Bóveda!".to_string());
        let ciphertext = encrypt(&plaintext, &key).unwrap();
        assert_ne!(plaintext.as_str(), ciphertext); // Not plaintext
        
        let decrypted = decrypt(&ciphertext, &key).unwrap();
        assert_eq!(plaintext.as_str(), decrypted.as_str());
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key1 = [42u8; 32];
        let key2 = [43u8; 32];
        let plaintext = SecretString::new("Hello, Bóveda!".to_string());
        let ciphertext = encrypt(&plaintext, &key1).unwrap();
        
        let result = decrypt(&ciphertext, &key2);
        assert!(result.is_err(), "Decrypting with wrong key should fail");
    }

    #[test]
    fn test_decrypt_tampered_ciphertext_fails() {
        let key = [42u8; 32];
        let plaintext = SecretString::new("Hello, Bóveda!".to_string());
        let mut ciphertext = encrypt(&plaintext, &key).unwrap();
        
        // Corrupt the base64 string
        let len = ciphertext.len();
        if ciphertext.as_bytes()[len - 1] == b'=' {
            ciphertext.replace_range(len-2..len-1, "A");
        } else {
            ciphertext.replace_range(len-1..len, "A");
        }

        let result = decrypt(&ciphertext, &key);
        assert!(result.is_err(), "Decrypting tampered ciphertext should fail");
    }

    #[test]
    fn test_generate_password() {
        let pw1 = generate_password(16, true);
        assert_eq!(pw1.as_str().len(), 16);
        let pw2 = generate_password(32, false);
        assert_eq!(pw2.as_str().len(), 32);
        assert_ne!(pw1.as_str(), pw2.as_str());
    }
}
