//! mutsea-network/src/lludp_server/movement_handler.rs
//! Core agent movement handler - focused on basic movement processing

use crate::NetworkResult;
use mutsea_core::{Vector3, Quaternion, UserId};
use mutsea_protocol::{Packet, constants::packet_types};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, debug, warn};

use super::{CircuitInfo, ServerStats};

/// Core movement handler for basic agent updates
#[derive(Clone)]
pub struct MovementHandler;

impl MovementHandler {
    pub fn new() -> Self {
        Self
    }

    /// Handle AgentUpdate message (avatar movement)
    pub async fn handle_agent_update(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 65 { // Minimum size for AgentUpdate
            warn!("AgentUpdate packet too short from {}", addr);
            return Ok(());
        }

        // Find circuit by address
        let circuit_code = {
            let circuits_guard = circuits.read().await;
            circuits_guard.iter()
                .find(|(_, circuit)| circuit.address == addr)
                .map(|(code, _)| *code)
        };

        let Some(circuit_code) = circuit_code else {
            warn!("No circuit found for address {}", addr);
            return Ok(());
        };

        // Parse AgentUpdate structure
        let movement_data = self.parse_agent_update_packet(&packet.payload)?;

        // Update circuit with movement data
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.position = movement_data.camera_center; // Use camera center as agent position
            circuit.look_at = movement_data.camera_at;
            circuit.last_activity = Instant::now();

            debug!("Agent update for circuit {}: pos=({:.1}, {:.1}, {:.1}) flags=0x{:08X}", 
                   circuit_code, circuit.position.x, circuit.position.y, circuit.position.z, 
                   movement_data.control_flags);
        }

        Ok(())
    }

    /// Parse AgentUpdate packet data
    fn parse_agent_update_packet(&self, payload: &[u8]) -> NetworkResult<AgentUpdateData> {
        let mut offset = 1; // Skip message ID

        // AgentData block
        let _agent_id_bytes = &payload[offset..offset + 16];
        offset += 16;
        
        let _session_id_bytes = &payload[offset..offset + 16];
        offset += 16;

        // Body rotation (quaternion)
        let body_rotation = Quaternion::new(
            f32::from_le_bytes([payload[offset], payload[offset + 1], 
                               payload[offset + 2], payload[offset + 3]]),
            f32::from_le_bytes([payload[offset + 4], payload[offset + 5], 
                               payload[offset + 6], payload[offset + 7]]),
            f32::from_le_bytes([payload[offset + 8], payload[offset + 9], 
                               payload[offset + 10], payload[offset + 11]]),
            f32::from_le_bytes([payload[offset + 12], payload[offset + 13], 
                               payload[offset + 14], payload[offset + 15]]),
        );
        offset += 16;

        // Head rotation (quaternion)
        let head_rotation = Quaternion::new(
            f32::from_le_bytes([payload[offset], payload[offset + 1], 
                               payload[offset + 2], payload[offset + 3]]),
            f32::from_le_bytes([payload[offset + 4], payload[offset + 5], 
                               payload[offset + 6], payload[offset + 7]]),
            f32::from_le_bytes([payload[offset + 8], payload[offset + 9], 
                               payload[offset + 10], payload[offset + 11]]),
            f32::from_le_bytes([payload[offset + 12], payload[offset + 13], 
                               payload[offset + 14], payload[offset + 15]]),
        );
        offset += 16;

        // State
        let state = payload[offset];
        offset += 1;

        // Camera center (vector3)
        let camera_center = Vector3::new(
            f32::from_le_bytes([payload[offset], payload[offset + 1], 
                               payload[offset + 2], payload[offset + 3]]),
            f32::from_le_bytes([payload[offset + 4], payload[offset + 5], 
                               payload[offset + 6], payload[offset + 7]]),
            f32::from_le_bytes([payload[offset + 8], payload[offset + 9], 
                               payload[offset + 10], payload[offset + 11]]),
        );
        offset += 12;

        // Camera at axis (vector3)
        let camera_at = Vector3::new(
            f32::from_le_bytes([payload[offset], payload[offset + 1], 
                               payload[offset + 2], payload[offset + 3]]),
            f32::from_le_bytes([payload[offset + 4], payload[offset + 5], 
                               payload[offset + 6], payload[offset + 7]]),
            f32::from_le_bytes([payload[offset + 8], payload[offset + 9], 
                               payload[offset + 10], payload[offset + 11]]),
        );
        offset += 12;

        // Skip camera left and up axis for now
        offset += 24;

        // Far (f32)
        let far = f32::from_le_bytes([payload[offset], payload[offset + 1], 
                                     payload[offset + 2], payload[offset + 3]]);
        offset += 4;

        // Control flags
        let control_flags = u32::from_le_bytes([payload[offset], payload[offset + 1], 
                                              payload[offset + 2], payload[offset + 3]]);
        offset += 4;

        // Flags
        let flags = payload[offset];

        Ok(AgentUpdateData {
            body_rotation,
            head_rotation,
            state,
            camera_center,
            camera_at,
            far,
            control_flags,
            flags,
        })
    }

    /// Update agent position with validation
    pub async fn update_agent_position(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
        position: Vector3,
        look_at: Vector3,
    ) -> NetworkResult<bool> {
        let mut circuits_guard = circuits.write().await;
        
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            // Validate position bounds (simple check)
            if self.is_valid_position(&position) {
                circuit.position = position;
                circuit.look_at = look_at;
                circuit.last_activity = Instant::now();
                
                debug!("Updated position for circuit {}: ({:.1}, {:.1}, {:.1})", 
                       circuit_code, position.x, position.y, position.z);
                Ok(true)
            } else {
                warn!("Invalid position for circuit {}: ({:.1}, {:.1}, {:.1})", 
                      circuit_code, position.x, position.y, position.z);
                Ok(false)
            }
        } else {
            warn!("Circuit {} not found for position update", circuit_code);
            Ok(false)
        }
    }

    /// Validate position is within acceptable bounds
    fn is_valid_position(&self, position: &Vector3) -> bool {
        position.x >= 0.0 && position.x <= 256.0 && 
        position.y >= 0.0 && position.y <= 256.0 &&
        position.z >= 0.0 && position.z <= 4096.0
    }

    /// Get agent position
    pub async fn get_agent_position(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
    ) -> Option<Vector3> {
        let circuits_guard = circuits.read().await;
        circuits_guard.get(&circuit_code).map(|c| c.position)
    }
}

/// Parsed agent update data
#[derive(Debug, Clone)]
pub struct AgentUpdateData {
    pub body_rotation: Quaternion,
    pub head_rotation: Quaternion,
    pub state: u8,
    pub camera_center: Vector3,
    pub camera_at: Vector3,
    pub far: f32,
    pub control_flags: u32,
    pub flags: u8,
}

impl Default for MovementHandler {
    fn default() -> Self {
        Self::new()
    }
}