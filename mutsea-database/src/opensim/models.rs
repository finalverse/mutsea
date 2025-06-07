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