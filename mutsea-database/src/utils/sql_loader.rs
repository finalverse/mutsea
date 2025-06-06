// mutsea-database/src/utils/sql_loader.rs

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::error::{DatabaseError, DatabaseResult};
use crate::traits::query_builder::DatabaseDialect;

/// Global SQL cache for loaded queries
static SQL_CACHE: Lazy<Arc<RwLock<HashMap<String, String>>>> = 
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

/// SQL loader configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlLoaderConfig {
    /// Base directory for SQL files
    pub sql_base_path: PathBuf,
    /// Whether to cache loaded SQL
    pub enable_caching: bool,
    /// Whether to validate SQL syntax on load
    pub validate_syntax: bool,
    /// Maximum cache size (number of queries)
    pub max_cache_size: usize,
    /// Whether to watch files for changes in development
    pub watch_files: bool,
}

impl Default for SqlLoaderConfig {
    fn default() -> Self {
        Self {
            sql_base_path: PathBuf::from("sql"),
            enable_caching: true,
            validate_syntax: false, // Set to true in development
            max_cache_size: 1000,
            watch_files: cfg!(debug_assertions),
        }
    }
}

/// SQL file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlFileMetadata {
    pub file_path: PathBuf,
    pub dialect: DatabaseDialect,
    pub category: String,
    pub operation: String,
    pub description: Option<String>,
    pub parameters: Vec<SqlParameter>,
    pub return_type: Option<String>,
    pub last_modified: std::time::SystemTime,
}

/// SQL parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub default_value: Option<String>,
}

/// SQL loader for managing external SQL files
#[derive(Debug)]
pub struct SqlLoader {
    config: SqlLoaderConfig,
    metadata_cache: Arc<RwLock<HashMap<String, SqlFileMetadata>>>,
}

impl SqlLoader {
    /// Create a new SQL loader with default configuration
    pub fn new() -> Self {
        Self::with_config(SqlLoaderConfig::default())
    }

