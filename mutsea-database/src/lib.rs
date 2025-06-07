// src/lib.rs
//! Mutsea Database Layer - OpenSim Compatible
//! 
//! This crate provides database abstraction for the Mutsea AI-driven world engine
//! with compatibility for OpenSim's existing database schema.

pub mod error;
pub mod backends;
pub mod models;
pub mod queries;
pub mod utils;
pub mod traits;
pub mod manager;

// OpenSim Compatibility Layer
pub mod opensim;

use error::DatabaseError;
use backends::DatabaseBackend;
use manager::DatabaseManager;

pub type Result<T> = std::result::Result<T, DatabaseError>;

/// Main database interface for Mutsea
pub struct MutseaDatabase {
    manager: DatabaseManager,
}

impl MutseaDatabase {
    /// Create a new database instance with OpenSim compatibility
    pub async fn new_opensim_compatible(database_url: &str) -> Result<Self> {
        let manager = DatabaseManager::new(database_url).await?;
        Ok(Self { manager })
    }

    /// Get database manager for direct access
    pub fn manager(&self) -> &DatabaseManager {
        &self.manager
    }

    /// Initialize OpenSim compatible tables
    pub async fn initialize_opensim_schema(&self) -> Result<()> {
        self.manager.initialize_opensim_tables().await
    }

    /// Check if database is ready for OpenSim operations
    pub async fn verify_opensim_compatibility(&self) -> Result<bool> {
        self.manager.verify_opensim_tables().await
    }
}

// Re-exports for convenience
pub use backends::{BackendType, DatabasePool};
pub use models::*;
pub use error::DatabaseError;
pub use manager::DatabaseManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_opensim_compatibility() {
        // This would test against a test database
        // For now, just ensure the types compile
        assert!(true);
    }
}

// src/opensim/mod.rs
//! OpenSim compatibility layer
//! 
//! This module provides compatibility with OpenSimulator's database schema
//! while allowing for AI enhancements.

pub mod schema;
pub mod models;
pub mod queries;

use crate::{DatabaseManager, Result};

/// OpenSim database operations
impl DatabaseManager {
    /// Initialize OpenSim compatible database tables
    pub async fn initialize_opensim_tables(&self) -> Result<()> {
        let backend = self.get_backend().await?;
        
        // Create core OpenSim tables
        let table_queries = vec![
            include_str!("../sql/opensim/create_regions.sql"),
            include_str!("../sql/opensim/create_users.sql"),
            include_str!("../sql/opensim/create_assets.sql"),
            include_str!("../sql/opensim/create_inventory.sql"),
            include_str!("../sql/opensim/create_primitives.sql"),
            include_str!("../sql/opensim/create_terrain.sql"),
            include_str!("../sql/opensim/create_parcels.sql"),
        ];

        for query in table_queries {
            backend.execute(query, &[]).await?;
        }

        Ok(())
    }

