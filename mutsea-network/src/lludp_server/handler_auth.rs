//! mutsea-network/src/lludp_server/auth_handler.rs
//! Authentication and circuit management handler

use crate::NetworkResult;
use mutsea_core::{UserId, RegionId, Vector3};
use mutsea_protocol::{Packet, constants::packet_types, login::LoginService};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use super::{CircuitInfo, ClientInfo, ReliablePacketData, ServerStats};

/// Authentication handler for login and logout operations
#[derive(Clone)]
pub struct AuthHandler;

impl AuthHandler {
    pub fn new() -> Self {
        Self
    }

    /// Handle UseCircuitCode message
    pub async fn handle_use_circuit_code(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
        login_service: &LoginService,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 52 {
            warn!("UseCircuitCode packet too short from {}", addr);
            return Ok(());
        }

        // Parse UseCircuitCode packet structure
        let mut offset = 1; // Skip message ID
        
        let circuit_code = u32::from_le_bytes([
            packet.payload[offset], packet.payload[offset + 1], 
            packet.payload[offset + 2], packet.payload[offset + 3]
        ]);
        offset += 4;

        let session_id = uuid::Uuid::from_slice(&packet.payload[offset..offset + 16])
            .map_err(|e| crate::NetworkError::Protocol(format!("Invalid session ID: {}", e)))?;
        offset += 16;

        let agent_id_bytes = &packet.payload[offset..offset + 16];
        let agent_id = UserId::from_uuid(
            uuid::Uuid::from_slice(agent_id_bytes)
                .map_err(|e| crate::NetworkError::Protocol(format!("Invalid agent ID: {}", e)))?
        );

        info!("UseCircuitCode from {}: circuit={}, session={}, agent={}", 
              addr, circuit_code, session_id, agent_id);

        // Validate session with login service
        if !login_service.validate_session(&session_id.to_string(), &agent_id) {
            warn!("Invalid session for circuit {} from {}", circuit_code, addr);
            self.send_logout_response(socket, addr, "Invalid session").await?;
            return Ok(());
        }

        // Create or update circuit
        let mut circuits_guard = circuits.write().await;
        
        if let Some(existing_circuit) = circuits_guard.get_mut(&circuit_code) {
            // Update existing circuit
            existing_circuit.user_id = Some(agent_id);
            existing_circuit.agent_id = Some(agent_id);
            existing_circuit.session_id = Some(session_id);
            existing_circuit.authenticated = true;
            existing_circuit.last_activity = Instant::now();
            existing_circuit.address = addr;
        } else {
            // Create new circuit
            let circuit = CircuitInfo {
                circuit_code,
                address: addr,
                user_id: Some(agent_id),
                agent_id: Some(agent_id),
                session_id: Some(session_id),
                secure_session_id: None,
                created_at: Instant::now(),
                last_activity: Instant::now(),
                sequence_in: 0,
                sequence_out: 0,
                pending_acks: Vec::new(),
                reliable_packets: HashMap::new(),
                authenticated: true,
                region_id: Some(RegionId::new()),
                position: Vector3::new(128.0, 128.0, 21.0), // Default spawn position
                look_at: Vector3::new(1.0, 0.0, 0.0),
                client_info: None,
            };
            circuits_guard.insert(circuit_code, circuit);
        }

        info!("Circuit {} authenticated successfully from {}", circuit_code, addr);

        // Send RegionHandshake to establish the connection
        self.send_region_handshake(socket, addr, circuit_code).await?;

        Ok(())
    }

    /// Handle LogoutRequest message
    pub async fn handle_logout_request(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
    ) -> NetworkResult<()> {
        info!("Logout request from {}", addr);

        // Find and remove circuit for this address
        let mut circuits_guard = circuits.write().await;
        let mut circuit_to_remove = None;

        for (circuit_code, circuit) in circuits_guard.iter() {
            if circuit.address == addr {
                circuit_to_remove = Some(*circuit_code);
                break;
            }
        }

        if let Some(circuit_code) = circuit_to_remove {
            circuits_guard.remove(&circuit_code);
            info!("Circuit {} logged out from {}", circuit_code, addr);
        }

        Ok(())
    }

    /// Send logout response
    async fn send_logout_response(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        reason: &str,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::KICK_USER);
        
