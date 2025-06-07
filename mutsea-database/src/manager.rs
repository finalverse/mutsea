//! mutsea-database/src/manager.rs
//! Main database manager implementation

use crate::{
    backends::{DatabaseBackend, DatabasePool},
    error::{DatabaseError, DatabaseResult},
    metrics::DatabaseMetrics,
    models::*,
    queries::*,
};
use mutsea_core::{
    config::DatabaseConfig,
    MutseaResult,
    UserId, UserAccount, AssetId, Asset, RegionId, RegionInfo
};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, error, debug};

/// Main database manager implementation
pub struct DatabaseManager {
    pool: DatabasePool,
    backend: DatabaseBackend,
    config: DatabaseConfig,
    running: Arc<std::sync::atomic::AtomicBool>,
    metrics: Arc<RwLock<DatabaseMetrics>>,
    user_queries: UserQueries,
    asset_queries: AssetQueries,
    region_queries: RegionQueries,
}

impl DatabaseManager {
    /// Create a new database manager
    pub async fn new(config: DatabaseConfig) -> DatabaseResult<Self> {
        let backend = DatabaseBackend::detect(&config.url)?;
        let pool = DatabasePool::create(&config, backend).await?;
        
        info!("Database manager initialized with {} backend", backend.as_str());
        
        let user_queries = UserQueries::new(backend);
        let asset_queries = AssetQueries::new(backend);
        let region_queries = RegionQueries::new(backend);
        
        Ok(Self {
            pool,
            backend,
            config,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            metrics: Arc::new(RwLock::new(DatabaseMetrics::default())),
            user_queries,
            asset_queries,
            region_queries,
        })
    }

    /// Start the database manager
    pub async fn start(&self) -> MutseaResult<()> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        if self.config.auto_migrate {
            self.migrate().await
                .map_err(|e| mutsea_core::MutseaError::Database(e.to_string()))?;
        }
        
