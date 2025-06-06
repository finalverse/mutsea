//! mutsea-database/src/lib.rs
//! High-performance database layer with migrations, connection pooling, and multi-backend support

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod backends;
pub mod connection;
pub mod migrations;
pub mod models;
pub mod queries;
pub mod error;
pub mod manager;
pub mod metrics;

// Re-export commonly used types
pub use backends::*;
pub use connection::*;
pub use error::*;
pub use manager::*;
pub use metrics::*;
pub use models::*;
pub use queries::*;

use async_trait::async_trait;
use mutsea_core::{
    Service, ServiceHealth, ServiceStatus, MutseaResult,
    config::DatabaseConfig,
};
use std::sync::Arc;
use tracing::info;

/// Database service implementation that wraps the manager
pub struct DatabaseService {
    manager: Arc<DatabaseManager>,
}

impl DatabaseService {
    /// Create a new database service
    pub async fn new(config: DatabaseConfig) -> DatabaseResult<Self> {
        let manager = Arc::new(DatabaseManager::new(config).await?);
        
        Ok(Self {
            manager,
        })
    }

    /// Get reference to the database manager
    pub fn manager(&self) -> &Arc<DatabaseManager> {
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

    /// Get database metrics
    pub async fn get_metrics(&self) -> DatabaseMetrics {
        self.manager.get_metrics().await
    }

    /// Get database backend type
    pub fn backend_type(&self) -> DatabaseBackend {
        self.manager.backend_type()
    }

    /// Check database health
    pub async fn health_check(&self) -> DatabaseResult<bool> {
        self.manager.health_check().await
    }
}

#[async_trait]
impl Service for DatabaseService {
    async fn start(&self) -> MutseaResult<()> {
        info!("Starting database service...");
        self.manager.start().await
            .map_err(|e| mutsea_core::MutseaError::Database(e.to_string()))
    }

    async fn stop(&self) -> MutseaResult<()> {
        info!("Stopping database service...");
        self.manager.stop().await
            .map_err(|e| mutsea_core::MutseaError::Database(e.to_string()))
    }

    fn is_running(&self) -> bool {
        self.manager.is_running()
    }

    async fn health_check(&self) -> ServiceHealth {
        let status = if self.manager.is_running() {
            match self.manager.health_check().await {
                Ok(true) => ServiceStatus::Healthy,
                Ok(false) => ServiceStatus::Degraded,
                Err(_) => ServiceStatus::Unhealthy,
            }
        } else {
            ServiceStatus::Unhealthy
        };

        let mut metrics = std::collections::HashMap::new();
        if let Ok(db_metrics) = self.manager.try_get_metrics().await {
            metrics.insert("total_queries".to_string(), db_metrics.total_queries as f64);
            metrics.insert("successful_queries".to_string(), db_metrics.successful_queries as f64);
            metrics.insert("failed_queries".to_string(), db_metrics.failed_queries as f64);
            metrics.insert("avg_query_time_ms".to_string(), db_metrics.avg_query_time_ms);
            metrics.insert("active_connections".to_string(), db_metrics.active_connections as f64);
            metrics.insert("max_connections".to_string(), db_metrics.max_connections as f64);
        }

        ServiceHealth {
            status,
            message: format!("Database service ({})", self.manager.backend_type().as_str()),
            metrics,
        }
    }
}

impl Clone for DatabaseService {
    fn clone(&self) -> Self {
        Self {
            manager: Arc::clone(&self.manager),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_service_creation() {
        let config = DatabaseConfig {
            url: "sqlite://test.db".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: 30,
            query_timeout: 60,
            auto_migrate: false,
            log_queries: false,
        };

        let service = DatabaseService::new(config).await;
        assert!(service.is_ok());
    }
}