//! mutsea-network/src/lludp_server/handler_chat.rs
//! Chat and communication handler

use crate::NetworkResult;
use mutsea_core::{Vector3, UserId};
use mutsea_protocol::{Packet, constants::packet_types};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{debug, warn, info};

use super::{CircuitInfo, ServerStats};

/// Chat handler for communication between agents
#[derive(Clone)]
pub struct ChatHandler;

impl ChatHandler {
    pub fn new() -> Self {
        Self
    }

    /// Handle ChatFromViewer message
    pub async fn handle_chat_from_viewer(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 33 { // Minimum size for ChatFromViewer
            warn!("ChatFromViewer packet too short from {}", addr);
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

        // Parse chat message
        let chat_data = self.parse_chat_message(&packet.payload)?;
        
        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }
        drop(circuits_guard);

        info!("Chat from circuit {}: {} says: '{}'", 
              circuit_code, chat_data.from_name, chat_data.message);

        // Broadcast to nearby users
        self.broadcast_chat_message(circuits, socket, circuit_code, &chat_data).await?;

        Ok(())
    }

    /// Parse chat message from packet payload
    fn parse_chat_message(&self, payload: &[u8]) -> NetworkResult<ChatMessageData> {
        let mut offset = 1; // Skip message ID

        // AgentData block
        let _agent_id = &payload[offset..offset + 16];
        offset += 16;
        
        let _session_id = &payload[offset..offset + 16];
        offset += 16;

        // ChatData block
        let message_length = payload[offset] as usize;
        offset += 1;
        
        if offset + message_length > payload.len() {
            return Err(crate::NetworkError::InvalidPacket("Chat message too long".to_string()));
        }
        
        let message = String::from_utf8_lossy(&payload[offset..offset + message_length]).to_string();
        offset += message_length;

        let chat_type = if offset < payload.len() { payload[offset] } else { 0 };
        offset += 1;
        
        let channel = if offset + 4 <= payload.len() {
            i32::from_le_bytes([
                payload[offset], payload[offset + 1], 
                payload[offset + 2], payload[offset + 3]
            ])
        } else {
            0
        };

        Ok(ChatMessageData {
            message,
            chat_type,
            channel,
            from_name: "Unknown".to_string(), // Would be filled from circuit info
            position: Vector3::ZERO, // Would be filled from circuit info
        })
    }

    /// Broadcast chat message to nearby users
    pub async fn broadcast_chat_message(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        source_circuit: u32,
        chat_data: &ChatMessageData,
    ) -> NetworkResult<usize> {
        let circuits_guard = circuits.read().await;
        
        let Some(source) = circuits_guard.get(&source_circuit) else {
            return Ok(0);
        };
        
        let source_position = source.position;
        let source_name = format!("Agent {}", source_circuit); // Would use actual name
        let chat_range = self.get_chat_range(chat_data.chat_type);
        
        // Find nearby circuits within chat range
        let mut nearby_circuits = Vec::new();
        for (circuit_code, circuit) in circuits_guard.iter() {
            if *circuit_code != source_circuit && circuit.authenticated {
                let distance = (circuit.position - source_position).length();
                if distance <= chat_range {
                    nearby_circuits.push((*circuit_code, circuit.address));
                }
            }
        }
        
        drop(circuits_guard); // Release the lock
        
        let broadcast_count = nearby_circuits.len();
        
        // Create ChatFromSimulator packet
        let chat_packet = self.create_chat_packet(
            &source_name,
            &chat_data.message,
            chat_data.chat_type,
            source_position,
            source.agent_id.unwrap_or_default(),
        )?;
        
        let packet_data = chat_packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize chat packet: {}", e)))?;
        
        // Send to nearby circuits
        for (target_circuit, target_address) in nearby_circuits {
            if let Err(e) = socket.send_to(&packet_data, target_address).await {
                warn!("Failed to send chat to circuit {}: {}", target_circuit, e);
            }
        }
        
        debug!("Broadcasted chat from circuit {} to {} nearby users", source_circuit, broadcast_count);
        Ok(broadcast_count)
    }

