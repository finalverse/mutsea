//! mutsea-network/src/lludp_server/handler_region.rs
//! Region and world management handler

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

/// Region handler for managing world state and region information
#[derive(Clone)]
pub struct RegionHandler;

impl RegionHandler {
    pub fn new() -> Self {
        Self
    }

    /// Handle RegionHandshakeReply message
    pub async fn handle_region_handshake_reply(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 5 { // Minimum size for RegionHandshakeReply
            warn!("RegionHandshakeReply packet too short from {}", addr);
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

        debug!("Region handshake reply from circuit {}", circuit_code);

        // Parse handshake reply (simplified)
        let flags = u32::from_le_bytes([
            packet.payload[1], packet.payload[2], 
            packet.payload[3], packet.payload[4]
        ]);

        // Update circuit with handshake completion
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
            info!("Region handshake completed for circuit {} with flags: 0x{:08X}", 
                  circuit_code, flags);
        }

        // Send follow-up messages for complete region setup
        self.send_region_setup_messages(socket, addr, circuit_code).await?;

        Ok(())
    }

    /// Send region setup messages after handshake
    async fn send_region_setup_messages(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        circuit_code: u32,
    ) -> NetworkResult<()> {
        // Send LayerData (terrain) packet
        self.send_layer_data(socket, addr).await?;

        // Send WindData packet
        self.send_wind_data(socket, addr).await?;

        // Send CloudData packet
        self.send_cloud_data(socket, addr).await?;

        debug!("Sent region setup messages to circuit {}", circuit_code);
        Ok(())
    }

    /// Send terrain layer data
    async fn send_layer_data(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::LAYER_DATA as u8);

        // LayerID block
        payload.extend_from_slice(&0u8.to_le_bytes()); // Layer type (Land = 0)

        // LayerData block - simplified flat terrain
        // In a real implementation, this would contain compressed terrain height data
        let terrain_size = 16 * 16; // 16x16 patches
        let terrain_data = vec![0u8; terrain_size]; // Flat terrain at height 0
        
        payload.extend_from_slice(&(terrain_data.len() as u16).to_le_bytes());
        payload.extend_from_slice(&terrain_data);

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize LayerData packet: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent LayerData to {}", addr);
        Ok(())
    }

    /// Send wind data
    async fn send_wind_data(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::SIMULATOR_FEATURES as u8);

        // Simple wind data - no wind
        payload.extend_from_slice(&Vector3::ZERO.x.to_le_bytes());
        payload.extend_from_slice(&Vector3::ZERO.y.to_le_bytes());
        payload.extend_from_slice(&Vector3::ZERO.z.to_le_bytes());

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize WindData packet: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent WindData to {}", addr);
        Ok(())
    }

    /// Send cloud data
    async fn send_cloud_data(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::CLOUD_DATA as u8);

        // Simple cloud data - clear skies
        for _ in 0..4 {
            payload.extend_from_slice(&0.0f32.to_le_bytes()); // Cloud coverage
        }

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize CloudData packet: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent CloudData to {}", addr);
        Ok(())
    }

    /// Handle teleport request
    pub async fn handle_teleport_request(
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

        debug!("Teleport request from circuit {}", circuit_code);

        // For now, just acknowledge the teleport request
        // In a real implementation, this would validate the destination and initiate teleport
        self.send_teleport_start(socket, addr).await?;

        Ok(())
    }

    /// Send teleport start message
    async fn send_teleport_start(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::TELEPORT_START as u8);

        // TeleportData block (simplified)
        payload.extend_from_slice(&0u32.to_le_bytes()); // Teleport flags

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize TeleportStart packet: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent TeleportStart to {}", addr);
        Ok(())
    }

    /// Update region statistics
    pub async fn update_region_stats(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<RegionStatistics> {
        let circuits_guard = circuits.read().await;
        let authenticated_count = circuits_guard.values().filter(|c| c.authenticated).count();
        
        // Calculate center of activity
        let positions: Vec<Vector3> = circuits_guard.values()
            .filter(|c| c.authenticated)
            .map(|c| c.position)
            .collect();
        
        let center_of_activity = if positions.is_empty() {
            Vector3::ZERO
        } else {
            let sum = positions.iter().fold(Vector3::ZERO, |acc, pos| acc + *pos);
            sum * (1.0 / positions.len() as f32)
        };

        let stats_guard = stats.read().await;
        
        Ok(RegionStatistics {
            active_agents: authenticated_count,
            total_connections: circuits_guard.len(),
            center_of_activity,
            packets_per_second: stats_guard.packets_per_second(),
            error_rate: stats_guard.error_rate(),
            uptime: stats_guard.uptime(),
        })
    }

    /// Send region restart notification
    pub async fn send_region_restart_notification(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        restart_in_seconds: u32,
        message: &str,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<usize> {
        let circuits_guard = circuits.read().await;
        let mut broadcast_count = 0;

        for circuit in circuits_guard.values() {
            if circuit.authenticated {
                if let Err(e) = self.send_restart_notification(
                    socket, 
                    circuit.address, 
                    restart_in_seconds, 
                    message
                ).await {
                    warn!("Failed to send restart notification to circuit {}: {}", 
                          circuit.circuit_code, e);
                } else {
                    broadcast_count += 1;
                }
            }
        }

        // Update stats
        if broadcast_count > 0 {
            let mut stats_guard = stats.write().await;
            stats_guard.packets_sent += broadcast_count as u64;
        }

        info!("Sent region restart notification to {} circuits", broadcast_count);
        Ok(broadcast_count)
    }

    /// Send restart notification to specific client
    async fn send_restart_notification(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        restart_in_seconds: u32,
        message: &str,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::SIMULATOR_SHUTDOWN as u8);

        // Shutdown data
        payload.extend_from_slice(&restart_in_seconds.to_le_bytes());
        
        // Message (variable string)
        let message_bytes = message.as_bytes();
        payload.extend_from_slice(&(message_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(message_bytes);

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize restart notification: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Get region health metrics
    pub async fn get_region_health(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> RegionHealth {
        let circuits_guard = circuits.read().await;
        let stats_guard = stats.read().await;
        
        let active_agents = circuits_guard.values().filter(|c| c.authenticated).count();
        let total_connections = circuits_guard.len();
        
        let health_score = if stats_guard.errors > 0 {
            let error_rate = stats_guard.error_rate();
            if error_rate > 10.0 {
                0.0 // Poor health
            } else if error_rate > 5.0 {
                0.5 // Degraded health
            } else {
                0.8 // Good health
            }
        } else {
            1.0 // Excellent health
        };

        RegionHealth {
            health_score,
            active_agents,
            total_connections,
            packets_per_second: stats_guard.packets_per_second(),
            error_rate: stats_guard.error_rate(),
            memory_usage_mb: 0.0, // Would be calculated from actual memory usage
            cpu_usage_percent: 0.0, // Would be calculated from actual CPU usage
        }
    }
}

/// Region statistics
#[derive(Debug, Clone)]
pub struct RegionStatistics {
    pub active_agents: usize,
    pub total_connections: usize,
    pub center_of_activity: Vector3,
    pub packets_per_second: f64,
    pub error_rate: f64,
    pub uptime: std::time::Duration,
}

/// Region health metrics
#[derive(Debug, Clone)]
pub struct RegionHealth {
    pub health_score: f64, // 0.0 to 1.0
    pub active_agents: usize,
    pub total_connections: usize,
    pub packets_per_second: f64,
    pub error_rate: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

impl Default for RegionHandler {
    fn default() -> Self {
        Self::new()
    }
}