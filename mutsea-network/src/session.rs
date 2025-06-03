//! Client session management

use mutsea_core::{UserId, RegionId, Vector3, Quaternion};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Client session information
#[derive(Debug, Clone)]
pub struct ClientSession {
    pub session_id: Uuid,
    pub user_id: Option<UserId>,
    pub agent_id: Option<UserId>,
    pub circuit_code: u32,
    pub region_id: Option<RegionId>,
    pub position: Vector3,
    pub look_at: Vector3,
    pub velocity: Vector3,
    pub rotation: Quaternion,
    pub connected_at: Instant,
    pub last_seen: Instant,
    pub client_info: ClientInfo,
    pub capabilities: HashMap<String, String>,
    pub sequence_in: u32,
    pub sequence_out: u32,
    pub pending_acks: Vec<u32>,
    pub reliable_packets: HashMap<u32, ReliablePacket>,
}

/// Client information
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub viewer_name: String,
    pub viewer_version: String,
    pub platform: String,
    pub mac_address: String,
    pub id0: String,
    pub channel: String,
}

/// Pending acknowledgment for reliable packets
#[derive(Debug, Clone)]
pub struct PendingAck {
    pub sequence: u32,
    pub timestamp: Instant,
    pub resend_count: u8,
}

/// Reliable packet waiting for acknowledgment
#[derive(Debug, Clone)]
pub struct ReliablePacket {
    pub sequence: u32,
    pub data: Vec<u8>,
    pub timestamp: Instant,
    pub resend_count: u8,
}

impl ClientSession {
    /// Create a new client session
    pub fn new(circuit_code: u32, client_info: ClientInfo) -> Self {
        let now = Instant::now();
        Self {
            session_id: Uuid::new_v4(),
            user_id: None,
            agent_id: None,
            circuit_code,
            region_id: None,
            position: Vector3::ZERO,
            look_at: Vector3::new(1.0, 0.0, 0.0),
            velocity: Vector3::ZERO,
            rotation: Quaternion::IDENTITY,
            connected_at: now,
            last_seen: now,
            client_info,
            capabilities: HashMap::new(),
            sequence_in: 0,
            sequence_out: 0,
            pending_acks: Vec::new(),
            reliable_packets: HashMap::new(),
        }
    }
    
    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = Instant::now();
    }
    
    /// Get session duration
    pub fn duration(&self) -> Duration {
        self.last_seen.duration_since(self.connected_at)
    }
    
    /// Check if session has timed out
    pub fn is_timed_out(&self, timeout: Duration) -> bool {
        self.last_seen.elapsed() > timeout
    }
    
    /// Authenticate the session with user information
    pub fn authenticate(&mut self, user_id: UserId, agent_id: UserId) {
        self.user_id = Some(user_id);
        self.agent_id = Some(agent_id);
    }
    
    /// Check if session is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.user_id.is_some() && self.agent_id.is_some()
    }
    
    /// Get next outbound sequence number
    pub fn next_sequence_out(&mut self) -> u32 {
        self.sequence_out += 1;
        self.sequence_out
    }
    
    /// Process inbound sequence number
    pub fn process_sequence_in(&mut self, sequence: u32) -> bool {
        if sequence > self.sequence_in {
            self.sequence_in = sequence;
            true
        } else {
            false // Duplicate or out-of-order packet
        }
    }
    
    /// Add acknowledgment to pending list
    pub fn add_ack(&mut self, sequence: u32) {
        if !self.pending_acks.contains(&sequence) {
            self.pending_acks.push(sequence);
        }
    }
    
    /// Add reliable packet waiting for acknowledgment
    pub fn add_reliable_packet(&mut self, sequence: u32, data: Vec<u8>) {
        let packet = ReliablePacket {
            sequence,
            data,
            timestamp: Instant::now(),
            resend_count: 0,
        };
        self.reliable_packets.insert(sequence, packet);
    }
    
    /// Process acknowledgment for reliable packet
    pub fn process_ack(&mut self, sequence: u32) -> bool {
        self.reliable_packets.remove(&sequence).is_some()
    }
    
    /// Get packets that need resending
    pub fn get_packets_for_resend(&mut self, timeout: Duration, max_resends: u8) -> Vec<ReliablePacket> {
        let now = Instant::now();
        let mut packets_to_resend = Vec::new();
        let mut packets_to_remove = Vec::new();
        
        for (sequence, packet) in &mut self.reliable_packets {
            if packet.timestamp.elapsed() > timeout {
                if packet.resend_count < max_resends {
                    packet.resend_count += 1;
                    packet.timestamp = now;
                    packets_to_resend.push(packet.clone());
                } else {
                    // Max resends reached, remove packet
                    packets_to_remove.push(*sequence);
                }
            }
        }
        
        // Remove packets that exceeded max resends
        for sequence in packets_to_remove {
            self.reliable_packets.remove(&sequence);
        }
        
        packets_to_resend
    }
    
    /// Update agent position and movement
    pub fn update_movement(&mut self, position: Vector3, look_at: Vector3, velocity: Vector3, rotation: Quaternion) {
        self.position = position;
        self.look_at = look_at;
        self.velocity = velocity;
        self.rotation = rotation;
        self.update_last_seen();
    }
    
    /// Set current region
    pub fn set_region(&mut self, region_id: RegionId) {
        self.region_id = Some(region_id);
    }
    
    /// Add capability
    pub fn add_capability(&mut self, name: String, url: String) {
        self.capabilities.insert(name, url);
    }
    
    /// Get capability URL
    pub fn get_capability(&self, name: &str) -> Option<&String> {
        self.capabilities.get(name)
    }
    
    /// Get pending acknowledgments and clear the list
    pub fn take_pending_acks(&mut self) -> Vec<u32> {
        std::mem::take(&mut self.pending_acks)
    }
}

