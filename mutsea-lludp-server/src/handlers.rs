//! mutsea-network/src/lludp_server/handlers.rs
//! Combined packet handlers for LLUDP server

use crate::NetworkResult;
use mutsea_core::config::LLUDPConfig;
use mutsea_protocol::{Packet, constants::packet_types, login::LoginService};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{debug, error, warn};

use super::{
    CircuitInfo, ServerStats, AuthHandler, MovementHandler, 
    ChatHandler, PingHandler, RegionHandler
};

/// Combined packet handlers that route packets to specialized handlers
#[derive(Clone)]
pub struct PacketHandlers {
    auth_handler: AuthHandler,
    movement_handler: MovementHandler,
    chat_handler: ChatHandler,
    ping_handler: PingHandler,
    region_handler: RegionHandler,
}

impl PacketHandlers {
    /// Create new packet handlers
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
        session_manager: &crate::SessionManager,
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
            self.handle_message_packet(
                circuits, socket, addr, &packet, message_id, 
                config, login_service, stats
            ).await?;
        } else {
            // Handle raw packet
            self.handle_raw_packet(
                circuits, socket, addr, &packet, config, stats
            ).await?;
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
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<()> {
        match message_id {
            // Authentication messages
            packet_types::USE_CIRCUIT_CODE => {
                self.auth_handler.handle_use_circuit_code(
                    circuits, socket, addr, packet, login_service
                ).await?;
            }
            packet_types::LOGOUT_REQUEST => {
                self.auth_handler.handle_logout_request(circuits, addr).await?;
            }

            // Movement messages
            packet_types::AGENT_UPDATE => {
                self.movement_handler.handle_agent_update(circuits, addr, packet).await?;
            }
            packet_types::COMPLETE_AGENT_MOVEMENT => {
                self.handle_complete_agent_movement(circuits, socket, addr, packet).await?;
            }

            // Chat messages
            packet_types::CHAT_FROM_VIEWER => {
                self.chat_handler.handle_chat_from_viewer(
                    circuits, socket, addr, packet
                ).await?;
            }

            // Region messages
            packet_types::REGION_HANDSHAKE_REPLY => {
                self.region_handler.handle_region_handshake_reply(
                    circuits, socket, addr, packet
                ).await?;
            }

            // Object messages
            packet_types::OBJECT_SELECT => {
                self.handle_object_select(circuits, socket, addr, packet).await?;
            }
            packet_types::OBJECT_DESELECT => {
                self.handle_object_deselect(circuits, addr, packet).await?;
            }

            // Asset messages
            packet_types::REQUEST_IMAGE => {
                self.handle_request_image(circuits, socket, addr, packet).await?;
            }
            packet_types::TRANSFER_REQUEST => {
                self.handle_transfer_request(circuits, socket, addr, packet).await?;
            }

            // Animation messages
            packet_types::AGENT_ANIMATION => {
                self.handle_agent_animation(circuits, addr, packet).await?;
            }

            // Teleport messages
            packet_types::TELEPORT_REQUEST => {
                self.region_handler.handle_teleport_request(
                    circuits, socket, addr, packet
                ).await?;
            }

            // Inventory messages
            packet_types::FETCH_INVENTORY_DESCENDENTS => {
                self.handle_fetch_inventory(circuits, socket, addr, packet).await?;
            }

            // Money/Economy messages
            packet_types::MONEY_BALANCE_REQUEST => {
                self.handle_money_balance_request(circuits, socket, addr, packet).await?;
            }

            // Group messages
            packet_types::GROUP_MEMBERSHIP_DATA => {
                self.handle_group_membership_data(circuits, socket, addr, packet).await?;
            }

            // Parcel messages
            packet_types::PARCEL_INFO_REQUEST => {
                self.handle_parcel_info_request(circuits, socket, addr, packet).await?;
            }

            // Map messages
            packet_types::MAP_BLOCK_REQUEST => {
                self.handle_map_block_request(circuits, socket, addr, packet).await?;
            }

            // Script messages
            packet_types::SCRIPT_DIALOG_REPLY => {
                self.handle_script_dialog_reply(circuits, addr, packet).await?;
            }

            // Voice messages
            packet_types::PROVISION_VOICE_ACCOUNT_REQUEST => {
                self.handle_provision_voice_account(circuits, socket, addr, packet).await?;
            }

            _ => {
                debug!("Unhandled message type: 0x{:08X} from {}", message_id, addr);
                // Update stats for unhandled packets
                let mut stats_guard = stats.write().await;
                stats_guard.errors += 1;
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
        stats: &Arc<RwLock<ServerStats>>,
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

    /// Handle CompleteAgentMovement message
    async fn handle_complete_agent_movement(
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

        debug!("CompleteAgentMovement from circuit {}", circuit_code);

        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = std::time::Instant::now();
        }

        // Send agent movement complete response
        self.send_agent_movement_complete(socket, addr).await?;

        Ok(())
    }

    /// Send AgentMovementComplete response
    async fn send_agent_movement_complete(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::AGENT_MOVEMENT_COMPLETE as u8);

        // AgentData block
        payload.extend_from_slice(&mutsea_core::UserId::new().as_uuid().as_bytes());
        payload.extend_from_slice(&uuid::Uuid::new_v4().as_bytes());

        // Data block
        payload.extend_from_slice(&0u64.to_le_bytes()); // RegionHandle
        payload.extend_from_slice(&std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_le_bytes());

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize AgentMovementComplete: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent AgentMovementComplete to {}", addr);
        Ok(())
    }

    /// Handle object selection
    async fn handle_object_select(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("ObjectSelect from {}", addr);

        // Find circuit and update activity
        let mut circuits_guard = circuits.write().await;
        for circuit in circuits_guard.values_mut() {
            if circuit.address == addr {
                circuit.last_activity = std::time::Instant::now();
                break;
            }
        }

        // TODO: Parse object selection and send object properties
        Ok(())
    }

    /// Handle object deselection
    async fn handle_object_deselect(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("ObjectDeselect from {}", addr);

        // Update circuit activity
        let mut circuits_guard = circuits.write().await;
        for circuit in circuits_guard.values_mut() {
            if circuit.address == addr {
                circuit.last_activity = std::time::Instant::now();
                break;
            }
        }

        Ok(())
    }

    /// Handle agent animation
    async fn handle_agent_animation(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("AgentAnimation from {}", addr);

        // Update circuit activity
        let mut circuits_guard = circuits.write().await;
        for circuit in circuits_guard.values_mut() {
            if circuit.address == addr {
                circuit.last_activity = std::time::Instant::now();
                break;
            }
        }

        // TODO: Parse animation data and broadcast to nearby agents
        Ok(())
    }

    /// Handle image/texture request
    async fn handle_request_image(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("RequestImage from {}", addr);

        // Send image not found response for now
        let mut payload = Vec::new();
        payload.push(packet_types::IMAGE_NOT_IN_DATABASE as u8);

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize ImageNotInDatabase: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Handle asset transfer request
    async fn handle_transfer_request(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("TransferRequest from {}", addr);

        // Send transfer info with not found status
        let mut payload = Vec::new();
        payload.push(packet_types::TRANSFER_INFO as u8);

        // TransferInfo block
        payload.extend_from_slice(&uuid::Uuid::new_v4().as_bytes()); // TransferID
        payload.extend_from_slice(&2u32.to_le_bytes()); // ChannelType (Asset)
        payload.extend_from_slice(&(-1i32).to_le_bytes()); // Status (not found)
        payload.extend_from_slice(&uuid::Uuid::new_v4().as_bytes()); // TargetID
        payload.extend_from_slice(&0u32.to_le_bytes()); // Size
        payload.extend_from_slice(&vec![0u8; 0]); // Params

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize TransferInfo: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Handle inventory fetch request
    async fn handle_fetch_inventory(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("FetchInventoryDescendents from {}", addr);

        // Send empty inventory response
        let mut payload = Vec::new();
        payload.push(packet_types::INVENTORY_DESCENDENTS as u8);

        // AgentData block
        payload.extend_from_slice(&mutsea_core::UserId::new().as_uuid().as_bytes());

        // InventoryData block (empty)
        payload.extend_from_slice(&uuid::Uuid::new_v4().as_bytes()); // FolderID
        payload.extend_from_slice(&mutsea_core::UserId::new().as_uuid().as_bytes()); // OwnerID
        payload.extend_from_slice(&0u32.to_le_bytes()); // Version
        payload.extend_from_slice(&0u32.to_le_bytes()); // Descendents

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize InventoryDescendents: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Handle money balance request
    async fn handle_money_balance_request(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("MoneyBalanceRequest from {}", addr);

        // Send money balance reply with 1000 credits
        let mut payload = Vec::new();
        payload.push(packet_types::MONEY_BALANCE_REPLY as u8);

        // MoneyData block
        payload.extend_from_slice(&mutsea_core::UserId::new().as_uuid().as_bytes()); // AgentID
        payload.extend_from_slice(&uuid::Uuid::new_v4().as_bytes()); // TransactionID
        payload.push(1u8); // TransactionSuccess
        payload.extend_from_slice(&1000i32.to_le_bytes()); // MoneyBalance
        payload.extend_from_slice(&0i32.to_le_bytes()); // SquareMetersCredit
        payload.extend_from_slice(&0i32.to_le_bytes()); // SquareMetersCommitted
        
        // Description (variable string)
        let desc = "Your current balance";
        payload.push(desc.len() as u8);
        payload.extend_from_slice(desc.as_bytes());

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize MoneyBalanceReply: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Handle group membership data request
    async fn handle_group_membership_data(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("GroupMembershipData from {}", addr);
        // TODO: Implement group membership
        Ok(())
    }

    /// Handle parcel info request
    async fn handle_parcel_info_request(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("ParcelInfoRequest from {}", addr);
        // TODO: Implement parcel information
        Ok(())
    }

    /// Handle map block request
    async fn handle_map_block_request(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("MapBlockRequest from {}", addr);

        // Send empty map block reply
        let mut payload = Vec::new();
        payload.push(packet_types::MAP_BLOCK_REPLY as u8);

        // AgentData block
        payload.extend_from_slice(&mutsea_core::UserId::new().as_uuid().as_bytes());
        payload.extend_from_slice(&0u32.to_le_bytes()); // Flags

        // Data block (empty - no regions)
        payload.push(0u8); // No map blocks

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize MapBlockReply: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Handle script dialog reply
    async fn handle_script_dialog_reply(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("ScriptDialogReply from {}", addr);
        // TODO: Process script dialog response
        Ok(())
    }

    /// Handle voice account provisioning
    async fn handle_provision_voice_account(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        debug!("ProvisionVoiceAccountRequest from {}", addr);

        // Send voice account reply (voice disabled)
        let mut payload = Vec::new();
        payload.push(packet_types::PROVISION_VOICE_ACCOUNT_REPLY as u8);

        // AgentData block
        payload.extend_from_slice(&mutsea_core::UserId::new().as_uuid().as_bytes());

        // VoiceData block
        let voice_server_type = "none";
        payload.push(voice_server_type.len() as u8);
        payload.extend_from_slice(voice_server_type.as_bytes());

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize ProvisionVoiceAccountReply: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Get handler statistics
    pub async fn get_handler_stats(&self) -> HandlerStats {
        HandlerStats {
            auth_operations: 0, // Would track actual operations
            movement_updates: 0,
            chat_messages: 0,
            ping_checks: 0,
            region_operations: 0,
        }
    }
}

/// Handler statistics
#[derive(Debug, Clone)]
pub struct HandlerStats {
    pub auth_operations: u64,
    pub movement_updates: u64,
    pub chat_messages: u64,
    pub ping_checks: u64,
    pub region_operations: u64,
}

impl Default for PacketHandlers {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for HandlerStats {
    fn default() -> Self {
        Self {
            auth_operations: 0,
            movement_updates: 0,
            chat_messages: 0,
            ping_checks: 0,
            region_operations: 0,
        }
    }
}