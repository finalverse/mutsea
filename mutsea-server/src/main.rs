//! Mutsea server main application

use mutsea_core::{Service, config::MutseaConfig};
use mutsea_network::NetworkService;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mutsea=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Mutsea server...");

    // Load configuration
    let config = load_config().await?;
    
    // Validate configuration
    if let Err(errors) = config.validate() {
        for error in errors {
            error!("Configuration error: {}", error);
        }
        return Err("Invalid configuration".into());
    }

    info!("Configuration loaded successfully");

    // Create network service
    let network_service = NetworkService::new();

    // Start LLUDP server
    if config.opensim.enabled {
        info!("Starting LLUDP server for OpenSim compatibility...");
        network_service.start_lludp(&config.network.lludp).await?;
    }

    // Start HTTP server
    info!("Starting HTTP server...");
    network_service.start_http(&config.network.http).await?;

    // Start WebSocket server (optional)
    if config.network.http.port != config.network.lludp.port {
        info!("Starting WebSocket server...");
        network_service.start_websocket(config.network.http.port + 1).await?;
    }

    // Start the network service
    network_service.start().await?;

    info!("Mutsea server started successfully!");
    info!("HTTP server listening on http://{}:{}", config.network.http.bind_address, config.network.http.port);
    info!("LLUDP server listening on {}:{}", config.network.lludp.bind_address, config.network.lludp.port);
    
    if config.opensim.enabled {
        info!("OpenSim compatibility enabled");
        info!("Grid name: {}", config.opensim.grid_name);
        info!("Login URI: {}", config.opensim.login_uri);
        info!("");
        info!("To connect with Firestorm:");
        info!("  1. Add new grid with Login URI: {}", config.opensim.login_uri);
        info!("  2. Create a user with: cargo run --bin mutsea-cli -- create-user");
        info!("  3. Login with your created credentials");
    }

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Received shutdown signal, stopping server...");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Stop services
    info!("Stopping network service...");
    network_service.stop().await?;

    info!("Mutsea server stopped successfully");
    Ok(())
}

async fn load_config() -> Result<MutseaConfig, Box<dyn std::error::Error>> {
    // Try to load from config file
    if std::path::Path::new("config/mutsea.toml").exists() {
        info!("Loading configuration from config/mutsea.toml");
        MutseaConfig::from_file("config/mutsea.toml").map_err(|e| e.into())
    } else if std::path::Path::new("mutsea.toml").exists() {
        info!("Loading configuration from mutsea.toml");
        MutseaConfig::from_file("mutsea.toml").map_err(|e| e.into())
    } else {
        warn!("No configuration file found, using default configuration");
        warn!("Copy config/mutsea.example.toml to config/mutsea.toml to customize settings");
        Ok(MutseaConfig::default())
    }
}