        info!("Database manager started successfully");
        Ok(())
    }

    /// Stop the database manager
    pub async fn stop(&self) -> MutseaResult<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        info!("Database manager stopped");
        Ok(())
    }

    /// Check if database manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Get database backend type
    pub fn backend_type(&self) -> DatabaseBackend {
        self.backend
    }

    /// Run database migrations
    pub async fn migrate(&self) -> DatabaseResult<()> {
        info!("Running database migrations for {} backend", self.backend.as_str());
        self.pool.migrate().await?;
        info!("Database migrations completed successfully");
        Ok(())
    }

    /// Initialize AI-specific database schema
    pub async fn initialize_ai_schema(&self) -> DatabaseResult<()> {
        info!("Initializing AI database schema for {} backend", self.backend.as_str());

        if self.backend != DatabaseBackend::PostgreSQL {
            return Err(DatabaseError::UnsupportedBackend(self.backend.as_str().to_string()));
        }

        let sql_files = [
            include_str!("../migrations/postgresql/ai/ai_decisions.sql"),
            include_str!("../migrations/postgresql/ai/ai_global_mind_state.sql"),
            include_str!("../migrations/postgresql/ai/emergent_behaviors.sql"),
            include_str!("../migrations/postgresql/ai/learning_data.sql"),
            include_str!("../migrations/postgresql/ai/npc_states.sql"),
        ];

        for sql in sql_files.iter() {
            self.pool.execute_raw(sql).await?;
        }

        info!("AI schema initialization completed");
        Ok(())
    }

    /// Get database metrics
    pub async fn get_metrics(&self) -> DatabaseMetrics {
        let mut metrics = self.metrics.read().await.clone();
        
        // Update connection pool metrics
        self.pool.update_metrics(&mut metrics).await;
        
        metrics
    }

    /// Try to get database metrics (non-blocking)
    pub async fn try_get_metrics(&self) -> DatabaseResult<DatabaseMetrics> {
        let metrics = self.metrics.try_read()
            .map_err(|_| DatabaseError::Generic("Failed to acquire metrics lock".to_string()))?
            .clone();
        Ok(metrics)
    }

    /// Update metrics after query execution
    async fn update_metrics(&self, success: bool, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.total_queries += 1;
        
        if success {
            metrics.successful_queries += 1;
        } else {
            metrics.failed_queries += 1;
        }

        // Update average query time (exponential moving average)
        let duration_ms = duration.as_millis() as f64;
        if metrics.total_queries == 1 {
            metrics.avg_query_time_ms = duration_ms;
        } else {
            metrics.avg_query_time_ms = (metrics.avg_query_time_ms * 0.9) + (duration_ms * 0.1);
        }
    }

    /// Execute a query with metrics tracking
    async fn execute_with_metrics<F, T>(&self, operation: F) -> DatabaseResult<T>
    where
        F: std::future::Future<Output = DatabaseResult<T>>,
    {
        let start = std::time::Instant::now();
        let result = operation.await;
        let duration = start.elapsed();
        
        self.update_metrics(result.is_ok(), duration).await;
        result
    }

    // === USER MANAGEMENT ===

    /// Create a new user
    pub async fn create_user(&self, account: &UserAccount) -> DatabaseResult<()> {
        self.execute_with_metrics(async {
            self.user_queries.create(&self.pool, account).await
        }).await
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: UserId) -> DatabaseResult<Option<UserAccount>> {
        self.execute_with_metrics(async {
            self.user_queries.get_by_id(&self.pool, user_id).await
        }).await
    }

    /// Find user by name
    pub async fn find_user_by_name(&self, first_name: &str, last_name: &str) -> DatabaseResult<Option<UserAccount>> {
        self.execute_with_metrics(async {
            self.user_queries.find_by_name(&self.pool, first_name, last_name).await
        }).await
    }

    /// Update user
    pub async fn update_user(&self, account: &UserAccount) -> DatabaseResult<()> {
        self.execute_with_metrics(async {
            self.user_queries.update(&self.pool, account).await
        }).await
    }

    /// Delete user
    pub async fn delete_user(&self, user_id: UserId) -> DatabaseResult<()> {
        self.execute_with_metrics(async {
            self.user_queries.delete(&self.pool, user_id).await
        }).await
    }

    /// List all users (with pagination)
    pub async fn list_users(&self, limit: i64, offset: i64) -> DatabaseResult<Vec<UserAccount>> {
        self.execute_with_metrics(async {
            self.user_queries.list(&self.pool, limit, offset).await
        }).await
    }

    /// Get user count
    pub async fn get_user_count(&self) -> DatabaseResult<i64> {
        self.execute_with_metrics(async {
            self.user_queries.count(&self.pool).await
        }).await
    }

    // === ASSET MANAGEMENT ===

    /// Store asset metadata
    pub async fn store_asset_metadata(&self, asset: &Asset) -> DatabaseResult<()> {
        self.execute_with_metrics(async {
            self.asset_queries.create(&self.pool, asset).await
        }).await
    }

    /// Get asset metadata by ID
    pub async fn get_asset_metadata(&self, asset_id: AssetId) -> DatabaseResult<Option<AssetMetadata>> {
        self.execute_with_metrics(async {
            self.asset_queries.get_metadata(&self.pool, asset_id).await
        }).await
    }

    /// Check if asset exists
    pub async fn asset_exists(&self, asset_id: AssetId) -> DatabaseResult<bool> {
        self.execute_with_metrics(async {
            self.asset_queries.exists(&self.pool, asset_id).await
        }).await
    }

    /// Delete asset metadata
    pub async fn delete_asset_metadata(&self, asset_id: AssetId) -> DatabaseResult<()> {
        self.execute_with_metrics(async {
            self.asset_queries.delete(&self.pool, asset_id).await
        }).await
    }

    /// List assets by creator
    pub async fn list_assets_by_creator(&self, creator_id: UserId, limit: i64, offset: i64) -> DatabaseResult<Vec<AssetMetadata>> {
        self.execute_with_metrics(async {
            self.asset_queries.list_by_creator(&self.pool, creator_id, limit, offset).await
        }).await
    }

    // === REGION MANAGEMENT ===

    /// Register a region
    pub async fn register_region(&self, region: &RegionInfo) -> DatabaseResult<()> {
        self.execute_with_metrics(async {
            self.region_queries.create(&self.pool, region).await
        }).await
    }

    /// Get region by ID
    pub async fn get_region(&self, region_id: RegionId) -> DatabaseResult<Option<RegionInfo>> {
        self.execute_with_metrics(async {
            self.region_queries.get_by_id(&self.pool, region_id).await
        }).await
    }

    /// Find region by name
    pub async fn find_region_by_name(&self, name: &str) -> DatabaseResult<Option<RegionInfo>> {
        self.execute_with_metrics(async {
            self.region_queries.find_by_name(&self.pool, name).await
        }).await
    }

    /// Update region
    pub async fn update_region(&self, region: &RegionInfo) -> DatabaseResult<()> {
        self.execute_with_metrics(async {
            self.region_queries.update(&self.pool, region).await
        }).await
    }

    /// Delete region
    pub async fn delete_region(&self, region_id: RegionId) -> DatabaseResult<()> {
        self.execute_with_metrics(async {
            self.region_queries.delete(&self.pool, region_id).await
        }).await
    }

    /// List all regions
    pub async fn list_regions(&self) -> DatabaseResult<Vec<RegionInfo>> {
        self.execute_with_metrics(async {
            self.region_queries.list_all(&self.pool).await
        }).await
    }

    /// Get regions by location range
    pub async fn get_regions_by_location(&self, x_min: u32, y_min: u32, x_max: u32, y_max: u32) -> DatabaseResult<Vec<RegionInfo>> {
        self.execute_with_metrics(async {
            self.region_queries.get_by_location(&self.pool, x_min, y_min, x_max, y_max).await
        }).await
    }

    /// Health check for database connectivity
    pub async fn health_check(&self) -> DatabaseResult<bool> {
        self.pool.health_check().await
    }

    /// Get connection pool statistics
    pub async fn get_pool_stats(&self) -> DatabaseResult<PoolStats> {
        self.pool.get_stats().await
    }

    /// Execute raw SQL query (for advanced use cases)
    pub async fn execute_raw(&self, query: &str) -> DatabaseResult<u64> {
        self.execute_with_metrics(async {
            self.pool.execute_raw(query).await
        }).await
    }

    /// Begin a transaction
    pub async fn begin_transaction(&self) -> DatabaseResult<DatabaseTransaction> {
        self.pool.begin_transaction().await
    }
}

