//! Client connection and management

use crate::{NetworkError, NetworkResult};
use mutsea_core::{UserId, Vector3, Quaternion};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

/// Client connection information
#[derive(Debug, Clone)]
pub struct ClientConnection {
    pub id: Uuid,
    pub address: SocketAddr,
    pub user_id: Option<UserId>,
    pub connected_at: Instant,
    pub last_activity: Instant,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

impl ClientConnection {
    /// Create a new client connection
    pub fn new(address: SocketAddr) -> Self {
        let now = Instant::now();
        Self {
            id: Uuid::new_v4(),
            address,
            user_id: None,
            connected_at: now,
            last_activity: now,
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
        }
    }
    
    /// Update activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
    }
    
    /// Record sent data
    pub fn record_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
        self.packets_sent += 1;
        self.update_activity();
    }
    
    /// Record received data
    pub fn record_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
        self.packets_received += 1;
        self.update_activity();
    }
    
    /// Get connection duration
    pub fn duration(&self) -> std::time::Duration {
        self.last_activity.duration_since(self.connected_at)
    }
    
    /// Check if connection is idle
    pub fn is_idle(&self, timeout: std::time::Duration) -> bool {
        self.last_activity.elapsed() > timeout
    }
}

/// Client manager for tracking active connections
pub struct ClientManager {
    clients: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<SocketAddr, ClientConnection>>>,
}

impl ClientManager {
    /// Create a new client manager
    pub fn new() -> Self {
        Self {
            clients: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Add a new client
    pub async fn add_client(&self, address: SocketAddr) -> ClientConnection {
        let client = ClientConnection::new(address);
        self.clients.write().await.insert(address, client.clone());
        client
    }
    
    /// Get client by address
    pub async fn get_client(&self, address: &SocketAddr) -> Option<ClientConnection> {
        self.clients.read().await.get(address).cloned()
    }
    
    /// Update client
    pub async fn update_client<F>(&self, address: &SocketAddr, updater: F) -> bool
    where
        F: FnOnce(&mut ClientConnection),
    {
        if let Some(client) = self.clients.write().await.get_mut(address) {
            updater(client);
            true
        } else {
            false
        }
    }
    
    /// Remove client
    pub async fn remove_client(&self, address: &SocketAddr) -> Option<ClientConnection> {
        self.clients.write().await.remove(address)
    }
    
    /// Get all clients
    pub async fn get_all_clients(&self) -> Vec<ClientConnection> {
        self.clients.read().await.values().cloned().collect()
    }
    
    /// Get client count
    pub async fn client_count(&self) -> usize {
        self.clients.read().await.len()
    }
    
    /// Clean up idle clients
    pub async fn cleanup_idle_clients(&self, timeout: std::time::Duration) -> usize {
        let mut clients = self.clients.write().await;
        let initial_count = clients.len();
        
        clients.retain(|_, client| !client.is_idle(timeout));
        
        initial_count - clients.len()
    }
}

impl Default for ClientManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ClientManager {
    fn clone(&self) -> Self {
        Self {
            clients: Arc::clone(&self.clients),
        }
    }
}