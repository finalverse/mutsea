//! mutsea-network/src/lludp_server/server.rs
//! Main LLUDP server implementation

use crate::{NetworkResult, SessionManager};
use mutsea_core::{
    Service, ServiceHealth, ServiceStatus, MutseaResult, 
    config::LLUDPConfig, Vector3, UserId, RegionId
};
use mutsea_protocol::{
    Packet, 
    constants::{flags, packet_types, timeouts, limits},
    login::LoginService
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::{
    circuit::{CircuitInfo, ClientInfo, ReliablePacketData},
    stats::ServerStats,
    handler_packet::PacketHandler,
};

/// Enhanced LLUDP server for handling OpenSim viewer connections
pub struct LLUDPServer {
    socket: Arc<UdpSocket>,
    session_manager: SessionManager,
    config: LLUDPConfig,
    running: Arc<std::sync::atomic::AtomicBool>,
    stats: Arc<RwLock<ServerStats>>,
    active_circuits: Arc<RwLock<HashMap<u32, CircuitInfo>>>,
    login_service: Arc<LoginService>,
    handlers: PacketHandler,
}

impl LLUDPServer {
    /// Create a new LLUDP server
    pub async fn new(config: &LLUDPConfig) -> NetworkResult<Self> {
        let bind_addr = format!("{}:{}", config.bind_address, config.port);
        let socket = UdpSocket::bind(&bind_addr).await
            .map_err(|e| crate::NetworkError::Io(e))?;
        info!("LLUDP server bound to {}", bind_addr);

        let session_manager = SessionManager::new(
            Duration::from_secs(60),
            Duration::from_secs(config.client_timeout),
        );

        let handlers = PacketHandler::new();

        Ok(Self {
            socket: Arc::new(socket),
            session_manager,
            config: config.clone(),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            stats: Arc::new(RwLock::new(ServerStats::default())),
            active_circuits: Arc::new(RwLock::new(HashMap::new())),
            login_service: Arc::new(LoginService::new()),
            handlers,
        })
    }

    /// Set login service for authentication
    pub fn set_login_service(&mut self, login_service: Arc<LoginService>) {
        self.login_service = login_service;
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
        let circuits = Arc::clone(&self.active_circuits);
        let running = Arc::clone(&self.running);
        let config = self.config.clone();
        let login_service = Arc::clone(&self.login_service);
        let handlers = self.handlers.clone();

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
                        if let Err(e) = handlers.handle_packet(
                            &circuits,
                            &socket,
                            addr,
                            packet_data,
                            &config,
                            &login_service,
                            &stats,
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

        // Start periodic tasks
        self.start_periodic_tasks().await;

        info!("LLUDP server started successfully on port {}", self.config.port);
        Ok(())
    }

    /// Stop the LLUDP server
    pub async fn stop(&self) -> NetworkResult<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        info!("LLUDP server stopped");
        Ok(())
    }

    /// Start periodic maintenance tasks
    async fn start_periodic_tasks(&self) {
        let circuits = Arc::clone(&self.active_circuits);
        let socket = Arc::clone(&self.socket);
        let config = self.config.clone();
        let running = Arc::clone(&self.running);
        let stats = Arc::clone(&self.stats);

        // Heartbeat and resend task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            
            while running.load(std::sync::atomic::Ordering::SeqCst) {
                interval.tick().await;
                
                let mut circuits_guard = circuits.write().await;
                let mut to_remove = Vec::new();
                
                for (circuit_code, circuit) in circuits_guard.iter_mut() {
                    // Check for timeout
                    if circuit.last_activity.elapsed() > Duration::from_secs(config.client_timeout) {
                        to_remove.push(*circuit_code);
                        continue;
                    }
                    
                    // Send heartbeat
                    if circuit.last_activity.elapsed() > Duration::from_secs(config.ping_interval) {
                        if let Err(e) = Self::send_heartbeat(&socket, circuit, &stats).await {
                            error!("Failed to send heartbeat to {}: {}", circuit.address, e);
                        }
                    }
                    
                    // Resend reliable packets
                    Self::process_reliable_resends(&socket, circuit, &config, &stats).await;
                }
                
                // Remove timed out circuits
                for circuit_code in to_remove {
                    if let Some(circuit) = circuits_guard.remove(&circuit_code) {
                        info!("Removed timed out circuit: {} from {}", circuit_code, circuit.address);
                        
                        // Update stats
                        let mut stats_guard = stats.write().await;
                        stats_guard.active_sessions = stats_guard.active_sessions.saturating_sub(1);
                    }
                }
            }
        });

        // Statistics reporting task
        let stats_clone = Arc::clone(&self.stats);
        let circuits_clone = Arc::clone(&self.active_circuits);
        let running_clone = Arc::clone(&self.running);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            while running_clone.load(std::sync::atomic::Ordering::SeqCst) {
                interval.tick().await;
                
                let circuits_count = circuits_clone.read().await.len();
                let stats_guard = stats_clone.read().await;
                
                debug!("LLUDP Server Stats - Circuits: {}, Packets RX: {}, TX: {}, Errors: {}", 
                       circuits_count, stats_guard.packets_received, stats_guard.packets_sent, stats_guard.errors);
                
                if circuits_count > 0 {
                    debug!("Active circuits with authenticated users: {}", 
                           circuits_clone.read().await.values().filter(|c| c.authenticated).count());
                }
            }
        });
    }

    /// Send heartbeat to circuit
    async fn send_heartbeat(
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

    /// Process reliable packet resends
    async fn process_reliable_resends(
        socket: &UdpSocket,
        circuit: &mut CircuitInfo,
        config: &LLUDPConfig,
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

    /// Get server statistics
    pub async fn get_stats(&self) -> ServerStats {
        self.stats.read().await.clone()
    }

    /// Get active circuits count
    pub async fn get_active_circuits_count(&self) -> usize {
        self.active_circuits.read().await.len()
    }

    /// Get all active circuits
    pub async fn get_all_circuits(&self) -> Vec<CircuitInfo> {
        self.active_circuits.read().await.values().cloned().collect()
    }

    /// Send packet to specific circuit
    pub async fn send_packet_to_circuit(
        &self,
        circuit_code: u32,
        packet: Packet,
    ) -> NetworkResult<()> {
        let circuits_guard = self.active_circuits.read().await;
        if let Some(circuit) = circuits_guard.get(&circuit_code) {
            let packet_data = packet.serialize()
                .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize packet: {}", e)))?;
            
            self.socket.send_to(&packet_data, circuit.address).await?;
            
            // Update stats
            let mut stats_guard = self.stats.write().await;
            stats_guard.packets_sent += 1;
            stats_guard.bytes_sent += packet_data.len() as u64;
            
            Ok(())
        } else {
            Err(crate::NetworkError::CircuitNotFound(circuit_code.to_string()))
        }
    }

    /// Broadcast packet to all authenticated circuits
    pub async fn broadcast_packet_to_authenticated(
        &self,
        packet: Packet,
    ) -> NetworkResult<usize> {
        let circuits_guard = self.active_circuits.read().await;
        let mut broadcast_count = 0;
        
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize packet: {}", e)))?;
        
        for circuit in circuits_guard.values() {
            if circuit.authenticated {
                if let Err(e) = self.socket.send_to(&packet_data, circuit.address).await {
                    warn!("Failed to broadcast to circuit {}: {}", circuit.circuit_code, e);
                } else {
                    broadcast_count += 1;
                }
            }
        }
        
        // Update stats
        if broadcast_count > 0 {
            let mut stats_guard = self.stats.write().await;
            stats_guard.packets_sent += broadcast_count as u64;
            stats_guard.bytes_sent += (packet_data.len() * broadcast_count) as u64;
        }
        
        Ok(broadcast_count)
    }

    /// Send emergency shutdown notification to all clients
    pub async fn emergency_shutdown(&self, reason: &str) -> NetworkResult<()> {
        info!("Sending emergency shutdown notification: {}", reason);
        
        let circuits_guard = self.active_circuits.read().await;
        let mut notifications_sent = 0;
        
        for circuit in circuits_guard.values() {
            if circuit.authenticated {
                if let Err(e) = self.send_shutdown_notification(circuit.address, reason).await {
                    warn!("Failed to send shutdown notification to circuit {}: {}", circuit.circuit_code, e);
                } else {
                    notifications_sent += 1;
                }
            }
        }
        
        info!("Sent shutdown notifications to {} circuits", notifications_sent);
        
        // Give clients time to process the notification
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        Ok(())
    }

    /// Send shutdown notification to specific client
    async fn send_shutdown_notification(
        &self,
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
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize shutdown notification: {}", e)))?;

        self.socket.send_to(&packet_data, addr).await?;
        Ok(())
    }

    /// Add a new circuit
    pub async fn add_circuit(&self, circuit: CircuitInfo) {
        let circuit_code = circuit.circuit_code;
        self.active_circuits.write().await.insert(circuit_code, circuit);
        
        // Update stats
        let mut stats_guard = self.stats.write().await;
        stats_guard.connections += 1;
        stats_guard.active_sessions += 1;
        
        info!("Added new circuit: {}", circuit_code);
    }

    /// Remove a circuit
    pub async fn remove_circuit(&self, circuit_code: u32) -> Option<CircuitInfo> {
        let removed = self.active_circuits.write().await.remove(&circuit_code);
        
        if removed.is_some() {
            // Update stats
            let mut stats_guard = self.stats.write().await;
            stats_guard.active_sessions = stats_guard.active_sessions.saturating_sub(1);
            
            info!("Removed circuit: {}", circuit_code);
        }
        
        removed
    }

    /// Update circuit information
    pub async fn update_circuit<F>(&self, circuit_code: u32, updater: F) -> bool
    where
        F: FnOnce(&mut CircuitInfo),
    {
        let mut circuits_guard = self.active_circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            updater(circuit);
            true
        } else {
            false
        }
    }

    /// Get circuit by address
    pub async fn get_circuit_by_address(&self, addr: SocketAddr) -> Option<CircuitInfo> {
        let circuits_guard = self.active_circuits.read().await;
        circuits_guard.values().find(|c| c.address == addr).cloned()
    }

    /// Get authenticated circuits count
    pub async fn get_authenticated_circuits_count(&self) -> usize {
        self.active_circuits.read().await.values().filter(|c| c.authenticated).count()
    }

    /// Send region handshake to all authenticated circuits
    pub async fn broadcast_region_handshake(&self) -> NetworkResult<usize> {
        let circuits_guard = self.active_circuits.read().await;
        let mut handshakes_sent = 0;
        
        for circuit in circuits_guard.values() {
            if circuit.authenticated {
                if let Err(e) = self.send_region_handshake_to_circuit(circuit).await {
                    warn!("Failed to send region handshake to circuit {}: {}", circuit.circuit_code, e);
                } else {
                    handshakes_sent += 1;
                }
            }
        }
        
        info!("Sent region handshakes to {} circuits", handshakes_sent);
        Ok(handshakes_sent)
    }

    /// Send region handshake to specific circuit
    async fn send_region_handshake_to_circuit(
        &self,
        circuit: &CircuitInfo,
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

        self.socket.send_to(&packet_data, circuit.address).await?;
        info!("Sent region handshake to circuit {} at {}", circuit.circuit_code, circuit.address);
        Ok(())
    }

    /// Get server performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        let stats = self.stats.read().await;
        let circuits = self.active_circuits.read().await;
        
        PerformanceMetrics {
            total_packets_received: stats.packets_received,
            total_packets_sent: stats.packets_sent,
            total_bytes_received: stats.bytes_received,
            total_bytes_sent: stats.bytes_sent,
            active_circuits: circuits.len(),
            authenticated_circuits: circuits.values().filter(|c| c.authenticated).count(),
            total_connections: stats.connections,
            total_errors: stats.errors,
            uptime: stats.uptime(),
            packets_per_second: stats.packets_per_second(),
            error_rate: stats.error_rate(),
        }
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
        let status = if self.is_running() {
            ServiceStatus::Healthy
        } else {
            ServiceStatus::Unhealthy
        };

        let mut metrics = std::collections::HashMap::new();
        let stats = self.stats.read().await;
        let circuits = self.active_circuits.read().await;

        metrics.insert("connections".to_string(), circuits.len() as f64);
        metrics.insert("packets_received".to_string(), stats.packets_received as f64);
        metrics.insert("packets_sent".to_string(), stats.packets_sent as f64);
        metrics.insert("errors".to_string(), stats.errors as f64);
        metrics.insert("packets_per_second".to_string(), stats.packets_per_second());
        metrics.insert("error_rate".to_string(), stats.error_rate());

        ServiceHealth {
            status,
            message: format!("LLUDP server on port {} with {} circuits", 
                           self.config.port, circuits.len()),
            metrics,
        }
    }
}

