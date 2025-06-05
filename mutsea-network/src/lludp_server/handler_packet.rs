//! mutsea-network/src/lludp_server/packet_handler.rs
//! Main packet dispatch and routing

use crate::NetworkResult;
use mutsea_core::config::LLUDPConfig;
use mutsea_protocol::{Packet, constants::packet_types, login::LoginService};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{debug, error};

use super::{
    CircuitInfo, ServerStats, AuthHandler, MovementHandler, 
    ChatHandler, PingHandler, RegionHandler
};

/// Main packet handler that routes packets to specialized handlers
#[derive(Clone)]
pub struct PacketHandler {
    auth_handler: AuthHandler,
    movement_handler: MovementHandler,
    chat_handler: ChatHandler,
    ping_handler: PingHandler,
    region_handler: RegionHandler,
}

impl PacketHandler {
    pub fn new() -> Self {
        Self {
            auth_handler: AuthHandler::new(),
            movement_handler: MovementHandler::new(),
            chat_handler: ChatHandler::new(),
            ping_handler: PingHandler::new(),
            region_handler: RegionHandler::new(),
        }
    }

    /// Main packet handling dispatch
    pub async fn handle_packet(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        data: &[u8],
        config: &LLUDPConfig,
        login_service: &LoginService,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<()> {
        // Parse packet
        let packet = match Packet::deserialize(data) {
            Ok(packet) => packet,
            Err(e) => {
                debug!("Failed to parse packet from {}: {}", addr, e);
                return Ok(()); // Ignore malformed packets
            }
        };

        debug!("Received packet from {}: seq={}, size={}, reliable={}", 
               addr, packet.header.sequence, data.len(), packet.header.is_reliable());

        // Update stats
        {
            let mut stats_guard = stats.write().await;
            stats_guard.packets_received += 1;
            stats_guard.bytes_received += data.len() as u64;
        }

        // Handle packet based on type
        if let Some(message_id) = packet.message_id {
            self.handle_message_packet(circuits, socket, addr, &packet, message_id, config, login_service).await?;
        } else {
            // Handle raw packet
            self.handle_raw_packet(circuits, socket, addr, &packet, config).await?;
        }

        Ok(())
    }

    /// Handle message packet with ID
    async fn handle_message_packet(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
        message_id: u32,
        config: &LLUDPConfig,
        login_service: &LoginService,
    ) -> NetworkResult<()> {
        match message_id {
            // Authentication messages
            packet_types::USE_CIRCUIT_CODE => {
                self.auth_handler.handle_use_circuit_code(circuits, socket, addr, packet, login_service).await?;
            }
            packet_types::LOGOUT_REQUEST => {
                self.auth_handler.handle_logout_request(circuits, addr).await?;
            }

            // Movement messages
            packet_types::AGENT_UPDATE => {
                self.movement_handler.handle_agent_update(circuits, addr, packet).await?;
            }
            packet_types::AGENT_ANIMATION => {
                self.movement_handler.handle_agent_animation(circuits, addr, packet).await?;
            }
            packet_types::COMPLETE_AGENT_MOVEMENT => {
                self.movement_handler.handle_complete_agent_movement(circuits, socket, addr, packet).await?;
            }

            // Chat messages
            packet_types::CHAT_FROM_VIEWER => {
                self.chat_handler.handle_chat_from_viewer(circuits, socket, addr, packet).await?;
            }

            // Region messages
            packet_types::REGION_HANDSHAKE_REPLY => {
                self.region_handler.handle_region_handshake_reply(circuits, socket, addr, packet).await?;
            }

            _ => {
                debug!("Unhandled message type: 0x{:02X} from {}", message_id, addr);
            }
        }

        Ok(())
    }

    /// Handle raw packet without message ID
    async fn handle_raw_packet(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
        config: &LLUDPConfig,
    ) -> NetworkResult<()> {
        if packet.payload.is_empty() {
            return Ok(());
        }

        let message_type = packet.payload[0];
        
        match message_type {
            packet_types::START_PING_CHECK => {
                self.ping_handler.handle_ping_check(socket, addr, packet).await?;
            }
            packet_types::COMPLETE_PING_CHECK => {
                self.ping_handler.handle_ping_response(circuits, addr, packet).await?;
            }
            packet_types::PACKET_ACK => {
                self.ping_handler.handle_packet_ack(circuits, addr, packet).await?;
            }
            _ => {
                debug!("Unhandled raw message type: 0x{:02X} from {}", message_type, addr);
            }
        }

        Ok(())
    }
}

impl Default for PacketHandler {
    fn default() -> Self {
        Self::new()
    }
}