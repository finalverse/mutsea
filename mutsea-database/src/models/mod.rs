// mutsea-database/src/models/mod.rs
//! Data models for the Mutsea AI-driven database system
//! 
//! This module contains all the data structures used to represent
//! world state, AI decisions, player behavior, and other core
//! entities in the Mutsea AI engine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod world_state;
pub mod player_behavior;
pub mod ai_decision;
pub mod emergent_behavior;
pub mod npc_state;
pub mod ecosystem_state;
pub mod learning_data;
pub mod performance_metrics;

// Re-export all models for convenience
pub use world_state::*;
pub use player_behavior::*;
pub use ai_decision::*;
pub use emergent_behavior::*;
pub use npc_state::*;
pub use ecosystem_state::*;
pub use learning_data::*;
pub use performance_metrics::*;

/// Common ID types used throughout the system
pub type EntityId = Uuid;
pub type PlayerId = Uuid;
pub type SessionId = Uuid;
pub type WorldId = Uuid;
pub type BiomeId = Uuid;
pub type QuestId = Uuid;
pub type NPCId = Uuid;

/// Timestamp type for consistent time handling
pub type Timestamp = DateTime<Utc>;

/// Common traits for all database models
pub trait DatabaseModel {
    /// Get the unique identifier for this model
    fn id(&self) -> EntityId;
    
    /// Get the creation timestamp
    fn created_at(&self) -> Timestamp;
    
    /// Get the last update timestamp
    fn updated_at(&self) -> Timestamp;
    
    /// Validate the model data
    fn validate(&self) -> Result<(), ValidationError>;
}

/// AI-specific model trait
pub trait AIModel: DatabaseModel {
    /// Get AI confidence score (0.0 to 1.0)
    fn confidence_score(&self) -> f32;
    
    /// Get AI decision context
    fn ai_context(&self) -> &AIContext;
    
    /// Check if this model contains learning data
    fn has_learning_data(&self) -> bool;
}

/// Validation error for model data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub code: ValidationErrorCode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationErrorCode {
    Required,
    InvalidFormat,
    OutOfRange,
    InvalidLength,
    InvalidValue,
    AIDataInconsistent,
    TimestampInvalid,
}

/// Common AI context information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIContext {
    pub model_version: String,
    pub decision_algorithm: String,
    pub training_data_hash: Option<String>,
    pub confidence_threshold: f32,
    pub processing_time_ms: u64,
    pub resource_usage: ResourceUsage,
}

/// Resource usage metrics for AI operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub gpu_usage_percent: Option<f32>,
    pub network_io_bytes: u64,
    pub disk_io_bytes: u64,
}

/// Coordinate system for world positions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub chunk_id: Option<EntityId>,
    pub region_id: Option<EntityId>,
}

impl WorldPosition {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            chunk_id: None,
            region_id: None,
        }
    }
    
    pub fn distance_to(&self, other: &WorldPosition) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Vector3 for various 3D calculations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self::new(self.x / mag, self.y / mag, self.z / mag)
        } else {
            Self::zero()
        }
    }
}

/// Common metadata for all entities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityMetadata {
    pub tags: Vec<String>,
    pub properties: HashMap<String, String>,
    pub version: u32,
    pub is_active: bool,
    pub created_by: Option<EntityId>,
    pub modified_by: Option<EntityId>,
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self {
            tags: Vec::new(),
            properties: HashMap::new(),
            version: 1,
            is_active: true,
            created_by: None,
            modified_by: None,
        }
    }
}

/// Pagination parameters for queries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: u64,
    pub limit: u64,
    pub total_count: Option<u64>,
}

impl Pagination {
    pub fn new(offset: u64, limit: u64) -> Self {
        Self {
            offset,
            limit,
            total_count: None,
        }
    }
    
    pub fn with_total(offset: u64, limit: u64, total: u64) -> Self {
        Self {
            offset,
            limit,
            total_count: Some(total),
        }
    }
    
    pub fn page_count(&self) -> Option<u64> {
        self.total_count.map(|total| (total + self.limit - 1) / self.limit)
    }
}

/// Sorting parameters for queries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sorting {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Sorting {
    pub fn asc(field: &str) -> Self {
        Self {
            field: field.to_string(),
            direction: SortDirection::Ascending,
        }
    }
    
    pub fn desc(field: &str) -> Self {
        Self {
            field: field.to_string(),
            direction: SortDirection::Descending,
        }
    }
}

/// Filter parameters for queries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: FilterValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
    IsNull,
    IsNotNull,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<FilterValue>),
    Null,
}

/// Query parameters combining pagination, sorting, and filtering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryParams {
    pub pagination: Option<Pagination>,
    pub sorting: Vec<Sorting>,
    pub filters: Vec<Filter>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self {
            pagination: None,
            sorting: Vec::new(),
            filters: Vec::new(),
        }
    }
    
    pub fn with_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }
    
    pub fn with_sort(mut self, sorting: Sorting) -> Self {
        self.sorting.push(sorting);
        self
    }
    
    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }
}

impl Default for QueryParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Bulk operation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BulkOperationResult {
    pub total_attempted: u64,
    pub successful: u64,
    pub failed: u64,
    pub errors: Vec<BulkOperationError>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BulkOperationError {
    pub index: u64,
    pub error: String,
    pub entity_id: Option<EntityId>,
}

/// Health check result for database systems
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub status: HealthStatus,
    pub response_time_ms: u64,
    pub details: HashMap<String, String>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Macro for implementing common DatabaseModel traits
#[macro_export]
macro_rules! impl_database_model {
    ($struct_name:ident, $id_field:ident, $created_field:ident, $updated_field:ident) => {
        impl DatabaseModel for $struct_name {
            fn id(&self) -> EntityId {
                self.$id_field
            }
            
            fn created_at(&self) -> Timestamp {
                self.$created_field
            }
            
            fn updated_at(&self) -> Timestamp {
                self.$updated_field
            }
            
            fn validate(&self) -> Result<(), ValidationError> {
                // Default validation - can be overridden
                Ok(())
            }
        }
    };
}