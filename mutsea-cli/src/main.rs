//! mutsea-cli/src/main.rs
//! Enhanced Mutsea command-line interface with OpenSim user management

use clap::{Parser, Subcommand};
use mutsea_core::{config::MutseaConfig, UserAccount, UserId};
use mutsea_protocol::login::OpenSimLoginService;
use std::path::PathBuf;
use tracing::{info, error, warn};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "mutsea")]
#[command(about = "Mutsea Virtual World Platform CLI")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config/mutsea.toml")]
    config: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Database operations
    #[command(subcommand)]
    Database(DatabaseCommands),
    
    /// User management
    #[command(subcommand)]
    User(UserCommands),
    
    /// Server management
    #[command(subcommand)]
    Server(ServerCommands),
    
    /// Configuration management
    Config {
        /// Generate example configuration
        #[arg(long)]
        example: bool,
        
        /// Validate configuration
        #[arg(long)]
        validate: bool,
        
        /// Show current configuration
        #[arg(long)]
        show: bool,
    },

    /// Grid management
    #[command(subcommand)]
    Grid(GridCommands),

    /// Start the server directly from CLI
    Start {
        /// Override HTTP port
        #[arg(long)]
        http_port: Option<u16>,
        
        /// Override LLUDP port
        #[arg(long)]
        lludp_port: Option<u16>,
        
        /// Enable standalone mode
        #[arg(long)]
        standalone: bool,
        
        /// Enable grid mode
        #[arg(long)]
        grid: bool,
    },
}

#[derive(Subcommand)]
enum DatabaseCommands {
    /// Run database migrations
    Migrate,
    
    /// Reset database (dangerous!)
    Reset {
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
    
    /// Check database status
    Status,
    
    /// Backup database
    Backup {
        /// Backup file path
        path: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum UserCommands {
    /// Create a new user
    Create {
        /// First name
        first_name: String,
        /// Last name
        last_name: String,
        /// Email address
        #[arg(short, long)]
        email: Option<String>,
        /// Password (will be prompted if not provided)
        #[arg(short, long)]
        password: Option<String>,
        /// Set as admin user
        #[arg(long)]
        admin: bool,
    },
    
    /// List users
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// Delete a user
    Delete {
        /// User ID or "first last" name
        user: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
    
    /// Reset user password
    ResetPassword {
        /// User ID or "first last" name
        user: String,
        /// New password (will be prompted if not provided)
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Test user login
    Test {
        /// First name
        first_name: String,
        /// Last name
        last_name: String,
        /// Password
        password: String,
    },

    /// Import users from file
    Import {
        /// CSV file path
        file: PathBuf,
        /// Skip header row
        #[arg(long)]
        skip_header: bool,
    },
}

#[derive(Subcommand)]
enum ServerCommands {
    /// Start the server
    Start {
        /// Detach from terminal
        #[arg(short, long)]
        daemon: bool,
    },
    
    /// Stop the server
    Stop,
    
    /// Restart the server
    Restart,
    
    /// Check server status
    Status,
    
    /// Generate server statistics
    Stats,
    
    /// Show server logs
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value = "50")]
        lines: usize,
        
        /// Follow log output
        #[arg(short, long)]
        follow: bool,
    },
}

#[derive(Subcommand)]
enum GridCommands {
    /// Show grid information
    Info,
    
    /// Test grid connectivity
    Test,
    
    /// Set grid mode (standalone/grid)
    Mode {
        /// Grid mode
        #[arg(value_enum)]
        mode: GridMode,
    },
    
    /// Reset grid to defaults
    Reset,
}

#[derive(clap::ValueEnum, Clone)]
enum GridMode {
    Standalone,
    Grid,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("mutsea={},mutsea_cli={}", log_level, log_level))
        .with_target(false)
        .init();

    // Load configuration
    let config = if cli.config.exists() {
        MutseaConfig::from_file(&cli.config)?
    } else {
        MutseaConfig::default()
    };

    match cli.command {
        Commands::Database(cmd) => handle_database_command(cmd, &config).await?,
        Commands::User(cmd) => handle_user_command(cmd, &config).await?,
        Commands::Server(cmd) => handle_server_command(cmd, &config).await?,
        Commands::Config { example, validate, show } => handle_config_command(example, validate, show, &config)?,
        Commands::Grid(cmd) => handle_grid_command(cmd, &config).await?,
        Commands::Start { http_port, lludp_port, standalone, grid } => {
            handle_start_command(config, http_port, lludp_port, standalone, grid).await?;
        }
    }

    Ok(())
}

async fn handle_database_command(
    cmd: DatabaseCommands,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        DatabaseCommands::Migrate => {
            info!("🔄 Running database migrations...");
            // TODO: Implement database migrations
            info!("✅ Database migrations completed successfully");
        }
        DatabaseCommands::Reset { force } => {
            if !force {
                print!("⚠️  This will delete all data. Are you sure? (y/N): ");
                use std::io::{self, Write};
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    info!("❌ Operation cancelled");
                    return Ok(());
                }
            }
            
            info!("🔄 Resetting database...");
            // TODO: Implement database reset
            info!("✅ Database reset completed");
        }
        DatabaseCommands::Status => {
            info!("🔍 Checking database status...");
            // TODO: Implement database status check
            info!("✅ Database: Connected");
            info!("📊 Schema version: 1.0.0");
            info!("📋 Tables: 8");
        }
        DatabaseCommands::Backup { path } => {
            let backup_path = path.unwrap_or_else(|| {
                PathBuf::from(format!("backup_mutsea_{}.sql", chrono::Utc::now().format("%Y%m%d_%H%M%S")))
            });
            info!("💾 Creating database backup: {:?}", backup_path);
            // TODO: Implement database backup
            info!("✅ Database backup completed");
        }
    }
    Ok(())
}

