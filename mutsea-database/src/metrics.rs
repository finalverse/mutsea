//! Database metrics data structures

/// Runtime metrics collected for database operations.
#[derive(Debug, Clone, Default)]
pub struct DatabaseMetrics {
    /// Total number of queries executed through the manager
    pub total_queries: u64,
    /// Number of queries that completed successfully
    pub successful_queries: u64,
    /// Number of queries that resulted in an error
    pub failed_queries: u64,
    /// Average query execution time in milliseconds
    pub avg_query_time_ms: f64,
    /// Currently active connections in the pool
    pub active_connections: u32,
    /// Maximum number of connections allowed in the pool
    pub max_connections: u32,
}

