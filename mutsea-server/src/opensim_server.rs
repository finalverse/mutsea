//! OpenSim-compatible server implementation

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, Response},
    routing::{get, post},
    Router,
    body::Body,
};
use mutsea_core::{Service, ServiceHealth, ServiceStatus, MutseaResult, config::MutseaConfig};
use mutsea_protocol::opensim::login::{ParsedLoginRequest, OpenSimLoginService};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, error, debug};

/// OpenSim-compatible server
pub struct OpenSimServer {
    config: MutseaConfig,
    login_service: Arc<OpenSimLoginService>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

/// Server state
#[derive(Clone)]
pub struct OpenSimServerState {
    pub config: MutseaConfig,
    pub login_service: Arc<OpenSimLoginService>,
}

impl OpenSimServer {
    /// Create new OpenSim server
    pub fn new(config: MutseaConfig) -> Self {
        Self {
            config: config.clone(),
            login_service: Arc::new(OpenSimLoginService::new()),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Start the server
    pub async fn start(&self) -> MutseaResult<()> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);

        let bind_addr = format!("{}:{}", self.config.network.http.bind_address, http_port);
        let listener = TcpListener::bind(&bind_addr).await
            .map_err(|e| mutsea_core::MutseaError::Network(e.to_string()))?;

        info!("OpenSim-compatible server listening on {}", bind_addr);
        if standalone_mode {
            info!("Running in STANDALONE mode - connect with viewer to: http://{}:{}/", 
                  self.config.network.http.bind_address, http_port);
        } else {
            info!("Running in GRID mode - login URI: http://{}:{}/", 
                  self.config.network.http.bind_address, http_port);
        }

        let running = Arc::clone(&self.running);
        tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app).await {
                error!("OpenSim server error: {}", e);
            }
        });

        info!("OpenSim server started successfully");
        Ok(())
    }

    /// Stop the server
    pub async fn stop(&self) -> MutseaResult<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        info!("OpenSim server stopped");
        Ok(())
    }

    /// Add test user
    pub fn add_test_user(&self, first_name: String, last_name: String, password: String) {
        self.login_service.add_test_user(first_name, last_name, password);
    }
}

#[async_trait::async_trait]
impl Service for OpenSimServer {
    async fn start(&self) -> MutseaResult<()> {
        self.start().await
    }

    async fn stop(&self) -> MutseaResult<()> {
        self.stop().await
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
        metrics.insert("is_opensim_compatible".to_string(), 1.0);

        ServiceHealth {
            status,
            message: "OpenSim-compatible server".to_string(),
            metrics,
        }
    }
}

