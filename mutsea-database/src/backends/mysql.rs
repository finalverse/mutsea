//! mutsea-database/src/backends/mysql.rs
//! MySQL-specific implementations - Complete implementation

use crate::{
    error::{DatabaseError, DatabaseResult},
    connection::Transaction,
};
use sqlx::{MySql, Executor};
use tracing::{debug, error};

/// MySQL transaction wrapper
pub struct MySQLTransaction {
    tx: Option<sqlx::Transaction<'static, MySql>>,
}

impl MySQLTransaction {
    /// Create new MySQL transaction
    pub fn new(tx: sqlx::Transaction<'static, MySql>) -> Self {
        debug!("Created MySQL transaction");
        Self { tx: Some(tx) }
    }

    /// Execute query with parameters (MySQL-specific)
    pub async fn execute_with_params(
        &mut self,
        query: &str,
        params: &[&(dyn sqlx::Encode<'_, MySql> + sqlx::Type<MySql> + Sync)],
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

    /// Set transaction isolation level (MySQL-specific)
    pub async fn set_isolation_level(&mut self, level: MySQLIsolationLevel) -> DatabaseResult<()> {
        let query = match level {
            MySQLIsolationLevel::ReadUncommitted => "SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED",
            MySQLIsolationLevel::ReadCommitted => "SET TRANSACTION ISOLATION LEVEL READ COMMITTED",
            MySQLIsolationLevel::RepeatableRead => "SET TRANSACTION ISOLATION LEVEL REPEATABLE READ",
            MySQLIsolationLevel::Serializable => "SET TRANSACTION ISOLATION LEVEL SERIALIZABLE",
        };
        
        self.execute(query, &[]).await?;
        debug!("Set MySQL transaction isolation level to {:?}", level);
        Ok(())
    }

    /// Start named transaction (MySQL-specific)
    pub async fn start_named_transaction(&mut self, name: &str) -> DatabaseResult<()> {
        // MySQL doesn't support named transactions like PostgreSQL savepoints
        // But we can use a workaround with a transaction marker
        let query = format!("SELECT '{}_START' as transaction_marker", name);
        self.execute(&query, &[]).await?;
        debug!("Started MySQL named transaction: {}", name);
        Ok(())
    }

    /// Set session variable (MySQL-specific)
    pub async fn set_session_variable(&mut self, name: &str, value: &str) -> DatabaseResult<()> {
        let query = format!("SET SESSION {} = '{}'", name, value);
        self.execute(&query, &[]).await?;
        debug!("Set MySQL session variable: {} = {}", name, value);
        Ok(())
    }

    /// Get last insert ID (MySQL-specific)
    pub async fn get_last_insert_id(&mut self) -> DatabaseResult<u64> {
        if let Some(ref mut tx) = self.tx {
            let row = sqlx::query!("SELECT LAST_INSERT_ID() as id")
                .fetch_one(&mut **tx)
                .await?;
            Ok(row.id.unwrap_or(0))
        } else {
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }

    /// Lock tables (MySQL-specific)
    pub async fn lock_tables(&mut self, tables: &[&str], lock_type: MySQLLockType) -> DatabaseResult<()> {
        let lock_str = match lock_type {
            MySQLLockType::Read => "READ",
            MySQLLockType::Write => "WRITE",
            MySQLLockType::ReadLocal => "READ LOCAL",
        };
        
        let table_list = tables.join(&format!(" {}, ", lock_str));
        let query = format!("LOCK TABLES {} {}", table_list, lock_str);
        self.execute(&query, &[]).await?;
        debug!("Locked MySQL tables: {:?} with {:?}", tables, lock_type);
        Ok(())
    }

    /// Unlock tables (MySQL-specific)
    pub async fn unlock_tables(&mut self) -> DatabaseResult<()> {
        self.execute("UNLOCK TABLES", &[]).await?;
        debug!("Unlocked all MySQL tables");
        Ok(())
    }

    /// Enable/disable foreign key checks (MySQL-specific)
    pub async fn set_foreign_key_checks(&mut self, enabled: bool) -> DatabaseResult<()> {
        let value = if enabled { "1" } else { "0" };
        let query = format!("SET foreign_key_checks = {}", value);
        self.execute(&query, &[]).await?;
        debug!("Set MySQL foreign key checks to: {}", enabled);
        Ok(())
    }

    /// Set SQL mode (MySQL-specific)
    pub async fn set_sql_mode(&mut self, mode: &str) -> DatabaseResult<()> {
        let query = format!("SET sql_mode = '{}'", mode);
        self.execute(&query, &[]).await?;
        debug!("Set MySQL SQL mode to: {}", mode);
        Ok(())
    }

    /// Get connection ID (MySQL-specific)
    pub async fn get_connection_id(&mut self) -> DatabaseResult<u64> {
        if let Some(ref mut tx) = self.tx {
            let row = sqlx::query!("SELECT CONNECTION_ID() as id")
                .fetch_one(&mut **tx)
                .await?;
            Ok(row.id.unwrap_or(0))
        } else {
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }
}

#[async_trait::async_trait]
impl Transaction for MySQLTransaction {
    async fn execute(&mut self, query: &str, _params: &[&dyn std::fmt::Debug]) -> DatabaseResult<u64> {
        if let Some(ref mut tx) = self.tx {
            debug!("Executing MySQL query: {}", query);
            let result = sqlx::query(query).execute(&mut **tx).await?;
            let rows_affected = result.rows_affected();
            debug!("MySQL query affected {} rows", rows_affected);
            Ok(rows_affected)
        } else {
            error!("MySQL transaction already completed");
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }

    async fn commit(mut self: Box<Self>) -> DatabaseResult<()> {
        if let Some(tx) = self.tx.take() {
            debug!("Committing MySQL transaction");
            tx.commit().await?;
            debug!("MySQL transaction committed successfully");
            Ok(())
        } else {
            error!("MySQL transaction already completed");
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }

    async fn rollback(mut self: Box<Self>) -> DatabaseResult<()> {
        if let Some(tx) = self.tx.take() {
            debug!("Rolling back MySQL transaction");
            tx.rollback().await?;
            debug!("MySQL transaction rolled back successfully");
            Ok(())
        } else {
            error!("MySQL transaction already completed");
            Err(DatabaseError::Generic("Transaction already completed".to_string()))
        }
    }
}

/// MySQL isolation levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MySQLIsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

/// MySQL lock types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MySQLLockType {
    Read,
    Write,
    ReadLocal,
}

/// MySQL-specific utilities
pub struct MySQLUtils;

impl MySQLUtils {
    /// Generate UPSERT query for MySQL (using ON DUPLICATE KEY UPDATE)
    pub fn generate_upsert_query(
        table: &str,
        columns: &[&str],
        update_columns: &[&str],
    ) -> String {
        let columns_str = columns.join(", ");
        let placeholders = (1..=columns.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");
        
        let update_str = update_columns
            .iter()
            .map(|col| format!("{} = VALUES({})", col, col))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "INSERT INTO {} ({}) VALUES ({}) ON DUPLICATE KEY UPDATE {}",
            table, columns_str, placeholders, update_str
        )
    }

    /// Generate pagination query for MySQL
    pub fn generate_pagination_query(base_query: &str, limit: i64, offset: i64) -> String {
        format!("{} LIMIT {} OFFSET {}", base_query, limit, offset)
    }

    /// Generate GROUP_CONCAT query for MySQL
    pub fn generate_group_concat_query(
        table: &str, 
        column: &str, 
        group_by: &str,
        separator: Option<&str>
    ) -> String {
        let sep = separator.unwrap_or(",");
        format!(
            "SELECT {}, GROUP_CONCAT({} SEPARATOR '{}') as concatenated_values FROM {} GROUP BY {}",
            group_by, column, sep, table, group_by
        )
    }

    /// Generate JSON aggregation query for MySQL (5.7+)
    pub fn generate_json_agg_query(table: &str, columns: &[&str], group_by: &str) -> String {
        let json_object = columns
            .iter()
            .map(|col| format!("'{}', {}", col, col))
            .collect::<Vec<_>>()
            .join(", ");
        
        format!(
            "SELECT {}, JSON_ARRAYAGG(JSON_OBJECT({})) as json_data FROM {} GROUP BY {}",
            group_by, json_object, table, group_by
        )
    }

    /// Get MySQL version
    pub async fn get_version(pool: &sqlx::Pool<MySql>) -> DatabaseResult<String> {
        let row = sqlx::query!("SELECT VERSION() as version")
            .fetch_one(pool)
            .await?;

        Ok(row.version.unwrap_or_else(|| "Unknown".to_string()))
    }

    /// Get table size in bytes
    pub async fn get_table_size(
        pool: &sqlx::Pool<MySql>,
        schema_name: &str,
        table_name: &str,
    ) -> DatabaseResult<i64> {
        let row = sqlx::query!(
            "SELECT (data_length + index_length) as size 
             FROM information_schema.tables 
             WHERE table_schema = ? AND table_name = ?",
            schema_name,
            table_name
        )
        .fetch_one(pool)
        .await?;

        Ok(row.size.unwrap_or(0))
    }

    /// Optimize table for query performance
    pub async fn optimize_table(
        pool: &sqlx::Pool<MySql>,
        table_name: &str,
    ) -> DatabaseResult<()> {
        let query = format!("OPTIMIZE TABLE {}", table_name);
        sqlx::query(&query).execute(pool).await?;
        debug!("Optimized MySQL table: {}", table_name);
        Ok(())
    }

    /// Analyze table for query optimization
    pub async fn analyze_table(
        pool: &sqlx::Pool<MySql>,
        table_name: &str,
    ) -> DatabaseResult<()> {
        let query = format!("ANALYZE TABLE {}", table_name);
        sqlx::query(&query).execute(pool).await?;
        debug!("Analyzed MySQL table: {}", table_name);
        Ok(())
    }

    /// Repair table
    pub async fn repair_table(
        pool: &sqlx::Pool<MySql>,
        table_name: &str,
    ) -> DatabaseResult<()> {
        let query = format!("REPAIR TABLE {}", table_name);
        sqlx::query(&query).execute(pool).await?;
        debug!("Repaired MySQL table: {}", table_name);
        Ok(())
    }

    /// Check table integrity
    pub async fn check_table(
        pool: &sqlx::Pool<MySql>,
        table_name: &str,
    ) -> DatabaseResult<Vec<String>> {
        let query = format!("CHECK TABLE {}", table_name);
        let rows = sqlx::query(&query).fetch_all(pool).await?;
        
        let mut results = Vec::new();
        for row in rows {
            if let Ok(msg_level) = row.try_get::<String, _>("Msg_type") {
                if let Ok(msg_text) = row.try_get::<String, _>("Msg_text") {
                    results.push(format!("{}: {}", msg_level, msg_text));
                }
            }
        }
        
        debug!("Checked MySQL table: {} - {} results", table_name, results.len());
        Ok(results)
    }

    /// Show table status
    pub async fn show_table_status(
        pool: &sqlx::Pool<MySql>,
        table_name: &str,
    ) -> DatabaseResult<MySQLTableStatus> {
        let row = sqlx::query!(
            "SHOW TABLE STATUS WHERE Name = ?",
            table_name
        )
        .fetch_one(pool)
        .await?;

        Ok(MySQLTableStatus {
            name: row.Name.unwrap_or_default(),
            engine: row.Engine,
            version: row.Version,
            row_format: row.Row_format,
            rows: row.Rows,
            avg_row_length: row.Avg_row_length,
            data_length: row.Data_length,
            max_data_length: row.Max_data_length,
            index_length: row.Index_length,
            data_free: row.Data_free,
            auto_increment: row.Auto_increment,
            create_time: row.Create_time,
            update_time: row.Update_time,
            check_time: row.Check_time,
            collation: row.Collation,
            checksum: row.Checksum,
            create_options: row.Create_options,
            comment: row.Comment.unwrap_or_default(),
        })
    }

    /// Show processlist
    pub async fn show_processlist(pool: &sqlx::Pool<MySql>) -> DatabaseResult<Vec<MySQLProcess>> {
        let rows = sqlx::query!("SHOW PROCESSLIST")
            .fetch_all(pool)
            .await?;

        let mut processes = Vec::new();
        for row in rows {
            processes.push(MySQLProcess {
                id: row.Id,
                user: row.User.unwrap_or_default(),
                host: row.Host.unwrap_or_default(),
                db: row.db,
                command: row.Command.unwrap_or_default(),
                time: row.Time,
                state: row.State,
                info: row.Info,
            });
        }

        Ok(processes)
    }

    /// Kill process by ID
    pub async fn kill_process(pool: &sqlx::Pool<MySql>, process_id: u64) -> DatabaseResult<()> {
        let query = format!("KILL {}", process_id);
        sqlx::query(&query).execute(pool).await?;
        debug!("Killed MySQL process: {}", process_id);
        Ok(())
    }

    /// Set global variable
    pub async fn set_global_variable(
        pool: &sqlx::Pool<MySql>,
        name: &str,
        value: &str,
    ) -> DatabaseResult<()> {
        let query = format!("SET GLOBAL {} = '{}'", name, value);
        sqlx::query(&query).execute(pool).await?;
        debug!("Set MySQL global variable: {} = {}", name, value);
        Ok(())
    }

    /// Get global variable
    pub async fn get_global_variable(
        pool: &sqlx::Pool<MySql>,
        name: &str,
    ) -> DatabaseResult<Option<String>> {
        let row = sqlx::query!(
            "SHOW GLOBAL VARIABLES WHERE Variable_name = ?",
            name
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.and_then(|r| r.Value))
    }

    /// Flush privileges
    pub async fn flush_privileges(pool: &sqlx::Pool<MySql>) -> DatabaseResult<()> {
        sqlx::query("FLUSH PRIVILEGES").execute(pool).await?;
        debug!("Flushed MySQL privileges");
        Ok(())
    }

    /// Flush logs
    pub async fn flush_logs(pool: &sqlx::Pool<MySql>) -> DatabaseResult<()> {
        sqlx::query("FLUSH LOGS").execute(pool).await?;
        debug!("Flushed MySQL logs");
        Ok(())
    }
}

/// MySQL table status information
#[derive(Debug, Clone)]
pub struct MySQLTableStatus {
    pub name: String,
    pub engine: Option<String>,
    pub version: Option<u64>,
    pub row_format: Option<String>,
    pub rows: Option<u64>,
    pub avg_row_length: Option<u64>,
    pub data_length: Option<u64>,
    pub max_data_length: Option<u64>,
    pub index_length: Option<u64>,
    pub data_free: Option<u64>,
    pub auto_increment: Option<u64>,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    pub check_time: Option<chrono::DateTime<chrono::Utc>>,
    pub collation: Option<String>,
    pub checksum: Option<u64>,
    pub create_options: Option<String>,
    pub comment: String,
}

/// MySQL process information
#[derive(Debug, Clone)]
pub struct MySQLProcess {
    pub id: u64,
    pub user: String,
    pub host: String,
    pub db: Option<String>,
    pub command: String,
    pub time: Option<u32>,
    pub state: Option<String>,
    pub info: Option<String>,
}