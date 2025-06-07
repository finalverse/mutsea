// mutsea-database/src/queries/mod.rs

pub mod builder;
pub mod world_queries;
pub mod player_queries;
pub mod ai_queries;
pub mod ecosystem_queries;
pub mod performance_queries;
pub mod batch_operations;

use crate::error::Result;
use crate::utils::parameter_binding::ParameterBinder;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Common query result types
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows: Vec<HashMap<String, Value>>,
    pub affected_rows: Option<u64>,
}

/// Query execution context
#[derive(Debug, Clone)]
pub struct QueryContext {
    pub timeout_seconds: Option<u64>,
    pub max_rows: Option<u64>,
    pub cache_key: Option<String>,
    pub trace_id: Option<Uuid>,
}

impl Default for QueryContext {
    fn default() -> Self {
        Self {
            timeout_seconds: Some(30),
            max_rows: Some(10000),
            cache_key: None,
            trace_id: None,
        }
    }
}

/// Base query execution trait
#[async_trait::async_trait]
pub trait QueryExecutor {
    async fn execute_query(
        &self,
        sql: &str,
        params: ParameterBinder,
        context: QueryContext,
    ) -> Result<QueryResult>;

    async fn execute_batch(
        &self,
        sql: &str,
        param_batches: Vec<ParameterBinder>,
        context: QueryContext,
    ) -> Result<Vec<QueryResult>>;
}

/// Query builder trait for type-safe query construction
pub trait QueryBuilder<T> {
    fn select() -> SelectQueryBuilder<T>;
    fn insert() -> InsertQueryBuilder<T>;
    fn update() -> UpdateQueryBuilder<T>;
    fn delete() -> DeleteQueryBuilder<T>;
}

/// Select query builder
#[derive(Debug, Clone)]
pub struct SelectQueryBuilder<T> {
    table: String,
    columns: Vec<String>,
    where_clause: Option<String>,
    order_by: Vec<String>,
    limit: Option<u64>,
    offset: Option<u64>,
    joins: Vec<String>,
    group_by: Vec<String>,
    having: Option<String>,
    params: ParameterBinder,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> SelectQueryBuilder<T> {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            columns: vec!["*".to_string()],
            where_clause: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
            joins: Vec::new(),
            group_by: Vec::new(),
            having: None,
            params: ParameterBinder::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }

    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by.push(format!("{} {}", column, direction));
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn join(mut self, join_clause: &str) -> Self {
        self.joins.push(join_clause.to_string());
        self
    }

    pub fn group_by(mut self, column: &str) -> Self {
        self.group_by.push(column.to_string());
        self
    }

    pub fn having(mut self, condition: &str) -> Self {
        self.having = Some(condition.to_string());
        self
    }

    pub fn param(mut self, key: &str, value: impl Into<crate::utils::parameter_binding::ParameterValue>) -> Self {
        // This would need proper implementation based on the parameter type
        self
    }

    pub fn build(self) -> (String, ParameterBinder) {
        let mut sql = format!("SELECT {} FROM {}", self.columns.join(", "), self.table);

        // Add joins
        for join in &self.joins {
            sql.push_str(&format!(" {}", join));
        }

        // Add WHERE clause
        if let Some(where_clause) = &self.where_clause {
            sql.push_str(&format!(" WHERE {}", where_clause));
        }

        // Add GROUP BY
        if !self.group_by.is_empty() {
            sql.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        }

        // Add HAVING
        if let Some(having) = &self.having {
            sql.push_str(&format!(" HAVING {}", having));
        }

        // Add ORDER BY
        if !self.order_by.is_empty() {
            sql.push_str(&format!(" ORDER BY {}", self.order_by.join(", ")));
        }

        // Add LIMIT
        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        // Add OFFSET
        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        (sql, self.params)
    }
}

