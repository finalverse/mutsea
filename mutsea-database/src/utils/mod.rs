// mutsea-database/src/utils/mod.rs

//! Database utilities module for Mutsea AI-driven engine
//! 
//! This module provides essential utilities for database operations including:
//! - SQL file loading and management
//! - Parameter binding for queries
//! - Result parsing and type conversion
//! - Connection pooling utilities
//! - Migration helpers
//! - Performance monitoring
//! - Caching mechanisms

pub mod sql_loader;
pub mod parameter_binding;
pub mod result_parsing;

use crate::error::{DatabaseError, DatabaseResult};
use crate::traits::query_builder::{DatabaseDialect, QueryParam};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;

// Re-export commonly used types
pub use parameter_binding::{
    ParameterBinder, ParameterSet, BindingStrategy, 
    PostgreSQLBinder, SQLiteBinder, MySQLBinder
};
pub use result_parsing::{
    DatabaseRow, DatabaseValue, ResultParser, ResultParserFactory,
    AIDecisionParser, WorldStateParser, PlayerBehaviorParser,
    NPCStateParser, EcosystemStateParser, EmergentBehaviorParser,
    PerformanceMetricsParser, LearningDataParser
};
pub use sql_loader::{
    SQLLoader, SQLQuery, SQLQuerySet, QueryMetadata,
    FileBasedSQLLoader, EmbeddedSQLLoader, CachedSQLLoader
};

/// Database connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Minimum number of connections to maintain
    pub min_connections: u32,
    /// Maximum time to wait for a connection
    pub connection_timeout: Duration,
    /// Maximum lifetime of a connection
    pub max_connection_lifetime: Duration,
    /// Idle timeout before closing unused connections
    pub idle_timeout: Duration,
    /// Test query to validate connections
    pub test_query: Option<String>,
    /// Enable connection validation
    pub validate_connections: bool,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 50,
            min_connections: 5,
            connection_timeout: Duration::from_secs(30),
            max_connection_lifetime: Duration::from_secs(3600), // 1 hour
            idle_timeout: Duration::from_secs(600), // 10 minutes
            test_query: None,
            validate_connections: true,
        }
    }
}

/// Database connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub failed_connections: u32,
    pub average_connection_time: Duration,
    pub peak_connections: u32,
    pub total_queries_executed: u64,
    pub average_query_time: Duration,
    pub last_updated: DateTime<Utc>,
}

/// Query execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStats {
    pub query_name: String,
    pub execution_count: u64,
    pub total_execution_time: Duration,
    pub average_execution_time: Duration,
    pub min_execution_time: Duration,
    pub max_execution_time: Duration,
    pub success_count: u64,
    pub error_count: u64,
    pub last_executed: DateTime<Utc>,
    pub parameters_hash: Option<String>,
}

/// Database performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub connection_stats: ConnectionStats,
    pub query_stats: HashMap<String, QueryStats>,
    pub cache_stats: CacheStats,
    pub error_stats: ErrorStats,
    pub resource_usage: ResourceUsage,
}

/// Cache performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_ratio: f64,
    pub total_entries: usize,
    pub memory_usage_bytes: usize,
    pub eviction_count: u64,
    pub average_entry_size: usize,
}

/// Error tracking statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStats {
    pub total_errors: u64,
    pub connection_errors: u64,
    pub query_errors: u64,
    pub timeout_errors: u64,
    pub type_conversion_errors: u64,
    pub constraint_violation_errors: u64,
    pub error_rate_per_minute: f64,
    pub most_common_errors: Vec<(String, u64)>,
}

/// Resource usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub memory_usage_bytes: usize,
    pub cpu_usage_percent: f64,
    pub disk_io_bytes: u64,
    pub network_io_bytes: u64,
    pub active_transactions: u32,
    pub lock_wait_time: Duration,
}

/// Database migration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationInfo {
    pub version: String,
    pub name: String,
    pub description: String,
    pub applied_at: Option<DateTime<Utc>>,
    pub checksum: String,
    pub execution_time: Option<Duration>,
    pub success: Option<bool>,
}

