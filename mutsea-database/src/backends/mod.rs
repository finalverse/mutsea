//! mutsea-database/src/backends/mod.rs
//! Database backend implementations for PostgreSQL, MySQL, and SQLite

pub mod backend_type;
pub mod pool;
pub mod postgresql;
pub mod mysql;
pub mod sqlite;

// Re-export main types
pub use backend_type::DatabaseBackend;
pub use pool::DatabasePool;
pub use postgresql::PostgreSQLTransaction;
pub use mysql::MySQLTransaction;
pub use sqlite::SQLiteTransaction;