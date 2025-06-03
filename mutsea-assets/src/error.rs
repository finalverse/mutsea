//! Asset management errors

use thiserror::Error;

/// Asset management errors
#[derive(Error, Debug)]
pub enum AssetError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Asset not found
    #[error("Asset not found: {0}")]
    NotFound(String),
    
    /// Invalid asset format
    #[error("Invalid asset format: {0}")]
    InvalidFormat(String),
    
    /// Storage backend error
    #[error("Storage error: {0}")]
    Storage(String),
    
    /// Cache error
    #[error("Cache error: {0}")]
    Cache(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Asset too large
    #[error("Asset too large: {0} bytes (max: {1} bytes)")]
    TooLarge(usize, usize),
    
    /// Generic error
    #[error("{0}")]
    Generic(String),
}

/// Result type for asset operations
pub type AssetResult<T> = Result<T, AssetError>;