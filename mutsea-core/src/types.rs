//! Core type definitions for Mutsea

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for users/avatars
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    /// Generate a new random UserId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Create UserId from UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Get the underlying UUID
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for assets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId(pub Uuid);

impl AssetId {
    /// Generate a new random AssetId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Create AssetId from UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Get the underlying UUID
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for AssetId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for regions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegionId(pub Uuid);

impl RegionId {
    /// Generate a new random RegionId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Create RegionId from UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Get the underlying UUID
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for RegionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for RegionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for objects in the virtual world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectId(pub Uuid);

impl ObjectId {
    /// Generate a new random ObjectId
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    /// Create ObjectId from UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    /// Get the underlying UUID
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::new()
    }
}

/// 3D Vector for positions, velocities, etc.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    pub const UP: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
    
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;
    
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;
    
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    
    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

/// Quaternion for rotations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const IDENTITY: Quaternion = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    
    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let sin_half = half_angle.sin();
        let cos_half = half_angle.cos();
        
        Self {
            x: axis.x * sin_half,
            y: axis.y * sin_half,
            z: axis.z * sin_half,
            w: cos_half,
        }
    }
    
    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        } else {
            *self
        }
    }
}

/// Asset type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetType {
    /// Texture/Image asset
    Texture = 0,
    /// Sound asset
    Sound = 1,
    /// Calling card (unused in OpenSim)
    CallingCard = 2,
    /// Landmark asset
    Landmark = 3,
    /// Script/LSL code
    Script = 4,
    /// Clothing asset
    Clothing = 5,
    /// Object asset
    Object = 6,
    /// Notecard asset
    Notecard = 7,
    /// Folder (unused)
    Folder = 8,
    /// Root category folder (unused)
    RootCategory = 9,
    /// LSL text asset
    LSLText = 10,
    /// LSL bytecode
    LSLBytecode = 11,
    /// Texture TGA format
    TextureTGA = 12,
    /// Body part asset
    Bodypart = 13,
    /// Trash folder (unused)
    TrashFolder = 14,
    /// Snapshot folder (unused)
    SnapshotFolder = 15,
    /// Lost and found folder (unused)
    LostAndFoundFolder = 16,
    /// Sound WAV format
    SoundWAV = 17,
    /// Image TGA format
    ImageTGA = 18,
    /// Image JPEG format
    ImageJPEG = 19,
    /// Animation asset
    Animation = 20,
    /// Gesture asset
    Gesture = 21,
    /// Simstate file
    Simstate = 22,
    /// Unknown asset type
    Unknown = 255,
}

impl Default for AssetType {
    fn default() -> Self {
        AssetType::Unknown
    }
}

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub user_id: UserId,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub user_level: i32,
    pub user_flags: i32,
    pub user_title: Option<String>,
}

impl UserAccount {
    pub fn new(first_name: String, last_name: String, email: Option<String>, password_hash: String) -> Self {
        Self {
            user_id: UserId::new(),
            first_name,
            last_name,
            email,
            password_hash,
            created: chrono::Utc::now(),
            last_login: None,
            user_level: 0,
            user_flags: 0,
            user_title: None,
        }
    }
    
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

/// Asset metadata and content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: AssetId,
    pub asset_type: AssetType,
    pub name: String,
    pub description: String,
    pub data: Vec<u8>,
    pub temporary: bool,
    pub local: bool,
    pub created: chrono::DateTime<chrono::Utc>,
    pub creator_id: UserId,
}

impl Asset {
    pub fn new(
        asset_type: AssetType,
        name: String,
        description: String,
        data: Vec<u8>,
        creator_id: UserId,
    ) -> Self {
        Self {
            id: AssetId::new(),
            asset_type,
            name,
            description,
            data,
            temporary: false,
            local: false,
            created: chrono::Utc::now(),
            creator_id,
        }
    }
    
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

/// Region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionInfo {
    pub region_id: RegionId,
    pub region_name: String,
    pub location_x: u32,
    pub location_y: u32,
    pub size_x: u32,
    pub size_y: u32,
    pub external_endpoint: String,
    pub internal_endpoint: String,
    pub access: u8,
    pub scope_id: Uuid,
    pub estate_id: u32,
    pub flags: u32,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

impl RegionInfo {
    pub fn new(
        region_name: String,
        location_x: u32,
        location_y: u32,
        external_endpoint: String,
        internal_endpoint: String,
    ) -> Self {
        Self {
            region_id: RegionId::new(),
            region_name,
            location_x,
            location_y,
            size_x: 256,  // Default region size
            size_y: 256,
            external_endpoint,
            internal_endpoint,
            access: 1,    // Public access
            scope_id: Uuid::new_v4(),
            estate_id: 1,
            flags: 0,
            last_seen: chrono::Utc::now(),
        }
    }
}

/// Scene object in the virtual world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneObject {
    pub id: ObjectId,
    pub local_id: u32,
    pub name: String,
    pub description: String,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub velocity: Vector3,
    pub angular_velocity: Vector3,
    pub owner_id: UserId,
    pub creator_id: UserId,
    pub group_id: Option<Uuid>,
    pub flags: u32,
    pub material: u8,
    pub click_action: u8,
    pub shape: ObjectShape,
    pub created: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Object shape information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectShape {
    pub path_curve: u8,
    pub profile_curve: u8,
    pub path_begin: f32,
    pub path_end: f32,
    pub path_scale_x: f32,
    pub path_scale_y: f32,
    pub path_shear_x: f32,
    pub path_shear_y: f32,
    pub path_twist: f32,
    pub path_twist_begin: f32,
    pub path_radius_offset: f32,
    pub path_taper_x: f32,
    pub path_taper_y: f32,
    pub path_revolutions: f32,
    pub path_skew: f32,
    pub profile_begin: f32,
    pub profile_end: f32,
    pub profile_hollow: f32,
}

impl Default for ObjectShape {
    fn default() -> Self {
        Self {
            path_curve: 16,
            profile_curve: 1,
            path_begin: 0.0,
            path_end: 1.0,
            path_scale_x: 1.0,
            path_scale_y: 1.0,
            path_shear_x: 0.0,
            path_shear_y: 0.0,
            path_twist: 0.0,
            path_twist_begin: 0.0,
            path_radius_offset: 0.0,
            path_taper_x: 0.0,
            path_taper_y: 0.0,
            path_revolutions: 1.0,
            path_skew: 0.0,
            profile_begin: 0.0,
            profile_end: 1.0,
            profile_hollow: 0.0,
        }
    }
}
