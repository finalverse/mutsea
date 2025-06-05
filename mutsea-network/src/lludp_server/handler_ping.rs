//! mutsea-network/src/lludp_server/handler_ping.rs
//! Ping and acknowledgment handler

use crate::NetworkResult;
use mutsea_protocol::{Packet, constants::packet_types};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{debug, warn};

use super::{CircuitInfo, ServerStats};

/// Ping handler for connection health monitoring
#[derive(Clone)]
pub struct PingHandler;

impl PingHandler {
    pub fn new() -> Self {
        Self
    }

    /// Handle StartPingCheck message
    pub async fn handle_ping_check(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 6 {
            warn!("StartPingCheck packet too short from {}", addr);
            return Ok(());
        }

        let ping_id = packet.payload[1];
        let oldest_unacked = u32::from_le_bytes([
            packet.payload[2], packet.payload[3], 
            packet.payload[4], packet.payload[5]
        ]);

        debug!("Ping check from {}: ping_id={}, oldest_unacked={}", addr, ping_id, oldest_unacked);

        // Send CompletePingCheck response
        self.send_ping_response(socket, addr, ping_id).await?;

        Ok(())
    }

    /// Send CompletePingCheck response
    async fn send_ping_response(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        ping_id: u8,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::COMPLETE_PING_CHECK);
        payload.push(ping_id);

