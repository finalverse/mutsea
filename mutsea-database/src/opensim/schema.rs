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