    /// Verify OpenSim tables exist and are properly structured
    pub async fn verify_opensim_tables(&self) -> Result<bool> {
        let backend = self.get_backend().await?;
        
        let required_tables = vec![
            "regions", "users", "assets", "inventoryitems", 
            "inventoryfolders", "primitives", "primshapes", 
            "terrain", "land", "landaccesslist"
        ];

        for table in required_tables {
            let exists = backend.table_exists(table).await?;
            if !exists {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

// src/opensim/schema.rs
//! OpenSim database schema definitions

/// Region information compatible with OpenSim
#[derive(Debug, Clone)]
pub struct Region {
    pub uuid: String,
    pub region_name: String,
    pub region_recv_key: String,
    pub region_send_key: String,
    pub region_secret: String,
    pub region_data_uri: String,
    pub server_ip: String,
    pub server_port: u32,
    pub server_uri: String,
    pub loc_x: u32,
    pub loc_y: u32,
    pub loc_z: u32,
    pub east_override_handle: u64,
    pub west_override_handle: u64,
    pub south_override_handle: u64,
    pub north_override_handle: u64,
    pub region_asset_uri: String,
    pub region_asset_recv_key: String,
    pub region_asset_send_key: String,
    pub region_user_uri: String,
    pub region_user_recv_key: String,
    pub region_user_send_key: String,
    pub region_map_texture: String,
    pub server_http_port: u32,
    pub server_remote_admin_port: u32,
    pub scope_id: String,
    pub size_x: u32,
    pub size_y: u32,
    pub flags: u32,
    pub last_seen: i64,
    pub parcel_map_texture: Option<String>,
}

/// User account compatible with OpenSim
#[derive(Debug, Clone)]
pub struct UserAccount {
    pub principal_id: String,
    pub scope_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub service_urls: Option<String>,
    pub created: i32,
    pub user_level: i32,
    pub user_flags: i32,
    pub user_title: Option<String>,
    pub active: i32,
}

/// Asset compatible with OpenSim
#[derive(Debug, Clone)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub asset_type: i32,
    pub local: bool,
    pub temporary: bool,
    pub data: Vec<u8>,
    pub create_time: i32,
    pub access_time: i32,
    pub asset_flags: i32,
    pub creator_id: String,
}

// src/opensim/models.rs
//! OpenSim model implementations

use super::schema::*;
use crate::{Result, DatabaseError};
use serde::{Deserialize, Serialize};

impl Region {
    /// Create a new region with default values
    pub fn new(name: String, uuid: String, x: u32, y: u32) -> Self {
        Self {
            uuid,
            region_name: name,
            region_recv_key: String::new(),
            region_send_key: String::new(),
            region_secret: String::new(),
            region_data_uri: String::new(),
            server_ip: "127.0.0.1".to_string(),
            server_port: 9000,
            server_uri: String::new(),
            loc_x: x,
            loc_y: y,
            loc_z: 0,
            east_override_handle: 0,
            west_override_handle: 0,
            south_override_handle: 0,
            north_override_handle: 0,
            region_asset_uri: String::new(),
            region_asset_recv_key: String::new(),
            region_asset_send_key: String::new(),
            region_user_uri: String::new(),
            region_user_recv_key: String::new(),
            region_user_send_key: String::new(),
            region_map_texture: String::new(),
            server_http_port: 9000,
            server_remote_admin_port: 0,
            scope_id: "00000000-0000-0000-0000-000000000000".to_string(),
            size_x: 256,
            size_y: 256,
            flags: 0,
            last_seen: chrono::Utc::now().timestamp(),
            parcel_map_texture: None,
        }
    }
}

impl UserAccount {
    /// Create a new user account
    pub fn new(first_name: String, last_name: String, principal_id: String) -> Self {
        Self {
            principal_id,
            scope_id: "00000000-0000-0000-0000-000000000000".to_string(),
            first_name,
            last_name,
            email: None,
            service_urls: None,
            created: chrono::Utc::now().timestamp() as i32,
            user_level: 0,
            user_flags: 0,
            user_title: None,
            active: 1,
        }
    }
}

impl Asset {
    /// Create a new asset
    pub fn new(id: String, name: String, asset_type: i32, data: Vec<u8>) -> Self {
        let now = chrono::Utc::now().timestamp() as i32;
        Self {
            id,
            name: name.clone(),
            description: name,
            asset_type,
            local: false,
            temporary: false,
            data,
            create_time: now,
            access_time: now,
            asset_flags: 0,
            creator_id: "00000000-0000-0000-0000-000000000000".to_string(),
        }
    }
}

// src/opensim/queries.rs
//! OpenSim database queries

use super::{schema::*, models::*};
use crate::{DatabaseManager, Result, DatabaseError};

impl DatabaseManager {
    /// Insert a new region
    pub async fn insert_region(&self, region: &Region) -> Result<()> {
        let backend = self.get_backend().await?;
        let query = include_str!("../sql/opensim/insert_region.sql");
        
        backend.execute(query, &[
            &region.uuid,
            &region.region_name,
            &region.server_ip,
            &region.server_port,
            &region.loc_x,
            &region.loc_y,
            &region.size_x,
            &region.size_y,
        ]).await?;
        
        Ok(())
    }

    /// Get region by UUID
    pub async fn get_region(&self, uuid: &str) -> Result<Option<Region>> {
        let backend = self.get_backend().await?;
        let query = include_str!("../sql/opensim/select_region.sql");
        
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

    /// Insert a new user account
    pub async fn insert_user_account(&self, user: &UserAccount) -> Result<()> {
        let backend = self.get_backend().await?;
        let query = include_str!("../sql/opensim/insert_user_account.sql");
        
        backend.execute(query, &[
            &user.principal_id,
            &user.scope_id,
            &user.first_name,
            &user.last_name,
            &user.email,
            &user.created,
            &user.user_level,
            &user.user_flags,
            &user.active,
        ]).await?;
        
        Ok(())
    }

    /// Get user account by principal ID
    pub async fn get_user_account(&self, principal_id: &str) -> Result<Option<UserAccount>> {
        let backend = self.get_backend().await?;
        let query = include_str!("../sql/opensim/select_user_account.sql");
        
        let row = backend.query_optional(query, &[&principal_id]).await?;
        
        if let Some(row) = row {
            Ok(Some(UserAccount {
                principal_id: row.get("principal_id")?,
                scope_id: row.get("scope_id")?,
                first_name: row.get("first_name")?,
                last_name: row.get("last_name")?,
                email: row.get("email").ok(),
                service_urls: row.get("service_urls").ok(),
                created: row.get("created")?,
                user_level: row.get("user_level")?,
                user_flags: row.get("user_flags")?,
                user_title: row.get("user_title").ok(),
                active: row.get("active")?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Insert a new asset
    pub async fn insert_asset(&self, asset: &Asset) -> Result<()> {
        let backend = self.get_backend().await?;
        let query = include_str!("../sql/opensim/insert_asset.sql");
        
        backend.execute(query, &[
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
        ]).await?;
        
        Ok(())
    }

    /// Get asset by ID
    pub async fn get_asset(&self, id: &str) -> Result<Option<Asset>> {
        let backend = self.get_backend().await?;
        let query = include_str!("../sql/opensim/select_asset.sql");
        
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