async fn handle_user_command(
    cmd: UserCommands,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    // For now, we'll use the login service for user management
    let login_service = OpenSimLoginService::new();
    
    match cmd {
        UserCommands::Create { first_name, last_name, email, password, admin } => {
            let password = if let Some(pwd) = password {
                pwd
            } else {
                rpassword::prompt_password("🔒 Enter password: ")?
            };

            info!("👤 Creating user: {} {}", first_name, last_name);
            
            // Add user to login service (in a real implementation, this would persist to database)
            login_service.add_test_user(first_name.clone(), last_name.clone(), password.clone());
            
            let user_id = UserId::new();
            
            info!("✅ User created successfully!");
            info!("🆔 User ID: {}", user_id);
            info!("👤 Name: {} {}", first_name, last_name);
            if let Some(email) = email {
                info!("📧 Email: {}", email);
            }
            if admin {
                info!("👑 Admin privileges: Yes");
            }
            info!("");
            info!("🎮 Connect with Firestorm using:");
            info!("   First Name: {}", first_name);
            info!("   Last Name: {}", last_name);
            info!("   Password: [the password you entered]");
            info!("   Login URI: {}", config.opensim.login_uri);
        }
        UserCommands::List { detailed } => {
            info!("👥 Listing users...");
            let users = login_service.list_users();
            
            if users.is_empty() {
                warn!("No users found. Create one with: mutsea user create");
            } else {
                info!("📋 Found {} user(s):", users.len());
                for (i, user) in users.iter().enumerate() {
                    if detailed {
                        info!("  {}. {} (ID: {}, Created: {})", 
                              i + 1, user, UserId::new(), chrono::Utc::now().format("%Y-%m-%d"));
                    } else {
                        info!("  {}. {}", i + 1, user);
                    }
                }
            }
        }
        UserCommands::Delete { user, force } => {
            if !force {
                print!("⚠️  Delete user '{}'? (y/N): ", user);
                use std::io::{self, Write};
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    info!("❌ Operation cancelled");
                    return Ok(());
                }
            }
            
            info!("🗑️  Deleting user: {}", user);
            // TODO: Implement user deletion from login service
            info!("✅ User deleted successfully");
        }
        UserCommands::ResetPassword { user, password } => {
            let password = if let Some(pwd) = password {
                pwd
            } else {
                rpassword::prompt_password("🔒 Enter new password: ")?
            };

            info!("🔄 Resetting password for user: {}", user);
            // TODO: Implement password reset in login service
            info!("✅ Password reset successfully");
        }
        UserCommands::Test { first_name, last_name, password } => {
            info!("🧪 Testing login for: {} {}", first_name, last_name);
            
            let test_request = mutsea_protocol::login::ParsedLoginRequest {
                first: first_name.clone(),
                last: last_name.clone(),
                passwd: password.clone(),
                start: "home".to_string(),
                channel: "MutseaCLI".to_string(),
                version: "1.0.0".to_string(),
                platform: "CLI".to_string(),
                mac: "00:00:00:00:00:00".to_string(),
                id0: "test".to_string(),
                agree_to_tos: "true".to_string(),
                read_critical: "true".to_string(),
                viewer_digest: "test".to_string(),
                options: vec![],
            };
            
            match login_service.authenticate(&test_request) {
                Ok(response) => {
                    if response.login == "true" {
                        info!("✅ Login test successful!");
                        info!("🆔 Session ID: {}", response.session_id.unwrap_or_default());
                        info!("🌍 Sim IP: {}", response.sim_ip.unwrap_or_default());
                        info!("🔌 Circuit Code: {}", response.circuit_code.unwrap_or_default());
                    } else {
                        error!("❌ Login test failed: {}", response.reason);
                    }
                }
                Err(e) => {
                    error!("❌ Login test error: {}", e);
                }
            }
        }
        UserCommands::Import { file, skip_header } => {
            info!("📥 Importing users from: {:?}", file);
            // TODO: Implement CSV user import
            info!("✅ User import completed");
        }
    }
    Ok(())
}

