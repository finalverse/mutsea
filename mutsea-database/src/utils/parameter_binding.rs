// mutsea-database/src/utils/parameter_binding.rs

use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

use crate::error::{DatabaseError, DatabaseResult};
use crate::traits::query_builder::{QueryParam, DatabaseDialect};

/// Parameter binding context for SQL query execution
#[derive(Debug, Clone)]
pub struct ParameterBindingContext {
    pub dialect: DatabaseDialect,
    pub named_parameters: HashMap<String, QueryParam>,
    pub positional_parameters: Vec<QueryParam>,
    pub parameter_style: ParameterStyle,
}

/// Different parameter binding styles supported by databases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterStyle {
    /// PostgreSQL style: $1, $2, $3...
    Dollar,
    /// MySQL/SQLite style: ?
    Question,
    /// Named parameters: :name, :value...
    Named,
    /// At-style parameters: @name, @value...
    At,
}

/// Parameter binder trait for different database backends
pub trait ParameterBinder {
    /// Bind parameters to a SQL query string
    fn bind_parameters(
        &self,
        sql: &str,
        params: &[QueryParam],
        context: &ParameterBindingContext,
    ) -> DatabaseResult<BoundQuery>;

    /// Convert named parameters to positional parameters
    fn convert_named_to_positional(
        &self,
        sql: &str,
        named_params: &HashMap<String, QueryParam>,
    ) -> DatabaseResult<(String, Vec<QueryParam>)>;

    /// Validate parameter types against expected schema
    fn validate_parameters(
        &self,
        params: &[QueryParam],
        expected_types: &[ParameterType],
    ) -> DatabaseResult<()>;
}

/// Bound query with parameters ready for execution
#[derive(Debug, Clone)]
pub struct BoundQuery {
    pub sql: String,
    pub parameters: Vec<BoundParameter>,
    pub parameter_count: usize,
}

/// Individual bound parameter with type information
#[derive(Debug, Clone)]
pub struct BoundParameter {
    pub value: QueryParam,
    pub sql_type: SqlType,
    pub is_nullable: bool,
}

/// SQL type mapping for parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlType {
    Text,
    Integer,
    BigInteger,
    Real,
    Boolean,
    Uuid,
    Json,
    Jsonb,
    Binary,
    Timestamp,
    Date,
    Time,
}

/// Expected parameter types for validation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Uuid,
    Json,
    Binary,
    DateTime,
    Optional(Box<ParameterType>),
}

/// Default parameter binder implementation
#[derive(Debug)]
pub struct DefaultParameterBinder {
    dialect: DatabaseDialect,
}

impl DefaultParameterBinder {
    pub fn new(dialect: DatabaseDialect) -> Self {
        Self { dialect }
    }

    /// Get parameter style for the database dialect
    fn parameter_style(&self) -> ParameterStyle {
        match self.dialect {
            DatabaseDialect::PostgreSQL => ParameterStyle::Dollar,
            DatabaseDialect::SQLite => ParameterStyle::Question,
            DatabaseDialect::MySQL => ParameterStyle::Question,
        }
    }

    /// Format parameter placeholder for the dialect
    fn format_placeholder(&self, index: usize) -> String {
        match self.parameter_style() {
            ParameterStyle::Dollar => format!("${}", index + 1),
            ParameterStyle::Question => "?".to_string(),
            ParameterStyle::Named => format!(":param_{}", index),
            ParameterStyle::At => format!("@param_{}", index),
        }
    }

    /// Convert QueryParam to appropriate SQL type
    fn map_sql_type(&self, param: &QueryParam) -> SqlType {
        match param {
            QueryParam::String(_) => SqlType::Text,
            QueryParam::Integer(_) => SqlType::BigInteger,
            QueryParam::Float(_) => SqlType::Real,
            QueryParam::Boolean(_) => SqlType::Boolean,
            QueryParam::Uuid(_) => match self.dialect {
                DatabaseDialect::PostgreSQL => SqlType::Uuid,
                _ => SqlType::Text,
            },
            QueryParam::Json(_) => match self.dialect {
                DatabaseDialect::PostgreSQL => SqlType::Jsonb,
                _ => SqlType::Text,
            },
            QueryParam::Binary(_) => SqlType::Binary,
            QueryParam::Null => SqlType::Text, // Default for null values
        }
    }

