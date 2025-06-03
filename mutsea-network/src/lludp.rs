//! LLUDP server implementation for OpenSim compatibility

use crate::{ClientSession, SessionManager};
use mutsea_core::{Service, ServiceHealth, ServiceStatus, MutseaResult, config::LLUDPConfig, NetworkError, NetworkResult};
use mutsea_protocol::{Packet, PacketHeader, constants::*};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// LLUDP server for handling OpenSim viewer connections
pub struct LLUDPServer {
    socket: Arc<UdpSocket>,
    session_manager: SessionManager,
    config: LLUDPConfig,
    running: Arc<std::sync::atomic::AtomicBool>,
    stats: Arc<RwLock<ServerStats>>,
}

/// Server statistics
#[derive(Debug, Default)]
pub struct ServerStats {
    pub packets_received: u64,
    pub packets_sent: u64,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub connections: u64,
    pub active_sessions: u64,
    pub errors: u64,
}

impl LLUDPServer {
    /// Create a new LLUDP server
    pub async fn new(config: &LLUDPConfig) -> NetworkResult<Self> {
        let bind_addr = format!("{}:{}", config.bind_address, config.port);
        let socket = UdpSocket::bind(&bind_addr).await?;
        info!("LLUDP server bound to {}", bind_addr);
        
        let session_manager = SessionManager::new(
            Duration::from_secs(60),
            Duration::from_secs(config.client_timeout),
        );
        
        Ok(Self {
            socket: Arc::new(socket),
            session_manager,
            config: config.clone(),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            stats: Arc::new(RwLock::new(ServerStats::default())),
        })
    }
    
    /// Start the LLUDP server
    pub async fn start(&self) -> NetworkResult<()> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Start session cleanup task
        self.session_manager.start_cleanup_task().await;
        
        // Start main packet handling loop
        let socket = Arc::clone(&self.socket);
        let session_manager = self.session_manager.clone();
        let stats = Arc::clone(&self.stats);
        let running = Arc::clone(&self.running);
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut buffer = vec![0u8; config.max_packet_size];
            