        let packet = Packet::new(0, 0, payload); // Non-reliable ping response
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize ping response: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent ping response to {} with ping_id={}", addr, ping_id);
        Ok(())
    }

    /// Handle CompletePingCheck message (ping response from client)
    pub async fn handle_ping_response(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 2 {
            warn!("CompletePingCheck packet too short from {}", addr);
            return Ok(());
        }

        let ping_id = packet.payload[1];

        // Find circuit by address and update ping info
        let mut circuits_guard = circuits.write().await;
        for (circuit_code, circuit) in circuits_guard.iter_mut() {
            if circuit.address == addr {
                let ping_time = circuit.last_ping_time.elapsed();
                debug!("Ping response from circuit {}: ping_id={}, rtt={:?}", 
                       circuit_code, ping_id, ping_time);
                
                circuit.last_activity = Instant::now();
                // Could store RTT statistics here
                break;
            }
        }

        Ok(())
    }

    /// Handle PacketAck message
    pub async fn handle_packet_ack(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 2 {
            warn!("PacketAck too short from {}", addr);
            return Ok(());
        }

        // Parse acknowledgments
        let ack_count = packet.payload[1] as usize;
        let mut acks = Vec::new();
        
        let mut offset = 2;
        for _ in 0..ack_count {
            if offset + 4 <= packet.payload.len() {
                let ack = u32::from_be_bytes([
                    packet.payload[offset], packet.payload[offset + 1],
                    packet.payload[offset + 2], packet.payload[offset + 3]
                ]);
                acks.push(ack);
                offset += 4;
            }
        }

        debug!("Received {} acknowledgments from {}: {:?}", acks.len(), addr, acks);

        // Find circuit and process acknowledgments
        let mut circuits_guard = circuits.write().await;
        for (circuit_code, circuit) in circuits_guard.iter_mut() {
            if circuit.address == addr {
                // Remove acknowledged reliable packets
                for ack in &acks {
                    circuit.reliable_packets.remove(ack);
                }
                
                circuit.last_activity = Instant::now();
                debug!("Processed {} acks for circuit {}", acks.len(), circuit_code);
                break;
            }
        }

        Ok(())
    }

    /// Send ping check to circuit
    pub async fn send_ping_check(
        &self,
        socket: &UdpSocket,
        circuit: &mut CircuitInfo,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<()> {
        let ping_id = circuit.last_ping_id.wrapping_add(1);
        circuit.last_ping_id = ping_id;
        circuit.last_ping_time = Instant::now();

        let mut payload = Vec::new();
        payload.push(packet_types::START_PING_CHECK);
        payload.push(ping_id);
        
        // Add oldest unacked sequence (simplified)
        let oldest_unacked = circuit.reliable_packets.keys().min().copied().unwrap_or(0);
        payload.extend_from_slice(&oldest_unacked.to_le_bytes());

        let packet = Packet::new(0, 0, payload); // Non-reliable ping
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize ping check: {}", e)))?;

        socket.send_to(&packet_data, circuit.address).await?;

        // Update stats
        let mut stats_guard = stats.write().await;
        stats_guard.heartbeats_sent += 1;
        stats_guard.packets_sent += 1;
        stats_guard.bytes_sent += packet_data.len() as u64;

        debug!("Sent ping check to circuit {} with ping_id={}", circuit.circuit_code, ping_id);
        Ok(())
    }

    /// Send heartbeat packet to maintain connection
    pub async fn send_heartbeat(
        &self,
        socket: &UdpSocket,
        circuit: &mut CircuitInfo,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<()> {
        // Use ping check as heartbeat
        self.send_ping_check(socket, circuit, stats).await
    }

    /// Process reliable packet resends
    pub async fn process_reliable_resends(
        &self,
        socket: &UdpSocket,
        circuit: &mut CircuitInfo,
        config: &mutsea_core::config::LLUDPConfig,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<()> {
        let timeout = std::time::Duration::from_millis(config.resend_timeout);
        let max_resends = config.max_resends;
        let now = Instant::now();

        let mut packets_to_resend = Vec::new();
        let mut packets_to_remove = Vec::new();

        // Check which packets need resending
        for (sequence, reliable_packet) in &mut circuit.reliable_packets {
            if reliable_packet.timestamp.elapsed() > timeout {
                if reliable_packet.resend_count < max_resends {
                    reliable_packet.resend_count += 1;
                    reliable_packet.timestamp = now;
                    packets_to_resend.push((*sequence, reliable_packet.data.clone()));
                } else {
                    // Max resends exceeded, remove packet
                    packets_to_remove.push(*sequence);
                }
            }
        }

        // Remove expired packets
        for sequence in packets_to_remove {
            circuit.reliable_packets.remove(&sequence);
            debug!("Removed expired reliable packet {} from circuit {}", sequence, circuit.circuit_code);
        }

        // Resend packets
        for (sequence, data) in packets_to_resend {
            if let Err(e) = socket.send_to(&data, circuit.address).await {
                warn!("Failed to resend packet {} to circuit {}: {}", sequence, circuit.circuit_code, e);
            } else {
                debug!("Resent packet {} to circuit {}", sequence, circuit.circuit_code);
                
                // Update stats
                let mut stats_guard = stats.write().await;
                stats_guard.reliable_resends += 1;
                stats_guard.packets_sent += 1;
                stats_guard.bytes_sent += data.len() as u64;
            }
        }

        Ok(())
    }

    /// Calculate ping statistics for circuit
    pub async fn calculate_ping_stats(
        &self,
        circuit: &CircuitInfo,
    ) -> PingStatistics {
        // In a real implementation, you would maintain a history of ping times
        // For now, return basic statistics
        PingStatistics {
            last_ping_time: circuit.last_ping_time.elapsed(),
            average_rtt: std::time::Duration::from_millis(50), // Placeholder
            min_rtt: std::time::Duration::from_millis(20),     // Placeholder
            max_rtt: std::time::Duration::from_millis(100),    // Placeholder
            packet_loss: 0.0, // Placeholder
        }
    }

    /// Check if circuit is healthy based on ping
    pub fn is_circuit_healthy(&self, circuit: &CircuitInfo, timeout: std::time::Duration) -> bool {
        circuit.last_activity.elapsed() < timeout
    }

    /// Send keep-alive packet
    pub async fn send_keep_alive(
        &self,
        socket: &UdpSocket,
        circuit: &mut CircuitInfo,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<()> {
        // Simple keep-alive using ping
        self.send_ping_check(socket, circuit, stats).await
    }
}

/// Ping statistics for a circuit
#[derive(Debug, Clone)]
pub struct PingStatistics {
    pub last_ping_time: std::time::Duration,
    pub average_rtt: std::time::Duration,
    pub min_rtt: std::time::Duration,
    pub max_rtt: std::time::Duration,
    pub packet_loss: f64, // 0.0 to 1.0
}

/// Circuit manager utility functions for ping handling
pub struct CircuitManager;

impl CircuitManager {
    /// Send heartbeat to all circuits that need it
    pub async fn send_heartbeats_to_all(
        socket: &UdpSocket,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        ping_interval: std::time::Duration,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<usize> {
        let ping_handler = PingHandler::new();
        let mut circuits_guard = circuits.write().await;
        let mut heartbeats_sent = 0;

        for circuit in circuits_guard.values_mut() {
            if circuit.last_activity.elapsed() > ping_interval {
                if let Err(e) = ping_handler.send_heartbeat(socket, circuit, stats).await {
                    warn!("Failed to send heartbeat to circuit {}: {}", circuit.circuit_code, e);
                } else {
                    heartbeats_sent += 1;
                }
            }
        }

        Ok(heartbeats_sent)
    }

    /// Process reliable resends for all circuits
    pub async fn process_reliable_resends_for_all(
        socket: &UdpSocket,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        config: &mutsea_core::config::LLUDPConfig,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<usize> {
        let ping_handler = PingHandler::new();
        let mut circuits_guard = circuits.write().await;
        let mut resends_processed = 0;

        for circuit in circuits_guard.values_mut() {
            if let Err(e) = ping_handler.process_reliable_resends(socket, circuit, config, stats).await {
                warn!("Failed to process resends for circuit {}: {}", circuit.circuit_code, e);
            } else {
                resends_processed += 1;
            }
        }

        Ok(resends_processed)
    }
}

impl Default for PingHandler {
    fn default() -> Self {
        Self::new()
    }
}