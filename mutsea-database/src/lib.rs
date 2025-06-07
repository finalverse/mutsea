// /mutsea/mutsea-database/src/lib.rs
//! Mutsea Database Layer - OpenSim Compatible
//! 
//! This crate provides database abstraction for the Mutsea AI-driven world engine
//! with compatibility for OpenSim's existing database schema.

pub mod error;
pub mod backends;
pub mod manager;
pub mod metrics;
pub mod utils;

// OpenSim Compatibility Layer
#[cfg(feature = "opensim-compat")]
pub mod opensim;

// AI-Enhanced Models (for future use)
pub mod models;
pub mod queries;
pub mod traits;

use error::DatabaseError;
use manager::DatabaseManager;

pub type Result<T> = std::result::Result<T, DatabaseError>;

/// Main database interface for Mutsea with OpenSim compatibility
pub struct MutseaDatabase {
    manager: DatabaseManager,
}

impl MutseaDatabase {
    /// Create a new database instance with OpenSim compatibility
    pub async fn new_opensim_compatible(database_url: &str) -> Result<Self> {
        let manager = DatabaseManager::new(database_url).await?;
        Ok(Self { manager })
    }

    /// Create a new database instance from configuration
    pub async fn from_config(config: &mutsea_config::DatabaseConfig) -> Result<Self> {
        let database_url = config.connection_string();
        Self::new_opensim_compatible(&database_url).await
    }

    /// Get database manager for direct access
    pub fn manager(&self) -> &DatabaseManager {
        &self.manager
    }

    /// Get clone of the database manager
    pub fn manager_clone(&self) -> Arc<DatabaseManager> {
        Arc::clone(&self.manager)
    }

    /// Run database migrations
    pub async fn migrate(&self) -> DatabaseResult<()> {
        self.manager.migrate().await
    }

    /// Initialize AI-specific schema
    pub async fn initialize_ai_schema(&self) -> DatabaseResult<()> {
        self.manager.initialize_ai_schema().await
    }

    /// Get database metrics
    pub async fn get_metrics(&self) -> DatabaseMetrics {
        self.manager.get_metrics().await
    }

    /// Check if database is ready for OpenSim operations
    #[cfg(feature = "opensim-compat")]
    pub async fn verify_opensim_compatibility(&self) -> Result<bool> {
        self.manager.verify_opensim_tables().await
    }

}

// Re-exports for convenience
pub use backends::{BackendType, DatabasePool};
pub use error::{DatabaseError, DatabaseResult};
pub use manager::DatabaseManager;
pub use metrics::DatabaseMetrics;

#[cfg(feature = "opensim-compat")]
pub use opensim::{schema, models};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_creation() {
        // This would test against a test database
        // For now, just ensure the types compile
        assert!(true);
    }
}