    /// Validate individual parameter
    fn validate_parameter(&self, param: &QueryParam, expected: &ParameterType) -> DatabaseResult<()> {
        match (param, expected) {
            (QueryParam::String(_), ParameterType::String) => Ok(()),
            (QueryParam::Integer(_), ParameterType::Integer) => Ok(()),
            (QueryParam::Float(_), ParameterType::Float) => Ok(()),
            (QueryParam::Boolean(_), ParameterType::Boolean) => Ok(()),
            (QueryParam::Uuid(_), ParameterType::Uuid) => Ok(()),
            (QueryParam::Json(_), ParameterType::Json) => Ok(()),
            (QueryParam::Binary(_), ParameterType::Binary) => Ok(()),
            (QueryParam::Null, ParameterType::Optional(_)) => Ok(()),
            (param, ParameterType::Optional(inner_type)) => {
                self.validate_parameter(param, inner_type)
            }
            _ => Err(DatabaseError::ParameterTypeMismatch {
                expected: format!("{:?}", expected),
                actual: format!("{:?}", param),
            }),
        }
    }
}

impl ParameterBinder for DefaultParameterBinder {
    fn bind_parameters(
        &self,
        sql: &str,
        params: &[QueryParam],
        context: &ParameterBindingContext,
    ) -> DatabaseResult<BoundQuery> {
        let mut bound_sql = sql.to_string();
        let mut bound_parameters = Vec::with_capacity(params.len());

        // Handle different parameter styles
        match context.parameter_style {
            ParameterStyle::Question | ParameterStyle::Dollar => {
                // Replace placeholders if needed
                for (index, param) in params.iter().enumerate() {
                    let placeholder = self.format_placeholder(index);
                    let sql_type = self.map_sql_type(param);
                    
                    bound_parameters.push(BoundParameter {
                        value: param.clone(),
                        sql_type,
                        is_nullable: matches!(param, QueryParam::Null),
                    });

                    // For dollar notation, replace numbered placeholders
                    if self.parameter_style() == ParameterStyle::Dollar {
                        let old_placeholder = format!("${}", index + 1);
                        if !bound_sql.contains(&old_placeholder) {
                            // If placeholder doesn't exist, this might be a named parameter query
                            continue;
                        }
                    }
                }
            }
            ParameterStyle::Named | ParameterStyle::At => {
                // Handle named parameters
                for (name, param) in &context.named_parameters {
                    let placeholder = match context.parameter_style {
                        ParameterStyle::Named => format!(":{}", name),
                        ParameterStyle::At => format!("@{}", name),
                        _ => unreachable!(),
                    };

                    if bound_sql.contains(&placeholder) {
                        let sql_type = self.map_sql_type(param);
                        bound_parameters.push(BoundParameter {
                            value: param.clone(),
                            sql_type,
                            is_nullable: matches!(param, QueryParam::Null),
                        });
                    }
                }
            }
        }

        Ok(BoundQuery {
            sql: bound_sql,
            parameters: bound_parameters,
            parameter_count: params.len(),
        })
    }

