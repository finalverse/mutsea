//! Asset management system for Mutsea

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod service;
pub mod storage;
pub mod cache;
pub mod error;

pub use service::*;
pub use storage::*;
pub use cache::*;
pub use error::*;

use mutsea_core::{Asset, AssetId, AssetType, UserId, MutseaResult, AssetService};

/// Asset management facade
pub struct AssetManager {
    service: Box<dyn AssetService>,
}

impl AssetManager {
    /// Create a new asset manager
    pub async fn new() -> MutseaResult<Self> {
        let service = Box::new(service::AssetService::new().await
            .map_err(|e| mutsea_core::MutseaError::Generic(e.to_string()))?);
        Ok(Self { service })
    }
    
    /// Create asset manager with custom service
    pub fn with_service(service: Box<dyn AssetService>) -> Self {
        Self { service }
    }
    
    /// Store an asset
    pub async fn store_asset(&self, asset: Asset) -> MutseaResult<AssetId> {
        self.service.store_asset(&asset).await
    }
    
    /// Get an asset
    pub async fn get_asset(&self, asset_id: AssetId) -> MutseaResult<Option<Asset>> {
        self.service.get_asset(asset_id).await
    }
    
    /// Delete an asset
    pub async fn delete_asset(&self, asset_id: AssetId) -> MutseaResult<()> {
        self.service.delete_asset(asset_id).await
    }
    
    /// Check if an asset exists
    pub async fn asset_exists(&self, asset_id: AssetId) -> MutseaResult<bool> {
        self.service.asset_exists(asset_id).await
    }
    
    /// Get asset metadata only (without data)
    pub async fn get_asset_metadata(&self, asset_id: AssetId) -> MutseaResult<Option<mutsea_core::traits::AssetMetadata>> {
        self.service.get_asset_metadata(asset_id).await
    }
}