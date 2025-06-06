//! mutsea-database/src/backends/postgresql.rs
//! PostgreSQL-specific implementations

use crate::{
    error::{DatabaseError, DatabaseResult},
    connection::Transaction,
};
use sqlx::{Postgres, Executor};
use tracing::{debug, error};

/// PostgreSQL transaction wrapper
pub struct PostgreSQLTransaction {
    tx: Option<sqlx::Transaction<'static, Postgres>>,
}

impl PostgreSQLTransaction {
    /// Create new PostgreSQL transaction
    pub fn new(tx: sqlx::Transaction<'static, Postgres>) -> Self {
        debug!("Created PostgreSQL transaction");
        Self { tx: Some(tx) }
    }

    /// Execute query with parameters (PostgreSQL-specific)
    pub async fn execute_with_params(
        &mut self,
        query: &str,
        params: &[&(dyn sqlx::Encode<'_, Postgres> + sqlx::Type<Postgres> + Sync)],
    ) -> DatabaseResult<u64> {
        if let Some(ref mut tx) = self.tx {
            let mut query_builder = sqlx::query(query);
            for param in params {
                query_builder = query_builder.bind(*param);
            }
            
            let result = query_builder.execute(&mut **tx).await?;
            Ok(result.rows_affected())
        } else {
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }

    /// Execute prepared statement (PostgreSQL-specific)
    pub async fn execute_prepared(
        &mut self,
        statement: &sqlx::postgres::PgStatement<'_>,
        params: &[&(dyn sqlx::Encode<'_, Postgres> + sqlx::Type<Postgres> + Sync)],
    ) -> DatabaseResult<u64> {
        if let Some(ref mut tx) = self.tx {
            let mut query = statement.query();
            for param in params {
                query = query.bind(*param);
            }
            
            let result = query.execute(&mut **tx).await?;
            Ok(result.rows_affected())
        } else {
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }

    /// Set transaction isolation level (PostgreSQL-specific)
    pub async fn set_isolation_level(&mut self, level: PostgreSQLIsolationLevel) -> DatabaseResult<()> {
        let query = match level {
            PostgreSQLIsolationLevel::ReadUncommitted => "SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED",
            PostgreSQLIsolationLevel::ReadCommitted => "SET TRANSACTION ISOLATION LEVEL READ COMMITTED",
            PostgreSQLIsolationLevel::RepeatableRead => "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ",
            PostgreSQLIsolationLevel::Serializable => "SET TRANSACTION ISOLATION LEVEL SERIALIZABLE",
        };
        
        self.execute(query, &[]).await?;
        debug!("Set PostgreSQL transaction isolation level to {:?}", level);
        Ok(())
    }

    /// Create savepoint (PostgreSQL-specific)
    pub async fn create_savepoint(&mut self, name: &str) -> DatabaseResult<()> {
        let query = format!("SAVEPOINT {}", name);
        self.execute(&query, &[]).await?;
        debug!("Created PostgreSQL savepoint: {}", name);
        Ok(())
    }

    /// Rollback to savepoint (PostgreSQL-specific)
    pub async fn rollback_to_savepoint(&mut self, name: &str) -> DatabaseResult<()> {
        let query = format!("ROLLBACK TO SAVEPOINT {}", name);
        self.execute(&query, &[]).await?;
        debug!("Rolled back to PostgreSQL savepoint: {}", name);
        Ok(())
    }

    /// Release savepoint (PostgreSQL-specific)
    pub async fn release_savepoint(&mut self, name: &str) -> DatabaseResult<()> {
        let query = format!("RELEASE SAVEPOINT {}", name);
        self.execute(&query, &[]).await?;
        debug!("Released PostgreSQL savepoint: {}", name);
        Ok(())
    }
}

#[async_trait::async_trait]
impl Transaction for PostgreSQLTransaction {
    async fn execute(&mut self, query: &str, _params: &[&dyn std::fmt::Debug]) -> DatabaseResult<u64> {
        if let Some(ref mut tx) = self.tx {
            debug!("Executing PostgreSQL query: {}", query);
            let result = sqlx::query(query).execute(&mut **tx).await?;
            let rows_affected = result.rows_affected();
            debug!("PostgreSQL query affected {} rows", rows_affected);
            Ok(rows_affected)
        } else {
            error!("PostgreSQL transaction already completed");
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }

    async fn commit(mut self: Box<Self>) -> DatabaseResult<()> {
        if let Some(tx) = self.tx.take() {
            debug!("Committing PostgreSQL transaction");
            tx.commit().await?;
            debug!("PostgreSQL transaction committed successfully");
            Ok(())
        } else {
            error!("PostgreSQL transaction already completed");
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }

    async fn rollback(mut self: Box<Self>) -> DatabaseResult<()> {
        if let Some(tx) = self.tx.take() {
            debug!("Rolling back PostgreSQL transaction");
            tx.rollback().await?;
            debug!("PostgreSQL transaction rolled back successfully");
            Ok(())
        } else {
            error!("PostgreSQL transaction already completed");
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }
}

/// PostgreSQL isolation levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostgreSQLIsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

/// PostgreSQL-specific utilities
pub struct PostgreSQLUtils;

impl PostgreSQLUtils {
    /// Generate UPSERT query for PostgreSQL
    pub fn generate_upsert_query(
        table: &str,
        columns: &[&str],
        conflict_columns: &[&str],
        update_columns: &[&str],
    ) -> String {
        let columns_str = columns.join(", ");
        let placeholders = (1..=columns.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ");
        
        let conflict_str = conflict_columns.join(", ");
        let update_str = update_columns
            .iter()
            .map(|col| format!("{} = EXCLUDED.{}", col, col))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT ({}) DO UPDATE SET {}",
            table, columns_str, placeholders, conflict_str, update_str
        )
    }

    /// Generate pagination query for PostgreSQL
    pub fn generate_pagination_query(base_query: &str, limit: i64, offset: i64) -> String {
        format!("{} LIMIT {} OFFSET {}", base_query, limit, offset)
    }

    /// Generate array aggregation query for PostgreSQL
    pub fn generate_array_agg_query(table: &str, column: &str, group_by: &str) -> String {
        format!(
            "SELECT {}, ARRAY_AGG({}) as aggregated_values FROM {} GROUP BY {}",
            group_by, column, table, group_by
        )
    }

    /// Generate JSON aggregation query for PostgreSQL
    pub fn generate_json_agg_query(table: &str, columns: &[&str], group_by: &str) -> String {
        let json_object = columns
            .iter()
            .map(|col| format!("'{}'", col))
            .collect::<Vec<_>>()
            .join(", ");
        
        format!(
            "SELECT {}, JSON_AGG(JSON_BUILD_OBJECT({})) as json_data FROM {} GROUP BY {}",
            group_by, json_object, table, group_by
        )
    }

    /// Check if PostgreSQL extension is available
    pub async fn extension_available(
        pool: &sqlx::Pool<Postgres>,
        extension_name: &str,
    ) -> DatabaseResult<bool> {
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM pg_available_extensions WHERE name = $1",
            extension_name
        )
        .fetch_one(pool)
        .await?;

        Ok(row.count.unwrap_or(0) > 0)
    }

    /// Enable PostgreSQL extension if available
    pub async fn enable_extension(
        pool: &sqlx::Pool<Postgres>,
        extension_name: &str,
    ) -> DatabaseResult<()> {
        if Self::extension_available(pool, extension_name).await? {
            let query = format!("CREATE EXTENSION IF NOT EXISTS {}", extension_name);
            sqlx::query(&query).execute(pool).await?;
            debug!("Enabled PostgreSQL extension: {}", extension_name);
            Ok(())
        } else {
            Err(DatabaseError::Generic(format!(
                "PostgreSQL extension '{}' is not available",
                extension_name
            )))
        }
    }

    /// Get PostgreSQL version
    pub async fn get_version(pool: &sqlx::Pool<Postgres>) -> DatabaseResult<String> {
        let row = sqlx::query!("SELECT version() as version")
            .fetch_one(pool)
            .await?;

        Ok(row.version.unwrap_or_else(|| "Unknown".to_string()))
    }

    /// Get table size in bytes
    pub async fn get_table_size(
        pool: &sqlx::Pool<Postgres>,
        table_name: &str,
    ) -> DatabaseResult<i64> {
        let row = sqlx::query!(
            "SELECT pg_total_relation_size($1) as size",
            table_name
        )
        .fetch_one(pool)
        .await?;

        Ok(row.size.unwrap_or(0))
    }

    /// Analyze table for query optimization
    pub async fn analyze_table(
        pool: &sqlx::Pool<Postgres>,
        table_name: &str,
    ) -> DatabaseResult<()> {
        let query = format!("ANALYZE {}", table_name);
        sqlx::query(&query).execute(pool).await?;
        debug!("Analyzed PostgreSQL table: {}", table_name);
        Ok(())
    }

    /// Vacuum table
    pub async fn vacuum_table(
        pool: &sqlx::Pool<Postgres>,
        table_name: &str,
        full: bool,
    ) -> DatabaseResult<()> {
        let query = if full {
            format!("VACUUM FULL {}", table_name)
        } else {
            format!("VACUUM {}", table_name)
        };
        
        sqlx::query(&query).execute(pool).await?;
        debug!("Vacuumed PostgreSQL table: {} (full: {})", table_name, full);
        Ok(())
    }
}