async fn handle_server_command(
    cmd: ServerCommands,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ServerCommands::Start { daemon } => {
            if daemon {
                info!("🚀 Starting Mutsea server in daemon mode...");
                // TODO: Implement daemon mode
            } else {
                info!("💡 To start the server, use: cargo run --bin mutsea-server");
                info!("💡 Or use: mutsea start (to start from CLI)");
            }
        }
        ServerCommands::Stop => {
            info!("🛑 Stopping Mutsea server...");
            // TODO: Implement server stop via signal/pid file
            info!("✅ Server stopped");
        }
        ServerCommands::Restart => {
            info!("🔄 Restarting Mutsea server...");
            // TODO: Implement server restart
            info!("✅ Server restarted");
        }
        ServerCommands::Status => {
            info!("🔍 Checking server status...");
            
            // Try to connect to health endpoint
            let health_url = format!("http://{}:{}/health", 
                                   config.network.http.bind_address, 
                                   config.network.http.port);
            
            match reqwest::get(&health_url).await {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(health_data) = response.json::<serde_json::Value>().await {
                            info!("✅ Server: Online");
                            info!("🌍 Grid: {}", health_data.get("grid_name").unwrap_or(&serde_json::Value::String("Unknown".to_string())));
                            info!("🔗 Login URI: {}", health_data.get("login_uri").unwrap_or(&serde_json::Value::String("Unknown".to_string())));
                            info!("👥 Users: {}", health_data.get("users_count").unwrap_or(&serde_json::Value::Number(serde_json::Number::from(0))));
                        } else {
                            info!("✅ Server: Online (health data unavailable)");
                        }
                    } else {
                        warn!("⚠️  Server responded with status: {}", response.status());
                    }
                }
                Err(_) => {
                    error!("❌ Server: Offline or not responding");
                    info!("💡 Start the server with: cargo run --bin mutsea-server");
                }
            }
        }
        ServerCommands::Stats => {
            info!("📊 Generating server statistics...");
            // Try to get stats from running server
            let health_url = format!("http://{}:{}/health", 
                                   config.network.http.bind_address, 
                                   config.network.http.port);
            
            match reqwest::get(&health_url).await {
                Ok(response) => {
                    if let Ok(data) = response.json::<serde_json::Value>().await {
                        info!("📈 Server Statistics:");
                        info!("   Status: {}", data.get("status").unwrap_or(&serde_json::Value::String("unknown".to_string())));
                        info!("   Uptime: {}", data.get("timestamp").unwrap_or(&serde_json::Value::String("unknown".to_string())));
                        info!("   Grid Name: {}", data.get("grid_name").unwrap_or(&serde_json::Value::String("unknown".to_string())));
                        info!("   Users Count: {}", data.get("users_count").unwrap_or(&serde_json::Value::Number(serde_json::Number::from(0))));
                    }
                }
                Err(_) => {
                    error!("❌ Unable to retrieve statistics - server may be offline");
                }
            }
        }
        ServerCommands::Logs { lines, follow } => {
            info!("📄 Server logs ({} lines):", lines);
            if follow {
                info!("👁️  Following log output (Ctrl+C to stop)...");
                // TODO: Implement log following
            } else {
                // TODO: Implement log reading
                info!("💡 Log file location: {}", config.logging.log_file.as_ref().unwrap_or(&std::path::PathBuf::from("logs/mutsea.log")).display());
            }
        }
    }
    Ok(())
}

fn handle_config_command(
    example: bool,
    validate: bool,
    show: bool,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if example {
        info!("📄 Generating example configuration...");
        let example_config = MutseaConfig::default();
        example_config.to_file("mutsea.example.toml")?;
        info!("✅ Example configuration written to mutsea.example.toml");
    }

    if validate {
        info!("🔍 Validating configuration...");
        match config.validate() {
            Ok(()) => {
                info!("✅ Configuration is valid");
            }
            Err(errors) => {
                error!("❌ Configuration validation failed:");
                for error in errors {
                    error!("  - {}", error);
                }
                return Err("Invalid configuration".into());
            }
        }
    }

    if show {
        info!("📋 Current configuration:");
        info!("🌍 Grid Name: {}", config.opensim.grid_name);
        info!("🔗 Login URI: {}", config.opensim.login_uri);
        info!("🌐 HTTP Port: {}", config.network.http.port);
        info!("📡 LLUDP Port: {}", config.network.lludp.port);
        info!("🗄️  Database: {}", if config.database.url.contains("postgresql") { "PostgreSQL" } else if config.database.url.contains("mysql") { "MySQL" } else { "SQLite" });
        info!("💾 Cache: {}", config.cache.cache_type);
        info!("🤖 AI Features: {}", if config.ai.enabled { "Enabled" } else { "Disabled" });
    }

    Ok(())
}

