//! Configuration management for Mutsea

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main configuration structure for Mutsea
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutseaConfig {
    /// Server configuration
    pub server: ServerConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Asset storage configuration
    pub assets: AssetConfig,
    /// OpenSim compatibility configuration
    pub opensim: OpenSimConfig,
    /// AI configuration (Phase II)
    #[serde(default)]
    pub ai: AIConfig,
    /// Custom configuration values
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server name
    pub name: String,
    /// Server bind address
    pub bind_address: String,
    /// Server port
    pub port: u16,
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    /// Worker thread count (0 = auto)
    pub worker_threads: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Keep-alive timeout in seconds
    pub keep_alive_timeout: u64,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Monitoring bind address
    pub monitoring_address: String,
    /// Monitoring port
    pub monitoring_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "Mutsea Server".to_string(),
            bind_address: "0.0.0.0".to_string(),
            port: 9000,
            max_connections: 1000,
            worker_threads: 0,
            request_timeout: 30,
            keep_alive_timeout: 300,
            enable_monitoring: true,
            monitoring_address: "127.0.0.1".to_string(),
            monitoring_port: 9001,
        }
    }
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database URL
    pub url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Minimum number of connections in the pool
    pub min_connections: u32,
    /// Connection timeout in seconds
    pub connect_timeout: u64,
    /// Query timeout in seconds
    pub query_timeout: u64,
    /// Enable automatic migrations
    pub auto_migrate: bool,
    /// Enable SQL query logging
    pub log_queries: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://mutsea:mutsea@localhost/mutsea".to_string(),
            max_connections: 100,
            min_connections: 5,
            connect_timeout: 30,
            query_timeout: 60,
            auto_migrate: true,
            log_queries: false,
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Cache type (redis, memory)
    pub cache_type: String,
    /// Redis URL (if using Redis)
    pub redis_url: Option<String>,
    /// Maximum memory cache size in MB
    pub max_memory_mb: usize,
    /// Default TTL in seconds
    pub default_ttl: u64,
    /// Enable cache compression
    pub enable_compression: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_type: "redis".to_string(),
            redis_url: Some("redis://localhost:6379".to_string()),
            max_memory_mb: 1024,
            default_ttl: 3600,
            enable_compression: true,
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// LLUDP server configuration
    pub lludp: LLUDPConfig,
    /// HTTP server configuration
    pub http: HTTPConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,
}

/// LLUDP protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLUDPConfig {
    /// LLUDP bind address
    pub bind_address: String,
    /// LLUDP port
    pub port: u16,
    /// Maximum packet size
    pub max_packet_size: usize,
    /// Resend timeout in milliseconds
    pub resend_timeout: u64,
    /// Maximum resend attempts
    pub max_resends: u8,
    /// Ack timeout in milliseconds
    pub ack_timeout: u64,
    /// Ping interval in seconds
    pub ping_interval: u64,
    /// Client timeout in seconds
    pub client_timeout: u64,
}

impl Default for LLUDPConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 9000,
            max_packet_size: 1200,
            resend_timeout: 100,
            max_resends: 3,
            ack_timeout: 1000,
            ping_interval: 5,
            client_timeout: 60,
        }
    }
}

/// HTTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPConfig {
    /// HTTP bind address
    pub bind_address: String,
    /// HTTP port
    pub port: u16,
    /// Enable HTTPS
    pub enable_https: bool,
    /// TLS certificate file path
    pub cert_file: Option<PathBuf>,
    /// TLS private key file path
    pub key_file: Option<PathBuf>,
    /// Enable CORS
    pub enable_cors: bool,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
}

