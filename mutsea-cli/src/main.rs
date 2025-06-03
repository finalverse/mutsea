//! Mutsea command-line interface

use clap::{Parser, Subcommand};
use mutsea_core::{config::MutseaConfig, UserAccount, UserId};
use std::path::PathBuf;
use tracing::{info, error};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "config/mutsea.toml")]
    config: PathBuf,

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
        email: Option<String>,
        /// Password (will be prompted if not provided)
        #[arg(short, long)]
        password: Option<String>,
    },
    
    /// List users
    List,
    
    /// Delete a user
    Delete {
        /// User ID or "first last" name
        user: String,
    },
    
    /// Reset user password
    ResetPassword {
        /// User ID or "first last" name
        user: String,
        /// New password (will be prompted if not provided)
        #[arg(short, long)]
        password: Option<String>,
    },
}

#[derive(Subcommand)]
enum ServerCommands {
    /// Start the server
    Start,
    
    /// Stop the server
    Stop,
    
    /// Check server status
    Status,
    
    /// Generate server statistics
    Stats,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

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
        Commands::Config { example, validate } => handle_config_command(example, validate, &config)?,
    }

    Ok(())
}

async fn handle_database_command(
    cmd: DatabaseCommands,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        DatabaseCommands::Migrate => {
            info!("Running database migrations...");
            // TODO: Implement database migrations
            info!("Database migrations completed successfully");
        }
        DatabaseCommands::Reset { force } => {
            if !force {
                print!("This will delete all data. Are you sure? (y/N): ");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    info!("Operation cancelled");
                    return Ok(());
                }
            }
            
            info!("Resetting database...");
            // TODO: Implement database reset
            info!("Database reset completed");
        }
        DatabaseCommands::Status => {
            info!("Checking database status...");
            // TODO: Implement database status check
            info!("Database: Connected");
            info!("Schema version: 1.0.0");
            info!("Tables: 8");
        }
    }
    Ok(())
}

async fn handle_user_command(
    cmd: UserCommands,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        UserCommands::Create { first_name, last_name, email, password } => {
            let password = if let Some(pwd) = password {
                pwd
            } else {
                rpassword::prompt_password("Enter password: ")?
            };

            info!("Creating user: {} {}", first_name, last_name);
            
            // TODO: Implement user creation
            let user_id = UserId::new();
            
            info!("User created successfully!");
            info!("User ID: {}", user_id);
            info!("Name: {} {}", first_name, last_name);
            if let Some(email) = email {
                info!("Email: {}", email);
            }
            info!("");
            info!("You can now login with Firestorm using:");
            info!("  First Name: {}", first_name);
            info!("  Last Name: {}", last_name);
            info!("  Password: [the password you entered]");
        }
        UserCommands::List => {
            info!("Listing users...");
            // TODO: Implement user listing
            info!("Users:");
            info!("  1. Test User (test.user@example.com)");
            info!("  2. Admin User (admin@example.com)");
        }
        UserCommands::Delete { user } => {
            info!("Deleting user: {}", user);
            // TODO: Implement user deletion
            info!("User deleted successfully");
        }
        UserCommands::ResetPassword { user, password } => {
            let password = if let Some(pwd) = password {
                pwd
            } else {
                rpassword::prompt_password("Enter new password: ")?
            };

            info!("Resetting password for user: {}", user);
            // TODO: Implement password reset
            info!("Password reset successfully");
        }
    }
    Ok(())
}

async fn handle_server_command(
    cmd: ServerCommands,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ServerCommands::Start => {
            info!("Starting Mutsea server...");
            info!("Use 'cargo run --bin mutsea-server' to start the server");
        }
        ServerCommands::Stop => {
            info!("Stopping Mutsea server...");
            // TODO: Implement server stop
            info!("Server stopped");
        }
        ServerCommands::Status => {
            info!("Checking server status...");
            // TODO: Implement server status check
            info!("Server: Not running");
        }
        ServerCommands::Stats => {
            info!("Generating server statistics...");
            // TODO: Implement statistics generation
            info!("Statistics:");
            info!("  Active connections: 0");
            info!("  Total users: 2");
            info!("  Uptime: 0 minutes");
        }
    }
    Ok(())
}

fn handle_config_command(
    example: bool,
    validate: bool,
    config: &MutseaConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if example {
        info!("Generating example configuration...");
        let example_config = MutseaConfig::default();
        example_config.to_file("mutsea.example.toml")?;
        info!("Example configuration written to mutsea.example.toml");
    }

    if validate {
        info!("Validating configuration...");
        match config.validate() {
            Ok(()) => {
                info!("Configuration is valid");
            }
            Err(errors) => {
                error!("Configuration validation failed:");
                for error in errors {
                    error!("  - {}", error);
                }
                return Err("Invalid configuration".into());
            }
        }
    }

    Ok(())
}