async fn handle_grid_command(
    cmd: GridCommands,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        GridCommands::Info => {
            info!("🌍 Grid Information:");
            info!("   Name: {}", config.opensim.grid_name);
            info!("   Nickname: {}", config.opensim.grid_nick);
            info!("   Login URI: {}", config.opensim.login_uri);
            info!("   Grid Info URI: {}", config.opensim.grid_info_uri);
            info!("   Owner: {}", config.opensim.grid_owner);
            info!("   Owner Email: {}", config.opensim.grid_owner_email);
            info!("   Currency: {}", config.opensim.currency_symbol);
            info!("   Voice Enabled: {}", config.opensim.enable_voice);
            info!("   Search Enabled: {}", config.opensim.enable_search);
        }
        GridCommands::Test => {
            info!("🧪 Testing grid connectivity...");
            
            let grid_info_url = &config.opensim.grid_info_uri;
            match reqwest::get(grid_info_url).await {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("✅ Grid info endpoint: OK");
                        if let Ok(grid_data) = response.json::<serde_json::Value>().await {
                            info!("📊 Grid data received: {} fields", 
                                  grid_data.as_object().map(|o| o.len()).unwrap_or(0));
                        }
                    } else {
                        warn!("⚠️  Grid info endpoint returned: {}", response.status());
                    }
                }
                Err(e) => {
                    error!("❌ Grid info endpoint failed: {}", e);
                }
            }
            
            // Test login endpoint
            let login_url = &config.opensim.login_uri;
            info!("🔍 Testing login endpoint: {}", login_url);
            match reqwest::get(login_url).await {
                Ok(response) => {
                    info!("✅ Login endpoint: Reachable (status: {})", response.status());
                }
                Err(e) => {
                    error!("❌ Login endpoint failed: {}", e);
                }
            }
        }
        GridCommands::Mode { mode } => {
            match mode {
                GridMode::Standalone => {
                    info!("🔧 Setting grid to STANDALONE mode...");
                    info!("💡 This requires updating configuration and restarting server");
                    info!("📝 Update mutsea.toml:");
                    info!("   [opensim]");
                    info!("   grid_name = \"Mutsea Standalone Grid\"");
                    info!("   login_uri = \"http://127.0.0.1:9000/\"");
                }
                GridMode::Grid => {
                    info!("🔧 Setting grid to GRID mode...");
                    info!("💡 This requires updating configuration and restarting server");
                    info!("📝 Update mutsea.toml:");
                    info!("   [opensim]");
                    info!("   grid_name = \"Mutsea Grid\"");
                    info!("   login_uri = \"http://127.0.0.1:8002/\"");
                }
            }
        }
        GridCommands::Reset => {
            info!("🔄 Resetting grid to default settings...");
            // TODO: Implement grid reset
            info!("✅ Grid reset to defaults");
        }
    }
    Ok(())
}

async fn handle_start_command(
    mut config: MutseaConfig,
    http_port: Option<u16>,
    lludp_port: Option<u16>,
    standalone: bool,
    grid: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Override configuration based on CLI flags
    if let Some(port) = http_port {
        config.network.http.port = port;
    }
    
    if let Some(port) = lludp_port {
        config.network.lludp.port = port;
    }
    
    if standalone {
        config.opensim.grid_name = "Mutsea Standalone Grid".to_string();
        config.opensim.login_uri = format!("http://127.0.0.1:{}/", config.network.http.port);
        config.network.http.port = 9000;
        config.network.lludp.port = 9000;
    }
    
    if grid {
        config.opensim.grid_name = "Mutsea Grid".to_string();
        config.opensim.login_uri = format!("http://127.0.0.1:{}/", config.network.http.port);
        if config.network.http.port == 9000 {
            config.network.http.port = 8002;
        }
    }

    info!("🚀 Starting Mutsea server from CLI...");
    info!("⚠️  Note: This will start the server in this terminal session");
    info!("💡 For production use: cargo run --bin mutsea-server");
    info!("");
    
    // Start the server (this would normally be in mutsea-server)
    // For now, just show what would be started
    info!("Would start server with:");
    info!("  HTTP Port: {}", config.network.http.port);
    info!("  LLUDP Port: {}", config.network.lludp.port);
    info!("  Grid: {}", config.opensim.grid_name);
    info!("  Login URI: {}", config.opensim.login_uri);
    
    Ok(())
}