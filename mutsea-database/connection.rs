//! mutsea-database/src/connection.rs
//! Database connection abstraction and transaction support

use crate::error::{DatabaseError, DatabaseResult};
use async_trait::async_trait;

/// Database transaction trait
#[async_trait]
pub trait Transaction: Send {
    /// Execute a query within the transaction
    async fn execute(&mut self, query: &str, params: &[&dyn std::fmt::Debug]) -> DatabaseResult<u64>;

    /// Commit the transaction
    async fn commit(self: Box<Self>) -> DatabaseResult<()>;

    /// Rollback the transaction
    async fn rollback(self: Box<Self>) -> DatabaseResult<()>;
}

/// Connection pool abstraction
#[async_trait]
pub trait ConnectionPool: Send + Sync {
    /// Execute a query
    async fn execute(&self, query: &str) -> DatabaseResult<u64>;

    /// Begin a transaction
    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction + Send>>;

    /// Health check
    async fn health_check(&self) -> DatabaseResult<bool>;

    /// Get pool statistics
    async fn get_pool_stats(&self) -> DatabaseResult<PoolStats>;

    /// Close the pool
    async fn close(&self);
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