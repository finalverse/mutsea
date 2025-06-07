// src/opensim/queries/region_queries.rs
//! Region-related database queries

use super::super::{schema::*, models::*};
use crate::{DatabaseManager, Result};

impl DatabaseManager {
    /// Insert a new region
    pub async fn insert_region(&self, region: &Region) -> Result<()> {
        let backend = self.get_backend().await?;
        let query = include_str!("../../sql/opensim/insert_region.sql");

        backend
            .execute(
                query,
                &[
                    &region.uuid,
                    &region.region_name,
                    &region.server_ip,
                    &region.server_port,
                    &region.loc_x,
                    &region.loc_y,
                    &region.size_x,
                    &region.size_y,
                ],
            )
            .await?;

        Ok(())
    }

    /// Get region by UUID
    pub async fn get_region(&self, uuid: &str) -> Result<Option<Region>> {
        let backend = self.get_backend().await?;
        let query = include_str!("../../sql/opensim/select_region.sql");

        let row = backend.query_optional(query, &[&uuid]).await?;

        if let Some(row) = row {
            Ok(Some(Region {
                uuid: row.get("uuid")?,
                region_name: row.get("region_name")?,
                region_recv_key: row.get("region_recv_key").unwrap_or_default(),
                region_send_key: row.get("region_send_key").unwrap_or_default(),
                region_secret: row.get("region_secret").unwrap_or_default(),
                region_data_uri: row.get("region_data_uri").unwrap_or_default(),
                server_ip: row.get("server_ip")?,
                server_port: row.get("server_port")?,
                server_uri: row.get("server_uri").unwrap_or_default(),
                loc_x: row.get("loc_x")?,
                loc_y: row.get("loc_y")?,
                loc_z: row.get("loc_z").unwrap_or(0),
                east_override_handle: row.get("east_override_handle").unwrap_or(0),
                west_override_handle: row.get("west_override_handle").unwrap_or(0),
                south_override_handle: row.get("south_override_handle").unwrap_or(0),
                north_override_handle: row.get("north_override_handle").unwrap_or(0),
                region_asset_uri: row.get("region_asset_uri").unwrap_or_default(),
                region_asset_recv_key: row.get("region_asset_recv_key").unwrap_or_default(),
                region_asset_send_key: row.get("region_asset_send_key").unwrap_or_default(),
                region_user_uri: row.get("region_user_uri").unwrap_or_default(),
                region_user_recv_key: row.get("region_user_recv_key").unwrap_or_default(),
                region_user_send_key: row.get("region_user_send_key").unwrap_or_default(),
                region_map_texture: row.get("region_map_texture").unwrap_or_default(),
                server_http_port: row.get("server_http_port").unwrap_or(9000),
                server_remote_admin_port: row.get("server_remote_admin_port").unwrap_or(0),
                scope_id: row.get("scope_id").unwrap_or_default(),
                size_x: row.get("size_x").unwrap_or(256),
                size_y: row.get("size_y").unwrap_or(256),
                flags: row.get("flags").unwrap_or(0),
                last_seen: row.get("last_seen").unwrap_or(0),
                parcel_map_texture: row.get("parcel_map_texture").ok(),
            }))
        } else {
            Ok(None)
        }
    }
}
