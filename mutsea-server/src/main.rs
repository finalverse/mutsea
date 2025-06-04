//! mutsea-server/src/main.rs
//! Updated Mutsea server with full OpenSim compatibility

use mutsea_core::{Service, config::MutseaConfig};
use mutsea_network::LLUDPServer;
use mutsea_protocol::login::OpenSimLoginService;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod opensim_server;
use opensim_server::OpenSimServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with better formatting
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mutsea=info,mutsea_server=info,mutsea_network=info,mutsea_protocol=info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_level(true)
        )
        .init();

    info!("🚀 Starting Mutsea Virtual World Server...");
    info!("Version: {}", mutsea_core::VERSION);

    // Load configuration
    let config = load_config().await?;
    
    // Validate configuration
    if let Err(errors) = config.validate() {
        for error in errors {
            error!("❌ Configuration error: {}", error);
        }
        return Err("Invalid configuration".into());
    }

    info!("✅ Configuration loaded and validated successfully");

    // Create shared login service
    let login_service = Arc::new(OpenSimLoginService::new());
    
    // Add some default test users
    login_service.add_test_user("Test".to_string(), "User".to_string(), "password".to_string());
    login_service.add_test_user("Admin".to_string(), "User".to_string(), "admin".to_string());
    login_service.add_test_user("Guest".to_string(), "User".to_string(), "guest".to_string());
    login_service.add_test_user("Demo".to_string(), "User".to_string(), "demo".to_string());
    
    info!("👥 Test users created: {}", login_service.list_users().join(", "));

    // Create OpenSim HTTP server
    let opensim_server = OpenSimServer::new(config.clone());
    
    // Create LLUDP server for viewer connections
    let mut lludp_server = LLUDPServer::new(&config.network.lludp).await?;
    lludp_server.set_login_service(Arc::clone(&login_service));

    // Determine server mode and ports
    let (http_port, lludp_port, mode) = if config.opensim.enabled {
        if config.opensim.grid_name.to_lowercase().contains("standalone") {
            (9000, 9000, "STANDALONE")
        } else {
            (8002, 9000, "GRID") 
        }
    } else {
        (config.network.http.port, config.network.lludp.port, "CUSTOM")
    };

    info!("🔧 Server Configuration:");
    info!("   Mode: {} mode", mode);
    info!("   HTTP Port: {} (Login/Web interface)", http_port);
    info!("   LLUDP Port: {} (Viewer connections)", lludp_port);
    info!("   Grid Name: {}", config.opensim.grid_name);
    info!("   Login URI: {}", config.opensim.login_uri);

    // Start LLUDP server first
    if config.opensim.enabled {
        info!("🌐 Starting LLUDP server for viewer connections...");
        lludp_server.start().await?;
        info!("✅ LLUDP server listening on {}:{}", config.network.lludp.bind_address, lludp_port);
    }

    // Start HTTP server
    info!("🌐 Starting HTTP server for login and web interface...");
    opensim_server.start().await?;
    info!("✅ HTTP server listening on {}:{}", config.network.http.bind_address, http_port);

    // Display connection information
    info!("");
    info!("🎉 Mutsea server started successfully!");
    info!("");
    info!("📱 Connect with Firestorm Viewer:");
    info!("   1. Open Firestorm");
    info!("   2. Grid Manager → Add Grid");
    info!("   3. Login URI: http://{}:{}/", 
          if config.network.http.bind_address == "0.0.0.0" { "127.0.0.1" } else { &config.network.http.bind_address }, 
          http_port);
    info!("   4. Grid Name: {}", config.opensim.grid_name);
    info!("   5. Login with test accounts:");
    for user in login_service.list_users() {
        info!("      - {} (password: see console)", user);
    }
    info!("");
    info!("🌐 Web Interface: http://{}:{}/", 
          if config.network.http.bind_address == "0.0.0.0" { "127.0.0.1" } else { &config.network.http.bind_address }, 
          http_port);
    info!("📊 Health Check: http://{}:{}/health", 
          if config.network.http.bind_address == "0.0.0.0" { "127.0.0.1" } else { &config.network.http.bind_address }, 
          http_port);
    info!("");

    // Start monitoring task
    start_monitoring_task(&lludp_server, &opensim_server).await;

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("📡 Received shutdown signal, stopping server...");
        }
        Err(err) => {
            error!("❌ Unable to listen for shutdown signal: {}", err);
        }
    }

    // Stop services gracefully
    info!("🛑 Stopping LLUDP server...");
    lludp_server.stop().await?;

    info!("🛑 Stopping HTTP server...");
    opensim_server.stop().await?;

    info!("✅ Mutsea server stopped successfully");
    Ok(())
}

async fn load_config() -> Result<MutseaConfig, Box<dyn std::error::Error>> {
    // Try to load from various config file locations
    let config_paths = [
        "config/mutsea.toml",
        "mutsea.toml",
        "mutsea.example.toml",
        "config/mutsea.example.toml",
    ];

    for path in &config_paths {
        if std::path::Path::new(path).exists() {
            info!("📄 Loading configuration from {}", path);
            return MutseaConfig::from_file(path).map_err(|e| e.into());
        }
    }

    // Try environment variables
    if let Ok(config) = MutseaConfig::from_env() {
        info!("🌍 Configuration loaded from environment variables");
        return Ok(config);
    }

    warn!("⚠️  No configuration file found, using default configuration");
    warn!("💡 Tip: Copy config/mutsea.example.toml to config/mutsea.toml to customize settings");
    Ok(MutseaConfig::default())
}