    fn convert_named_to_positional(
        &self,
        sql: &str,
        named_params: &HashMap<String, QueryParam>,
    ) -> DatabaseResult<(String, Vec<QueryParam>)> {
        let mut converted_sql = sql.to_string();
        let mut positional_params = Vec::new();
        let mut param_index = 0;

        // Find all named parameter references in the SQL
        let mut names_in_order = Vec::new();
        
        // Simple regex-like approach to find :name patterns
        let mut chars: Vec<char> = sql.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if chars[i] == ':' && i + 1 < chars.len() && chars[i + 1].is_alphabetic() {
                // Found start of named parameter
                let start = i + 1;
                let mut end = start;
                
                while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
                    end += 1;
                }
                
                let param_name: String = chars[start..end].iter().collect();
                names_in_order.push((i, end, param_name));
                i = end;
            } else {
                i += 1;
            }
        }

        // Replace named parameters with positional ones (in reverse order to maintain indices)
        for (start, end, param_name) in names_in_order.into_iter().rev() {
            if let Some(param_value) = named_params.get(&param_name) {
                let placeholder = self.format_placeholder(param_index);
                converted_sql.replace_range(start..end + 1, &placeholder);
                positional_params.insert(0, param_value.clone());
                param_index += 1;
            } else {
                return Err(DatabaseError::MissingNamedParameter {
                    parameter_name: param_name,
                });
            }
        }

        // Reverse to get correct order
        positional_params.reverse();

        Ok((converted_sql, positional_params))
    }

    fn validate_parameters(
        &self,
        params: &[QueryParam],
        expected_types: &[ParameterType],
    ) -> DatabaseResult<()> {
        if params.len() != expected_types.len() {
            return Err(DatabaseError::ParameterCountMismatch {
                expected: expected_types.len(),
                actual: params.len(),
            });
        }

        for (i, (param, expected_type)) in params.iter().zip(expected_types.iter()).enumerate() {
            self.validate_parameter(param, expected_type)
                .map_err(|e| match e {
                    DatabaseError::ParameterTypeMismatch { expected, actual } => {
                        DatabaseError::ParameterTypeMismatchAtIndex {
                            index: i,
                            expected,
                            actual,
                        }
                    }
                    other => other,
                })?;
        }

        Ok(())
    }
}

/// Builder for parameter binding contexts
#[derive(Debug, Default)]
pub struct ParameterBindingContextBuilder {
    dialect: Option<DatabaseDialect>,
    named_parameters: HashMap<String, QueryParam>,
    positional_parameters: Vec<QueryParam>,
    parameter_style: Option<ParameterStyle>,
}

impl ParameterBindingContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dialect(mut self, dialect: DatabaseDialect) -> Self {
        self.dialect = Some(dialect);
        self
    }

    pub fn add_named_parameter<T: Into<QueryParam>>(mut self, name: String, value: T) -> Self {
        self.named_parameters.insert(name, value.into());
        self
    }

    pub fn add_positional_parameter<T: Into<QueryParam>>(mut self, value: T) -> Self {
        self.positional_parameters.push(value.into());
        self
    }

    pub fn parameter_style(mut self, style: ParameterStyle) -> Self {
        self.parameter_style = Some(style);
        self
    }

    pub fn build(self) -> DatabaseResult<ParameterBindingContext> {
        let dialect = self.dialect.ok_or(DatabaseError::ConfigurationError {
            message: "Database dialect must be specified".to_string(),
        })?;

        let parameter_style = self.parameter_style.unwrap_or_else(|| match dialect {
            DatabaseDialect::PostgreSQL => ParameterStyle::Dollar,
            DatabaseDialect::SQLite | DatabaseDialect::MySQL => ParameterStyle::Question,
        });

        Ok(ParameterBindingContext {
            dialect,
            named_parameters: self.named_parameters,
            positional_parameters: self.positional_parameters,
            parameter_style,
        })
    }
}

/// Convenience implementations for converting Rust types to QueryParam
impl From<String> for QueryParam {
    fn from(value: String) -> Self {
        QueryParam::String(value)
    }
}

impl From<&str> for QueryParam {
    fn from(value: &str) -> Self {
        QueryParam::String(value.to_string())
    }
}

impl From<i32> for QueryParam {
    fn from(value: i32) -> Self {
        QueryParam::Integer(value as i64)
    }
}

impl From<i64> for QueryParam {
    fn from(value: i64) -> Self {
        QueryParam::Integer(value)
    }
}

impl From<f32> for QueryParam {
    fn from(value: f32) -> Self {
        QueryParam::Float(value as f64)
    }
}

impl From<f64> for QueryParam {
    fn from(value: f64) -> Self {
        QueryParam::Float(value)
    }
}

impl From<bool> for QueryParam {
    fn from(value: bool) -> Self {
        QueryParam::Boolean(value)
    }
}

impl From<Uuid> for QueryParam {
    fn from(value: Uuid) -> Self {
        QueryParam::Uuid(value)
    }
}

impl From<JsonValue> for QueryParam {
    fn from(value: JsonValue) -> Self {
        QueryParam::Json(value)
    }
}

impl From<Vec<u8>> for QueryParam {
    fn from(value: Vec<u8>) -> Self {
        QueryParam::Binary(value)
    }
}

