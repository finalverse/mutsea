//! Error types for Mutsea

use thiserror::Error;

/// Main error type for Mutsea operations
#[derive(Error, Debug)]
pub enum MutseaError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// Database error
    #[error("Database error: {0}")]
    Database(String),
    
    /// Network error
    #[error("Network error: {0}")]
    Network(String),
    
    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    /// Authorization error
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    /// Asset not found
    #[error("Asset not found: {0}")]
    AssetNotFound(String),
    
    /// User not found
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    /// Region not found
    #[error("Region not found: {0}")]
    RegionNotFound(String),
    
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    
    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    /// Generic error with message
    #[error("{0}")]
    Generic(String),
}

/// Result type alias for Mutsea operations
pub type MutseaResult<T> = Result<T, MutseaError>;