        // Reason (variable string)
        let reason_bytes = reason.as_bytes();
        payload.extend_from_slice(&(reason_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(reason_bytes);
        
        let packet = Packet::reliable(0, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize logout packet: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Send region handshake to establish connection
    async fn send_region_handshake(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        circuit_code: u32,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::REGION_HANDSHAKE);

        // RegionInfo block
        payload.extend_from_slice(&128u32.to_le_bytes()); // RegionFlags
        payload.extend_from_slice(&1u8.to_le_bytes());    // SimAccess
        
        // Region name (variable string)
        let region_name = "Mutsea Region".as_bytes();
        payload.push(region_name.len() as u8);
        payload.extend_from_slice(region_name);
        
        // SimOwner UUID (16 bytes)
        let sim_owner = uuid::Uuid::new_v4();
        payload.extend_from_slice(sim_owner.as_bytes());
        
        // IsEstateManager
        payload.push(1u8);
        
        // WaterHeight
        payload.extend_from_slice(&20.0f32.to_le_bytes());
        
        // BillableFactor
        payload.extend_from_slice(&1.0f32.to_le_bytes());
        
        // CacheID UUID
        let cache_id = uuid::Uuid::new_v4();
        payload.extend_from_slice(cache_id.as_bytes());
        
        // TerrainBase0-3 UUIDs
        for _ in 0..4 {
            let terrain_id = uuid::Uuid::new_v4();
            payload.extend_from_slice(terrain_id.as_bytes());
        }
        
        // TerrainDetail0-3 UUIDs
        for _ in 0..4 {
            let detail_id = uuid::Uuid::new_v4();
            payload.extend_from_slice(detail_id.as_bytes());
        }
        
        // TerrainStartHeight00-11
        for _ in 0..4 {
            payload.extend_from_slice(&10.0f32.to_le_bytes());
        }
        
        // TerrainHeightRange00-11
        for _ in 0..4 {
            payload.extend_from_slice(&60.0f32.to_le_bytes());
        }

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize handshake packet: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        info!("Sent region handshake to circuit {} at {}", circuit_code, addr);
        Ok(())
    }

    /// Authenticate circuit and update session
    pub async fn authenticate_circuit(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
        user_id: UserId,
        session_id: uuid::Uuid,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<bool> {
        let mut circuits_guard = circuits.write().await;
        
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.user_id = Some(user_id);
            circuit.agent_id = Some(user_id);
            circuit.session_id = Some(session_id);
            circuit.authenticated = true;
            circuit.last_activity = Instant::now();
            
            // Update stats
            let mut stats_guard = stats.write().await;
            stats_guard.successful_logins += 1;
            
            info!("Circuit {} authenticated for user {}", circuit_code, user_id);
            Ok(true)
        } else {
            warn!("Circuit {} not found for authentication", circuit_code);
            Ok(false)
        }
    }

    /// Deauthenticate circuit
    pub async fn deauthenticate_circuit(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
    ) -> NetworkResult<()> {
        let mut circuits_guard = circuits.write().await;
        
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.authenticated = false;
            circuit.user_id = None;
            circuit.agent_id = None;
            circuit.session_id = None;
            info!("Circuit {} deauthenticated", circuit_code);
        }
        
        Ok(())
    }

    /// Check if circuit is authenticated
    pub async fn is_circuit_authenticated(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
    ) -> bool {
        let circuits_guard = circuits.read().await;
        circuits_guard.get(&circuit_code)
            .map(|c| c.authenticated)
            .unwrap_or(false)
    }

    /// Get authenticated user ID for circuit
    pub async fn get_circuit_user_id(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
    ) -> Option<UserId> {
        let circuits_guard = circuits.read().await;
        circuits_guard.get(&circuit_code)
            .and_then(|c| c.user_id)
    }

    /// Update client information for circuit
    pub async fn update_client_info(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        circuit_code: u32,
        client_info: ClientInfo,
    ) -> NetworkResult<()> {
        let mut circuits_guard = circuits.write().await;
        
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.client_info = Some(client_info);
            info!("Updated client info for circuit {}", circuit_code);
        } else {
            warn!("Circuit {} not found for client info update", circuit_code);
        }
        
        Ok(())
    }

    /// Send EnableSimulator message to client
    pub async fn send_enable_simulator(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        region_handle: u64,
        sim_ip: std::net::Ipv4Addr,
        sim_port: u16,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::ENABLE_SIMULATOR);

        // SimulatorInfo block
        payload.extend_from_slice(&region_handle.to_le_bytes());
        payload.extend_from_slice(&sim_ip.octets());
        payload.extend_from_slice(&sim_port.to_le_bytes());

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize EnableSimulator packet: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        info!("Sent EnableSimulator to {}", addr);
        Ok(())
    }

    /// Send EstablishAgentCommunication message
    pub async fn send_establish_agent_communication(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        agent_id: UserId,
        session_id: uuid::Uuid,
        seed_capability: &str,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::ESTABLISH_AGENT_COMMUNICATION);

        // AgentData block
        payload.extend_from_slice(agent_id.as_uuid().as_bytes());
        payload.extend_from_slice(session_id.as_bytes());

        // SeedCapability (variable string)
        let seed_bytes = seed_capability.as_bytes();
        payload.extend_from_slice(&(seed_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(seed_bytes);

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize EstablishAgentCommunication packet: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        info!("Sent EstablishAgentCommunication to {}", addr);
        Ok(())
    }
}

impl Default for AuthHandler {
    fn default() -> Self {
        Self::new()
    }
}