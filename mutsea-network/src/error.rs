//! Update mutsea-network/src/error.rs

use thiserror::Error;

/// Network-specific errors
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

// Add conversion from MutseaError
impl From<mutsea_core::MutseaError> for NetworkError {
    fn from(err: mutsea_core::MutseaError) -> Self {
        match err {
            mutsea_core::MutseaError::Network(msg) => NetworkError::Generic(msg),
            mutsea_core::MutseaError::Authentication(msg) => NetworkError::AuthenticationFailed(msg),
            mutsea_core::MutseaError::Authorization(msg) => NetworkError::AuthorizationFailed(msg),
            _ => NetworkError::Generic(err.to_string()),
        }
    }
}

// Add conversion from ProtocolError
impl From<mutsea_protocol::ProtocolError> for NetworkError {
    fn from(err: mutsea_protocol::ProtocolError) -> Self {
        match err {
            mutsea_protocol::ProtocolError::InvalidPacket(msg) => NetworkError::InvalidPacket(msg),
            mutsea_protocol::ProtocolError::AuthenticationFailed(msg) => NetworkError::AuthenticationFailed(msg),
            mutsea_protocol::ProtocolError::Timeout(msg) => NetworkError::Timeout(msg),
            mutsea_protocol::ProtocolError::Encoding(msg) => NetworkError::Serialization(msg),
            mutsea_protocol::ProtocolError::Decoding(msg) => NetworkError::Serialization(msg),
            _ => NetworkError::Protocol(err.to_string()),
        }
    }
}

/// Result type for network operations
pub type NetworkResult<T> = Result<T, NetworkError>;