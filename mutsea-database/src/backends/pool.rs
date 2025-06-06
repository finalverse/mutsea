//! mutsea-database/src/backends/pool.rs
//! Database connection pool wrapper

use crate::{
    error::{DatabaseError, DatabaseResult},
    connection::Transaction,
    manager::PoolStats,
    backends::{DatabaseBackend, PostgreSQLTransaction, MySQLTransaction, SQLiteTransaction},
};
use mutsea_core::config::DatabaseConfig;
use sqlx::{
    postgres::PgPoolOptions,
    mysql::MySqlPoolOptions,
    sqlite::SqlitePoolOptions,
    Pool, Postgres, MySql, Sqlite, Executor,
};
use std::time::Duration;
use tracing::{info, debug, warn};

/// Database connection pool wrapper
#[derive(Debug, Clone)]
pub enum DatabasePool {
    PostgreSQL(Pool<Postgres>),
    MySQL(Pool<MySql>),
    SQLite(Pool<Sqlite>),
}

impl DatabasePool {
    /// Create connection pool for the specified backend
    pub async fn create(config: &DatabaseConfig, backend: DatabaseBackend) -> DatabaseResult<Self> {
        match backend {
            DatabaseBackend::PostgreSQL => {
                Self::create_postgresql_pool(config).await
            }
            DatabaseBackend::MySQL => {
                Self::create_mysql_pool(config).await
            }
            DatabaseBackend::SQLite => {
                Self::create_sqlite_pool(config).await
            }
        }
    }

