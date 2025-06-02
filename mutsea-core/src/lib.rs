//! # Mutsea Core
//! 
//! Core types, traits, and utilities for the Mutsea virtual world platform.
//! This crate provides the foundational types used across all Mutsea components.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod types;
pub mod traits;
pub mod error;
pub mod math;
pub mod events;
pub mod config;

// Re-export commonly used types
pub use types::*;
pub use traits::*;
pub use error::*;
pub use math::*;
pub use events::*;

/// Current version of the Mutsea platform
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Magic number for Mutsea protocol identification
pub const MUTSEA_MAGIC: u32 = 0x4D555453; // "MUTS"