/// Database transaction wrapper
pub struct DatabaseTransaction {
    inner: Box<dyn crate::connection::Transaction + Send>,
}

impl DatabaseTransaction {
    /// Create new transaction wrapper
    pub(crate) fn new(transaction: Box<dyn crate::connection::Transaction + Send>) -> Self {
        Self {
            inner: transaction,
        }
    }

    /// Execute query within transaction
    pub async fn execute(&mut self, query: &str, params: &[&dyn std::fmt::Debug]) -> DatabaseResult<u64> {
        self.inner.execute(query, params).await
    }

    /// Commit the transaction
    pub async fn commit(self) -> DatabaseResult<()> {
        self.inner.commit().await
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> DatabaseResult<()> {
        self.inner.rollback().await
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub active_connections: u32,
    pub idle_connections: u32,
    pub max_connections: u32,
    pub total_connections: u64,
    pub failed_connections: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mutsea_core::config::DatabaseConfig;

    async fn create_test_manager() -> DatabaseManager {
        let config = DatabaseConfig {
            url: "sqlite::memory:".to_string(),
            max_connections: 5,
            min_connections: 1,
            connect_timeout: 30,
            query_timeout: 60,
            auto_migrate: true,
            log_queries: false,
        };

        DatabaseManager::new(config).await.unwrap()
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = create_test_manager().await;
        assert_eq!(manager.backend_type(), DatabaseBackend::SQLite);
        assert!(!manager.is_running());
    }

    #[tokio::test]
    async fn test_manager_start_stop() {
        let manager = create_test_manager().await;
        
        manager.start().await.unwrap();
        assert!(manager.is_running());
        
        manager.stop().await.unwrap();
        assert!(!manager.is_running());
    }

    #[tokio::test]
    async fn test_health_check() {
        let manager = create_test_manager().await;
        manager.start().await.unwrap();
        
        let health = manager.health_check().await.unwrap();
        assert!(health);
    }
}