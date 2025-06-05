//! mutsea-network/src/lludp_server.rs
//! Main LLUDP server implementation

use crate::{ClientSession, SessionManager, message::*};
use mutsea_core::{
    Service, ServiceHealth, ServiceStatus, MutseaResult, 
    config::LLUDPConfig, NetworkError, NetworkResult,
    Vector3, Quaternion, UserId, RegionId
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

// Import modular components
mod circuit;
mod handlers;
mod stats;

pub use circuit::*;
pub use handlers::*;
pub use stats::*;

/// Enhanced LLUDP server for handling OpenSim viewer connections
pub struct LLUDPServer {
    socket: Arc<UdpSocket>,
    session_manager: SessionManager,
    config: LLUDPConfig,
    running: Arc<std::sync::atomic::AtomicBool>,
    stats: Arc<RwLock<ServerStats>>,
    active_circuits: Arc<RwLock<HashMap<u32, CircuitInfo>>>,
    login_service: Arc<LoginService>,
    handlers: PacketHandlers,
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

        let handlers = PacketHandlers::new();

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
                            &session_manager,
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
                        if let Err(e) = CircuitManager::send_heartbeat(&socket, circuit, &stats).await {
                            error!("Failed to send heartbeat to {}: {}", circuit.address, e);
                        }
                    }
                    
                    // Resend reliable packets
                    CircuitManager::process_reliable_resends(&socket, circuit, &config, &stats).await;
                }
                
                // Remove timed out circuits
                for circuit_code in to_remove {
                    if let Some(circuit) = circuits_guard.remove(&circuit_code) {
                        info!("Removed timed out circuit: {} from {}", circuit_code, circuit.address);
                    }
                }
            }
        });
    }

    /// Get server statistics
    pub async fn get_stats(&self) -> ServerStats {
        self.stats.read().await.clone()
    }

    /// Get active circuits count
    pub async fn get_active_circuits_count(&self) -> usize {
        self.active_circuits.read().await.len()
    }

    /// Get circuit info
    pub async fn get_circuit_info(&self, circuit_code: u32) -> Option<CircuitInfo> {
        self.active_circuits.read().await.get(&circuit_code).cloned()
    }

    /// Get all active circuits
    pub async fn get_all_circuits(&self) -> Vec<CircuitInfo> {
        self.active_circuits.read().await.values().cloned().collect()
    }

    /// Send packet to specific circuit
    pub async fn send_to_circuit(&self, circuit_code: u32, packet: Packet) -> NetworkResult<()> {
        let circuits = self.active_circuits.read().await;
        if let Some(circuit) = circuits.get(&circuit_code) {
            let packet_data = packet.serialize()
                .map_err(|e| NetworkError::Protocol(format!("Failed to serialize packet: {}", e)))?;
            
            self.socket.send_to(&packet_data, circuit.address).await?;
            
            // Update stats
            let mut stats = self.stats.write().await;
            stats.packets_sent += 1;
            stats.bytes_sent += packet_data.len() as u64;
        }
        
        Ok(())
    }

    /// Broadcast packet to all authenticated circuits
    pub async fn broadcast_to_all(&self, packet: Packet) -> NetworkResult<()> {
        let circuits = self.active_circuits.read().await;
        let packet_data = packet.serialize()
            .map_err(|e| NetworkError::Protocol(format!("Failed to serialize packet: {}", e)))?;
        
        let mut sent_count = 0u64;
        for circuit in circuits.values() {
            if circuit.authenticated {
                if let Err(e) = self.socket.send_to(&packet_data, circuit.address).await {
                    error!("Failed to broadcast to circuit {}: {}", circuit.circuit_code, e);
                } else {
                    sent_count += 1;
                }
            }
        }
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.packets_sent += sent_count;
        stats.bytes_sent += (packet_data.len() as u64) * sent_count;
        
        info!("Broadcasted packet to {} circuits", sent_count);
        Ok(())
    }

    /// Emergency shutdown
    pub async fn emergency_shutdown(&self, reason: &str) -> NetworkResult<()> {
        warn!("Emergency shutdown initiated: {}", reason);
        
        // Send logout messages to all connected circuits
        let circuits = self.active_circuits.read().await;
        for circuit in circuits.values() {
            if circuit.authenticated {
                // Send KickUser message
                let mut payload = Vec::new();
                payload.push(packet_types::KICK_USER);
                
                // Reason (variable string)
                let reason_bytes = reason.as_bytes();
                payload.extend_from_slice(&(reason_bytes.len() as u16).to_le_bytes());
                payload.extend_from_slice(reason_bytes);
                
                let packet = Packet::reliable(0, payload);
                if let Err(e) = self.send_to_circuit(circuit.circuit_code, packet).await {
                    error!("Failed to send logout to circuit {}: {}", circuit.circuit_code, e);
                }
            }
        }
        
        // Wait a moment for packets to be sent
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // Stop the server
        self.stop().await?;
        
        Ok(())
    }

    /// Get region statistics
    pub async fn get_region_stats(&self) -> RegionStats {
        let circuits = self.active_circuits.read().await;
        let authenticated_count = circuits.values().filter(|c| c.authenticated).count();
        let total_connections = circuits.len();
        
        // Calculate average position
        let positions: Vec<Vector3> = circuits.values()
            .filter(|c| c.authenticated)
            .map(|c| c.position)
            .collect();
        
        let avg_position = if positions.is_empty() {
            Vector3::ZERO
        } else {
            let sum = positions.iter().fold(Vector3::ZERO, |acc, pos| acc + *pos);
            sum * (1.0 / positions.len() as f32)
        };
        
        RegionStats {
            total_connections,
            authenticated_users: authenticated_count,
            average_position: avg_position,
            region_uptime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default(),
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
        let stats = self.stats.read().await;
        let circuits_count = self.active_circuits.read().await.len();
        let authenticated_count = self.active_circuits.read().await.values()
            .filter(|c| c.authenticated).count();
        
        let mut metrics = std::collections::HashMap::new();
        metrics.insert("packets_received".to_string(), stats.packets_received as f64);
        metrics.insert("packets_sent".to_string(), stats.packets_sent as f64);
        metrics.insert("total_circuits".to_string(), circuits_count as f64);
        metrics.insert("authenticated_circuits".to_string(), authenticated_count as f64);
        metrics.insert("errors".to_string(), stats.errors as f64);

        let status = if self.is_running() {
            if stats.errors > 0 && stats.errors > stats.packets_received / 10 {
                ServiceStatus::Degraded
            } else {
                ServiceStatus::Healthy
            }
        } else {
            ServiceStatus::Unhealthy
        };

        ServiceHealth {
            status,
            message: format!("LLUDP server on port {} with {}/{} authenticated circuits", 
                           self.config.port, authenticated_count, circuits_count),
            metrics,
        }
    }
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