/// Insert query builder
#[derive(Debug, Clone)]
pub struct InsertQueryBuilder<T> {
    table: String,
    columns: Vec<String>,
    values: Vec<Vec<String>>,
    on_conflict: Option<String>,
    returning: Vec<String>,
    params: ParameterBinder,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> InsertQueryBuilder<T> {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            columns: Vec::new(),
            values: Vec::new(),
            on_conflict: None,
            returning: Vec::new(),
            params: ParameterBinder::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn values(mut self, values: &[&str]) -> Self {
        self.values.push(values.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn on_conflict(mut self, action: &str) -> Self {
        self.on_conflict = Some(action.to_string());
        self
    }

    pub fn returning(mut self, columns: &[&str]) -> Self {
        self.returning = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(self) -> (String, ParameterBinder) {
        let mut sql = format!("INSERT INTO {} ({})", self.table, self.columns.join(", "));

        if self.values.len() == 1 {
            sql.push_str(&format!(" VALUES ({})", self.values[0].join(", ")));
        } else if self.values.len() > 1 {
            let value_clauses: Vec<String> = self.values
                .iter()
                .map(|row| format!("({})", row.join(", ")))
                .collect();
            sql.push_str(&format!(" VALUES {}", value_clauses.join(", ")));
        }

        if let Some(on_conflict) = &self.on_conflict {
            sql.push_str(&format!(" ON CONFLICT {}", on_conflict));
        }

        if !self.returning.is_empty() {
            sql.push_str(&format!(" RETURNING {}", self.returning.join(", ")));
        }

        (sql, self.params)
    }
}

/// Update query builder
#[derive(Debug, Clone)]
pub struct UpdateQueryBuilder<T> {
    table: String,
    set_clauses: Vec<String>,
    where_clause: Option<String>,
    returning: Vec<String>,
    params: ParameterBinder,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> UpdateQueryBuilder<T> {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            set_clauses: Vec::new(),
            where_clause: None,
            returning: Vec::new(),
            params: ParameterBinder::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn set(mut self, column: &str, value: &str) -> Self {
        self.set_clauses.push(format!("{} = {}", column, value));
        self
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }

    pub fn returning(mut self, columns: &[&str]) -> Self {
        self.returning = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(self) -> (String, ParameterBinder) {
        let mut sql = format!("UPDATE {} SET {}", self.table, self.set_clauses.join(", "));

        if let Some(where_clause) = &self.where_clause {
            sql.push_str(&format!(" WHERE {}", where_clause));
        }

        if !self.returning.is_empty() {
            sql.push_str(&format!(" RETURNING {}", self.returning.join(", ")));
        }

        (sql, self.params)
    }
}

/// Delete query builder
#[derive(Debug, Clone)]
pub struct DeleteQueryBuilder<T> {
    table: String,
    where_clause: Option<String>,
    returning: Vec<String>,
    params: ParameterBinder,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> DeleteQueryBuilder<T> {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            where_clause: None,
            returning: Vec::new(),
            params: ParameterBinder::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }

    pub fn returning(mut self, columns: &[&str]) -> Self {
        self.returning = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(self) -> (String, ParameterBinder) {
        let mut sql = format!("DELETE FROM {}", self.table);

        if let Some(where_clause) = &self.where_clause {
            sql.push_str(&format!(" WHERE {}", where_clause));
        }

        if !self.returning.is_empty() {
            sql.push_str(&format!(" RETURNING {}", self.returning.join(", ")));
        }

        (sql, self.params)
    }
}

/// Pagination support
#[derive(Debug, Clone)]
pub struct PaginationParams {
    pub page: u64,
    pub page_size: u64,
}

impl PaginationParams {
    pub fn new(page: u64, page_size: u64) -> Self {
        Self { page, page_size }
    }

    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> u64 {
        self.page_size
    }
}

/// Query result pagination
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

impl<T> PaginatedResult<T> {
    pub fn new(data: Vec<T>, total_count: u64, page: u64, page_size: u64) -> Self {
        let total_pages = (total_count + page_size - 1) / page_size;
        Self {
            data,
            total_count,
            page,
            page_size,
            total_pages,
        }
    }

    pub fn has_next_page(&self) -> bool {
        self.page < self.total_pages
    }

    pub fn has_previous_page(&self) -> bool {
        self.page > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_query_builder() {
        let (sql, _) = SelectQueryBuilder::<()>::new("users")
            .columns(&["id", "name", "email"])
            .where_clause("active = true")
            .order_by("name", "ASC")
            .limit(10)
            .build();

        assert!(sql.contains("SELECT id, name, email FROM users"));
        assert!(sql.contains("WHERE active = true"));
        assert!(sql.contains("ORDER BY name ASC"));
        assert!(sql.contains("LIMIT 10"));
    }

    #[test]
    fn test_insert_query_builder() {
        let (sql, _) = InsertQueryBuilder::<()>::new("users")
            .columns(&["name", "email"])
            .values(&[":name", ":email"])
            .returning(&["id"])
            .build();

        assert!(sql.contains("INSERT INTO users (name, email)"));
        assert!(sql.contains("VALUES (:name, :email)"));
        assert!(sql.contains("RETURNING id"));
    }

    #[test]
    fn test_pagination_params() {
        let pagination = PaginationParams::new(3, 20);
        assert_eq!(pagination.offset(), 40);
        assert_eq!(pagination.limit(), 20);
    }
}