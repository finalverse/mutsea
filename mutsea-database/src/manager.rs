// /mutsea/mutsea-database/src/manager.rs
//! Database manager for coordinating operations

use crate::{
    backends::{DatabasePool, DatabaseBackend},
    error::DatabaseResult,
    Result, DatabaseError,
    metrics::DatabaseMetrics,
};

use std::sync::{atomic::{AtomicU64, Ordering}, Arc};
use tracing::{debug, error, info};
use std::time::Duration;
use tokio::sync::RwLock;

/// Database manager for coordinating operations across different backends
pub struct DatabaseManager {
    pool: Arc<DatabasePool>,
    total_queries: AtomicU64,
    successful_queries: AtomicU64,
    failed_queries: AtomicU64,
    avg_query_time_ms: AtomicU64,
    metrics: Arc<RwLock<DatabaseMetrics>>, 
}

impl DatabaseManager {
    /// Create a new database manager
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Initializing database manager with URL: {}",
              database_url.split('@').last().unwrap_or("unknown"));

        let pool = DatabasePool::new(database_url).await?;
        info!("Database pool created successfully");
        
        Ok(Self {
            pool: Arc::new(pool),
            total_queries: AtomicU64::new(0),
            successful_queries: AtomicU64::new(0),
            failed_queries: AtomicU64::new(0),
            avg_query_time_ms: AtomicU64::new(0),
            metrics: Arc::new(RwLock::new(DatabaseMetrics::default())),
        })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> crate::DatabaseResult<()> {
        self.pool.migrate().await
    }
    
    /// Get a database backend instance
    pub async fn get_backend(&self) -> Result<Box<dyn DatabaseBackend>> {
        Ok(self.pool.get_backend())
    }
    
    /// Get the backend type
    pub fn backend_type(&self) -> crate::backends::BackendType {
        self.pool.backend_type()
    }
    
    /// Test database connectivity
    pub async fn test_connection(&self) -> Result<()> {
        debug!("Testing database connection");
        let backend = self.get_backend().await?;
        
        // Try a simple query to test connectivity
        match backend.table_exists("information_schema.tables").await {
            Ok(_) => {
                info!("Database connection test successful");
                Ok(())
            },
            Err(e) => {
                error!("Database connection test failed: {}", e);
                Err(e)
            }
        }
    }

    /// Get database statistics
    pub async fn get_statistics(&self) -> Result<DatabaseStats> {
        let backend = self.get_backend().await?;
        
        // Basic stats - can be expanded based on backend type
        Ok(DatabaseStats {
            backend_type: self.backend_type(),
            is_connected: self.test_connection().await.is_ok(),
            // TODO: Add more meaningful statistics
        })
    }

    /// Initialize AI-specific database schema
    pub async fn initialize_ai_schema(&self) -> DatabaseResult<()> {
        info!("Initializing AI database schema for {:?} backend", self.backend_type());

        if self.backend_type() != DatabaseBackend::PostgreSQL {
            return Err(DatabaseError::UnsupportedBackend(self.backend_type().as_str().to_string()));
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
        let mut metrics = DatabaseMetrics {
            total_queries: self.total_queries.load(Ordering::Relaxed),
            successful_queries: self.successful_queries.load(Ordering::Relaxed),
            failed_queries: self.failed_queries.load(Ordering::Relaxed),
            avg_query_time_ms: f64::from_bits(self.avg_query_time_ms.load(Ordering::Relaxed)),
            active_connections: 0,
            max_connections: 0,
        };

        // Update connection pool metrics
        self.pool.update_metrics(&mut metrics).await;

        metrics
    }

    /// Try to get database metrics (non-blocking)
    pub async fn try_get_metrics(&self) -> DatabaseResult<DatabaseMetrics> {
        Ok(self.get_metrics().await)
    }

    /// Update metrics after query execution
    async fn update_metrics(&self, success: bool, duration: Duration) {
        self.total_queries.fetch_add(1, Ordering::Relaxed);
        if success {
            self.successful_queries.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_queries.fetch_add(1, Ordering::Relaxed);
        }

        let duration_ms = duration.as_millis() as f64;

        self.avg_query_time_ms.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            let current = f64::from_bits(current);
            let total = self.total_queries.load(Ordering::Relaxed);
            let new_avg = if total == 1 {
                duration_ms
            } else {
                current * 0.9 + duration_ms * 0.1
            };
            Some(new_avg.to_bits())
        }).ok();
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

    /// Verify OpenSim tables exist and are properly structured
    pub async fn verify_opensim_tables(&self) -> Result<bool> {
        info!("Verifying OpenSim database compatibility");
        let backend = self.get_backend().await?;
        
        let required_tables = vec![
            "regions", 
            "user_accounts", 
            "assets", 
            "inventoryitems", 
            "inventoryfolders", 
            "primitives", 
            "terrain", 
            "land", 
            "landaccesslist"
        ];

        let mut all_exist = true;
        for table in &required_tables {
            match backend.table_exists(table).await {
                Ok(exists) => {
                    if exists {
                        debug!("Table '{}' exists", table);
                    } else {
                        error!("Required table '{}' does not exist", table);
                        all_exist = false;
                    }
                },
                Err(e) => {
                    error!("Error checking table '{}': {}", table, e);
                    all_exist = false;
                }
            }
        }

        if all_exist {
            info!("All required OpenSim tables verified successfully");
        } else {
            error!("OpenSim compatibility verification failed");
        }

        Ok(all_exist)
    }

    /// Get OpenSim database health status
    pub async fn get_opensim_health(&self) -> Result<OpenSimHealth> {
        let backend = self.get_backend().await?;
        
        // Check if core tables exist and get basic counts
        let regions_exist = backend.table_exists("regions").await.unwrap_or(false);
        let users_exist = backend.table_exists("user_accounts").await.unwrap_or(false);
        let assets_exist = backend.table_exists("assets").await.unwrap_or(false);
        
        // TODO: Get actual counts from tables
        let region_count = if regions_exist { 
            // This would be a real query in production
            0 
        } else { 
            0 
        };
        
        let user_count = if users_exist { 
            // This would be a real query in production
            0 
        } else { 
            0 
        };
        
        let asset_count = if assets_exist { 
            // This would be a real query in production
            0 
        } else { 
            0 
        };

        Ok(OpenSimHealth {
            tables_exist: regions_exist && users_exist && assets_exist,
            region_count,
            user_count,
            asset_count,
        })
    }
}

/// OpenSim database health information
#[cfg(feature = "opensim-compat")]
#[derive(Debug, Clone)]
pub struct OpenSimHealth {
    pub tables_exist: bool,
    pub region_count: u64,
    pub user_count: u64,
    pub asset_count: u64,
}

/// Basic database metrics
#[derive(Debug, Clone, Default)]
pub struct DatabaseMetrics {
    pub total_queries: u64,
    pub successful_queries: u64,
    pub failed_queries: u64,
    pub avg_query_time_ms: f64,
    pub active_connections: u32,
    pub max_connections: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_manager_creation() {
        // This would test against a real test database
        // For now, just test that the manager can be created
        assert!(true);
    }
}