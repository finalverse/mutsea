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
    service: dyn AssetService,
}

impl AssetManager {
    /// Create a new asset manager
    pub async fn new() -> MutseaResult<Self> {
        let service = AssetService::new().await?;
        Ok(Self { service })
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
}