/// Home page handler
async fn home_handler(State(state): State<OpenSimServerState>) -> Html<String> {
    let grid_name = &state.config.opensim.grid_name;
    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>{}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background-color: #f5f5f5; }}
        .container {{ max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        h1 {{ color: #333; border-bottom: 2px solid #007acc; padding-bottom: 10px; }}
        .info {{ background: #e7f3ff; padding: 15px; border-radius: 5px; margin: 20px 0; }}
        .users {{ background: #f0f8f0; padding: 15px; border-radius: 5px; margin: 20px 0; }}
        .status {{ background: #fff3cd; padding: 15px; border-radius: 5px; margin: 20px 0; }}
        code {{ background: #f4f4f4; padding: 2px 5px; border-radius: 3px; font-family: monospace; }}
        ul {{ list-style-type: none; padding: 0; }}
        li {{ background: #f8f9fa; margin: 5px 0; padding: 8px; border-radius: 3px; border-left: 3px solid #007acc; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Welcome to {}</h1>
        
        <div class="info">
            <h3>🌍 Grid Information</h3>
            <p><strong>Grid Name:</strong> {}</p>
            <p><strong>Login URI:</strong> <code>{}</code></p>
            <p><strong>Server Status:</strong> ✅ Online and Ready</p>
            <p><strong>Protocol:</strong> OpenSimulator Compatible</p>
        </div>

        <div class="status">
            <h3>🔗 How to Connect with Firestorm</h3>
            <ol>
                <li>Open Firestorm Viewer</li>
                <li>Click on "Grid Manager"</li>
                <li>Click "Add Grid"</li>
                <li>Set Login URI to: <code>{}</code></li>
                <li>Set Grid Name to: <code>{}</code></li>
                <li>Save and select the grid</li>
                <li>Login with one of the test accounts below</li>
            </ol>
        </div>

        <div class="users">
            <h3>👥 Available Test Users</h3>
            <p>You can login with any of these test accounts:</p>
            <ul>
                <li><strong>Test User</strong> - Password: <code>password</code></li>
                <li><strong>Admin User</strong> - Password: <code>admin</code></li>
                <li><strong>Guest User</strong> - Password: <code>guest</code></li>
            </ul>
            <p><em>To create more users, use: <code>cargo run --bin mutsea-cli -- user create "First" "Last" --password "yourpassword"</code></em></p>
        </div>

        <div class="info">
            <h3>📋 Quick Links</h3>
            <p>
                <a href="/get_grid_info">Grid Info</a> | 
                <a href="/health">Health Check</a> | 
                <a href="https://github.com/finalverse/mutsea">GitHub Repository</a>
            </p>
        </div>

        <footer style="text-align: center; margin-top: 30px; color: #666; border-top: 1px solid #eee; padding-top: 20px;">
            <p>Powered by <strong>Mutsea</strong> - Next Generation Virtual World Platform</p>
            <p><em>Built with ❤️ in Rust | OpenSimulator Compatible</em></p>
        </footer>
    </div>
</body>
</html>"#, 
        grid_name, grid_name, grid_name, 
        state.config.opensim.login_uri,
        state.config.opensim.login_uri,
        grid_name
    );
    Html(html)
}

/// Grid info handler for OpenSim compatibility
async fn grid_info_handler(State(state): State<OpenSimServerState>) -> Result<Response<Body>, StatusCode> {
    let grid_info = serde_json::json!({
        "gridname": state.config.opensim.grid_name,
        "gridnick": state.config.opensim.grid_nick,
        "login": state.config.opensim.login_uri,
        "welcome": format!("http://{}:{}/", state.config.network.http.bind_address, state.config.network.http.port),
        "economy": format!("http://{}:{}/", state.config.network.http.bind_address, state.config.network.http.port),
        "about": format!("http://{}:{}/", state.config.network.http.bind_address, state.config.network.http.port),
        "register": format!("http://{}:{}/", state.config.network.http.bind_address, state.config.network.http.port),
        "help": format!("http://{}:{}/", state.config.network.http.bind_address, state.config.network.http.port),
        "password": format!("http://{}:{}/", state.config.network.http.bind_address, state.config.network.http.port),
        "gatekeeper": format!("http://{}:{}/", state.config.network.http.bind_address, state.config.network.http.port),
        "uas": format!("http://{}:{}/", state.config.network.http.bind_address, state.config.network.http.port)
    });

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&grid_info).unwrap()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}

/// Login handler for XMLRPC compatibility
async fn login_handler(
    State(state): State<OpenSimServerState>,
    headers: HeaderMap,
    body: String,
) -> Result<Response<Body>, StatusCode> {
    debug!("Login request received: {} bytes", body.len());
    debug!("Headers: {:?}", headers);
    debug!("Body preview: {}", &body[..std::cmp::min(200, body.len())]);

    // Parse XMLRPC login request
    let login_request = match ParsedLoginRequest::from_xmlrpc(&body) {
        Ok(req) => req,
        Err(e) => {
            error!("Failed to parse login request: {}", e);
            let error_response = mutsea_protocol::opensim::login::OpenSimLoginResponse::failure(
                "Invalid login request format".to_string()
            );
            let response = Response::builder()
                .status(200)
                .header("Content-Type", "text/xml")
                .body(Body::from(error_response.to_xmlrpc()))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            return Ok(response);
        }
    };

    info!("Login attempt for user: {} {}", login_request.first, login_request.last);

    // Authenticate user
    let login_response = match state.login_service.authenticate(&login_request) {
        Ok(response) => response,
        Err(e) => {
            error!("Authentication error: {}", e);
            mutsea_protocol::opensim::login::OpenSimLoginResponse::failure(
                "Authentication service error".to_string()
            )
        }
    };

    if login_response.login == "true" {
        info!("User {} {} logged in successfully", login_request.first, login_request.last);
    } else {
        info!("Login failed for {} {}: {}", login_request.first, login_request.last, login_response.reason);
    }

    // Return XMLRPC response
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/xml")
        .header("Cache-Control", "no-cache")
        .body(Body::from(login_response.to_xmlrpc()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}

/// Capabilities handler
async fn caps_handler(
    Path((cap_id, path)): Path<(String, String)>,
    State(_state): State<OpenSimServerState>,
    body: String,
) -> Result<Response<Body>, StatusCode> {
    debug!("Capability request: cap_id={}, path={}", cap_id, path);

    // Handle different capability requests
    let response_data = match path.as_str() {
        "EventQueueGet" => {
            // Return empty event queue
            serde_json::json!({
                "events": [],
                "id": 1
            })
        }
        "GetTexture" => {
            // Return texture not found
            serde_json::json!({
                "error": "Texture not found"
            })
        }
        "GetMesh" => {
            // Return mesh not found
            serde_json::json!({
                "error": "Mesh not found"
            })
        }
        "FetchInventoryDescendents2" | "WebFetchInventoryDescendents" => {
            // Return empty inventory
            serde_json::json!({
                "folders": []
            })
        }
        _ => {
            // Generic capability response
            serde_json::json!({
                "error": format!("Capability '{}' not implemented", path)
            })
        }
    };

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&response_data).unwrap()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}

/// Health check handler
async fn health_handler(State(state): State<OpenSimServerState>) -> Result<Response<Body>, StatusCode> {
    let health_info = serde_json::json!({
        "status": "healthy",
        "service": "mutsea-opensim-server",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "grid_name": state.config.opensim.grid_name,
        "login_uri": state.config.opensim.login_uri,
        "users_count": state.login_service.list_users().len()
    });

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&health_info).unwrap()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}