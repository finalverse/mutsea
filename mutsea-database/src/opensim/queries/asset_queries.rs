// src/opensim/queries/asset_queries.rs
//! Asset-related database queries

use super::super::{schema::*, models::*};
use crate::{DatabaseManager, Result};

impl DatabaseManager {
    /// Insert a new asset
    pub async fn insert_asset(&self, asset: &Asset) -> Result<()> {
        let backend = self.get_backend().await?;
        let query = include_str!("../../sql/opensim/insert_asset.sql");

        backend
            .execute(
                query,
                &[
                    &asset.id,
                    &asset.name,
                    &asset.description,
                    &asset.asset_type,
                    &asset.local,
                    &asset.temporary,
                    &asset.data,
                    &asset.create_time,
                    &asset.access_time,
                    &asset.asset_flags,
                    &asset.creator_id,
                ],
            )
            .await?;

        Ok(())
    }

    /// Get asset by ID
    pub async fn get_asset(&self, id: &str) -> Result<Option<Asset>> {
        let backend = self.get_backend().await?;
        let query = include_str!("../../sql/opensim/select_asset.sql");

        let row = backend.query_optional(query, &[&id]).await?;

        if let Some(row) = row {
            Ok(Some(Asset {
                id: row.get("id")?,
                name: row.get("name")?,
                description: row.get("description")?,
                asset_type: row.get("asset_type")?,
                local: row.get("local")?,
                temporary: row.get("temporary")?,
                data: row.get("data")?,
                create_time: row.get("create_time")?,
                access_time: row.get("access_time")?,
                asset_flags: row.get("asset_flags")?,
                creator_id: row.get("creator_id")?,
            }))
        } else {
            Ok(None)
        }
    }
}
