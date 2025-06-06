//! mutsea-database/src/error.rs
//! Database error types and result handling

use thiserror::Error;

/// Database-specific errors
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// SQL error
    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),

    /// Migration error
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Constraint violation
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    /// Data not found
    #[error("Data not found: {0}")]
    NotFound(String),

    /// Data already exists
    #[error("Data already exists: {0}")]
    AlreadyExists(String),

    /// Invalid data format
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),

    /// Unsupported database backend
    #[error("Unsupported database backend: {0}")]
    UnsupportedBackend(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Timeout error
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Generic database error
    #[error("{0}")]
    Generic(String),
}

/// Result type for database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

impl From<mutsea_core::MutseaError> for DatabaseError {
    fn from(err: mutsea_core::MutseaError) -> Self {
        match err {
            mutsea_core::MutseaError::Database(msg) => DatabaseError::Generic(msg),
            _ => DatabaseError::Generic(err.to_string()),
        }
    }
}

impl From<DatabaseError> for mutsea_core::MutseaError {
    fn from(err: DatabaseError) -> Self {
        mutsea_core::MutseaError::Database(err.to_string())
    }
}