use crate::error::{BovedaError, BovedaResult};

pub const MAX_SITE_LEN: usize = 256;
pub const MAX_USERNAME_LEN: usize = 256;
pub const MAX_PASSWORD_LEN: usize = 512;
pub const MAX_NOTES_LEN: usize = 16384;
pub const MAX_GROUP_NAME_LEN: usize = 64;
pub const MAX_PREF_KEY_LEN: usize = 64;
pub const MAX_PREF_VALUE_LEN: usize = 8192;
pub const MAX_PIN_NAME_LEN: usize = 128;
pub const MAX_PIN_LEN: usize = 64;

pub fn validate_string(s: &str, field: &str, max_len: usize, required: bool) -> BovedaResult<()> {
    let trimmed = s.trim();
    if required && trimmed.is_empty() {
        return Err(BovedaError::EmptyInput { field: field.to_string() });
    }
    if s.len() > max_len {
        return Err(BovedaError::InputTooLong {
            field: field.to_string(),
            max: max_len,
        });
    }
    Ok(())
}
