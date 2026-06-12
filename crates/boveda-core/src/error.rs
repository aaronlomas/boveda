use thiserror::Error;

#[derive(Error, Debug)]
pub enum BovedaError {
    #[error("Bóveda está bloqueado")]
    VaultLocked,

    #[error("Incorrect password or corrupted file")]
    InvalidPassword,

    #[error("Invalid or expired TOTP code")]
    InvalidTotpCode,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Decoding error: {0}")]
    DecodeError(String),

    #[error("Data too long: {field} (máx {max} length)")]
    InputTooLong { field: String, max: usize },

    #[error("The name cannot be empty.: {field}")]
    EmptyInput { field: String },

    #[error("E/S Error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("The resource was not found.: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

pub type BovedaResult<T> = Result<T, BovedaError>;

impl From<sqlx::Error> for BovedaError {
    fn from(e: sqlx::Error) -> Self {
        // SOC2: Avoid leaking database schema or technical details.
        // Log the real error internally if needed, but the public error is generic.
        match e {
            sqlx::Error::RowNotFound => Self::NotFound("Recurso no encontrado".to_string()),
            _ => Self::DatabaseError("Error interno de persistencia".to_string()),
        }
    }
}

impl From<std::io::Error> for BovedaError {
    fn from(e: std::io::Error) -> Self {
        // SOC2: Avoid leaking file paths or system structure.
        match e.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound("Archivo no encontrado".to_string()),
            std::io::ErrorKind::PermissionDenied => Self::CryptoError("Permiso denegado al acceder a datos".to_string()),
            _ => Self::IoError("Error de acceso a disco".to_string()),
        }
    }
}

impl From<serde_json::Error> for BovedaError {
    fn from(_: serde_json::Error) -> Self {
        Self::SerializationError("Error al procesar formato de datos".to_string())
    }
}

impl From<base64::DecodeError> for BovedaError {
    fn from(_: base64::DecodeError) -> Self {
        Self::DecodeError("Error de formato en cadena codificada".to_string())
    }
}

impl From<anyhow::Error> for BovedaError {
    fn from(_: anyhow::Error) -> Self {
        Self::Other("Error interno del sistema".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversions() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let bev_err: BovedaError = io_err.into();
        assert!(format!("{}", bev_err).contains("Archivo no encontrado"));

        let b64_err = base64::DecodeError::InvalidByte(0, 0);
        let bev_err: BovedaError = b64_err.into();
        assert!(format!("{}", bev_err).contains("Error de formato"));

        let json_err = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        let bev_err: BovedaError = json_err.into();
        assert!(format!("{}", bev_err).contains("Error al procesar formato"));
    }
}
