//! Error Handling (thiserror, anyhow, custom error)

use thiserror::Error;

/// App-level error (customize as needed)
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Example usage:
// return Err(AppError::ValidationError("Invalid data".to_string()));
