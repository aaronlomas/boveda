use serde::{Deserialize, Serialize};

use crate::crypto::secret::SecretString;

/// A decrypted account entry as returned to the frontend.
/// Passwords and notes are decrypted in memory before serialization —
/// they are never stored in plaintext on disk.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub site: SecretString,
    pub username: SecretString,
    pub password_cipher: String,
    pub notes_cipher: Option<String>,
    pub favicon_url: Option<String>,
    pub group_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
/// A decrypted PIN entry as returned to the frontend.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pin {
    pub id: String,
    pub name: SecretString,
    pub encrypted_pin: String,
    pub encrypted_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
