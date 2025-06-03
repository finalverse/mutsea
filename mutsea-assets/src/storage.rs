//! Asset storage backends

use crate::AssetError;
use mutsea_core::{Asset, AssetId};
use std::path::PathBuf;

/// Asset storage backend trait
#[async_trait::async_trait]
pub trait AssetStorage: Send + Sync {
    async fn store(&self, asset: &Asset) -> Result<(), AssetError>;
    async fn retrieve(&self, asset_id: AssetId) -> Result<Option<Asset>, AssetError>;
    async fn delete(&self, asset_id: AssetId) -> Result<(), AssetError>;
    async fn exists(&self, asset_id: AssetId) -> Result<bool, AssetError>;
}

/// Local file system storage
pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    /// Create a new local storage backend
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
    
    fn asset_path(&self, asset_id: AssetId) -> PathBuf {
        let id_str = asset_id.to_string();
        self.base_path.join(&id_str[0..2]).join(&id_str[2..4]).join(&id_str)
    }
}

#[async_trait::async_trait]
impl AssetStorage for LocalStorage {
    async fn store(&self, asset: &Asset) -> Result<(), AssetError> {
        let path = self.asset_path(asset.id);
        
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let serialized = bincode::serialize(asset)
            .map_err(|e| AssetError::Serialization(e.to_string()))?;
            
        tokio::fs::write(&path, serialized).await?;
        Ok(())
    }
    
    async fn retrieve(&self, asset_id: AssetId) -> Result<Option<Asset>, AssetError> {
        let path = self.asset_path(asset_id);
        
        match tokio::fs::read(&path).await {
            Ok(data) => {
                let asset = bincode::deserialize(&data)
                    .map_err(|e| AssetError::Serialization(e.to_string()))?;
                Ok(Some(asset))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AssetError::Io(e)),
        }
    }
    
    async fn delete(&self, asset_id: AssetId) -> Result<(), AssetError> {
        let path = self.asset_path(asset_id);
        
        match tokio::fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(AssetError::Io(e)),
        }
    }
    
    async fn exists(&self, asset_id: AssetId) -> Result<bool, AssetError> {
        let path = self.asset_path(asset_id);
        Ok(path.exists())
    }
}