impl<T: Into<QueryParam>> From<Option<T>> for QueryParam {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => QueryParam::Null,
        }
    }
}

/// Utility functions for parameter handling
pub mod utils {
    use super::*;

    /// Create a named parameter map from key-value pairs
    pub fn named_params<I, K, V>(iter: I) -> HashMap<String, QueryParam>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<QueryParam>,
    {
        iter.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }

    /// Create a positional parameter vector
    pub fn positional_params<I, V>(iter: I) -> Vec<QueryParam>
    where
        I: IntoIterator<Item = V>,
        V: Into<QueryParam>,
    {
        iter.into_iter().map(|v| v.into()).collect()
    }

    /// Helper macro for creating named parameters
    #[macro_export]
    macro_rules! named_params {
        ($($key:expr => $value:expr),* $(,)?) => {
            {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert($key.to_string(), $value.into());
                )*
                map
            }
        };
    }

    /// Helper macro for creating positional parameters
    #[macro_export]
    macro_rules! positional_params {
        ($($value:expr),* $(,)?) => {
            vec![$($value.into()),*]
        };
    }
}

impl fmt::Display for ParameterStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterStyle::Dollar => write!(f, "dollar ($1, $2, ...)"),
            ParameterStyle::Question => write!(f, "question (?, ?, ...)"),
            ParameterStyle::Named => write!(f, "named (:name, :value, ...)"),
            ParameterStyle::At => write!(f, "at (@name, @value, ...)"),
        }
    }
}

impl fmt::Display for SqlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqlType::Text => write!(f, "TEXT"),
            SqlType::Integer => write!(f, "INTEGER"),
            SqlType::BigInteger => write!(f, "BIGINT"),
            SqlType::Real => write!(f, "REAL"),
            SqlType::Boolean => write!(f, "BOOLEAN"),
            SqlType::Uuid => write!(f, "UUID"),
            SqlType::Json => write!(f, "JSON"),
            SqlType::Jsonb => write!(f, "JSONB"),
            SqlType::Binary => write!(f, "BINARY"),
            SqlType::Timestamp => write!(f, "TIMESTAMP"),
            SqlType::Date => write!(f, "DATE"),
            SqlType::Time => write!(f, "TIME"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_conversion() {
        let param: QueryParam = "test".into();
        assert!(matches!(param, QueryParam::String(_)));

        let param: QueryParam = 42i64.into();
        assert!(matches!(param, QueryParam::Integer(42)));

        let param: QueryParam = true.into();
        assert!(matches!(param, QueryParam::Boolean(true)));
    }

    #[test]
    fn test_named_params_macro() {
        let params = named_params! {
            "name" => "Alice",
            "age" => 30,
            "active" => true,
        };

        assert_eq!(params.len(), 3);
        assert!(matches!(params.get("name"), Some(QueryParam::String(_))));
        assert!(matches!(params.get("age"), Some(QueryParam::Integer(30))));
        assert!(matches!(params.get("active"), Some(QueryParam::Boolean(true))));
    }

    #[test]
    fn test_parameter_binding_context_builder() {
        let context = ParameterBindingContextBuilder::new()
            .dialect(DatabaseDialect::PostgreSQL)
            .add_named_parameter("user_id".to_string(), Uuid::new_v4())
            .add_positional_parameter("test")
            .build()
            .unwrap();

        assert_eq!(context.dialect, DatabaseDialect::PostgreSQL);
        assert_eq!(context.named_parameters.len(), 1);
        assert_eq!(context.positional_parameters.len(), 1);
    }

    #[test]
    fn test_parameter_validation() {
        let binder = DefaultParameterBinder::new(DatabaseDialect::PostgreSQL);
        let params = vec![
            QueryParam::String("test".to_string()),
            QueryParam::Integer(42),
        ];
        let expected_types = vec![
            ParameterType::String,
            ParameterType::Integer,
        ];

        assert!(binder.validate_parameters(&params, &expected_types).is_ok());

        let wrong_types = vec![
            ParameterType::Integer,
            ParameterType::String,
        ];

        assert!(binder.validate_parameters(&params, &wrong_types).is_err());
    }
}