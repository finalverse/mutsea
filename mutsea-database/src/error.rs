// /mutsea/mutsea-database/src/error.rs
//! Database error types

use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    Connection(String),
    Query(String),
    Serialization(String),
    NotFound(String),
    Validation(String),
    Internal(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::Connection(msg) => write!(f, "Database connection error: {}", msg),
            DatabaseError::Query(msg) => write!(f, "Database query error: {}", msg),
            DatabaseError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            DatabaseError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DatabaseError::Validation(msg) => write!(f, "Validation error: {}", msg),
            DatabaseError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

// Convert from common database errors
impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DatabaseError::NotFound("Row not found".to_string()),
            sqlx::Error::Database(db_err) => DatabaseError::Query(db_err.to_string()),
            sqlx::Error::Io(io_err) => DatabaseError::Connection(io_err.to_string()),
            _ => DatabaseError::Internal(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> Self {
        DatabaseError::Serialization(err.to_string())
    }
}