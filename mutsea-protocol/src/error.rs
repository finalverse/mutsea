//! Protocol error types

use thiserror::Error;

/// Protocol-specific errors
#[derive(Error, Debug)]
pub enum ProtocolError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid packet format
    #[error("Invalid packet: {0}")]
    InvalidPacket(String),

    /// Unsupported protocol version
    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(String),

    /// Invalid message format
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Circuit not found
    #[error("Circuit not found: {0}")]
    CircuitNotFound(u32),

    /// Sequence error
    #[error("Sequence error: {0}")]
    SequenceError(String),

    /// Timeout error
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    Encoding(String),

    /// Decoding error
    #[error("Decoding error: {0}")]
    Decoding(String),

    /// Generic protocol error
    #[error("{0}")]
    Generic(String),
}

/// Result type for protocol operations
pub type ProtocolResult<T> = Result<T, ProtocolError>;