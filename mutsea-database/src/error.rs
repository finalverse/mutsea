// mutsea-database/src/error.rs
use std::fmt;

/// Database-specific error types for the Mutsea AI engine
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseError {
    /// Connection errors
    ConnectionFailed(String),
    ConnectionTimeout,
    ConnectionPoolExhausted,
    
    /// Query errors
    QueryExecutionFailed(String),
    QueryTimeout,
    InvalidQuery(String),
    ParameterBindingFailed(String),
    
    /// Data errors
    SerializationFailed(String),
    DeserializationFailed(String),
    DataIntegrityViolation(String),
    ConstraintViolation(String),
    
    /// AI-specific errors
    AIModelDataCorrupted(String),
    LearningDataInconsistent(String),
    EmergentBehaviorAnalysisFailed(String),
    PerformanceMetricsInvalid(String),
    
    /// Analytics errors
    AnalyticsQueryFailed(String),
    CacheMiss(String),
    CacheCorrupted(String),
    
    /// Migration errors
    MigrationFailed(String),
    SchemaVersionMismatch(String),
    
    /// File system errors
    SqlFileNotFound(String),
    SqlFileReadFailed(String),
    SqlParsingFailed(String),
    
    /// Generic errors
    Unknown(String),
    NotImplemented(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed(msg) => write!(f, "Database connection failed: {}", msg),
            DatabaseError::ConnectionTimeout => write!(f, "Database connection timed out"),
            DatabaseError::ConnectionPoolExhausted => write!(f, "Database connection pool exhausted"),
            
            DatabaseError::QueryExecutionFailed(msg) => write!(f, "Query execution failed: {}", msg),
            DatabaseError::QueryTimeout => write!(f, "Query execution timed out"),
            DatabaseError::InvalidQuery(msg) => write!(f, "Invalid query: {}", msg),
            DatabaseError::ParameterBindingFailed(msg) => write!(f, "Parameter binding failed: {}", msg),
            
            DatabaseError::SerializationFailed(msg) => write!(f, "Data serialization failed: {}", msg),
            DatabaseError::DeserializationFailed(msg) => write!(f, "Data deserialization failed: {}", msg),
            DatabaseError::DataIntegrityViolation(msg) => write!(f, "Data integrity violation: {}", msg),
            DatabaseError::ConstraintViolation(msg) => write!(f, "Database constraint violation: {}", msg),
            
            DatabaseError::AIModelDataCorrupted(msg) => write!(f, "AI model data corrupted: {}", msg),
            DatabaseError::LearningDataInconsistent(msg) => write!(f, "Learning data inconsistent: {}", msg),
            DatabaseError::EmergentBehaviorAnalysisFailed(msg) => write!(f, "Emergent behavior analysis failed: {}", msg),
            DatabaseError::PerformanceMetricsInvalid(msg) => write!(f, "Performance metrics invalid: {}", msg),
            
            DatabaseError::AnalyticsQueryFailed(msg) => write!(f, "Analytics query failed: {}", msg),
            DatabaseError::CacheMiss(msg) => write!(f, "Cache miss: {}", msg),
            DatabaseError::CacheCorrupted(msg) => write!(f, "Cache corrupted: {}", msg),
            
            DatabaseError::MigrationFailed(msg) => write!(f, "Database migration failed: {}", msg),
            DatabaseError::SchemaVersionMismatch(msg) => write!(f, "Schema version mismatch: {}", msg),
            
            DatabaseError::SqlFileNotFound(msg) => write!(f, "SQL file not found: {}", msg),
            DatabaseError::SqlFileReadFailed(msg) => write!(f, "Failed to read SQL file: {}", msg),
            DatabaseError::SqlParsingFailed(msg) => write!(f, "SQL parsing failed: {}", msg),
            
            DatabaseError::Unknown(msg) => write!(f, "Unknown database error: {}", msg),
            DatabaseError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

/// Result type alias for database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// Error context trait for better error handling
pub trait ErrorContext<T> {
    fn with_context(self, msg: &str) -> DatabaseResult<T>;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: fmt::Display,
{
    fn with_context(self, msg: &str) -> DatabaseResult<T> {
        self.map_err(|e| DatabaseError::Unknown(format!("{}: {}", msg, e)))
    }
}

/// Macro for creating context-aware errors
#[macro_export]
macro_rules! db_error {
    ($variant:ident, $msg:expr) => {
        DatabaseError::$variant($msg.to_string())
    };
    ($variant:ident, $fmt:expr, $($arg:tt)*) => {
        DatabaseError::$variant(format!($fmt, $($arg)*))
    };
}

/// Helper functions for common error patterns
impl DatabaseError {
    pub fn connection_failed<S: Into<String>>(msg: S) -> Self {
        DatabaseError::ConnectionFailed(msg.into())
    }
    
    pub fn query_failed<S: Into<String>>(msg: S) -> Self {
        DatabaseError::QueryExecutionFailed(msg.into())
    }
    
    pub fn serialization_failed<S: Into<String>>(msg: S) -> Self {
        DatabaseError::SerializationFailed(msg.into())
    }
    
    pub fn ai_data_corrupted<S: Into<String>>(msg: S) -> Self {
        DatabaseError::AIModelDataCorrupted(msg.into())
    }
    
    pub fn analytics_failed<S: Into<String>>(msg: S) -> Self {
        DatabaseError::AnalyticsQueryFailed(msg.into())
    }
    
    pub fn sql_file_error<S: Into<String>>(msg: S) -> Self {
        DatabaseError::SqlFileNotFound(msg.into())
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            DatabaseError::ConnectionTimeout
                | DatabaseError::QueryTimeout
                | DatabaseError::CacheMiss(_)
                | DatabaseError::ConnectionPoolExhausted
        )
    }
    
    /// Check if error is related to AI systems
    pub fn is_ai_related(&self) -> bool {
        matches!(
            self,
            DatabaseError::AIModelDataCorrupted(_)
                | DatabaseError::LearningDataInconsistent(_)
                | DatabaseError::EmergentBehaviorAnalysisFailed(_)
                | DatabaseError::PerformanceMetricsInvalid(_)
        )
    }
    
    /// Get error category for logging and metrics
    pub fn category(&self) -> &'static str {
        match self {
            DatabaseError::ConnectionFailed(_)
            | DatabaseError::ConnectionTimeout
            | DatabaseError::ConnectionPoolExhausted => "connection",
            
            DatabaseError::QueryExecutionFailed(_)
            | DatabaseError::QueryTimeout
            | DatabaseError::InvalidQuery(_)
            | DatabaseError::ParameterBindingFailed(_) => "query",
            
            DatabaseError::SerializationFailed(_)
            | DatabaseError::DeserializationFailed(_)
            | DatabaseError::DataIntegrityViolation(_)
            | DatabaseError::ConstraintViolation(_) => "data",
            
            DatabaseError::AIModelDataCorrupted(_)
            | DatabaseError::LearningDataInconsistent(_)
            | DatabaseError::EmergentBehaviorAnalysisFailed(_)
            | DatabaseError::PerformanceMetricsInvalid(_) => "ai",
            
            DatabaseError::AnalyticsQueryFailed(_)
            | DatabaseError::CacheMiss(_)
            | DatabaseError::CacheCorrupted(_) => "analytics",
            
            DatabaseError::MigrationFailed(_)
            | DatabaseError::SchemaVersionMismatch(_) => "migration",
            
            DatabaseError::SqlFileNotFound(_)
            | DatabaseError::SqlFileReadFailed(_)
            | DatabaseError::SqlParsingFailed(_) => "filesystem",
            
            DatabaseError::Unknown(_)
            | DatabaseError::NotImplemented(_) => "generic",
        }
    }
}