/// Transaction isolation levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

/// Query hint for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryHint {
    UseIndex(String),
    NoIndex(String),
    ForceOrder,
    UseNestedLoop,
    UseHashJoin,
    UseMergeJoin,
    Parallel(u32),
    MaxRows(u64),
    Timeout(Duration),
}

/// Database utility functions and helpers
pub struct DatabaseUtils;

impl DatabaseUtils {
    /// Generate a unique query identifier
    pub fn generate_query_id(sql: &str, params: &[QueryParam]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        sql.hash(&mut hasher);
        for param in params {
            param.hash(&mut hasher);
        }
        format!("query_{:x}", hasher.finish())
    }
    
    /// Calculate query complexity score
    pub fn calculate_query_complexity(sql: &str) -> u32 {
        let sql_lower = sql.to_lowercase();
        let mut complexity = 0u32;
        
        // Base complexity
        complexity += 1;
        
        // Table operations
        complexity += sql_lower.matches("select").count() as u32 * 2;
        complexity += sql_lower.matches("insert").count() as u32 * 3;
        complexity += sql_lower.matches("update").count() as u32 * 4;
        complexity += sql_lower.matches("delete").count() as u32 * 5;
        
        // Joins
        complexity += sql_lower.matches("join").count() as u32 * 5;
        complexity += sql_lower.matches("left join").count() as u32 * 3;
        complexity += sql_lower.matches("right join").count() as u32 * 3;
        complexity += sql_lower.matches("inner join").count() as u32 * 4;
        complexity += sql_lower.matches("outer join").count() as u32 * 6;
        
        // Subqueries
        complexity += sql_lower.matches("select").count().saturating_sub(1) as u32 * 10;
        
        // Aggregations
        complexity += sql_lower.matches("group by").count() as u32 * 5;
        complexity += sql_lower.matches("having").count() as u32 * 3;
        complexity += sql_lower.matches("order by").count() as u32 * 2;
        
        // Functions
        complexity += sql_lower.matches("count(").count() as u32 * 2;
        complexity += sql_lower.matches("sum(").count() as u32 * 2;
        complexity += sql_lower.matches("avg(").count() as u32 * 3;
        complexity += sql_lower.matches("max(").count() as u32 * 2;
        complexity += sql_lower.matches("min(").count() as u32 * 2;
        
        // Window functions
        complexity += sql_lower.matches("over(").count() as u32 * 8;
        complexity += sql_lower.matches("partition by").count() as u32 * 5;
        
        // CTEs
        complexity += sql_lower.matches("with").count() as u32 * 7;
        
        complexity
    }
    
