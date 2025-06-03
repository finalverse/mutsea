//! Core traits for Mutsea components

use crate::{AssetId, MutseaResult, ObjectId, RegionId, UserId};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for services that can be started and stopped
#[async_trait]
pub trait Service: Send + Sync {
    /// Start the service
    async fn start(&self) -> MutseaResult<()>;

    /// Stop the service gracefully
    async fn stop(&self) -> MutseaResult<()>;

    /// Check if the service is running
    fn is_running(&self) -> bool;

    /// Get service health status
    async fn health_check(&self) -> ServiceHealth;
}

/// Service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: ServiceStatus,
    pub message: String,
    pub metrics: HashMap<String, f64>,
}

/// Service status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Trait for user management services
#[async_trait]
pub trait UserService: Service {
    /// Authenticate a user by name and password
    async fn authenticate(
        &self,
        first_name: &str,
        last_name: &str,
        password: &str,
    ) -> MutseaResult<Option<UserId>>;

    /// Create a new user account
    async fn create_user(
        &self,
        first_name: &str,
        last_name: &str,
        email: Option<&str>,
        password: &str,
    ) -> MutseaResult<UserId>;

    /// Get user information
    async fn get_user(&self, user_id: UserId) -> MutseaResult<Option<crate::UserAccount>>;

    /// Update user information
    async fn update_user(&self, user_account: &crate::UserAccount) -> MutseaResult<()>;

    /// Delete a user account
    async fn delete_user(&self, user_id: UserId) -> MutseaResult<()>;

    /// Find user by name
    async fn find_user_by_name(
        &self,
        first_name: &str,
        last_name: &str,
    ) -> MutseaResult<Option<UserId>>;
}

/// Trait for asset management services
#[async_trait]
pub trait AssetService: Service {
    /// Store an asset
    async fn store_asset(&self, asset: &crate::Asset) -> MutseaResult<AssetId>;

    /// Retrieve an asset
    async fn get_asset(&self, asset_id: AssetId) -> MutseaResult<Option<crate::Asset>>;

    /// Delete an asset
    async fn delete_asset(&self, asset_id: AssetId) -> MutseaResult<()>;

    /// Check if an asset exists
    async fn asset_exists(&self, asset_id: AssetId) -> MutseaResult<bool>;

    /// Get asset metadata only (without data)
    async fn get_asset_metadata(&self, asset_id: AssetId) -> MutseaResult<Option<AssetMetadata>>;
}

/// Asset metadata without the actual data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub id: AssetId,
    pub asset_type: crate::AssetType,
    pub name: String,
    pub description: String,
    pub size: usize,
    pub temporary: bool,
    pub local: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    pub creator_id: UserId,
}

/// Trait for region management services
#[async_trait]
pub trait RegionService: Service {
    /// Register a new region
    async fn register_region(&self, region_info: &crate::RegionInfo) -> MutseaResult<RegionId>;

    /// Get region information
    async fn get_region(&self, region_id: RegionId) -> MutseaResult<Option<crate::RegionInfo>>;

    /// Update region information
    async fn update_region(&self, region_info: &crate::RegionInfo) -> MutseaResult<()>;

    /// Deregister a region
    async fn deregister_region(&self, region_id: RegionId) -> MutseaResult<()>;

    /// Find region by name
    async fn find_region_by_name(&self, name: &str) -> MutseaResult<Option<RegionId>>;

    /// Get all regions
    async fn get_all_regions(&self) -> MutseaResult<Vec<crate::RegionInfo>>;

    /// Get regions by location
    async fn get_regions_by_location(
        &self,
        x_min: u32,
        y_min: u32,
        x_max: u32,
        y_max: u32,
    ) -> MutseaResult<Vec<crate::RegionInfo>>;
}

/// Trait for caching services
#[async_trait]
pub trait CacheService: Service {
    /// Store a value in cache
    async fn set<T>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> MutseaResult<()>
    where
        T: Serialize + Send + Sync;

    /// Retrieve a value from cache
    async fn get<T>(&self, key: &str) -> MutseaResult<Option<T>>
    where
        T: for<'de> Deserialize<'de> + Send + Sync;