impl Default for HTTPConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 8080,
            enable_https: false,
            cert_file: None,
            key_file: None,
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst limit
    pub burst_limit: u32,
    /// Ban duration in minutes for exceeded limits
    pub ban_duration: u32,
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 60,
            burst_limit: 10,
            ban_duration: 5,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (json, pretty)
    pub format: String,
    /// Log to file
    pub log_to_file: bool,
    /// Log file path
    pub log_file: Option<PathBuf>,
    /// Maximum log file size in MB
    pub max_file_size_mb: usize,
    /// Number of log files to retain
    pub max_files: usize,
    /// Enable structured logging
    pub structured: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            log_to_file: true,
            log_file: Some(PathBuf::from("logs/mutsea.log")),
            max_file_size_mb: 100,
            max_files: 10,
            structured: true,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable authentication
    pub enable_auth: bool,
    /// Password hashing algorithm
    pub password_hash_algorithm: String,
    /// Password hash cost factor
    pub password_hash_cost: u32,
    /// Session timeout in hours
    pub session_timeout: u64,
    /// JWT secret key
    pub jwt_secret: String,
    /// Enable IP whitelisting
    pub enable_ip_whitelist: bool,
    /// Whitelisted IP addresses
    pub ip_whitelist: Vec<String>,
    /// Enable IP blacklisting
    pub enable_ip_blacklist: bool,
    /// Blacklisted IP addresses
    pub ip_blacklist: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_auth: true,
            password_hash_algorithm: "bcrypt".to_string(),
            password_hash_cost: 12,
            session_timeout: 24,
            jwt_secret: "change-me-in-production".to_string(),
            enable_ip_whitelist: false,
            ip_whitelist: vec![],
            enable_ip_blacklist: false,
            ip_blacklist: vec![],
        }
    }
}

/// Asset storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetConfig {
    /// Storage backend (local, s3, azure, gcp)
    pub backend: String,
    /// Local storage path
    pub local_path: Option<PathBuf>,
    /// S3 configuration
    pub s3: Option<S3Config>,
    /// Azure configuration
    pub azure: Option<AzureConfig>,
    /// GCP configuration
    pub gcp: Option<GCPConfig>,
    /// Maximum asset size in MB
    pub max_asset_size_mb: usize,
    /// Enable asset compression
    pub enable_compression: bool,
    /// Asset cache TTL in seconds
    pub cache_ttl: u64,
}

/// S3 storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    /// S3 bucket name
    pub bucket: String,
    /// S3 region
    pub region: String,
    /// S3 access key ID
    pub access_key_id: String,
    /// S3 secret access key
    pub secret_access_key: String,
    /// S3 endpoint URL (for S3-compatible services)
    pub endpoint_url: Option<String>,
}

/// Azure Blob Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    /// Storage account name
    pub account_name: String,
    /// Storage account key
    pub account_key: String,
    /// Container name
    pub container: String,
}

/// Google Cloud Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCPConfig {
    /// GCP project ID
    pub project_id: String,
    /// Bucket name
    pub bucket: String,
    /// Service account key file path
    pub service_account_key: PathBuf,
}

impl Default for AssetConfig {
    fn default() -> Self {
        Self {
            backend: "local".to_string(),
            local_path: Some(PathBuf::from("data/assets")),
            s3: None,
            azure: None,
            gcp: None,
            max_asset_size_mb: 100,
            enable_compression: true,
            cache_ttl: 3600,
        }
    }
}

/// OpenSim compatibility configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSimConfig {
    /// Enable OpenSim compatibility mode
    pub enabled: bool,
    /// OpenSim grid name
    pub grid_name: String,
    /// OpenSim grid nickname
    pub grid_nick: String,
    /// Login URI
    pub login_uri: String,
    /// Grid info URI
    pub grid_info_uri: String,
    /// Default start location
    pub default_start_location: String,
    /// Default home location
    pub default_home_location: String,
    /// Currency symbol
    pub currency_symbol: String,
    /// Real currency symbol
    pub real_currency_symbol: String,
    /// Directory fee
    pub directory_fee: i32,
    /// Upload fee
    pub upload_fee: i32,
    /// Group creation fee
    pub group_creation_fee: i32,
    /// Enable voice
    pub enable_voice: bool,
    /// Enable search
    pub enable_search: bool,
    /// Enable destination guide
    pub enable_destination_guide: bool,
    /// Grid owner
    pub grid_owner: String,
    /// Grid owner email
    pub grid_owner_email: String,
}

