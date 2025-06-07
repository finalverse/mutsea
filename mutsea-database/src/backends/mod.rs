// mutsea-database/src/backends/mod.rs
//! Database backend implementations

use async_trait::async_trait;
use crate::{DatabaseError, Result};

pub mod postgresql;
pub mod sqlite;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    PostgreSQL,
    SQLite,
}

/// Core database operations trait
#[async_trait]
pub trait DatabaseBackend: Send + Sync {
    /// Execute a query with parameters
    async fn execute(&self, query: &str, params: &[&dyn ToSql]) -> Result<u64>;
    
    /// Query for multiple rows
    async fn query(&self, query: &str, params: &[&dyn ToSql]) -> Result<Vec<Row>>;
    
    /// Query for a single row
    async fn query_one(&self, query: &str, params: &[&dyn ToSql]) -> Result<Row>;
    
    /// Check if a table exists
    async fn table_exists(&self, table_name: &str) -> Result<bool>;
    
    /// Begin a transaction
    async fn begin_transaction(&self) -> Result<Box<dyn Transaction>>;
    
    /// Get backend type
    fn backend_type(&self) -> BackendType;
}

/// Transaction trait for database operations
#[async_trait]
pub trait Transaction: Send + Sync {
    async fn execute(&mut self, query: &str, params: &[&dyn ToSql]) -> Result<u64>;
    async fn query(&mut self, query: &str, params: &[&dyn ToSql]) -> Result<Vec<Row>>;
    async fn commit(self: Box<Self>) -> Result<()>;
    async fn rollback(self: Box<Self>) -> Result<()>;
}

/// Database row abstraction
pub trait Row: Send + Sync {
    fn get<T>(&self, index: usize) -> Result<T> where T: FromSql;
    fn get_by_name<T>(&self, name: &str) -> Result<T> where T: FromSql;
    fn column_count(&self) -> usize;
}

/// Parameter binding trait
pub trait ToSql: Send + Sync {
    fn to_sql(&self) -> SqlValue;
}

/// SQL value representation
#[derive(Debug, Clone)]
pub enum SqlValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
    Json(serde_json::Value),
}

/// Value extraction trait
pub trait FromSql: Sized {
    fn from_sql(value: SqlValue) -> Result<Self>;
}

/// Database pool for managing connections
pub struct DatabasePool {
    backend_type: BackendType,
    inner: PoolInner,
}

enum PoolInner {
    #[cfg(feature = "postgresql")]
    PostgreSQL(postgresql::PostgreSQLPool),
    #[cfg(feature = "sqlite")]
    SQLite(sqlite::SQLitePool),
}

impl DatabasePool {
    pub async fn new(database_url: &str) -> Result<Self> {
        if database_url.starts_with("postgresql://") || database_url.starts_with("postgres://") {
            #[cfg(feature = "postgresql")]
            {
                let pool = postgresql::PostgreSQLPool::new(database_url).await?;
                Ok(Self {
                    backend_type: BackendType::PostgreSQL,
                    inner: PoolInner::PostgreSQL(pool),
                })
            }
            #[cfg(not(feature = "postgresql"))]
            {
                Err(DatabaseError::Internal("PostgreSQL support not enabled".to_string()))
            }
        } else if database_url.starts_with("sqlite://") {
            #[cfg(feature = "sqlite")]
            {
                let pool = sqlite::SQLitePool::new(database_url).await?;
                Ok(Self {
                    backend_type: BackendType::SQLite,
                    inner: PoolInner::SQLite(pool),
                })
            }
            #[cfg(not(feature = "sqlite"))]
            {
                Err(DatabaseError::Internal("SQLite support not enabled".to_string()))
            }
        } else {
            Err(DatabaseError::Connection(format!("Unsupported database URL: {}", database_url)))
        }
    }

    pub fn backend_type(&self) -> BackendType {
        self.backend_type
    }

    pub fn get_backend(&self) -> Box<dyn DatabaseBackend> {
        match &self.inner {
            #[cfg(feature = "postgresql")]
            PoolInner::PostgreSQL(pool) => Box::new(pool.clone()),
            #[cfg(feature = "sqlite")]
            PoolInner::SQLite(pool) => Box::new(pool.clone()),
        }
    }
}

// Implementations for common types
impl ToSql for String {
    fn to_sql(&self) -> SqlValue {
        SqlValue::Text(self.clone())
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> SqlValue {
        SqlValue::Text(self.to_string())
    }
}

impl ToSql for i32 {
    fn to_sql(&self) -> SqlValue {
        SqlValue::Int(*self as i64)
    }
}

impl ToSql for i64 {
    fn to_sql(&self) -> SqlValue {
        SqlValue::Int(*self)
    }
}

impl ToSql for f64 {
    fn to_sql(&self) -> SqlValue {
        SqlValue::Float(*self)
    }
}

impl ToSql for bool {
    fn to_sql(&self) -> SqlValue {
        SqlValue::Bool(*self)
    }
}

impl ToSql for serde_json::Value {
    fn to_sql(&self) -> SqlValue {
        SqlValue::Json(self.clone())
    }
}

impl<T: ToSql> ToSql for Option<T> {
    fn to_sql(&self) -> SqlValue {
        match self {
            Some(val) => val.to_sql(),
            None => SqlValue::Null,
        }
    }
}

impl FromSql for String {
    fn from_sql(value: SqlValue) -> Result<Self> {
        match value {
            SqlValue::Text(s) => Ok(s),
            _ => Err(DatabaseError::Serialization("Expected text value".to_string())),
        }
    }
}

impl FromSql for i64 {
    fn from_sql(value: SqlValue) -> Result<Self> {
        match value {
            SqlValue::Int(i) => Ok(i),
            _ => Err(DatabaseError::Serialization("Expected integer value".to_string())),
        }
    }
}

impl FromSql for f64 {
    fn from_sql(value: SqlValue) -> Result<Self> {
        match value {
            SqlValue::Float(f) => Ok(f),
            SqlValue::Int(i) => Ok(i as f64),
            _ => Err(DatabaseError::Serialization("Expected numeric value".to_string())),
        }
    }
}

impl FromSql for bool {
    fn from_sql(value: SqlValue) -> Result<Self> {
        match value {
            SqlValue::Bool(b) => Ok(b),
            _ => Err(DatabaseError::Serialization("Expected boolean value".to_string())),
        }
    }
}

impl FromSql for serde_json::Value {
    fn from_sql(value: SqlValue) -> Result<Self> {
        match value {
            SqlValue::Json(j) => Ok(j),
            SqlValue::Text(s) => serde_json::from_str(&s).map_err(|e| e.into()),
            _ => Err(DatabaseError::Serialization("Expected JSON value".to_string())),
        }
    }
}

impl<T: FromSql> FromSql for Option<T> {
    fn from_sql(value: SqlValue) -> Result<Self> {
        match value {
            SqlValue::Null => Ok(None),
            _ => T::from_sql(value).map(Some),
        }
    }
}