/// Performance metrics for the LLUDP server
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_packets_received: u64,
    pub total_packets_sent: u64,
    pub total_bytes_received: u64,
    pub total_bytes_sent: u64,
    pub active_circuits: usize,
    pub authenticated_circuits: usize,
    pub total_connections: u64,
    pub total_errors: u64,
    pub uptime: std::time::Duration,
    pub packets_per_second: f64,
    pub error_rate: f64,
}

impl Clone for LLUDPServer {
    fn clone(&self) -> Self {
        Self {
            socket: Arc::clone(&self.socket),
            session_manager: self.session_manager.clone(),
            config: self.config.clone(),
            running: Arc::clone(&self.running),
            stats: Arc::clone(&self.stats),
            active_circuits: Arc::clone(&self.active_circuits),
            login_service: Arc::clone(&self.login_service),
            handlers: self.handlers.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mutsea_core::config::LLUDPConfig;

    #[tokio::test]
    async fn test_lludp_server_creation() {
        let config = LLUDPConfig {
            bind_address: "127.0.0.1".to_string(),
            port: 0, // Let OS choose port
            max_packet_size: 1200,
            resend_timeout: 100,
            max_resends: 3,
            ack_timeout: 1000,
            ping_interval: 5,
            client_timeout: 60,
        };

        let server = LLUDPServer::new(&config).await;
        assert!(server.is_ok());

        let server = server.unwrap();
        assert!(!server.is_running());
        assert_eq!(server.get_active_circuits_count().await, 0);
    }

    #[tokio::test]
    async fn test_lludp_server_stats() {
        let config = LLUDPConfig::default();
        let server = LLUDPServer::new(&config).await.unwrap();

        let stats = server.get_stats().await;
        assert_eq!(stats.packets_received, 0);
        assert_eq!(stats.packets_sent, 0);
        assert_eq!(stats.connections, 0);
    }

    #[tokio::test]
    async fn test_circuit_management() {
        let config = LLUDPConfig::default();
        let server = LLUDPServer::new(&config).await.unwrap();

        let circuit = CircuitInfo {
            circuit_code: 12345,
            address: "127.0.0.1:8080".parse().unwrap(),
            user_id: Some(UserId::new()),
            agent_id: Some(UserId::new()),
            session_id: Some(uuid::Uuid::new_v4()),
            secure_session_id: None,
            created_at: Instant::now(),
            last_activity: Instant::now(),
            sequence_in: 0,
            sequence_out: 0,
            pending_acks: Vec::new(),
            reliable_packets: HashMap::new(),
            authenticated: true,
            region_id: Some(RegionId::new()),
            position: Vector3::ZERO,
            look_at: Vector3::new(1.0, 0.0, 0.0),
            client_info: None,
            last_ping_id: 0,
            last_ping_time: Instant::now(),
        };

        server.add_circuit(circuit).await;
        assert_eq!(server.get_active_circuits_count().await, 1);
        assert_eq!(server.get_authenticated_circuits_count().await, 1);

        let removed = server.remove_circuit(12345).await;
        assert!(removed.is_some());
        assert_eq!(server.get_active_circuits_count().await, 0);
    }
}