    /// Create ChatFromSimulator packet
    fn create_chat_packet(
        &self,
        from_name: &str,
        message: &str,
        chat_type: u8,
        position: Vector3,
        source_id: UserId,
    ) -> NetworkResult<Packet> {
        let mut payload = Vec::new();
        payload.push(packet_types::CHAT_FROM_SIMULATOR as u8);

        // ChatData block
        // FromName (variable string)
        let name_bytes = from_name.as_bytes();
        payload.push(name_bytes.len() as u8);
        payload.extend_from_slice(name_bytes);

        // SourceID (UUID)
        payload.extend_from_slice(source_id.as_uuid().as_bytes());

        // OwnerID (UUID) - same as source for agents
        payload.extend_from_slice(source_id.as_uuid().as_bytes());

        // SourceType (U8) - 0 = agent
        payload.push(0);

        // ChatType (U8)
        payload.push(chat_type);

        // Audible (U8)
        payload.push(1); // Audible

        // Position (Vector3)
        payload.extend_from_slice(&position.x.to_le_bytes());
        payload.extend_from_slice(&position.y.to_le_bytes());
        payload.extend_from_slice(&position.z.to_le_bytes());

        // Message (variable string)
        let message_bytes = message.as_bytes();
        payload.extend_from_slice(&(message_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(message_bytes);

        Ok(Packet::reliable(0, payload))
    }

    /// Get chat range based on chat type
    fn get_chat_range(&self, chat_type: u8) -> f32 {
        match chat_type {
            0 => 20.0,  // Say
            1 => 100.0, // Shout
            2 => 10.0,  // Whisper
            _ => 20.0,  // Default to say range
        }
    }

    /// Handle instant message
    pub async fn handle_instant_message(
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

        // Parse IM message (simplified)
        if packet.payload.len() < 50 {
            warn!("InstantMessage packet too short from {}", addr);
            return Ok(());
        }

        debug!("Instant message from circuit {}", circuit_code);

        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }

        // TODO: Process IM message and route to target user
        Ok(())
    }

    /// Handle script dialog
    pub async fn handle_script_dialog(
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

        debug!("Script dialog from circuit {}", circuit_code);

        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }

        // TODO: Process script dialog response
        Ok(())
    }

    /// Send system message to specific circuit
    pub async fn send_system_message(
        &self,
        socket: &UdpSocket,
        target_address: SocketAddr,
        message: &str,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<()> {
        let system_chat = self.create_chat_packet(
            "System",
            message,
            6, // Debug/System chat type
            Vector3::ZERO,
            UserId::new(), // System user ID
        )?;

        let packet_data = system_chat.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize system message: {}", e)))?;

        socket.send_to(&packet_data, target_address).await?;

        // Update stats
        let mut stats_guard = stats.write().await;
        stats_guard.packets_sent += 1;
        stats_guard.bytes_sent += packet_data.len() as u64;

        debug!("Sent system message to {}: {}", target_address, message);
        Ok(())
    }

    /// Broadcast system announcement to all users
    pub async fn broadcast_system_announcement(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        message: &str,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<usize> {
        let circuits_guard = circuits.read().await;
        let mut broadcast_count = 0;

        let system_chat = self.create_chat_packet(
            "System",
            message,
            6, // Debug/System chat type
            Vector3::ZERO,
            UserId::new(), // System user ID
        )?;

        let packet_data = system_chat.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize system announcement: {}", e)))?;

        for circuit in circuits_guard.values() {
            if circuit.authenticated {
                if let Err(e) = socket.send_to(&packet_data, circuit.address).await {
                    warn!("Failed to send system announcement to circuit {}: {}", circuit.circuit_code, e);
                } else {
                    broadcast_count += 1;
                }
            }
        }

        // Update stats
        if broadcast_count > 0 {
            let mut stats_guard = stats.write().await;
            stats_guard.packets_sent += broadcast_count as u64;
            stats_guard.bytes_sent += (packet_data.len() * broadcast_count) as u64;
        }

        info!("Broadcasted system announcement to {} users: {}", broadcast_count, message);
        Ok(broadcast_count)
    }
}

/// Parsed chat message data
#[derive(Debug, Clone)]
pub struct ChatMessageData {
    pub message: String,
    pub chat_type: u8,
    pub channel: i32,
    pub from_name: String,
    pub position: Vector3,
}

impl Default for ChatHandler {
    fn default() -> Self {
        Self::new()
    }
}