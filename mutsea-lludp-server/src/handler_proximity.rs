//! mutsea-network/src/lludp_server/handler_proximity.rs
//! Agent proximity detection and update broadcasting

use crate::NetworkResult;
use mutsea_core::Vector3;
use mutsea_protocol::Packet;
use std::collections::HashMap;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{debug, warn};

use super::{CircuitInfo, ServerStats};

/// Proximity handler for detecting nearby agents and broadcasting updates
#[derive(Clone)]
pub struct ProximityHandler;

impl ProximityHandler {
    pub fn new() -> Self {
        Self
    }

    /// Calculate agent velocity from position changes
    pub async fn calculate_velocity(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
        new_position: Vector3,
        delta_time: f32,
    ) -> Vector3 {
        let circuits_guard = circuits.read().await;
        if let Some(circuit) = circuits_guard.get(&circuit_code) {
            let old_position = circuit.position;
            let delta_pos = new_position - old_position;
            if delta_time > 0.0 {
                delta_pos * (1.0 / delta_time)
            } else {
                Vector3::ZERO
            }
        } else {
            Vector3::ZERO
        }
    }

    /// Get agents within range of a position
    pub async fn get_agents_in_range(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        center: Vector3,
        range: f32,
    ) -> Vec<u32> {
        let circuits_guard = circuits.read().await;
        let mut agents_in_range = Vec::new();
        
        for (circuit_code, circuit) in circuits_guard.iter() {
            if circuit.authenticated {
                let distance = (circuit.position - center).length();
                if distance <= range {
                    agents_in_range.push(*circuit_code);
                }
            }
        }
        
        agents_in_range
    }

    /// Get agents within range of another agent
    pub async fn get_nearby_agents(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        source_circuit: u32,
        range: f32,
    ) -> Vec<u32> {
        let circuits_guard = circuits.read().await;
        
        let Some(source) = circuits_guard.get(&source_circuit) else {
            return Vec::new();
        };
        
        let source_position = source.position;
        let mut nearby_agents = Vec::new();
        
        for (circuit_code, circuit) in circuits_guard.iter() {
            if *circuit_code != source_circuit && circuit.authenticated {
                let distance = (circuit.position - source_position).length();
                if distance <= range {
                    nearby_agents.push(*circuit_code);
                }
            }
        }
        
        nearby_agents
    }

    /// Broadcast agent update to nearby agents
    pub async fn broadcast_agent_update(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        circuit_code: u32,
        range: f32,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<usize> {
        let circuits_guard = circuits.read().await;
        
        let Some(source_circuit) = circuits_guard.get(&circuit_code) else {
            return Ok(0);
        };
        
        let source_position = source_circuit.position;
        let source_agent_id = source_circuit.agent_id.unwrap_or_default();
        
        // Find nearby circuits
        let mut nearby_circuits = Vec::new();
        for (other_circuit_code, other_circuit) in circuits_guard.iter() {
            if *other_circuit_code != circuit_code && other_circuit.authenticated {
                let distance = (other_circuit.position - source_position).length();
                if distance <= range {
                    nearby_circuits.push((*other_circuit_code, other_circuit.address));
                }
            }
        }
        
        drop(circuits_guard); // Release the lock
        
        let broadcast_count = nearby_circuits.len();
        
        // Create agent update packet
        let mut payload = Vec::new();
        payload.push(mutsea_protocol::constants::packet_types::OBJECT_UPDATE as u8);
        
        // Add agent update data (simplified)
        payload.extend_from_slice(source_agent_id.as_uuid().as_bytes()); // Agent ID
        payload.extend_from_slice(&source_position.x.to_le_bytes());
        payload.extend_from_slice(&source_position.y.to_le_bytes());
        payload.extend_from_slice(&source_position.z.to_le_bytes());
        
        let packet = Packet::reliable(0, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize agent update: {}", e)))?;
        
        // Send updates to nearby circuits
        for (target_circuit, target_address) in nearby_circuits {
            if let Err(e) = socket.send_to(&packet_data, target_address).await {
                warn!("Failed to send agent update to circuit {}: {}", target_circuit, e);
            }
        }
        
        // Update stats
        if broadcast_count > 0 {
            let mut stats_guard = stats.write().await;
            stats_guard.packets_sent += broadcast_count as u64;
            stats_guard.bytes_sent += (packet_data.len() * broadcast_count) as u64;
        }
        
        debug!("Broadcasted agent update for circuit {} to {} nearby agents", circuit_code, broadcast_count);
        Ok(broadcast_count)
    }

    /// Calculate distance between two positions
    pub fn calculate_distance(&self, pos1: Vector3, pos2: Vector3) -> f32 {
        (pos2 - pos1).length()
    }

    /// Check if position is within region bounds
    pub fn is_position_in_bounds(&self, position: Vector3, region_size: f32) -> bool {
        position.x >= 0.0 && position.x <= region_size &&
        position.y >= 0.0 && position.y <= region_size &&
        position.z >= 0.0 && position.z <= 4096.0 // Max height
    }

    /// Get agents in a rectangular area
    pub async fn get_agents_in_area(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        min_pos: Vector3,
        max_pos: Vector3,
    ) -> Vec<u32> {
        let circuits_guard = circuits.read().await;
        let mut agents_in_area = Vec::new();
        
        for (circuit_code, circuit) in circuits_guard.iter() {
            if circuit.authenticated {
                let pos = circuit.position;
                if pos.x >= min_pos.x && pos.x <= max_pos.x &&
                   pos.y >= min_pos.y && pos.y <= max_pos.y &&
                   pos.z >= min_pos.z && pos.z <= max_pos.z {
                    agents_in_area.push(*circuit_code);
                }
            }
        }
        
        agents_in_area
    }

    /// Update interest management for circuit
    pub async fn update_interest_management(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
        view_distance: f32,
    ) -> NetworkResult<Vec<u32>> {
        let nearby_agents = self.get_nearby_agents(circuits, circuit_code, view_distance).await;
        
        // Store interest list for future culling
        // In a full implementation, this would maintain per-circuit interest lists
        
        Ok(nearby_agents)
    }

    /// Check line of sight between two positions (simplified)
    pub fn has_line_of_sight(&self, from: Vector3, to: Vector3) -> bool {
        // Simplified line of sight check
        // In a real implementation, this would check terrain and objects
        let distance = (to - from).length();
        distance <= 512.0 // Max visible distance
    }

    /// Broadcast object update to interested agents
    pub async fn broadcast_object_update(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        object_position: Vector3,
        object_data: &[u8],
        range: f32,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<usize> {
        let interested_agents = self.get_agents_in_range(circuits, object_position, range).await;
        
        let circuits_guard = circuits.read().await;
        let mut broadcast_count = 0;
        
        for circuit_code in interested_agents {
            if let Some(circuit) = circuits_guard.get(&circuit_code) {
                if circuit.authenticated {
                    if let Err(e) = socket.send_to(object_data, circuit.address).await {
                        warn!("Failed to send object update to circuit {}: {}", circuit_code, e);
                    } else {
                        broadcast_count += 1;
                    }
                }
            }
        }
        
        // Update stats
        if broadcast_count > 0 {
            let mut stats_guard = stats.write().await;
            stats_guard.packets_sent += broadcast_count as u64;
            stats_guard.bytes_sent += (object_data.len() * broadcast_count) as u64;
        }
        
        Ok(broadcast_count)
    }
}

impl Default for ProximityHandler {
    fn default() -> Self {
        Self::new()
    }
}