/// Session manager for handling multiple client sessions
pub struct SessionManager {
    sessions_by_address: Arc<RwLock<HashMap<SocketAddr, ClientSession>>>,
    sessions_by_circuit: Arc<RwLock<HashMap<u32, SocketAddr>>>,
    sessions_by_user: Arc<RwLock<HashMap<UserId, SocketAddr>>>,
    cleanup_interval: Duration,
    session_timeout: Duration,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(cleanup_interval: Duration, session_timeout: Duration) -> Self {
        Self {
            sessions_by_address: Arc::new(RwLock::new(HashMap::new())),
            sessions_by_circuit: Arc::new(RwLock::new(HashMap::new())),
            sessions_by_user: Arc::new(RwLock::new(HashMap::new())),
            cleanup_interval,
            session_timeout,
        }
    }
    
    /// Add a new session
    pub async fn add_session(&self, address: SocketAddr, session: ClientSession) {
        let circuit_code = session.circuit_code;
        
        // Store session by address
        self.sessions_by_address.write().await.insert(address, session);
        
        // Store mapping by circuit code
        self.sessions_by_circuit.write().await.insert(circuit_code, address);
    }
    
    /// Get session by address
    pub async fn get_session(&self, address: &SocketAddr) -> Option<ClientSession> {
        self.sessions_by_address.read().await.get(address).cloned()
    }
    
    /// Get session by circuit code
    pub async fn get_session_by_circuit(&self, circuit_code: u32) -> Option<ClientSession> {
        if let Some(address) = self.sessions_by_circuit.read().await.get(&circuit_code) {
            self.sessions_by_address.read().await.get(address).cloned()
        } else {
            None
        }
    }
    
    /// Get session by user ID
    pub async fn get_session_by_user(&self, user_id: &UserId) -> Option<ClientSession> {
        if let Some(address) = self.sessions_by_user.read().await.get(user_id) {
            self.sessions_by_address.read().await.get(address).cloned()
        } else {
            None
        }
    }
    
    /// Update session
    pub async fn update_session<F>(&self, address: &SocketAddr, updater: F) -> bool
    where
        F: FnOnce(&mut ClientSession),
    {
        if let Some(session) = self.sessions_by_address.write().await.get_mut(address) {
            updater(session);
            
            // Update user mapping if session is authenticated
            if let Some(user_id) = session.user_id {
                self.sessions_by_user.write().await.insert(user_id, *address);
            }
            
            true
        } else {
            false
        }
    }
    
    /// Remove session
    pub async fn remove_session(&self, address: &SocketAddr) -> Option<ClientSession> {
        if let Some(session) = self.sessions_by_address.write().await.remove(address) {
            // Remove circuit mapping
            self.sessions_by_circuit.write().await.remove(&session.circuit_code);
            
            // Remove user mapping
            if let Some(user_id) = session.user_id {
                self.sessions_by_user.write().await.remove(&user_id);
            }
            
            Some(session)
        } else {
            None
        }
    }
    
