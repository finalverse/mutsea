//! mutsea-network/src/lludp_server/handler_teleport.rs
//! Teleport and region crossing handler

use crate::NetworkResult;
use mutsea_core::{Vector3, RegionId, UserId};
use mutsea_protocol::{Packet, constants::packet_types};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{debug, warn, info};

use super::{CircuitInfo, ServerStats};

/// Teleport handler for agent teleportation and region crossing
#[derive(Clone)]
pub struct TeleportHandler;

/// Teleport request data
#[derive(Debug, Clone)]
pub struct TeleportRequestData {
    pub region_id: RegionId,
    pub position: Vector3,
    pub look_at: Vector3,
    pub teleport_flags: u32,
}

/// Teleport status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeleportStatus {
    Start,
    Progress,
    Failed,
    Finished,
    Local,
}

impl TeleportHandler {
    pub fn new() -> Self {
        Self
    }

    /// Handle TeleportRequest message
    pub async fn handle_teleport_request(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 69 { // Minimum size for TeleportRequest
            warn!("TeleportRequest packet too short from {}", addr);
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

        // Parse teleport request
        let teleport_data = self.parse_teleport_request(&packet.payload)?;
        
        info!("Teleport request from circuit {}: region={}, pos=({:.1}, {:.1}, {:.1})", 
              circuit_code, teleport_data.region_id, 
              teleport_data.position.x, teleport_data.position.y, teleport_data.position.z);

        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }
        drop(circuits_guard);

        // Process teleport (simplified - in reality would validate destination)
        self.process_teleport(circuits, socket, addr, circuit_code, &teleport_data).await?;

        Ok(())
    }

    /// Parse teleport request packet
    fn parse_teleport_request(&self, payload: &[u8]) -> NetworkResult<TeleportRequestData> {
        let mut offset = 1; // Skip message ID

        // AgentData block
        let _agent_id = &payload[offset..offset + 16];
        offset += 16;
        
        let _session_id = &payload[offset..offset + 16];
        offset += 16;

        // Info block
        let region_handle = u64::from_le_bytes([
            payload[offset], payload[offset + 1], payload[offset + 2], payload[offset + 3],
            payload[offset + 4], payload[offset + 5], payload[offset + 6], payload[offset + 7]
        ]);
        offset += 8;

        let position = Vector3::new(
            f32::from_le_bytes([payload[offset], payload[offset + 1], 
                               payload[offset + 2], payload[offset + 3]]),
            f32::from_le_bytes([payload[offset + 4], payload[offset + 5], 
                               payload[offset + 6], payload[offset + 7]]),
            f32::from_le_bytes([payload[offset + 8], payload[offset + 9], 
                               payload[offset + 10], payload[offset + 11]]),
        );
        offset += 12;

        let look_at = Vector3::new(
            f32::from_le_bytes([payload[offset], payload[offset + 1], 
                               payload[offset + 2], payload[offset + 3]]),
            f32::from_le_bytes([payload[offset + 4], payload[offset + 5], 
                               payload[offset + 6], payload[offset + 7]]),
            f32::from_le_bytes([payload[offset + 8], payload[offset + 9], 
                               payload[offset + 10], payload[offset + 11]]),
        );
        offset += 12;

        let teleport_flags = u32::from_le_bytes([
            payload[offset], payload[offset + 1], 
            payload[offset + 2], payload[offset + 3]
        ]);

        // Convert region handle to RegionId (simplified)
        let region_id = RegionId::new();

        Ok(TeleportRequestData {
            region_id,
            position,
            look_at,
            teleport_flags,
        })
    }

