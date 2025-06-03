//! # Mutsea Network
//! 
//! Network protocols and communication layer for Mutsea.
//! Provides both LLUDP (for Firestorm compatibility) and modern HTTP/WebSocket APIs.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod lludp;
pub mod http;
pub mod websocket;
pub mod client;
pub mod message;
pub mod session;
pub mod error;

// Re-export commonly used types
pub use error::*;
pub use message::*;
pub use session::*;
pub use client::*;

use mutsea_core::{MutseaResult, Service, ServiceHealth, ServiceStatus, NetworkError};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Network service that manages all network protocols
pub struct NetworkService {
    lludp_server: Arc<RwLock<Option<lludp::LLUDPServer>>>,
    http_server: Arc<RwLock<Option<http::HTTPServer>>>,
    websocket_server: Arc<RwLock<Option<websocket::WebSocketServer>>>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl NetworkService {
    /// Create a new network service
    pub fn new() -> Self {
        Self {
            lludp_server: Arc::new(RwLock::new(None)),
            http_server: Arc::new(RwLock::new(None)),
            websocket_server: Arc::new(RwLock::new(None)),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    /// Start LLUDP server for Firestorm compatibility
    pub async fn start_lludp(&self, config: &mutsea_core::config::LLUDPConfig) -> MutseaResult<()> {
        let server = lludp::LLUDPServer::new(config).await
            .map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))?;
        *self.lludp_server.write().await = Some(server);
        Ok(())
    }
    
    /// Start HTTP server for web APIs
    pub async fn start_http(&self, config: &mutsea_core::config::HTTPConfig) -> MutseaResult<()> {
        let server = http::HTTPServer::new(config).await
            .map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))?;
        *self.http_server.write().await = Some(server);
        Ok(())
    }
    
    /// Start WebSocket server for real-time communication
    pub async fn start_websocket(&self, port: u16) -> MutseaResult<()> {
        let server = websocket::WebSocketServer::new(port).await
            .map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))?;
        *self.websocket_server.write().await = Some(server);
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for NetworkService {
    async fn start(&self) -> MutseaResult<()> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Start all servers concurrently
        let lludp_task = async {
            if let Some(server) = self.lludp_server.read().await.as_ref() {
                server.start().await.map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))
            } else {
                Ok(())
            }
        };
        
        let http_task = async {
            if let Some(server) = self.http_server.read().await.as_ref() {
                server.start().await.map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))
            } else {
                Ok(())
            }
        };
        
        let websocket_task = async {
            if let Some(server) = self.websocket_server.read().await.as_ref() {
                server.start().await.map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))
            } else {
                Ok(())
            }
        };
        
        tokio::try_join!(lludp_task, http_task, websocket_task)?;
        
        Ok(())
    }
    
    async fn stop(&self) -> MutseaResult<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        
        // Stop all servers
        if let Some(server) = self.lludp_server.write().await.as_mut() {
            server.stop().await.map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))?;
        }
        
        if let Some(server) = self.http_server.write().await.as_mut() {
            server.stop().await.map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))?;
        }
        
        if let Some(server) = self.websocket_server.write().await.as_mut() {
            server.stop().await.map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))?;
        }
        
        Ok(())
    }
    
    fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    async fn health_check(&self) -> ServiceHealth {
        let mut healthy = true;
        let mut metrics = std::collections::HashMap::new();
        
        // Check LLUDP server health
        if let Some(server) = self.lludp_server.read().await.as_ref() {
            let health = server.health_check().await;
            healthy &= health.status == ServiceStatus::Healthy;
            metrics.insert("lludp_connections".to_string(), health.metrics.get("connections").copied().unwrap_or(0.0));
        }
        
        // Check HTTP server health
        if let Some(server) = self.http_server.read().await.as_ref() {
            let health = server.health_check().await;
            healthy &= health.status == ServiceStatus::Healthy;
            metrics.insert("http_requests_per_sec".to_string(), health.metrics.get("requests_per_sec").copied().unwrap_or(0.0));
        }
        
        // Check WebSocket server health
        if let Some(server) = self.websocket_server.read().await.as_ref() {
            let health = server.health_check().await;
            healthy &= health.status == ServiceStatus::Healthy;
            metrics.insert("websocket_connections".to_string(), health.metrics.get("connections").copied().unwrap_or(0.0));
        }
        
        ServiceHealth {
            status: if healthy { ServiceStatus::Healthy } else { ServiceStatus::Degraded },
            message: if healthy { "All network services healthy".to_string() } else { "Some network services degraded".to_string() },
            metrics,
        }
    }
}