    /// Create a new SQL loader with custom configuration
    pub fn with_config(config: SqlLoaderConfig) -> Self {
        Self {
            config,
            metadata_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load SQL content from file
    pub fn load_sql(&self, dialect: DatabaseDialect, category: &str, operation: &str) -> DatabaseResult<String> {
        let cache_key = self.generate_cache_key(dialect, category, operation);
        
        // Check cache first if enabled
        if self.config.enable_caching {
            if let Some(cached_sql) = self.get_from_cache(&cache_key)? {
                return Ok(cached_sql);
            }
        }

        // Load from file
        let file_path = self.build_file_path(dialect, category, operation);
        let sql_content = self.load_sql_file(&file_path)?;

        // Validate syntax if enabled
        if self.config.validate_syntax {
            self.validate_sql_syntax(&sql_content, dialect)?;
        }

        // Cache the result if enabled
        if self.config.enable_caching {
            self.store_in_cache(cache_key, sql_content.clone())?;
        }

        Ok(sql_content)
    }

    /// Load SQL with parameter substitution
    pub fn load_sql_with_params(
        &self,
        dialect: DatabaseDialect,
        category: &str,
        operation: &str,
        params: &HashMap<String, String>,
    ) -> DatabaseResult<String> {
        let mut sql = self.load_sql(dialect, category, operation)?;
        
        // Substitute parameters
        for (key, value) in params {
            let placeholder = format!("{{{{ {} }}}}", key);
            sql = sql.replace(&placeholder, value);
        }

        // Validate that all required parameters were substituted
        self.validate_parameter_substitution(&sql)?;

        Ok(sql)
    }

    /// Load SQL metadata from header comments
    pub fn load_sql_metadata(&self, dialect: DatabaseDialect, category: &str, operation: &str) -> DatabaseResult<SqlFileMetadata> {
        let cache_key = format!("meta_{}", self.generate_cache_key(dialect, category, operation));
        
        // Check metadata cache
        {
            let cache = self.metadata_cache.read().map_err(|_| DatabaseError::CacheError("Failed to read metadata cache".to_string()))?;
            if let Some(metadata) = cache.get(&cache_key) {
                return Ok(metadata.clone());
            }
        }

        // Load and parse metadata
        let file_path = self.build_file_path(dialect, category, operation);
        let metadata = self.parse_sql_metadata(&file_path)?;

        // Cache metadata
        {
            let mut cache = self.metadata_cache.write().map_err(|_| DatabaseError::CacheError("Failed to write metadata cache".to_string()))?;
            cache.insert(cache_key, metadata.clone());
        }

        Ok(metadata)
    }

    /// Batch load multiple SQL files
    pub fn batch_load_sql(&self, queries: &[(DatabaseDialect, &str, &str)]) -> DatabaseResult<HashMap<String, String>> {
        let mut results = HashMap::new();
        
        for (dialect, category, operation) in queries {
            let cache_key = self.generate_cache_key(*dialect, category, operation);
            let sql = self.load_sql(*dialect, category, operation)?;
            results.insert(cache_key, sql);
        }

        Ok(results)
    }

    /// Preload SQL files for better performance
    pub fn preload_sql_files(&self, dialect: DatabaseDialect) -> DatabaseResult<usize> {
        let dialect_path = self.config.sql_base_path.join(self.dialect_to_string(dialect));
        let mut loaded_count = 0;

        self.preload_directory(&dialect_path, &mut loaded_count)?;
        
        Ok(loaded_count)
    }

    /// Clear SQL cache
    pub fn clear_cache(&self) -> DatabaseResult<()> {
        let mut cache = SQL_CACHE.write().map_err(|_| DatabaseError::CacheError("Failed to clear cache".to_string()))?;
        cache.clear();
        
        let mut meta_cache = self.metadata_cache.write().map_err(|_| DatabaseError::CacheError("Failed to clear metadata cache".to_string()))?;
        meta_cache.clear();
        
        Ok(())
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> DatabaseResult<CacheStats> {
        let cache = SQL_CACHE.read().map_err(|_| DatabaseError::CacheError("Failed to read cache".to_string()))?;
        let meta_cache = self.metadata_cache.read().map_err(|_| DatabaseError::CacheError("Failed to read metadata cache".to_string()))?;
        
        Ok(CacheStats {
            sql_cache_size: cache.len(),
            metadata_cache_size: meta_cache.len(),
            max_cache_size: self.config.max_cache_size,
            cache_hit_ratio: 0.0, // Would need additional tracking for accurate ratio
        })
    }

    /// Refresh a specific SQL file in cache
    pub fn refresh_sql(&self, dialect: DatabaseDialect, category: &str, operation: &str) -> DatabaseResult<String> {
        let cache_key = self.generate_cache_key(dialect, category, operation);
        
        // Remove from cache
        {
            let mut cache = SQL_CACHE.write().map_err(|_| DatabaseError::CacheError("Failed to write cache".to_string()))?;
            cache.remove(&cache_key);
        }

        // Reload
        self.load_sql(dialect, category, operation)
    }

    // Private helper methods

    fn generate_cache_key(&self, dialect: DatabaseDialect, category: &str, operation: &str) -> String {
        format!("{}_{}", self.dialect_to_string(dialect), self.build_relative_path(category, operation))
    }

    fn build_file_path(&self, dialect: DatabaseDialect, category: &str, operation: &str) -> PathBuf {
        self.config.sql_base_path
            .join(self.dialect_to_string(dialect))
            .join(category)
            .join(format!("{}.sql", operation))
    }

    fn build_relative_path(&self, category: &str, operation: &str) -> String {
        format!("{}/{}", category, operation)
    }

    fn dialect_to_string(&self, dialect: DatabaseDialect) -> &'static str {
        match dialect {
            DatabaseDialect::PostgreSQL => "postgresql",
            DatabaseDialect::SQLite => "sqlite",
            DatabaseDialect::MySQL => "mysql",
        }
    }

    fn load_sql_file(&self, file_path: &Path) -> DatabaseResult<String> {
        if !file_path.exists() {
            return Err(DatabaseError::SqlFileNotFound(file_path.to_string_lossy().to_string()));
        }

        fs::read_to_string(file_path)
            .map_err(|e| DatabaseError::IoError(format!("Failed to read SQL file {}: {}", file_path.display(), e)))
    }

    fn get_from_cache(&self, cache_key: &str) -> DatabaseResult<Option<String>> {
        let cache = SQL_CACHE.read().map_err(|_| DatabaseError::CacheError("Failed to read cache".to_string()))?;
        Ok(cache.get(cache_key).cloned())
    }

    fn store_in_cache(&self, cache_key: String, sql_content: String) -> DatabaseResult<()> {
        let mut cache = SQL_CACHE.write().map_err(|_| DatabaseError::CacheError("Failed to write cache".to_string()))?;
        
        // Check cache size limit
        if cache.len() >= self.config.max_cache_size {
            // Simple LRU-like behavior: remove oldest entries
            // In a production system, you might want a more sophisticated LRU implementation
            if cache.len() > self.config.max_cache_size * 3 / 4 {
                cache.clear();
            }
        }
        
        cache.insert(cache_key, sql_content);
        Ok(())
    }

    fn validate_sql_syntax(&self, _sql: &str, _dialect: DatabaseDialect) -> DatabaseResult<()> {
        // Basic SQL validation - in a real implementation, you might use a proper SQL parser
        // For now, just check for basic syntax issues
        
        // TODO: Implement proper SQL syntax validation based on dialect
        // This could use libraries like sqlparser-rs or custom validation logic
        
        Ok(())
    }

    fn validate_parameter_substitution(&self, sql: &str) -> DatabaseResult<()> {
        // Check for remaining unsubstituted parameters
        if sql.contains("{{") && sql.contains("}}") {
            // Find unsubstituted parameters
            let mut unsubstituted = Vec::new();
            let mut chars = sql.chars().peekable();
            
            while let Some(ch) = chars.next() {
                if ch == '{' && chars.peek() == Some(&'{') {
                    chars.next(); // consume second '{'
                    let mut param = String::new();
                    
                    // Extract parameter name
                    while let Some(ch) = chars.next() {
                        if ch == '}' && chars.peek() == Some(&'}') {
                            chars.next(); // consume second '}'
                            unsubstituted.push(param.trim().to_string());
                            break;
                        }
                        param.push(ch);
                    }
                }
            }
            
            if !unsubstituted.is_empty() {
                return Err(DatabaseError::ParameterSubstitutionError(
                    format!("Unsubstituted parameters: {}", unsubstituted.join(", "))
                ));
            }
        }
        
        Ok(())
    }

    fn parse_sql_metadata(&self, file_path: &Path) -> DatabaseResult<SqlFileMetadata> {
        let content = self.load_sql_file(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        let mut metadata = SqlFileMetadata {
            file_path: file_path.to_path_buf(),
            dialect: DatabaseDialect::PostgreSQL, // Default, should be determined from path
            category: String::new(),
            operation: String::new(),
            description: None,
            parameters: Vec::new(),
            return_type: None,
            last_modified: fs::metadata(file_path)
                .map_err(|e| DatabaseError::IoError(format!("Failed to get file metadata: {}", e)))?
                .modified()
                .map_err(|e| DatabaseError::IoError(format!("Failed to get modification time: {}", e)))?,
        };

        // Parse header comments for metadata
        for line in lines.iter().take(50) { // Only check first 50 lines for metadata
            let line = line.trim();
            
            if line.starts_with("-- @description:") {
                metadata.description = Some(line.trim_start_matches("-- @description:").trim().to_string());
            } else if line.starts_with("-- @param:") {
                if let Some(param) = self.parse_parameter_line(line)? {
                    metadata.parameters.push(param);
                }
            } else if line.starts_with("-- @returns:") {
                metadata.return_type = Some(line.trim_start_matches("-- @returns:").trim().to_string());
            } else if line.starts_with("-- @category:") {
                metadata.category = line.trim_start_matches("-- @category:").trim().to_string();
            } else if line.starts_with("-- @operation:") {
                metadata.operation = line.trim_start_matches("-- @operation:").trim().to_string();
            }
        }

        // Extract category and operation from file path if not specified in comments
        if metadata.category.is_empty() || metadata.operation.is_empty() {
            if let Some(parent) = file_path.parent() {
                if let Some(category) = parent.file_name() {
                    metadata.category = category.to_string_lossy().to_string();
                }
            }
            
            if let Some(file_stem) = file_path.file_stem() {
                metadata.operation = file_stem.to_string_lossy().to_string();
            }
        }

        Ok(metadata)
    }

    fn parse_parameter_line(&self, line: &str) -> DatabaseResult<Option<SqlParameter>> {
        // Parse format: -- @param: name:type:required:description
        let param_str = line.trim_start_matches("-- @param:").trim();
        let parts: Vec<&str> = param_str.split(':').collect();
        
        if parts.len() >= 2 {
            Ok(Some(SqlParameter {
                name: parts[0].trim().to_string(),
                param_type: parts[1].trim().to_string(),
                required: parts.get(2).map_or(true, |s| s.trim().to_lowercase() == "required"),
                description: parts.get(3).map(|s| s.trim().to_string()),
                default_value: parts.get(4).map(|s| s.trim().to_string()),
            }))
        } else {
            Ok(None)
        }
    }

    fn preload_directory(&self, dir_path: &Path, loaded_count: &mut usize) -> DatabaseResult<()> {
        if !dir_path.exists() {
            return Ok(());
        }

        let entries = fs::read_dir(dir_path)
            .map_err(|e| DatabaseError::IoError(format!("Failed to read directory {}: {}", dir_path.display(), e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| DatabaseError::IoError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                self.preload_directory(&path, loaded_count)?;
            } else if path.extension().map_or(false, |ext| ext == "sql") {
                // Extract category and operation from path
                if let (Some(category), Some(operation)) = (
                    path.parent().and_then(|p| p.file_name()).map(|n| n.to_string_lossy()),
                    path.file_stem().map(|n| n.to_string_lossy())
                ) {
                    // Determine dialect from path
                    let dialect = if path.ancestors().any(|p| p.file_name().map_or(false, |n| n == "postgresql")) {
                        DatabaseDialect::PostgreSQL
                    } else if path.ancestors().any(|p| p.file_name().map_or(false, |n| n == "sqlite")) {
                        DatabaseDialect::SQLite
                    } else if path.ancestors().any(|p| p.file_name().map_or(false, |n| n == "mysql")) {
                        DatabaseDialect::MySQL
                    } else {
                        DatabaseDialect::PostgreSQL // Default
                    };

                    match self.load_sql(dialect, &category, &operation) {
                        Ok(_) => *loaded_count += 1,
                        Err(e) => {
                            // Log error but continue preloading
                            eprintln!("Warning: Failed to preload SQL file {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub sql_cache_size: usize,
    pub metadata_cache_size: usize,
    pub max_cache_size: usize,
    pub cache_hit_ratio: f64,
}

/// SQL template processor for dynamic SQL generation
#[derive(Debug)]
pub struct SqlTemplateProcessor {
    loader: SqlLoader,
}

impl SqlTemplateProcessor {
    pub fn new(loader: SqlLoader) -> Self {
        Self { loader }
    }

    /// Process SQL template with dynamic conditions
    pub fn process_template(
        &self,
        dialect: DatabaseDialect,
        category: &str,
        operation: &str,
        template_params: &SqlTemplateParams,
    ) -> DatabaseResult<String> {
        let base_sql = self.loader.load_sql(dialect, category, operation)?;
        let mut processed_sql = base_sql;

        // Process conditional blocks
        processed_sql = self.process_conditional_blocks(&processed_sql, template_params)?;
        
        // Process parameter substitution
        processed_sql = self.process_parameter_substitution(&processed_sql, &template_params.parameters)?;
        
        // Process dynamic table names
        processed_sql = self.process_dynamic_tables(&processed_sql, &template_params.table_mappings)?;

        Ok(processed_sql)
    }

    fn process_conditional_blocks(&self, sql: &str, params: &SqlTemplateParams) -> DatabaseResult<String> {
        let mut result = sql.to_string();
        
        // Process {{#if condition}} ... {{/if}} blocks
        // This is a simplified implementation - a production system might use a proper template engine
        for (condition, value) in &params.conditions {
            let if_block = format!("{{{{#if {}}}}}", condition);
            let endif_block = "{{/if}}".to_string();
            
            if let (Some(start), Some(end)) = (result.find(&if_block), result.find(&endif_block)) {
                if start < end {
                    let block_content = &result[start + if_block.len()..end];
                    let replacement = if *value { block_content } else { "" };
                    result = result.replace(&format!("{}{}{}", if_block, block_content, endif_block), replacement);
                }
            }
        }

        Ok(result)
    }

    fn process_parameter_substitution(&self, sql: &str, params: &HashMap<String, String>) -> DatabaseResult<String> {
        let mut result = sql.to_string();
        
        for (key, value) in params {
            let placeholder = format!("{{{{ {} }}}}", key);
            result = result.replace(&placeholder, value);
        }

        Ok(result)
    }

    fn process_dynamic_tables(&self, sql: &str, table_mappings: &HashMap<String, String>) -> DatabaseResult<String> {
        let mut result = sql.to_string();
        
        for (logical_table, physical_table) in table_mappings {
            let placeholder = format!("{{{{ table.{} }}}}", logical_table);
            result = result.replace(&placeholder, physical_table);
        }

        Ok(result)
    }
}

/// Parameters for SQL template processing
#[derive(Debug, Clone, Default)]
pub struct SqlTemplateParams {
    pub parameters: HashMap<String, String>,
    pub conditions: HashMap<String, bool>,
    pub table_mappings: HashMap<String, String>,
}

impl SqlTemplateParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }

    pub fn with_condition(mut self, condition: impl Into<String>, value: bool) -> Self {
        self.conditions.insert(condition.into(), value);
        self
    }

    pub fn with_table_mapping(mut self, logical: impl Into<String>, physical: impl Into<String>) -> Self {
        self.table_mappings.insert(logical.into(), physical.into());
        self
    }
}