    /// Delete a value from cache
    async fn delete(&self, key: &str) -> MutseaResult<()>;

    /// Check if a key exists in cache
    async fn exists(&self, key: &str) -> MutseaResult<bool>;

    /// Clear all cache entries (use with caution)
    async fn clear(&self) -> MutseaResult<()>;
}

/// Trait for database services
#[async_trait]
pub trait DatabaseService: Service {
    /// Execute a query and return results
    async fn query<T>(&self, query: &str, params: &[&dyn std::fmt::Debug]) -> MutseaResult<Vec<T>>
    where
        T: for<'de> Deserialize<'de> + Send + Sync;

    /// Execute a query that returns a single result
    async fn query_one<T>(
        &self,
        query: &str,
        params: &[&dyn std::fmt::Debug],
    ) -> MutseaResult<Option<T>>
    where
        T: for<'de> Deserialize<'de> + Send + Sync;

    /// Execute a query that doesn't return results
    async fn execute(&self, query: &str, params: &[&dyn std::fmt::Debug]) -> MutseaResult<u64>;

    /// Begin a transaction
    async fn begin_transaction(&self) -> MutseaResult<Box<dyn Transaction + Send>>;
}

/// Database transaction trait
#[async_trait]
pub trait Transaction: Send {
    /// Execute a query within the transaction
    async fn execute(&mut self, query: &str, params: &[&dyn std::fmt::Debug]) -> MutseaResult<u64>;

    /// Commit the transaction
    async fn commit(self: Box<Self>) -> MutseaResult<()>;

    /// Rollback the transaction
    async fn rollback(self: Box<Self>) -> MutseaResult<()>;
}

/// Trait for event publishing and subscription
#[async_trait]
pub trait EventBus: Service {
    /// Publish an event
    async fn publish<T>(&self, topic: &str, event: &T) -> MutseaResult<()>
    where
        T: Serialize + Send + Sync;

    /// Subscribe to events on a topic
    async fn subscribe<T, F>(&self, topic: &str, handler: F) -> MutseaResult<()>
    where
        T: for<'de> Deserialize<'de> + Send + Sync + 'static,
        F: Fn(T) -> MutseaResult<()> + Send + Sync + 'static;

    /// Unsubscribe from a topic
    async fn unsubscribe(&self, topic: &str) -> MutseaResult<()>;
}

/// Trait for metrics collection
pub trait MetricsCollector: Send + Sync {
    /// Increment a counter
    fn increment_counter(&self, name: &str, labels: &[(&str, &str)]);

    /// Record a histogram value
    fn record_histogram(&self, name: &str, value: f64, labels: &[(&str, &str)]);

    /// Set a gauge value
    fn set_gauge(&self, name: &str, value: f64, labels: &[(&str, &str)]);

    /// Record timing information
    fn record_timing(&self, name: &str, duration: std::time::Duration, labels: &[(&str, &str)]);
}

/// Trait for configuration management
pub trait ConfigManager: Send + Sync {
    /// Get a configuration value
    fn get_string(&self, key: &str) -> Option<String>;

    /// Get a configuration value with default
    fn get_string_or(&self, key: &str, default: &str) -> String {
        self.get_string(key).unwrap_or_else(|| default.to_string())
    }

    /// Get an integer configuration value
    fn get_int(&self, key: &str) -> Option<i64>;

    /// Get an integer configuration value with default
    fn get_int_or(&self, key: &str, default: i64) -> i64 {
        self.get_int(key).unwrap_or(default)
    }

    /// Get a boolean configuration value
    fn get_bool(&self, key: &str) -> Option<bool>;

    /// Get a boolean configuration value with default
    fn get_bool_or(&self, key: &str, default: bool) -> bool {
        self.get_bool(key).unwrap_or(default)
    }

    /// Get a float configuration value
    fn get_float(&self, key: &str) -> Option<f64>;

    /// Get a float configuration value with default
    fn get_float_or(&self, key: &str, default: f64) -> f64 {
        self.get_float(key).unwrap_or(default)
    }
}
