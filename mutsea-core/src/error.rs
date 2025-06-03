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

// Add conversion from NetworkError
impl From<crate::NetworkError> for MutseaError {
    fn from(err: crate::NetworkError) -> Self {
        MutseaError::Network(err.to_string())
    }
}

/// Result type alias for Mutsea operations
pub type MutseaResult<T> = Result<T, MutseaError>;

/// Network-specific errors (moved from mutsea-network)
#[derive(Error, Debug)]
pub enum NetworkError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Address parse error
    #[error("Invalid address: {0}")]
    InvalidAddress(#[from] std::net::AddrParseError),
    
    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    /// Invalid packet
    #[error("Invalid packet: {0}")]
    InvalidPacket(String),
    
    /// Client not found
    #[error("Client not found: {0}")]
    ClientNotFound(String),
    
    /// Session error
    #[error("Session error: {0}")]
    Session(String),
    
    /// Timeout error
    #[error("Timeout: {0}")]
    Timeout(String),
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    /// Authorization failed
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Compression error
    #[error("Compression error: {0}")]
    Compression(String),
    
    /// Generic network error
    #[error("{0}")]
    Generic(String),
}

/// Result type for network operations
pub type NetworkResult<T> = Result<T, NetworkError>;