//! mutsea-database/src/backends/backend_type.rs
//! Database backend type definitions

use crate::error::{DatabaseError, DatabaseResult};

/// Database backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseBackend {
    PostgreSQL,
    MySQL,
    SQLite,
}

impl DatabaseBackend {
    /// Detect database backend from URL
    pub fn detect(url: &str) -> DatabaseResult<Self> {
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            Ok(DatabaseBackend::PostgreSQL)
        } else if url.starts_with("mysql://") {
            Ok(DatabaseBackend::MySQL)
        } else if url.starts_with("sqlite://") || url.ends_with(".db") || url.ends_with(".sqlite") {
            Ok(DatabaseBackend::SQLite)
        } else {
            Err(DatabaseError::UnsupportedBackend(url.to_string()))
        }
    }

    /// Get backend name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseBackend::PostgreSQL => "PostgreSQL",
            DatabaseBackend::MySQL => "MySQL",
            DatabaseBackend::SQLite => "SQLite",
        }
    }

    /// Get migration directory for this backend
    pub fn migration_dir(&self) -> &'static str {
        match self {
            DatabaseBackend::PostgreSQL => "./migrations/postgresql",
            DatabaseBackend::MySQL => "./migrations/mysql",
            DatabaseBackend::SQLite => "./migrations/sqlite",
        }
    }

    /// Get default port for this backend
    pub fn default_port(&self) -> u16 {
        match self {
            DatabaseBackend::PostgreSQL => 5432,
            DatabaseBackend::MySQL => 3306,
            DatabaseBackend::SQLite => 0, // No port for SQLite
        }
    }

    /// Check if backend supports transactions
    pub fn supports_transactions(&self) -> bool {
        match self {
            DatabaseBackend::PostgreSQL => true,
            DatabaseBackend::MySQL => true,
            DatabaseBackend::SQLite => true,
        }
    }

    /// Check if backend supports concurrent connections
    pub fn supports_concurrent_connections(&self) -> bool {
        match self {
            DatabaseBackend::PostgreSQL => true,
            DatabaseBackend::MySQL => true,
            DatabaseBackend::SQLite => false, // SQLite has limited concurrent write support
        }
    }

    /// Get recommended connection pool size
    pub fn recommended_pool_size(&self) -> (u32, u32) {
        match self {
            DatabaseBackend::PostgreSQL => (5, 100), // (min, max)
            DatabaseBackend::MySQL => (5, 100),
            DatabaseBackend::SQLite => (1, 1), // SQLite should use minimal connections
        }
    }

    /// Initialize AI-specific schema for this backend
    pub async fn initialize_ai_schema(&self, pool: &super::pool::DatabasePool) -> DatabaseResult<()> {
        match self {
            DatabaseBackend::PostgreSQL => {
                let sql_files = [
                    include_str!("../../migrations/postgresql/ai/ai_decisions.sql"),
                    include_str!("../../migrations/postgresql/ai/ai_global_mind_state.sql"),
                    include_str!("../../migrations/postgresql/ai/emergent_behaviors.sql"),
                    include_str!("../../migrations/postgresql/ai/learning_data.sql"),
                    include_str!("../../migrations/postgresql/ai/npc_states.sql"),
                ];

                for sql in sql_files.iter() {
                    pool.execute_raw(sql).await?;
                }

                Ok(())
            }
            DatabaseBackend::MySQL | DatabaseBackend::SQLite => {
                // AI schema migrations are not implemented for these backends yet
                Ok(())
            }
        }
    }
}

