//! Asset service implementation

use crate::AssetError;
use mutsea_core::{
    traits::{AssetService as AssetServiceTrait, Service, ServiceHealth, ServiceStatus},
    Asset, AssetId, AssetType, MutseaResult, UserId,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Asset service implementation
pub struct AssetService {
    // In-memory storage for now - would be replaced with proper storage backend
    assets: Arc<RwLock<HashMap<AssetId, Asset>>>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl AssetService {
    /// Create a new asset service
    pub async fn new() -> Result<Self, AssetError> {
        Ok(Self {
            assets: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }
}

#[async_trait::async_trait]
impl AssetServiceTrait for AssetService {
    async fn store_asset(&self, asset: &Asset) -> MutseaResult<AssetId> {
        let mut assets = self.assets.write().await;
        let asset_id = asset.id;
        assets.insert(asset_id, asset.clone());
        Ok(asset_id)
    }
    
    async fn get_asset(&self, asset_id: AssetId) -> MutseaResult<Option<Asset>> {
        let assets = self.assets.read().await;
        Ok(assets.get(&asset_id).cloned())
    }
    
    async fn delete_asset(&self, asset_id: AssetId) -> MutseaResult<()> {
        let mut assets = self.assets.write().await;
        assets.remove(&asset_id);
        Ok(())
    }
    
    async fn asset_exists(&self, asset_id: AssetId) -> MutseaResult<bool> {
        let assets = self.assets.read().await;
        Ok(assets.contains_key(&asset_id))
    }
    
    async fn get_asset_metadata(&self, asset_id: AssetId) -> MutseaResult<Option<mutsea_core::traits::AssetMetadata>> {
        let assets = self.assets.read().await;
        if let Some(asset) = assets.get(&asset_id) {
            Ok(Some(mutsea_core::traits::AssetMetadata {
                id: asset.id,
                asset_type: asset.asset_type,
                name: asset.name.clone(),
                description: asset.description.clone(),
                size: asset.data.len(),
                temporary: asset.temporary,
                local: asset.local,
                created: asset.created,
                creator_id: asset.creator_id,
            }))
        } else {
            Ok(None)
        }
    }
}

#[async_trait::async_trait]
impl Service for AssetService {
    async fn start(&self) -> MutseaResult<()> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
    
    async fn stop(&self) -> MutseaResult<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
    
    fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    async fn health_check(&self) -> ServiceHealth {
        let assets_count = self.assets.read().await.len();
        
        let mut metrics = HashMap::new();
        metrics.insert("assets_count".to_string(), assets_count as f64);
        
        ServiceHealth {
            status: if self.is_running() { ServiceStatus::Healthy } else { ServiceStatus::Unhealthy },
            message: format!("Asset service with {} assets", assets_count),
            metrics,
        }
    }
}