impl Default for OpenSimConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            grid_name: "Mutsea Grid".to_string(),
            grid_nick: "mutsea".to_string(),
            login_uri: "http://localhost:8080/".to_string(),
            grid_info_uri: "http://localhost:8080/get_grid_info".to_string(),
            default_start_location: "home".to_string(),
            default_home_location: "last".to_string(),
            currency_symbol: "M$".to_string(),
            real_currency_symbol: "USD".to_string(),
            directory_fee: 30,
            upload_fee: 0,
            group_creation_fee: 0,
            enable_voice: true,
            enable_search: true,
            enable_destination_guide: true,
            grid_owner: "Mutsea Administrator".to_string(),
            grid_owner_email: "admin@mutsea.dev".to_string(),
        }
    }
}

/// AI configuration (Phase II)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Enable AI features
    pub enabled: bool,
    /// Content generation AI configuration
    pub content_generation: ContentGenerationConfig,
    /// Social intelligence AI configuration
    pub social_intelligence: SocialIntelligenceConfig,
    /// Natural language processing configuration
    pub nlp: NLPConfig,
    /// MapleAI integration configuration
    pub maple_ai: MapleAIConfig,
}

/// Content generation AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentGenerationConfig {
    /// Enable content generation
    pub enabled: bool,
    /// Text-to-3D model endpoint
    pub text_to_3d_endpoint: String,
    /// Texture generation endpoint
    pub texture_generation_endpoint: String,
    /// Animation generation endpoint
    pub animation_generation_endpoint: String,
    /// Maximum generation time in seconds
    pub max_generation_time: u64,
    /// Quality level (low, medium, high)
    pub quality_level: String,
    /// Enable caching of generated content
    pub enable_caching: bool,
}

/// Social intelligence AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialIntelligenceConfig {
    /// Enable social intelligence
    pub enabled: bool,
    /// Relationship analysis endpoint
    pub relationship_analysis_endpoint: String,
    /// Conversation facilitation endpoint
    pub conversation_facilitation_endpoint: String,
    /// Group dynamics prediction endpoint
    pub group_dynamics_endpoint: String,
    /// Update interval in seconds
    pub update_interval: u64,
}

/// Natural language processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLPConfig {
    /// Enable NLP features
    pub enabled: bool,
    /// Intent classification model
    pub intent_model: String,
    /// Entity extraction model
    pub entity_model: String,
    /// Sentiment analysis model
    pub sentiment_model: String,
    /// Language detection model
    pub language_model: String,
    /// Default language
    pub default_language: String,
    /// Supported languages
    pub supported_languages: Vec<String>,
}

/// MapleAI integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapleAIConfig {
    /// Enable MapleAI integration
    pub enabled: bool,
    /// MapleAI server endpoint
    pub server_endpoint: String,
    /// Agent capabilities
    pub agent_capabilities: Vec<String>,
    /// Consensus timeout in seconds
    pub consensus_timeout: u64,
    /// Maximum concurrent consensus sessions
    pub max_concurrent_sessions: u32,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            content_generation: ContentGenerationConfig {
                enabled: false,
                text_to_3d_endpoint: "http://localhost:8001/text-to-3d".to_string(),
                texture_generation_endpoint: "http://localhost:8001/generate-texture".to_string(),
                animation_generation_endpoint: "http://localhost:8001/generate-animation".to_string(),
                max_generation_time: 30,
                quality_level: "medium".to_string(),
                enable_caching: true,
            },
            social_intelligence: SocialIntelligenceConfig {
                enabled: false,
                relationship_analysis_endpoint: "http://localhost:8002/analyze-relationships".to_string(),
                conversation_facilitation_endpoint: "http://localhost:8002/facilitate-conversation".to_string(),
                group_dynamics_endpoint: "http://localhost:8002/predict-group-dynamics".to_string(),
                update_interval: 60,
            },
            nlp: NLPConfig {
                enabled: false,
                intent_model: "distilbert-base-uncased".to_string(),
                entity_model: "dbmdz/bert-large-cased-finetuned-conll03-english".to_string(),
                sentiment_model: "cardiffnlp/twitter-roberta-base-sentiment-latest".to_string(),
                language_model: "papluca/xlm-roberta-base-language-detection".to_string(),
                default_language: "en".to_string(),
                supported_languages: vec!["en".to_string(), "es".to_string(), "fr".to_string(), "de".to_string()],
            },
            maple_ai: MapleAIConfig {
                enabled: false,
                server_endpoint: "http://localhost:8003".to_string(),
                agent_capabilities: vec![
                    "design".to_string(),
                    "physics".to_string(),
                    "social".to_string(),
                    "narrative".to_string(),
                ],
                consensus_timeout: 5,
                max_concurrent_sessions: 100,
            },
        }
    }
}

