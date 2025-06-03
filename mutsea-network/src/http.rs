//! HTTP server implementation for web APIs and capabilities

use crate::{NetworkError, NetworkResult};
use mutsea_core::{Service, ServiceHealth, ServiceStatus, MutseaResult, config::HTTPConfig};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, error};

/// HTTP server for web APIs and capabilities
pub struct HTTPServer {
    config: HTTPConfig,
    running: Arc<std::sync::atomic::AtomicBool>,
}

/// Server state shared across handlers
#[derive(Clone)]
pub struct ServerState {
    // Add shared state here as needed
}

impl HTTPServer {
    /// Create a new HTTP server
    pub async fn new(config: &HTTPConfig) -> NetworkResult<Self> {
        Ok(Self {
            config: config.clone(),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }
    
    /// Start the HTTP server
    pub async fn start(&self) -> NetworkResult<()> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        let state = ServerState {};
        
        // Build router
        let app = Router::new()
            .route("/", get(root_handler))
            .route("/get_grid_info", get(grid_info_handler))
            .route("/login.cgi", post(login_handler))
            .route("/caps/:cap_id/*path", get(caps_handler))
            .route("/health", get(health_handler))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive())
            )
            .with_state(state);
        
        // Bind and serve
        let bind_addr = format!("{}:{}", self.config.bind_address, self.config.port);
        let listener = TcpListener::bind(&bind_addr).await?;
        info!("HTTP server listening on {}", bind_addr);
        
        let running = Arc::clone(&self.running);
        tokio::spawn(async move {
            if let Err(e) = axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            ).await {
                error!("HTTP server error: {}", e);
            }
        });
        
        info!("HTTP server started successfully");
        Ok(())
    }
    
    /// Stop the HTTP server
    pub async fn stop(&self) -> NetworkResult<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        info!("HTTP server stopped");
        Ok(())
    }
}

#[async_trait::async_trait]
impl Service for HTTPServer {
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
        
        ServiceHealth {
            status,
            message: format!("HTTP server on port {}", self.config.port),
            metrics: std::collections::HashMap::new(),
        }
    }
}

/// Root handler
async fn root_handler() -> &'static str {
    "Mutsea Virtual World Platform"
}

/// Grid info handler for OpenSim compatibility
async fn grid_info_handler() -> Json<HashMap<String, serde_json::Value>> {
    let mut grid_info = HashMap::new();
    
    grid_info.insert("gridname".to_string(), serde_json::Value::String("Mutsea Grid".to_string()));
    grid_info.insert("gridnick".to_string(), serde_json::Value::String("mutsea".to_string()));
    grid_info.insert("login".to_string(), serde_json::Value::String("http://localhost:8080/login.cgi".to_string()));
    grid_info.insert("welcome".to_string(), serde_json::Value::String("http://localhost:8080/".to_string()));
    grid_info.insert("economy".to_string(), serde_json::Value::String("http://localhost:8080/".to_string()));
    grid_info.insert("about".to_string(), serde_json::Value::String("http://localhost:8080/".to_string()));
    grid_info.insert("register".to_string(), serde_json::Value::String("http://localhost:8080/".to_string()));
    grid_info.insert("help".to_string(), serde_json::Value::String("http://localhost:8080/".to_string()));
    grid_info.insert("password".to_string(), serde_json::Value::String("http://localhost:8080/".to_string()));
    
    Json(grid_info)
}

/// Login handler for OpenSim XMLRPC login
async fn login_handler(
    State(_state): State<ServerState>,
    body: String,
) -> Result<String, StatusCode> {
    // This is a simplified login handler
    // In a real implementation, this would parse XMLRPC and authenticate users
    
    info!("Login request received");
    
    // For now, return a basic failure response
    let response = r#"<?xml version="1.0"?>
<methodResponse>
    <params>
        <param>
            <value>
                <struct>
                    <member>
                        <name>login</name>
                        <value><string>false</string></value>
                    </member>
                    <member>
                        <name>reason</name>
                        <value><string>Login system not yet implemented</string></value>
                    </member>
                    <member>
                        <name>message</name>
                        <value><string>Mutsea is still in development</string></value>
                    </member>
                </struct>
            </value>
        </param>
    </params>
</methodResponse>"#;
    
    Ok(response.parse().unwrap())
}

/// Capabilities handler
async fn caps_handler(
    Path((cap_id, path)): Path<(String, String)>,
    State(_state): State<ServerState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Capabilities request: cap_id={}, path={}", cap_id, path);
    
    // Return empty capability response for now
    Ok(Json(serde_json::json!({
        "error": "Capability not implemented"
    })))
}

/// Health check handler
async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "mutsea-http",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}