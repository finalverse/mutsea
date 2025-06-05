//! Update mutsea-network/src/lludp_server/circuit.rs

use mutsea_core::{UserId, RegionId, Vector3, Quaternion};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use uuid::Uuid;

/// Circuit information for active connections
#[derive(Debug, Clone)]
pub struct CircuitInfo {
    pub circuit_code: u32,
    pub address: SocketAddr,
    pub user_id: Option<UserId>,
    pub agent_id: Option<UserId>,
    pub session_id: Option<Uuid>,
    pub secure_session_id: Option<Uuid>,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub sequence_in: u32,
    pub sequence_out: u32,
    pub pending_acks: Vec<u32>,
    pub reliable_packets: HashMap<u32, ReliablePacketData>,
    pub authenticated: bool,
    pub region_id: Option<RegionId>,
    pub position: Vector3,
    pub look_at: Vector3,
    pub client_info: Option<ClientInfo>,
    // Add missing ping fields
    pub last_ping_id: u8,
    pub last_ping_time: Instant,
}

/// Client information
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub viewer_name: String,
    pub viewer_version: String,
    pub platform: String,
    pub channel: String,
}

/// Reliable packet data
#[derive(Debug, Clone)]
pub struct ReliablePacketData {
    pub data: Vec<u8>,
    pub timestamp: Instant,
    pub resend_count: u8,
}

/// Circuit manager for handling multiple circuits
pub struct CircuitManager {
    circuits: HashMap<u32, CircuitInfo>,
    circuits_by_address: HashMap<SocketAddr, u32>,
}

impl CircuitManager {
    pub fn new() -> Self {
        Self {
            circuits: HashMap::new(),
            circuits_by_address: HashMap::new(),
        }
    }

    /// Add a new circuit
    pub fn add_circuit(&mut self, circuit: CircuitInfo) {
        let circuit_code = circuit.circuit_code;
        let address = circuit.address;
        
        self.circuits.insert(circuit_code, circuit);
        self.circuits_by_address.insert(address, circuit_code);
    }

    /// Get circuit by code
    pub fn get_circuit(&self, circuit_code: u32) -> Option<&CircuitInfo> {
        self.circuits.get(&circuit_code)
    }

    /// Get mutable circuit by code
    pub fn get_circuit_mut(&mut self, circuit_code: u32) -> Option<&mut CircuitInfo> {
        self.circuits.get_mut(&circuit_code)
    }

    /// Get circuit by address
    pub fn get_circuit_by_address(&self, address: &SocketAddr) -> Option<&CircuitInfo> {
        if let Some(&circuit_code) = self.circuits_by_address.get(address) {
            self.circuits.get(&circuit_code)
        } else {
            None
        }
    }

    /// Get mutable circuit by address
    pub fn get_circuit_by_address_mut(&mut self, address: &SocketAddr) -> Option<&mut CircuitInfo> {
        if let Some(&circuit_code) = self.circuits_by_address.get(address) {
            self.circuits.get_mut(&circuit_code)
        } else {
            None
        }
    }

    /// Remove circuit
    pub fn remove_circuit(&mut self, circuit_code: u32) -> Option<CircuitInfo> {
        if let Some(circuit) = self.circuits.remove(&circuit_code) {
            self.circuits_by_address.remove(&circuit.address);
            Some(circuit)
        } else {
            None
        }
    }

    /// Get all circuits
    pub fn get_all_circuits(&self) -> Vec<&CircuitInfo> {
        self.circuits.values().collect()
    }

    /// Get circuit count
    pub fn circuit_count(&self) -> usize {
        self.circuits.len()
    }

    /// Cleanup timed out circuits
    pub fn cleanup_timed_out(&mut self, timeout: std::time::Duration) -> usize {
        let now = Instant::now();
        let mut to_remove = Vec::new();

        for (circuit_code, circuit) in &self.circuits {
            if circuit.last_activity.elapsed() > timeout {
                to_remove.push(*circuit_code);
            }
        }

        let removed_count = to_remove.len();
        for circuit_code in to_remove {
            self.remove_circuit(circuit_code);
        }

        removed_count
    }
}

impl Default for CircuitManager {
    fn default() -> Self {
        Self::new()
    }
}