impl Default for MutseaConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            cache: CacheConfig::default(),
            network: NetworkConfig {
                lludp: LLUDPConfig::default(),
                http: HTTPConfig::default(),
                rate_limiting: RateLimitingConfig::default(),
            },
            logging: LoggingConfig::default(),
            security: SecurityConfig::default(),
            assets: AssetConfig::default(),
            opensim: OpenSimConfig::default(),
            ai: AIConfig::default(),
            custom: HashMap::new(),
        }
    }
}

impl MutseaConfig {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: MutseaConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to a file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();
        
        // Override with environment variables
        if let Ok(val) = std::env::var("MUTSEA_SERVER_PORT") {
            config.server.port = val.parse()?;
        }
        
        if let Ok(val) = std::env::var("MUTSEA_DATABASE_URL") {
            config.database.url = val;
        }
        
        if let Ok(val) = std::env::var("MUTSEA_REDIS_URL") {
            config.cache.redis_url = Some(val);
        }
        
        // Add more environment variable overrides as needed
        
        Ok(config)
    }
    
    /// Merge with another configuration
    pub fn merge(&mut self, other: MutseaConfig) {
        // Merge custom values
        for (key, value) in other.custom {
            self.custom.insert(key, value);
        }
        
        // Override specific values if needed
        if other.server.port != ServerConfig::default().port {
            self.server.port = other.server.port;
        }
        
        // Add more selective merging logic as needed
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validate server configuration
        if self.server.port == 0 {
            errors.push("Server port must be greater than 0".to_string());
        }
        
        if self.server.max_connections == 0 {
            errors.push("Max connections must be greater than 0".to_string());
        }
        
        // Validate database configuration
        if self.database.url.is_empty() {
            errors.push("Database URL is required".to_string());
        }
        
        // Validate security configuration
        if self.security.enable_auth && self.security.jwt_secret == "change-me-in-production" {
            errors.push("JWT secret must be changed in production".to_string());
        }
        
        // Validate asset configuration
        match self.assets.backend.as_str() {
            "local" => {
                if self.assets.local_path.is_none() {
                    errors.push("Local path is required for local asset backend".to_string());
                }
            }
            "s3" => {
                if self.assets.s3.is_none() {
                    errors.push("S3 configuration is required for S3 asset backend".to_string());
                }
            }
            "azure" => {
                if self.assets.azure.is_none() {
                    errors.push("Azure configuration is required for Azure asset backend".to_string());
                }
            }
            "gcp" => {
                if self.assets.gcp.is_none() {
                    errors.push("GCP configuration is required for GCP asset backend".to_string());
                }
            }
            _ => {
                errors.push(format!("Unknown asset backend: {}", self.assets.backend));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = MutseaConfig::default();
        assert_eq!(config.server.port, 9000);
        assert_eq!(config.server.name, "Mutsea Server");
        assert!(config.opensim.enabled);
        assert!(!config.ai.enabled);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = MutseaConfig::default();
        assert!(config.validate().is_ok());
        
        config.server.port = 0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_merge() {
        let mut config1 = MutseaConfig::default();
        let mut config2 = MutseaConfig::default();
        
        config2.server.port = 8080;
        config2.custom.insert("test_key".to_string(), serde_json::Value::String("test_value".to_string()));
        
        config1.merge(config2);
        
        assert_eq!(config1.server.port, 8080);
        assert_eq!(config1.custom.get("test_key").unwrap(), &serde_json::Value::String("test_value".to_string()));
    }
}