    /// Validate SQL query for safety
    pub fn validate_sql_safety(sql: &str) -> DatabaseResult<()> {
        let sql_lower = sql.to_lowercase();
        
        // Check for potentially dangerous operations
        let dangerous_keywords = [
            "drop", "truncate", "alter", "create", "grant", "revoke",
            "exec", "execute", "sp_", "xp_", "--", "/*", "*/",
        ];
        
        for keyword in &dangerous_keywords {
            if sql_lower.contains(keyword) {
                return Err(DatabaseError::UnsafeQuery {
                    reason: format!("Query contains potentially dangerous keyword: {}", keyword),
                    sql: sql.to_string(),
                });
            }
        }
        
        // Check for SQL injection patterns
        let injection_patterns = [
            "'; ", "'or'1'='1", "'or 1=1", "union select", "/**/",
            "char(", "ascii(", "substring(", "waitfor delay",
        ];
        
        for pattern in &injection_patterns {
            if sql_lower.contains(pattern) {
                return Err(DatabaseError::PotentialInjection {
                    pattern: pattern.to_string(),
                    sql: sql.to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Generate optimized query plan hints
    pub fn generate_query_hints(
        sql: &str,
        dialect: DatabaseDialect,
        table_stats: Option<&HashMap<String, TableStats>>,
    ) -> Vec<QueryHint> {
        let mut hints = Vec::new();
        let sql_lower = sql.to_lowercase();
        
        // Suggest index usage based on WHERE clauses
        if let Some(stats) = table_stats {
            for (table_name, table_stat) in stats {
                if sql_lower.contains(&table_name.to_lowercase()) {
                    if table_stat.row_count > 100_000 {
                        hints.push(QueryHint::UseIndex(
                            table_stat.primary_index.clone()
                        ));
                    }
                }
            }
        }
        
        // Suggest parallelization for large queries
        if sql_lower.contains("group by") && sql_lower.contains("count(") {
            hints.push(QueryHint::Parallel(4));
        }
        
        // Suggest join strategies
        let join_count = sql_lower.matches("join").count();
        match join_count {
            0..=2 => hints.push(QueryHint::UseNestedLoop),
            3..=5 => hints.push(QueryHint::UseHashJoin),
            _ => hints.push(QueryHint::UseMergeJoin),
        }
        
        // Suggest timeouts for complex queries
        let complexity = Self::calculate_query_complexity(sql);
        if complexity > 50 {
            hints.push(QueryHint::Timeout(Duration::from_secs(300))); // 5 minutes
        }
        
        hints
    }
    
    /// Format SQL query for logging
    pub fn format_sql_for_logging(sql: &str, params: &[QueryParam]) -> String {
        let mut formatted = sql.to_string();
        
        // Replace parameter placeholders with values for logging
        for (i, param) in params.iter().enumerate() {
            let placeholder = match i {
                0..=8 => format!("${}", i + 1), // PostgreSQL style
                _ => "?".to_string(), // Generic placeholder
            };
            
            let value_str = match param {
                QueryParam::String(s) => format!("'{}'", s.replace('\'', "''")),
                QueryParam::Integer(i) => i.to_string(),
                QueryParam::Float(f) => f.to_string(),
                QueryParam::Boolean(b) => b.to_string(),
                QueryParam::Uuid(u) => format!("'{}'", u),
                QueryParam::Json(j) => format!("'{}'", j),
                QueryParam::Binary(_) => "'<binary_data>'".to_string(),
                QueryParam::Null => "NULL".to_string(),
            };
            
            formatted = formatted.replace(&placeholder, &value_str);
        }
        
        formatted
    }
    
    /// Estimate query execution time
    pub fn estimate_execution_time(
        sql: &str,
        table_stats: &HashMap<String, TableStats>,
        historical_stats: &HashMap<String, QueryStats>,
    ) -> Duration {
        let complexity = Self::calculate_query_complexity(sql);
        let query_hash = Self::generate_query_id(sql, &[]);
        
        // Use historical data if available
        if let Some(stats) = historical_stats.get(&query_hash) {
            return stats.average_execution_time;
        }
        
        // Estimate based on complexity and table sizes
        let mut base_time = Duration::from_millis(10); // Base execution time
        
        // Factor in table sizes
        let total_rows: u64 = table_stats.values()
            .map(|stats| stats.row_count)
            .sum();
        
        if total_rows > 1_000_000 {
            base_time = base_time.mul_f64(total_rows as f64 / 1_000_000.0);
        }
        
        // Factor in complexity
        base_time.mul_f32(complexity as f32 / 10.0)
    }
    
    /// Generate database health report
    pub fn generate_health_report(metrics: &DatabaseMetrics) -> HealthReport {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Analyze connection health
        let conn_utilization = metrics.connection_stats.active_connections as f64 
            / metrics.connection_stats.total_connections as f64;
        
        if conn_utilization > 0.8 {
            issues.push("High connection pool utilization".to_string());
            recommendations.push("Consider increasing max_connections".to_string());
        }
        
        // Analyze query performance
        let slow_queries: Vec<_> = metrics.query_stats
            .iter()
            .filter(|(_, stats)| stats.average_execution_time > Duration::from_secs(5))
            .collect();
        
        if !slow_queries.is_empty() {
            issues.push(format!("{} slow queries detected", slow_queries.len()));
            recommendations.push("Review and optimize slow queries".to_string());
        }
        
        // Analyze error rates
        if metrics.error_stats.error_rate_per_minute > 1.0 {
            issues.push("High error rate detected".to_string());
            recommendations.push("Investigate error causes".to_string());
        }
        
        // Analyze cache performance
        if metrics.cache_stats.hit_ratio < 0.8 {
            issues.push("Low cache hit ratio".to_string());
            recommendations.push("Review caching strategy".to_string());
        }
        
        // Overall health score
        let health_score = calculate_health_score(&metrics);
        
        HealthReport {
            overall_health: health_score,
            issues,
            recommendations,
            metrics: metrics.clone(),
            generated_at: Utc::now(),
        }
    }
}

/// Table statistics for query optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStats {
    pub table_name: String,
    pub row_count: u64,
    pub avg_row_size: usize,
    pub total_size_bytes: u64,
    pub primary_index: String,
    pub secondary_indexes: Vec<String>,
    pub last_analyzed: DateTime<Utc>,
}

/// Database health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub overall_health: f64, // 0.0 to 1.0
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub metrics: DatabaseMetrics,
    pub generated_at: DateTime<Utc>,
}

/// Query execution context
#[derive(Debug, Clone)]
pub struct QueryContext {
    pub query_id: String,
    pub session_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub isolation_level: Option<IsolationLevel>,
    pub hints: Vec<QueryHint>,
    pub timeout: Option<Duration>,
    pub trace_enabled: bool,
    pub cache_enabled: bool,
}

impl Default for QueryContext {
    fn default() -> Self {
        Self {
            query_id: Uuid::new_v4().to_string(),
            session_id: None,
            user_id: None,
            isolation_level: None,
            hints: Vec::new(),
            timeout: Some(Duration::from_secs(30)),
            trace_enabled: false,
            cache_enabled: true,
        }
    }
}

/// Query execution result with metadata
#[derive(Debug, Clone)]
pub struct ExecutionResult<T> {
    pub data: T,
    pub execution_time: Duration,
    pub rows_affected: Option<u64>,
    pub from_cache: bool,
    pub query_plan: Option<String>,
    pub warnings: Vec<String>,
}

impl<T> ExecutionResult<T> {
    pub fn new(data: T, execution_time: Duration) -> Self {
        Self {
            data,
            execution_time,
            rows_affected: None,
            from_cache: false,
            query_plan: None,
            warnings: Vec::new(),
        }
    }
    
    pub fn with_rows_affected(mut self, rows: u64) -> Self {
        self.rows_affected = Some(rows);
        self
    }
    
    pub fn with_cache_hit(mut self, from_cache: bool) -> Self {
        self.from_cache = from_cache;
        self
    }
    
    pub fn with_query_plan(mut self, plan: String) -> Self {
        self.query_plan = Some(plan);
        self
    }
    
    pub fn add_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }
}

/// Calculate overall database health score
fn calculate_health_score(metrics: &DatabaseMetrics) -> f64 {
    let mut score = 1.0;
    
    // Connection health (25% weight)
    let conn_utilization = metrics.connection_stats.active_connections as f64 
        / metrics.connection_stats.total_connections as f64;
    score *= 0.75 + 0.25 * (1.0 - conn_utilization.min(0.9));
    
    // Error rate health (25% weight)
    let error_factor = if metrics.error_stats.error_rate_per_minute > 0.0 {
        1.0 / (1.0 + metrics.error_stats.error_rate_per_minute)
    } else {
        1.0
    };
    score *= 0.75 + 0.25 * error_factor;
    
    // Cache performance health (25% weight)
    score *= 0.75 + 0.25 * metrics.cache_stats.hit_ratio;
    
    // Query performance health (25% weight)
    let avg_query_time = if metrics.query_stats.is_empty() {
        Duration::from_millis(100)
    } else {
        let total_time: Duration = metrics.query_stats
            .values()
            .map(|stats| stats.average_execution_time)
            .sum();
        total_time / metrics.query_stats.len() as u32
    };
    
    let query_performance_factor = if avg_query_time > Duration::from_secs(1) {
        0.5
    } else if avg_query_time > Duration::from_millis(500) {
        0.7
    } else {
        1.0
    };
    score *= 0.75 + 0.25 * query_performance_factor;
    
    score.max(0.0).min(1.0)
}

/// Trait for database utility extensions
pub trait DatabaseUtilsExt {
    /// Execute query with full context and monitoring
    async fn execute_with_context<T>(
        &self,
        sql: &str,
        params: &[QueryParam],
        context: QueryContext,
    ) -> DatabaseResult<ExecutionResult<T>>
    where
        T: for<'de> Deserialize<'de> + Send;
    
    /// Get current database metrics
    async fn get_metrics(&self) -> DatabaseResult<DatabaseMetrics>;
    
    /// Get health report
    async fn get_health_report(&self) -> DatabaseResult<HealthReport>;
    
    /// Optimize query performance
    async fn optimize_query(&self, sql: &str) -> DatabaseResult<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_complexity_calculation() {
        let simple_query = "SELECT * FROM users WHERE id = $1";
        let complex_query = r#"
            WITH ranked_users AS (
                SELECT u.*, ROW_NUMBER() OVER (PARTITION BY u.department ORDER BY u.salary DESC) as rank
                FROM users u
                JOIN departments d ON u.department_id = d.id
                WHERE u.active = true
            )
            SELECT * FROM ranked_users WHERE rank <= 5
            ORDER BY department, rank
        "#;
        
        let simple_complexity = DatabaseUtils::calculate_query_complexity(simple_query);
        let complex_complexity = DatabaseUtils::calculate_query_complexity(complex_query);
        
        assert!(simple_complexity < complex_complexity);
        assert!(simple_complexity > 0);
        assert!(complex_complexity > 20);
    }

    #[test]
    fn test_sql_safety_validation() {
        let safe_query = "SELECT name, email FROM users WHERE active = true";
        let unsafe_query = "SELECT * FROM users; DROP TABLE users; --";
        
        assert!(DatabaseUtils::validate_sql_safety(safe_query).is_ok());
        assert!(DatabaseUtils::validate_sql_safety(unsafe_query).is_err());
    }

    #[test]
    fn test_query_id_generation() {
        let sql = "SELECT * FROM users WHERE id = $1";
        let params = vec![QueryParam::Integer(123)];
        
        let id1 = DatabaseUtils::generate_query_id(sql, &params);
        let id2 = DatabaseUtils::generate_query_id(sql, &params);
        let id3 = DatabaseUtils::generate_query_id(sql, &[QueryParam::Integer(456)]);
        
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_health_score_calculation() {
        let metrics = DatabaseMetrics {
            connection_stats: ConnectionStats {
                total_connections: 100,
                active_connections: 50,
                idle_connections: 50,
                failed_connections: 0,
                average_connection_time: Duration::from_millis(100),
                peak_connections: 75,
                total_queries_executed: 10000,
                average_query_time: Duration::from_millis(200),
                last_updated: Utc::now(),
            },
            query_stats: HashMap::new(),
            cache_stats: CacheStats {
                hit_count: 8000,
                miss_count: 2000,
                hit_ratio: 0.8,
                total_entries: 1000,
                memory_usage_bytes: 1024 * 1024,
                eviction_count: 100,
                average_entry_size: 1024,
            },
            error_stats: ErrorStats {
                total_errors: 10,
                connection_errors: 2,
                query_errors: 5,
                timeout_errors: 2,
                type_conversion_errors: 1,
                constraint_violation_errors: 0,
                error_rate_per_minute: 0.1,
                most_common_errors: vec![],
            },
            resource_usage: ResourceUsage {
                memory_usage_bytes: 1024 * 1024 * 100,
                cpu_usage_percent: 25.0,
                disk_io_bytes: 1024 * 1024,
                network_io_bytes: 512 * 1024,
                active_transactions: 5,
                lock_wait_time: Duration::from_millis(10),
            },
        };
        
        let score = calculate_health_score(&metrics);
        assert!(score > 0.7);
        assert!(score <= 1.0);
    }
}