//! # Mutsea Protocol
//! 
//! OpenSim LLUDP and HTTP protocol implementation for Mutsea.
//! Provides full compatibility with Firestorm and other OpenSim viewers.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod lludp;
pub mod http;
pub mod packet;
pub mod codec;
pub mod caps;
pub mod login;
pub mod error;
pub mod constants;

// Re-export commonly used types
pub use error::*;
pub use packet::*;
pub use codec::*;
pub use constants::*;

use mutsea_core::{MutseaResult, UserId, RegionId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Protocol version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u32,
}

impl ProtocolVersion {
    /// Create a new protocol version
    pub fn new(major: u8, minor: u8, patch: u8, build: u32) -> Self {
        Self { major, minor, patch, build }
    }
    
    /// Get version as string
    pub fn as_string(&self) -> String {
        format!("{}.{}.{}.{}", self.major, self.minor, self.patch, self.build)
    }
    
    /// Check compatibility with another version
    pub fn is_compatible(&self, other: &ProtocolVersion) -> bool {
        self.major == other.major && self.minor == other.minor
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::new(2, 1, 0, 0) // OpenSim compatible version
    }
}

/// Circuit information for LLUDP connections
#[derive(Debug, Clone)]
pub struct Circuit {
    pub code: u32,
    pub ip_endpoint: std::net::SocketAddr,
    pub sequence_in: u32,
    pub sequence_out: u32,
    pub pending_acks: Vec<u32>,
    pub last_ping_id: u8,
    pub last_ping_time: std::time::Instant,
}

impl Circuit {
    /// Create a new circuit
    pub fn new(code: u32, endpoint: std::net::SocketAddr) -> Self {
        Self {
            code,
            ip_endpoint: endpoint,
            sequence_in: 0,
            sequence_out: 0,
            pending_acks: Vec::new(),
            last_ping_id: 0,
            last_ping_time: std::time::Instant::now(),
        }
    }
    
    /// Get next outbound sequence number
    pub fn next_sequence_out(&mut self) -> u32 {
        self.sequence_out += 1;
        self.sequence_out
    }
    
    /// Process inbound sequence number
    pub fn process_sequence_in(&mut self, sequence: u32) -> bool {
        if sequence > self.sequence_in {
            // Add missing sequences to pending acks
            for missing in (self.sequence_in + 1)..sequence {
                self.pending_acks.push(missing);
            }
            self.sequence_in = sequence;
            true
        } else {
            false
        }
    }
    
    /// Add acknowledgment
    pub fn add_ack(&mut self, sequence: u32) {
        if !self.pending_acks.contains(&sequence) {
            self.pending_acks.push(sequence);
        }
    }
    
    /// Get and clear pending acknowledgments
    pub fn take_pending_acks(&mut self) -> Vec<u32> {
        std::mem::take(&mut self.pending_acks)
    }
}

/// Login response for client authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub login: String,
    pub reason: String,
    pub session_id: String,
    pub secure_session_id: String,
    pub agent_id: String,
    pub first_name: String,
    pub last_name: String,
    pub start_location: String,
    pub look_at: String,
    pub seed_capability: String,
    pub agent_access: String,
    pub agent_access_max: String,
    pub inventory_host: String,
    pub sim_ip: String,
    pub sim_port: i32,
    pub region_x: i32,
    pub region_y: i32,
    pub circuit_code: i32,
    pub home: String,
    pub message: String,
    pub seconds_since_epoch: i64,
    #[serde(default)]
    pub event_categories: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub event_notifications: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub classified_categories: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub ui_config: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub global_textures: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub login_flags: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub inventory_skeleton: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub inventory_lib_skeleton: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub inventory_lib_owner: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub buddy_list: Vec<HashMap<String, String>>,
}

impl LoginResponse {
    /// Create a successful login response
    pub fn success(
        session_id: uuid::Uuid,
        secure_session_id: uuid::Uuid,
        agent_id: UserId,
        first_name: String,
        last_name: String,
        region_id: RegionId,
        sim_ip: String,
        sim_port: i32,
        circuit_code: u32,
        seed_capability: String,
    ) -> Self {
        Self {
            login: "true".to_string(),
            reason: "".to_string(),
            session_id: session_id.to_string(),
            secure_session_id: secure_session_id.to_string(),
            agent_id: agent_id.to_string(),
            first_name,
            last_name,
            start_location: "home".to_string(),
            look_at: "[r1,r1,r0]".to_string(),
            seed_capability,
            agent_access: "M".to_string(),
            agent_access_max: "A".to_string(),
            inventory_host: "127.0.0.1".to_string(),
            sim_ip,
            sim_port,
            region_x: 1000,
            region_y: 1000,
            circuit_code: circuit_code as i32,
            home: format!("{{'region_handle':[r{},r{}], 'position':[r128,r128,r21], 'look_at':[r1,r0,r0]}}", 
                          1000 * 256, 1000 * 256),
            message: "Welcome to Mutsea!".to_string(),
            seconds_since_epoch: chrono::Utc::now().timestamp(),
            event_categories: Vec::new(),
            event_notifications: Vec::new(),
            classified_categories: Vec::new(),
            ui_config: Vec::new(),
            global_textures: Vec::new(),
            login_flags: Vec::new(),
            inventory_skeleton: Vec::new(),
            inventory_lib_skeleton: Vec::new(),
            inventory_lib_owner: Vec::new(),
            buddy_list: Vec::new(),
        }
    }
    
    /// Create a failed login response
    pub fn failure(reason: String) -> Self {
        Self {
            login: "false".to_string(),
            reason,
            session_id: String::new(),
            secure_session_id: String::new(),
            agent_id: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            start_location: String::new(),
            look_at: String::new(),
            seed_capability: String::new(),
            agent_access: String::new(),
            agent_access_max: String::new(),
            inventory_host: String::new(),
            sim_ip: String::new(),
            sim_port: 0,
            region_x: 0,
            region_y: 0,
            circuit_code: 0,
            home: String::new(),
            message: String::new(),
            seconds_since_epoch: chrono::Utc::now().timestamp(),
            event_categories: Vec::new(),
            event_notifications: Vec::new(),
            classified_categories: Vec::new(),
            ui_config: Vec::new(),
            global_textures: Vec::new(),
            login_flags: Vec::new(),
            inventory_skeleton: Vec::new(),
            inventory_lib_skeleton: Vec::new(),
            inventory_lib_owner: Vec::new(),
            buddy_list: Vec::new(),
        }
    }
}

/// Capability URL for HTTP services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub url: String,
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
}

impl Capability {
    /// Create a new capability
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            expires: None,
        }
    }
    
    /// Create a capability with expiration
    pub fn with_expiration(name: String, url: String, expires: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            name,
            url,
            expires: Some(expires),
        }
    }
    
    /// Check if capability has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            chrono::Utc::now() > expires
        } else {
            false
        }
    }
}