async fn start_monitoring_task(lludp_server: &LLUDPServer, opensim_server: &OpenSimServer) {
    let lludp_clone = lludp_server.clone();
    let opensim_clone = opensim_server.clone();
    
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            
            // Get statistics
            let lludp_stats = lludp_clone.get_stats().await;
            let circuits_count = lludp_clone.get_active_circuits_count().await;
            
            info!("📈 Server Statistics:");
            info!("   Active Circuits: {}", circuits_count);
            info!("   Packets Received: {}", lludp_stats.packets_received);
            info!("   Packets Sent: {}", lludp_stats.packets_sent);
            info!("   Bytes Received: {:.2} KB", lludp_stats.bytes_received as f64 / 1024.0);
            info!("   Bytes Sent: {:.2} KB", lludp_stats.bytes_sent as f64 / 1024.0);
            info!("   Total Connections: {}", lludp_stats.connections);
            info!("   Active Sessions: {}", lludp_stats.active_sessions);
            info!("   Errors: {}", lludp_stats.errors);
            info!("   Login Attempts: {}", lludp_stats.login_attempts);
            info!("   Successful Logins: {}", lludp_stats.successful_logins);
            info!("   Heartbeats Sent: {}", lludp_stats.heartbeats_sent);
            info!("   Reliable Resends: {}", lludp_stats.reliable_resends);
            
            if circuits_count > 0 {
                info!("🎮 {} active viewer connection(s)", circuits_count);
                
                // Get detailed circuit information
                let circuits = lludp_clone.get_all_circuits().await;
                for circuit in circuits {
                    if circuit.authenticated {
                        info!("   Circuit {}: {} @ ({:.1}, {:.1}, {:.1}) - {}",
                              circuit.circuit_code,
                              circuit.client_info.as_ref()
                                  .map(|c| format!("{} {}", c.viewer_name, c.viewer_version))
                                  .unwrap_or_else(|| "Unknown Client".to_string()),
                              circuit.position.x,
                              circuit.position.y,
                              circuit.position.z,
                              format_duration(circuit.last_activity.duration_since(circuit.created_at))
                        );
                    }
                }
            } else {
                info!("💤 No active viewer connections");
            }
            
            // Check server health
            let lludp_health = lludp_clone.health_check().await;
            let opensim_health = opensim_clone.health_check().await;
            
            match (lludp_health.status, opensim_health.status) {
                (mutsea_core::ServiceStatus::Healthy, mutsea_core::ServiceStatus::Healthy) => {
                    info!("💚 All services healthy");
                }
                _ => {
                    warn!("⚠️  Some services degraded: LLUDP={:?}, HTTP={:?}", 
                          lludp_health.status, opensim_health.status);
                }
            }
            
            // Memory and performance info
            info!("🔧 Performance:");
            info!("   LLUDP Error Rate: {:.2}%", 
                  if lludp_stats.packets_received > 0 {
                      (lludp_stats.errors as f64 / lludp_stats.packets_received as f64) * 100.0
                  } else {
                      0.0
                  });
            info!("   Success Rate: {:.2}%",
                  if lludp_stats.login_attempts > 0 {
                      (lludp_stats.successful_logins as f64 / lludp_stats.login_attempts as f64) * 100.0
                  } else {
                      0.0
                  });
        }
    });
}

/// Format duration in a human-readable way
fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Handle graceful shutdown
async fn shutdown_handler(lludp_server: Arc<LLUDPServer>, opensim_server: Arc<OpenSimServer>) {
    info!("🔄 Initiating graceful shutdown...");
    
    // Send logout messages to all connected clients
    if let Err(e) = lludp_server.emergency_shutdown("Server is shutting down for maintenance").await {
        error!("Error during emergency shutdown: {}", e);
    }
    
    // Wait a moment for clients to receive logout messages
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    
    // Stop services
    if let Err(e) = lludp_server.stop().await {
        error!("Error stopping LLUDP server: {}", e);
    }
    
    if let Err(e) = opensim_server.stop().await {
        error!("Error stopping OpenSim server: {}", e);
    }
    
    info!("✅ Graceful shutdown completed");
}

/// Print startup banner
fn print_startup_banner() {
    info!("");
    info!("╔══════════════════════════════════════════════════════════════╗");
    info!("║                    🌍 MUTSEA SERVER 🌍                      ║");
    info!("║              Next-Generation Virtual World Platform          ║");
    info!("║                                                              ║");
    info!("║  🔗 OpenSimulator Compatible  🤖 AI-Ready  ⚡ High Performance ║");
    info!("║                                                              ║");
    info!("║              Built with ❤️  in Rust                          ║");
    info!("╚══════════════════════════════════════════════════════════════╝");
    info!("");
}