    /// Create PostgreSQL connection pool
    async fn create_postgresql_pool(config: &DatabaseConfig) -> DatabaseResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.connect_timeout))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(3600))
            .test_before_acquire(true)
            .connect(&config.url)
            .await?;
        
        info!("PostgreSQL connection pool created: {}-{} connections",
              config.min_connections, config.max_connections);
        
        // Test the connection
        sqlx::query("SELECT 1").execute(&pool).await?;
        debug!("PostgreSQL connection pool test successful");
        
        Ok(DatabasePool::PostgreSQL(pool))
    }

    /// Create MySQL connection pool
    async fn create_mysql_pool(config: &DatabaseConfig) -> DatabaseResult<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.connect_timeout))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(3600))
            .test_before_acquire(true)
            .connect(&config.url)
            .await?;
        
        info!("MySQL connection pool created: {}-{} connections",
              config.min_connections, config.max_connections);
        
        // Test the connection
        sqlx::query("SELECT 1").execute(&pool).await?;
        debug!("MySQL connection pool test successful");
        
        Ok(DatabasePool::MySQL(pool))
    }

    /// Create SQLite connection pool
    async fn create_sqlite_pool(config: &DatabaseConfig) -> DatabaseResult<Self> {
        // For SQLite, limit connections to prevent lock contention
        let max_connections = std::cmp::min(config.max_connections, 5);
        let min_connections = std::cmp::min(config.min_connections, 1);
        
        if max_connections != config.max_connections {
            warn!("SQLite max connections limited to {} (requested: {})", 
                  max_connections, config.max_connections);
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .acquire_timeout(Duration::from_secs(config.connect_timeout))
            .test_before_acquire(true)
            .connect(&config.url)
            .await?;
        
        info!("SQLite connection pool created: {}-{} connections",
              min_connections, max_connections);
        
        // Test the connection and enable WAL mode for better concurrency
        sqlx::query("PRAGMA journal_mode = WAL").execute(&pool).await?;
        sqlx::query("PRAGMA synchronous = NORMAL").execute(&pool).await?;
        sqlx::query("PRAGMA cache_size = 1000").execute(&pool).await?;
        sqlx::query("PRAGMA temp_store = memory").execute(&pool).await?;
        sqlx::query("PRAGMA mmap_size = 268435456").execute(&pool).await?; // 256MB
        debug!("SQLite connection pool configured with optimized settings");
        
        Ok(DatabasePool::SQLite(pool))
    }

    /// Run database migrations
    pub async fn migrate(&self) -> DatabaseResult<()> {
        match self {
            DatabasePool::PostgreSQL(pool) => {
                sqlx::migrate!("./migrations/postgresql").run(pool).await?;
            }
            DatabasePool::MySQL(pool) => {
                sqlx::migrate!("./migrations/mysql").run(pool).await?;
            }
            DatabasePool::SQLite(pool) => {
                sqlx::migrate!("./migrations/sqlite").run(pool).await?;
            }
        }
        Ok(())
    }

    /// Execute raw SQL query
    pub async fn execute_raw(&self, query: &str) -> DatabaseResult<u64> {
        debug!("Executing raw query: {}", query);
        
        let rows_affected = match self {
            DatabasePool::PostgreSQL(pool) => {
                sqlx::query(query).execute(pool).await?.rows_affected()
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query(query).execute(pool).await?.rows_affected()
            }
            DatabasePool::SQLite(pool) => {
                sqlx::query(query).execute(pool).await?.rows_affected()
            }
        };
        
        debug!("Query affected {} rows", rows_affected);
        Ok(rows_affected)
    }

    /// Health check query
    pub async fn health_check(&self) -> DatabaseResult<bool> {
        debug!("Performing database health check");
        
        match self {
            DatabasePool::PostgreSQL(pool) => {
                sqlx::query!("SELECT 1 as test")
                    .fetch_one(pool)
                    .await?;
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query!("SELECT 1 as test")
                    .fetch_one(pool)
                    .await?;
            }
            DatabasePool::SQLite(pool) => {
                sqlx::query!("SELECT 1 as test")
                    .fetch_one(pool)
                    .await?;
            }
        }
        
        debug!("Database health check successful");
        Ok(true)
    }

    /// Get pool statistics
    pub async fn get_stats(&self) -> DatabaseResult<PoolStats> {
        let (active, idle, max) = match self {
            DatabasePool::PostgreSQL(pool) => {
                (pool.size(), pool.num_idle(), pool.options().get_max_connections())
            }
            DatabasePool::MySQL(pool) => {
                (pool.size(), pool.num_idle(), pool.options().get_max_connections())
            }
            DatabasePool::SQLite(pool) => {
                (pool.size(), pool.num_idle(), pool.options().get_max_connections())
            }
        };

        Ok(PoolStats {
            active_connections: active,
            idle_connections: idle,
            max_connections: max,
            total_connections: 0, // Would need to track this separately
            failed_connections: 0, // Would need to track this separately
        })
    }

    /// Update metrics with current pool state
    pub async fn update_metrics(&self, metrics: &mut crate::metrics::DatabaseMetrics) {
        if let Ok(stats) = self.get_stats().await {
            metrics.active_connections = stats.active_connections;
            metrics.max_connections = stats.max_connections;
        }
    }

    /// Begin a transaction
    pub async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction + Send>> {
        debug!("Beginning database transaction");
        
        match self {
            DatabasePool::PostgreSQL(pool) => {
                let tx = pool.begin().await?;
                Ok(Box::new(PostgreSQLTransaction::new(tx)))
            }
            DatabasePool::MySQL(pool) => {
                let tx = pool.begin().await?;
                Ok(Box::new(MySQLTransaction::new(tx)))
            }
            DatabasePool::SQLite(pool) => {
                let tx = pool.begin().await?;
                Ok(Box::new(SQLiteTransaction::new(tx)))
            }
        }
    }

    /// Get backend type
    pub fn backend_type(&self) -> DatabaseBackend {
        match self {
            DatabasePool::PostgreSQL(_) => DatabaseBackend::PostgreSQL,
            DatabasePool::MySQL(_) => DatabaseBackend::MySQL,
            DatabasePool::SQLite(_) => DatabaseBackend::SQLite,
        }
    }

    /// Close the connection pool
    pub async fn close(&self) {
        info!("Closing database connection pool");
        
        match self {
            DatabasePool::PostgreSQL(pool) => {
                pool.close().await;
            }
            DatabasePool::MySQL(pool) => {
                pool.close().await;
            }
            DatabasePool::SQLite(pool) => {
                pool.close().await;
            }
        }
        
        debug!("Database connection pool closed");
    }

    /// Get connection pool utilization percentage
    pub async fn get_utilization(&self) -> f64 {
        if let Ok(stats) = self.get_stats().await {
            if stats.max_connections > 0 {
                (stats.active_connections as f64 / stats.max_connections as f64) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Check if pool is at capacity
    pub async fn is_at_capacity(&self) -> bool {
        if let Ok(stats) = self.get_stats().await {
            stats.active_connections >= stats.max_connections
        } else {
            false
        }
    }
}