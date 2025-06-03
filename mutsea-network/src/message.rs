//! Network message types and serialization

use mutsea_core::{UserId, RegionId, ObjectId, AssetId, Vector3, Quaternion};
use serde::{Deserialize, Serialize};

/// Base message trait for all network messages
pub trait Message: Send + Sync + std::fmt::Debug {
    /// Get the message type
    fn message_type(&self) -> MessageType;
    
    /// Serialize the message to bytes
    fn serialize(&self) -> Result<Vec<u8>, crate::NetworkError>;
    
    /// Deserialize the message from bytes
    fn deserialize(data: &[u8]) -> Result<Self, crate::NetworkError>
    where
        Self: Sized;
}

/// Message types for routing and handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageType {
    // Authentication and session
    Login,
    Logout,
    SessionKeepAlive,
    
    // Agent (avatar) messages
    AgentUpdate,
    AgentMovement,
    AgentAnimation,
    AgentAppearance,
    
    // Object messages
    ObjectUpdate,
    ObjectProperties,
    ObjectSelect,
    ObjectDeselect,
    ObjectGrab,
    ObjectDrop,
    
    // Asset messages
    AssetRequest,
    AssetResponse,
    AssetUpload,
    
    // Chat and communication
    ChatMessage,
    InstantMessage,
    GroupMessage,
    
    // World and region
    RegionHandshake,
    RegionInfo,
    TerrainPatch,
    
    // Physics and simulation
    PhysicsUpdate,
    CollisionUpdate,
    
    // Script messages
    ScriptMessage,
    ScriptResponse,
    
    // System messages
    SystemMessage,
    ErrorMessage,
    
    // Custom message types for extensions
    Custom(u32),
}

/// Core network message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub sender: Option<UserId>,
    pub recipient: Option<UserId>,
    pub message_type: MessageType,
    pub sequence: u32,
    pub reliable: bool,
    pub compressed: bool,
    pub encrypted: bool,
    pub payload: Vec<u8>,
}

impl NetworkMessage {
    /// Create a new network message
    pub fn new(
        message_type: MessageType,
        payload: Vec<u8>,
        sender: Option<UserId>,
        recipient: Option<UserId>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            sender,
            recipient,
            message_type,
            sequence: 0,
            reliable: false,
            compressed: false,
            encrypted: false,
            payload,
        }
    }
    
    /// Set reliability flag
    pub fn with_reliable(mut self, reliable: bool) -> Self {
        self.reliable = reliable;
        self
    }
    
    /// Set compression flag
    pub fn with_compressed(mut self, compressed: bool) -> Self {
        self.compressed = compressed;
        self
    }
    
    /// Set encryption flag
    pub fn with_encrypted(mut self, encrypted: bool) -> Self {
        self.encrypted = encrypted;
        self
    }
    
    /// Set sequence number
    pub fn with_sequence(mut self, sequence: u32) -> Self {
        self.sequence = sequence;
        self
    }
}

impl Message for NetworkMessage {
    fn message_type(&self) -> MessageType {
        self.message_type
    }
    
    fn serialize(&self) -> Result<Vec<u8>, crate::NetworkError> {
        bincode::serialize(self).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, crate::NetworkError> {
        bincode::deserialize(data).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
}

/// Login request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginMessage {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub client_version: String,
    pub platform: String,
    pub mac_address: String,
    pub id0: String,
    pub start_location: String,
    pub channel: String,
    pub version: String,
}

impl Message for LoginMessage {
    fn message_type(&self) -> MessageType {
        MessageType::Login
    }
    
    fn serialize(&self) -> Result<Vec<u8>, crate::NetworkError> {
        bincode::serialize(self).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, crate::NetworkError> {
        bincode::deserialize(data).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
}

/// Agent update message for avatar movement and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentUpdateMessage {
    pub agent_id: UserId,
    pub session_id: uuid::Uuid,
    pub position: Vector3,
    pub velocity: Vector3,
    pub look_at: Vector3,
    pub up_direction: Vector3,
    pub body_rotation: Quaternion,
    pub head_rotation: Quaternion,
    pub control_flags: u32,
    pub flags: u8,
    pub state: u8,
    pub far: f32,
}

impl Message for AgentUpdateMessage {
    fn message_type(&self) -> MessageType {
        MessageType::AgentUpdate
    }
    
    fn serialize(&self) -> Result<Vec<u8>, crate::NetworkError> {
        bincode::serialize(self).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, crate::NetworkError> {
        bincode::deserialize(data).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
}

/// Object update message for scene objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectUpdateMessage {
    pub object_id: ObjectId,
    pub local_id: u32,
    pub region_id: RegionId,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub velocity: Vector3,
    pub angular_velocity: Vector3,
    pub material: u8,
    pub click_action: u8,
    pub flags: u32,
    pub path_curve: u8,
    pub profile_curve: u8,
    pub texture_entry: Vec<u8>,
    pub extra_params: Vec<u8>,
}

impl Message for ObjectUpdateMessage {
    fn message_type(&self) -> MessageType {
        MessageType::ObjectUpdate
    }
    
    fn serialize(&self) -> Result<Vec<u8>, crate::NetworkError> {
        bincode::serialize(self).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, crate::NetworkError> {
        bincode::deserialize(data).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub from_name: String,
    pub source_type: u8,
    pub chat_type: u8,
    pub audible: u8,
    pub position: Vector3,
    pub message: String,
    pub owner_id: UserId,
    pub source_id: Option<ObjectId>,
}

impl Message for ChatMessage {
    fn message_type(&self) -> MessageType {
        MessageType::ChatMessage
    }
    
    fn serialize(&self) -> Result<Vec<u8>, crate::NetworkError> {
        bincode::serialize(self).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, crate::NetworkError> {
        bincode::deserialize(data).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
}

/// Asset request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRequestMessage {
    pub asset_id: AssetId,
    pub asset_type: mutsea_core::AssetType,
    pub is_texture: bool,
    pub temp_file: bool,
    pub priority: f32,
}

impl Message for AssetRequestMessage {
    fn message_type(&self) -> MessageType {
        MessageType::AssetRequest
    }
    
    fn serialize(&self) -> Result<Vec<u8>, crate::NetworkError> {
        bincode::serialize(self).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, crate::NetworkError> {
        bincode::deserialize(data).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
}
 
/// Asset response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetResponseMessage {
    pub asset_id: AssetId,
    pub asset_type: mutsea_core::AssetType,
    pub data: Vec<u8>,
    pub success: bool,
    pub error_message: Option<String>,
}

impl Message for AssetResponseMessage {
    fn message_type(&self) -> MessageType {
        MessageType::AssetResponse
    }
    
    fn serialize(&self) -> Result<Vec<u8>, crate::NetworkError> {
        bincode::serialize(self).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, crate::NetworkError> {
        bincode::deserialize(data).map_err(|e| crate::NetworkError::Serialization(e.to_string()))
    }
}