    /// Get all active sessions
    pub async fn get_all_sessions(&self) -> Vec<(SocketAddr, ClientSession)> {
        self.sessions_by_address
            .read()
            .await
            .iter()
            .map(|(addr, session)| (*addr, session.clone()))
            .collect()
    }
    
    /// Get session count
    pub async fn session_count(&self) -> usize {
        self.sessions_by_address.read().await.len()
    }
    
    /// Cleanup timed out sessions
    pub async fn cleanup_sessions(&self) -> usize {
        let mut addresses_to_remove = Vec::new();
        
        // Find timed out sessions
        {
            let sessions = self.sessions_by_address.read().await;
            for (address, session) in sessions.iter() {
                if session.is_timed_out(self.session_timeout) {
                    addresses_to_remove.push(*address);
                }
            }
        }
        
        // Remove timed out sessions
        let removed_count = addresses_to_remove.len();
        for address in addresses_to_remove {
            self.remove_session(&address).await;
        }
        
        removed_count
    }
    
    /// Start automatic session cleanup
    pub async fn start_cleanup_task(&self) {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(manager.cleanup_interval);
            loop {
                interval.tick().await;
                let removed = manager.cleanup_sessions().await;
                if removed > 0 {
                    tracing::info!("Cleaned up {} timed out sessions", removed);
                }
            }
        });
    }
}

impl Clone for SessionManager {
    fn clone(&self) -> Self {
        Self {
            sessions_by_address: Arc::clone(&self.sessions_by_address),
            sessions_by_circuit: Arc::clone(&self.sessions_by_circuit),
            sessions_by_user: Arc::clone(&self.sessions_by_user),
            cleanup_interval: self.cleanup_interval,
            session_timeout: self.session_timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_client_session_creation() {
        let client_info = ClientInfo {
            viewer_name: "Test Viewer".to_string(),
            viewer_version: "1.0.0".to_string(),
            platform: "Test".to_string(),
            mac_address: "00:00:00:00:00:00".to_string(),
            id0: "test".to_string(),
            channel: "Test Channel".to_string(),
        };
        
        let session = ClientSession::new(12345, client_info);
        
        assert_eq!(session.circuit_code, 12345);
        assert!(!session.is_authenticated());
        assert_eq!(session.sequence_in, 0);
        assert_eq!(session.sequence_out, 0);
    }
    
    #[test]
    fn test_session_authentication() {
        let client_info = ClientInfo {
            viewer_name: "Test Viewer".to_string(),
            viewer_version: "1.0.0".to_string(),
            platform: "Test".to_string(),
            mac_address: "00:00:00:00:00:00".to_string(),
            id0: "test".to_string(),
            channel: "Test Channel".to_string(),
        };
        
        let mut session = ClientSession::new(12345, client_info);
        let user_id = UserId::new();
        let agent_id = UserId::new();
        
        session.authenticate(user_id, agent_id);
        
        assert!(session.is_authenticated());
        assert_eq!(session.user_id, Some(user_id));
        assert_eq!(session.agent_id, Some(agent_id));
    }
    
    #[tokio::test]
    async fn test_session_manager() {
        let manager = SessionManager::new(Duration::from_secs(60), Duration::from_secs(300));
        let address = "127.0.0.1:8080".parse().unwrap();
        
        let client_info = ClientInfo {
            viewer_name: "Test Viewer".to_string(),
            viewer_version: "1.0.0".to_string(),
            platform: "Test".to_string(),
            mac_address: "00:00:00:00:00:00".to_string(),
            id0: "test".to_string(),
            channel: "Test Channel".to_string(),
        };
        
        let session = ClientSession::new(12345, client_info);
        
        // Add session
        manager.add_session(address, session.clone()).await;
        
        // Retrieve session
        let retrieved = manager.get_session(&address).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().circuit_code, 12345);
        
        // Retrieve by circuit code
        let by_circuit = manager.get_session_by_circuit(12345).await;
        assert!(by_circuit.is_some());
        
        // Check session count
        assert_eq!(manager.session_count().await, 1);
        
        // Remove session
        let removed = manager.remove_session(&address).await;
        assert!(removed.is_some());
        assert_eq!(manager.session_count().await, 0);
    }
}