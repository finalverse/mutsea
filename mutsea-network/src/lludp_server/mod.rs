//! mutsea-network/src/lludp_server/mod.rs
//! LLUDP server modular components - Updated with all handlers

mod circuit;
mod stats;
mod handlers;

// Individual handler modules
mod handler_auth;
mod handler_movement;
mod handler_chat;
mod handler_ping;
mod handler_region;
mod handler_object;
mod handler_animation;
mod handler_proximity;

// Re-export all components
pub use circuit::*;
pub use stats::*;
pub use handlers::*;

// Re-export all handler types
pub use handler_auth::*;
pub use handler_movement::*;
pub use handler_chat::*;
pub use handler_ping::*;
pub use handler_region::*;
pub use handler_object::*;
pub use handler_animation::*;
pub use handler_proximity::*;

// Main server implementation
mod server;
pub use server::*;