            while running.load(std::sync::atomic::Ordering::SeqCst) {
                match socket.recv_from(&mut buffer).await {
                    Ok((size, addr)) => {
                        // Update stats
                        {
                            let mut stats_guard = stats.write().await;
                            stats_guard.packets_received += 1;
                            stats_guard.bytes_received += size as u64;
                        }
                        
                        // Process packet
                        let packet_data = &buffer[..size];
                        if let Err(e) = Self::handle_packet(
                            &session_manager,
                            &socket,
                            addr,
                            packet_data,
                            &config,
                        ).await {
                            error!("Error handling packet from {}: {}", addr, e);
                            let mut stats_guard = stats.write().await;
                            stats_guard.errors += 1;
                        }
                    }
                    Err(e) => {
                        error!("Error receiving packet: {}", e);
                        let mut stats_guard = stats.write().await;
                        stats_guard.errors += 1;
                    }
                }
            }
        });
        
        info!("LLUDP server started successfully");
        Ok(())
    }
    
    /// Stop the LLUDP server
    pub async fn stop(&self) -> NetworkResult<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        info!("LLUDP server stopped");
        Ok(())
    }
    
    /// Handle incoming packet
    async fn handle_packet(
        session_manager: &SessionManager,
        socket: &UdpSocket,
        addr: SocketAddr,
        data: &[u8],
        config: &LLUDPConfig,
    ) -> NetworkResult<()> {
        // Parse packet
        let packet = Packet::deserialize(data)
            .map_err(|e| NetworkError::Protocol(format!("Failed to parse packet: {}", e)))?;
        
        debug!("Received packet from {}: seq={}, size={}", addr, packet.header.sequence, data.len());
        
        // Get or create session
        let mut session = session_manager.get_session(&addr).await;
        if session.is_none() {
            // Create new session for new connection
            let circuit_code = rand::random::<u32>();
            let client_info = crate::session::ClientInfo {
                viewer_name: "Unknown".to_string(),
                viewer_version: "Unknown".to_string(),
                platform: "Unknown".to_string(),
                mac_address: "Unknown".to_string(),
                id0: "Unknown".to_string(),
                channel: "Unknown".to_string(),
            };
            
            let new_session = ClientSession::new(circuit_code, client_info);
            session_manager.add_session(addr, new_session).await;
            session = session_manager.get_session(&addr).await;
        }
        
        if let Some(mut session) = session {
            // Update session
            session.update_last_seen();
            session_manager.update_session(&addr, |s| {
                s.update_last_seen();
                if s.process_sequence_in(packet.header.sequence) {
                    // New packet, process it
                    if packet.header.is_reliable() {
                        s.add_ack(packet.header.sequence);
                    }
                }
            }).await;
            
            // Handle packet based on type
            Self::process_packet_content(&packet, &addr, socket, session_manager).await?;
            
            // Send acknowledgments if needed
            if !packet.appended_acks.is_empty() {
                session_manager.update_session(&addr, |s| {
                    for ack in &packet.appended_acks {
                        s.process_ack(*ack);
                    }
                }).await;
            }
        }
        
        Ok(())
    }
    
    /// Process packet content based on message type
    async fn process_packet_content(
        packet: &Packet,
        addr: &SocketAddr,
        socket: &UdpSocket,
        session_manager: &SessionManager,
    ) -> NetworkResult<()> {
        // Basic packet type detection based on OpenSim protocol
        if packet.payload.is_empty() {
            return Ok(());
        }
        
        let message_type = packet.payload[0];
        
        match message_type {
            packet_types::START_PING_CHECK => {
                Self::handle_ping_check(packet, addr, socket).await?;
            }
            packet_types::COMPLETE_PING_CHECK => {
                Self::handle_ping_response(packet, addr, session_manager).await?;
            }
            packet_types::AGENT_UPDATE => {
                Self::handle_agent_update(packet, addr, session_manager).await?;
            }
            packet_types::LOGOUT_REQUEST => {
                Self::handle_logout(addr, session_manager).await?;
            }
            _ => {
                debug!("Unhandled message type: 0x{:02X}", message_type);
            }
        }
        
        Ok(())
    }
    
    /// Handle ping check message
    async fn handle_ping_check(
        packet: &Packet,
        addr: &SocketAddr,
        socket: &UdpSocket,
    ) -> NetworkResult<()> {
        // Send ping response
        let response_payload = vec![packet_types::COMPLETE_PING_CHECK, packet.payload[1]];
        let response = Packet::new(0, 0, response_payload);
        let response_data = response.serialize()
            .map_err(|e| NetworkError::Protocol(format!("Failed to serialize ping response: {}", e)))?;
        
        socket.send_to(&response_data, addr).await?;
        debug!("Sent ping response to {}", addr);
        
        Ok(())
    }
    
    /// Handle ping response message
    async fn handle_ping_response(
        _packet: &Packet,
        addr: &SocketAddr,
        session_manager: &SessionManager,
    ) -> NetworkResult<()> {
        // Update session with ping response
        session_manager.update_session(addr, |session| {
            session.update_last_seen();
        }).await;
        
        debug!("Received ping response from {}", addr);
        Ok(())
    }
    
    /// Handle agent update message
    async fn handle_agent_update(
        packet: &Packet,
        addr: &SocketAddr,
        session_manager: &SessionManager,
    ) -> NetworkResult<()> {
        // Parse agent update data (simplified)
        if packet.payload.len() < 64 {
            return Ok(()); // Invalid packet size
        }
        
        session_manager.update_session(addr, |session| {
            // Update agent position and state
            // This would parse the actual agent update packet structure
            session.update_last_seen();
        }).await;
        
        debug!("Processed agent update from {}", addr);
        Ok(())
    }
    
    /// Handle logout request
    async fn handle_logout(
        addr: &SocketAddr,
        session_manager: &SessionManager,
    ) -> NetworkResult<()> {
        if let Some(session) = session_manager.remove_session(addr).await {
            info!("User logged out: session_id={}", session.session_id);
        }
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for LLUDPServer {
    async fn start(&self) -> MutseaResult<()> {
        self.start().await.map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))
    }
    
    async fn stop(&self) -> MutseaResult<()> {
        self.stop().await.map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))
    }
    
    fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    async fn health_check(&self) -> ServiceHealth {
        let stats = self.stats.read().await;
        let mut metrics = std::collections::HashMap::new();
        
        metrics.insert("packets_received".to_string(), stats.packets_received as f64);
        metrics.insert("packets_sent".to_string(), stats.packets_sent as f64);
        metrics.insert("connections".to_string(), stats.connections as f64);
        metrics.insert("active_sessions".to_string(), stats.active_sessions as f64);
        metrics.insert("errors".to_string(), stats.errors as f64);
        
        let status = if self.is_running() {
            ServiceStatus::Healthy
        } else {
            ServiceStatus::Unhealthy
        };
        
        ServiceHealth {
            status,
            message: format!("LLUDP server on port {}", self.config.port),
            metrics,
        }
    }
}