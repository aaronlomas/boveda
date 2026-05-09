use std::fmt;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
use subtle::{Choice, ConstantTimeEq};

/// A wrapper for sensitive byte arrays that zeroizes its contents upon drop.
#[derive(Clone, PartialEq)]
pub struct SecretBytes(Vec<u8>);

impl SecretBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl ConstantTimeEq for SecretBytes {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.as_slice().ct_eq(other.0.as_slice())
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl fmt::Debug for SecretBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretBytes([REDACTED])")
    }
}

/// A wrapper for fixed-size 32-byte cryptographic keys to prevent heap reallocations.
#[derive(Clone, PartialEq)]
pub struct SecretKey([u8; 32]);

impl SecretKey {
    pub fn new(key: [u8; 32]) -> Self {
        Self(key)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

impl ConstantTimeEq for SecretKey {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretKey([REDACTED])")
    }
}

/// A wrapper for sensitive strings that zeroizes its contents upon drop.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    pub fn into_bytes(self) -> SecretBytes {
        let mut s = self;
        let bytes = std::mem::take(&mut s.0).into_bytes();
        SecretBytes::new(bytes)
    }

    /// Constant-time comparison against another string.
    pub fn ct_eq_str(&self, other: &str) -> bool {
        self.0.as_bytes().ct_eq(other.as_bytes()).into()
    }
}

impl ConstantTimeEq for SecretString {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.as_bytes().ct_eq(other.0.as_bytes())
    }
}

impl From<&str> for SecretString {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl PartialEq<&str> for SecretString {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for SecretString {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq for SecretString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Drop for SecretString {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretString([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_string_redacted() {
        let s = SecretString::new("secret".to_string());
        assert_eq!(format!("{:?}", s), "SecretString([REDACTED])");
    }

    #[test]
    fn test_secret_key_redacted() {
        let k = SecretKey::new([0u8; 32]);
        assert_eq!(format!("{:?}", k), "SecretKey([REDACTED])");
    }

    #[test]
    fn test_secret_bytes_redacted() {
        let b = SecretBytes::new(vec![0u8; 10]);
        assert_eq!(format!("{:?}", b), "SecretBytes([REDACTED])");
    }

    #[test]
    fn test_constant_time_eq() {
        let s1 = SecretString::new("aaa".to_string());
        let s2 = SecretString::new("aaa".to_string());
        let s3 = SecretString::new("bbb".to_string());
        
        assert!(bool::from(s1.ct_eq(&s2)));
        assert!(!bool::from(s1.ct_eq(&s3)));
    }

    #[test]
    fn test_secret_string_to_bytes() {
        let s = SecretString::new("hello".to_string());
        let b = s.into_bytes();
        assert_eq!(b.as_bytes(), b"hello");
    }

    #[test]
    fn test_partial_eq_implementations() {
        let s = SecretString::new("test".to_string());
        assert_eq!(s, "test");
        assert_eq!(s, "test".to_string());
        assert_eq!(s, SecretString::new("test".to_string()));
    }
}