    /// Process teleport request
    async fn process_teleport(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        circuit_code: u32,
        teleport_data: &TeleportRequestData,
    ) -> NetworkResult<()> {
        // Send teleport start
        self.send_teleport_start(socket, addr).await?;

        // Validate teleport destination (simplified)
        if self.is_valid_teleport_destination(&teleport_data.position) {
            // Send teleport progress
            self.send_teleport_progress(socket, addr, "Preparing teleport...").await?;
            
            // Update agent position
            {
                let mut circuits_guard = circuits.write().await;
                if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
                    circuit.position = teleport_data.position;
                    circuit.look_at = teleport_data.look_at;
                    circuit.region_id = Some(teleport_data.region_id);
                }
            }

            // Send teleport finish
            self.send_teleport_finish(socket, addr, teleport_data).await?;
            
            info!("Teleport completed for circuit {}", circuit_code);
        } else {
            // Send teleport failed
            self.send_teleport_failed(socket, addr, "Invalid destination").await?;
            warn!("Teleport failed for circuit {}: invalid destination", circuit_code);
        }

        Ok(())
    }

    /// Send TeleportStart message
    async fn send_teleport_start(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::TELEPORT_START as u8);

        // Info block
        payload.extend_from_slice(&0u32.to_le_bytes()); // TeleportFlags

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize TeleportStart: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent TeleportStart to {}", addr);
        Ok(())
    }

    /// Send TeleportProgress message
    async fn send_teleport_progress(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        message: &str,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::TELEPORT_PROGRESS as u8);

        // Info block
        payload.extend_from_slice(&0u32.to_le_bytes()); // TeleportFlags
        
        // Message (variable string)
        let message_bytes = message.as_bytes();
        payload.extend_from_slice(&(message_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(message_bytes);

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize TeleportProgress: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent TeleportProgress to {}: {}", addr, message);
        Ok(())
    }

    /// Send TeleportFinish message
    async fn send_teleport_finish(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        teleport_data: &TeleportRequestData,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::TELEPORT_FINISH as u8);

        // Info block
        payload.extend_from_slice(&0u32.to_le_bytes()); // TeleportFlags
        payload.extend_from_slice(&(1000u32 * 256).to_le_bytes()); // RegionHandle X
        payload.extend_from_slice(&(1000u32 * 256).to_le_bytes()); // RegionHandle Y
        payload.extend_from_slice(&13u16.to_le_bytes()); // SimAccess (PG)
        
        // SIM IP (32-bit IP)
        let sim_ip: [u8; 4] = [127, 0, 0, 1];
        payload.extend_from_slice(&sim_ip);
        
        // SIM Port
        payload.extend_from_slice(&9000u16.to_le_bytes());
        
        // Location ID (16 bytes - can be random)
        payload.extend_from_slice(&uuid::Uuid::new_v4().as_bytes());
        
        // Seed capability (variable string)
        let seed_cap = format!("http://127.0.0.1:8080/caps/{}/", uuid::Uuid::new_v4());
        let seed_bytes = seed_cap.as_bytes();
        payload.extend_from_slice(&(seed_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(seed_bytes);

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize TeleportFinish: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent TeleportFinish to {}", addr);
        Ok(())
    }

    /// Send TeleportFailed message
    async fn send_teleport_failed(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        reason: &str,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::TELEPORT_FAILED as u8);

        // Info block
        payload.extend_from_slice(&0u32.to_le_bytes()); // TeleportFlags
        
        // Reason (variable string)
        let reason_bytes = reason.as_bytes();
        payload.extend_from_slice(&(reason_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(reason_bytes);

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize TeleportFailed: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent TeleportFailed to {}: {}", addr, reason);
        Ok(())
    }

    /// Handle TeleportLocal message (within same region)
    pub async fn handle_teleport_local(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
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

        debug!("Local teleport from circuit {}", circuit_code);

        // Parse local teleport data (simplified)
        if packet.payload.len() >= 45 {
            let mut offset = 1; // Skip message ID
            
            // Skip AgentData (32 bytes)
            offset += 32;
            
            // Parse position
            let position = Vector3::new(
                f32::from_le_bytes([packet.payload[offset], packet.payload[offset + 1], 
                                   packet.payload[offset + 2], packet.payload[offset + 3]]),
                f32::from_le_bytes([packet.payload[offset + 4], packet.payload[offset + 5], 
                                   packet.payload[offset + 6], packet.payload[offset + 7]]),
                f32::from_le_bytes([packet.payload[offset + 8], packet.payload[offset + 9], 
                                   packet.payload[offset + 10], packet.payload[offset + 11]]),
            );

            // Update position
            let mut circuits_guard = circuits.write().await;
            if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
                circuit.position = position;
                circuit.last_activity = Instant::now();
                debug!("Updated local teleport position for circuit {}: ({:.1}, {:.1}, {:.1})",
                       circuit_code, position.x, position.y, position.z);
            }
        }

        Ok(())
    }

    /// Validate teleport destination
    fn is_valid_teleport_destination(&self, position: &Vector3) -> bool {
        // Basic validation - position within region bounds
        position.x >= 0.0 && position.x <= 256.0 &&
        position.y >= 0.0 && position.y <= 256.0 &&
        position.z >= 0.0 && position.z <= 4096.0
    }

    /// Check if position is safe for teleport
    pub fn is_safe_teleport_position(&self, position: &Vector3) -> bool {
        // Check if position is not inside objects, over water, etc.
        // For now, just check basic bounds
        self.is_valid_teleport_destination(position) && position.z >= 20.0
    }

    /// Calculate teleport time based on distance
    pub fn calculate_teleport_time(&self, from: &Vector3, to: &Vector3) -> std::time::Duration {
        let distance = (*to - *from).length();
        
        if distance < 10.0 {
            // Local teleport - instant
            std::time::Duration::from_millis(100)
        } else if distance < 100.0 {
            // Short range teleport
            std::time::Duration::from_millis(500)
        } else {
            // Long range teleport
            std::time::Duration::from_secs(2)
        }
    }

    /// Handle cross-region teleport
    pub async fn handle_cross_region_teleport(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        circuit_code: u32,
        target_region: RegionId,
        position: Vector3,
    ) -> NetworkResult<()> {
        info!("Cross-region teleport for circuit {} to region {}", circuit_code, target_region);

        // In a full implementation, this would:
        // 1. Validate target region exists and is accessible
        // 2. Negotiate with target region server
        // 3. Transfer agent state
        // 4. Send EnableSimulator/DisableSimulator messages
        // 5. Complete the teleport

        // For now, just reject cross-region teleports
        if let Some(circuit) = circuits.read().await.get(&circuit_code) {
            self.send_teleport_failed(socket, circuit.address, "Cross-region teleport not yet supported").await?;
        }

        Ok(())
    }

    /// Get teleport statistics
    pub fn get_teleport_stats(&self) -> TeleportStats {
        TeleportStats {
            total_teleports: 0, // Would track actual stats
            successful_teleports: 0,
            failed_teleports: 0,
            local_teleports: 0,
            cross_region_teleports: 0,
            average_teleport_time: std::time::Duration::from_millis(500),
        }
    }
}

/// Teleport statistics
#[derive(Debug, Clone)]
pub struct TeleportStats {
    pub total_teleports: u64,
    pub successful_teleports: u64,
    pub failed_teleports: u64,
    pub local_teleports: u64,
    pub cross_region_teleports: u64,
    pub average_teleport_time: std::time::Duration,
}

/// Teleport flags
pub mod teleport_flags {
    pub const DEFAULT: u32 = 0;
    pub const VIA_LURE: u32 = 1 << 0;
    pub const VIA_LANDMARK: u32 = 1 << 1;
    pub const VIA_LOCATION: u32 = 1 << 2;
    pub const VIA_HOME: u32 = 1 << 3;
    pub const VIA_TELEHUB: u32 = 1 << 4;
    pub const FINISHED_VIA_NEWUSER: u32 = 1 << 8;
    pub const FINISHED_VIA_LOGIN: u32 = 1 << 9;
    pub const FORCE_REDIRECT: u32 = 1 << 10;
}

impl Default for TeleportHandler {
    fn default() -> Self {
        Self::new()
    }
}