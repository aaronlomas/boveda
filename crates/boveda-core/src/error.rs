use thiserror::Error;

#[derive(Error, Debug)]
pub enum BovedaError {
    #[error("El baúl está bloqueado")]
    VaultLocked,

    #[error("Contraseña incorrecta o archivo dañado")]
    InvalidPassword,

    #[error("Código TOTP inválido o expirado")]
    InvalidTotpCode,

    #[error("Error de base de datos: {0}")]
    DatabaseError(String),

    #[error("Error criptográfico: {0}")]
    CryptoError(String),

    #[error("Error de decodificación: {0}")]
    DecodeError(String),

    #[error("Dato demasiado largo: {field} (máx {max} caracteres)")]
    InputTooLong { field: String, max: usize },

    #[error("El nombre no puede estar vacío: {field}")]
    EmptyInput { field: String },

    #[error("Error de E/S: {0}")]
    IoError(String),

    #[error("Error de serialización: {0}")]
    SerializationError(String),

    #[error("Error de migración: {0}")]
    MigrationError(String),

    #[error("No se encontró el recurso: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

pub type BovedaResult<T> = Result<T, BovedaError>;

impl From<sqlx::Error> for BovedaError {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseError(e.to_string())
    }
}

impl From<std::io::Error> for BovedaError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for BovedaError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerializationError(e.to_string())
    }
}

impl From<base64::DecodeError> for BovedaError {
    fn from(e: base64::DecodeError) -> Self {
        Self::DecodeError(e.to_string())
    }
}

impl From<anyhow::Error> for BovedaError {
    fn from(e: anyhow::Error) -> Self {
        Self::Other(e.to_string())
    }
}
