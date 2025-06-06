//! mutsea-database/src/models.rs
//! Database model definitions for core entities

use mutsea_core::{UserId, AssetId, RegionId, AssetType};
use serde::{Deserialize, Serialize};

/// Asset metadata stored in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub id: AssetId,
    pub asset_type: AssetType,
    pub name: String,
    pub description: String,
    pub size: usize,
    pub temporary: bool,
    pub local: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    pub creator_id: UserId,
}

impl From<&mutsea_core::Asset> for AssetMetadata {
    fn from(asset: &mutsea_core::Asset) -> Self {
        Self {
            id: asset.id,
            asset_type: asset.asset_type,
            name: asset.name.clone(),
            description: asset.description.clone(),
            size: asset.data.len(),
            temporary: asset.temporary,
            local: asset.local,
            created: asset.created,
            creator_id: asset.creator_id,
        }
    }
}