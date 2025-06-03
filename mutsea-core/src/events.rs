//! Event system for Mutsea

use crate::{AssetId, ObjectId, Quaternion, RegionId, UserId, Vector3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Base event trait that all events implement
pub trait Event: Send + Sync + std::fmt::Debug {
    /// Get the event type name
    fn event_type(&self) -> &'static str;

    /// Get the event timestamp
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;

    /// Get the event ID
    fn event_id(&self) -> uuid::Uuid;
}

/// Core events in the Mutsea system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MutseaEvent {
    /// User-related events
    User(UserEvent),
    /// Region-related events
    Region(RegionEvent),
    /// Object-related events
    Object(ObjectEvent),
    /// Asset-related events
    Asset(AssetEvent),
    /// Network-related events
    Network(NetworkEvent),
    /// System-related events
    System(SystemEvent),
}

impl Event for MutseaEvent {
    fn event_type(&self) -> &'static str {
        match self {
            MutseaEvent::User(_) => "user",
            MutseaEvent::Region(_) => "region",
            MutseaEvent::Object(_) => "object",
            MutseaEvent::Asset(_) => "asset",
            MutseaEvent::Network(_) => "network",
            MutseaEvent::System(_) => "system",
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        match self {
            MutseaEvent::User(e) => e.timestamp,
            MutseaEvent::Region(e) => e.timestamp,
            MutseaEvent::Object(e) => e.timestamp,
            MutseaEvent::Asset(e) => e.timestamp,
            MutseaEvent::Network(e) => e.timestamp,
            MutseaEvent::System(e) => e.timestamp,
        }
    }

    fn event_id(&self) -> uuid::Uuid {
        match self {
            MutseaEvent::User(e) => e.event_id,
            MutseaEvent::Region(e) => e.event_id,
            MutseaEvent::Object(e) => e.event_id,
            MutseaEvent::Asset(e) => e.event_id,
            MutseaEvent::Network(e) => e.event_id,
            MutseaEvent::System(e) => e.event_id,
        }
    }
}

/// User-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEvent {
    pub event_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: UserId,
    pub region_id: Option<RegionId>,
    pub event_data: UserEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserEventData {
    Login {
        session_id: uuid::Uuid,
        client_info: String,
    },
    Logout {
        session_id: uuid::Uuid,
        duration: std::time::Duration,
    },
    Movement {
        old_position: Vector3,
        new_position: Vector3,
        velocity: Vector3,
    },
    Rotation {
        old_rotation: Quaternion,
        new_rotation: Quaternion,
    },
    Chat {
        message: String,
        chat_type: ChatType,
        channel: i32,
    },
    Teleport {
        from_region: RegionId,
        to_region: RegionId,
        position: Vector3,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatType {
    Say,
    Shout,
    Whisper,
    IM,
    Group,
    Region,
    Owner,
    Debug,
}

/// Region-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionEvent {
    pub event_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub region_id: RegionId,
    pub event_data: RegionEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegionEventData {
    Created {
        region_name: String,
        location: (u32, u32),
    },
    Started {
        startup_duration: std::time::Duration,
    },
    Stopped {
        reason: String,
    },
    UserEntered {
        user_id: UserId,
        position: Vector3,
    },
    UserLeft {
        user_id: UserId,
        duration: std::time::Duration,
    },
    ObjectAdded {
        object_id: ObjectId,
        creator_id: UserId,
    },
    ObjectRemoved {
        object_id: ObjectId,
        remover_id: UserId,
    },
}

/// Object-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectEvent {
    pub event_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub object_id: ObjectId,
    pub region_id: RegionId,
    pub event_data: ObjectEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectEventData {
    Created {
        creator_id: UserId,
        position: Vector3,
        object_type: String,
    },
    Destroyed {
        destroyer_id: UserId,
    },
    Moved {
        old_position: Vector3,
        new_position: Vector3,
        mover_id: UserId,
    },
    Rotated {
        old_rotation: Quaternion,
        new_rotation: Quaternion,
        rotator_id: UserId,
    },
    Scaled {
        old_scale: Vector3,
        new_scale: Vector3,
        scaler_id: UserId,
    },
    Touched {
        toucher_id: UserId,
        touch_position: Vector3,
    },
    ScriptEvent {
        script_name: String,
        event_name: String,
        parameters: HashMap<String, String>,
    },
}

/// Asset-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEvent {
    pub event_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub asset_id: AssetId,
    pub event_data: AssetEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetEventData {
    Created {
        creator_id: UserId,
        asset_type: crate::AssetType,
        size: usize,
    },
    Accessed {
        accessor_id: UserId,
        access_type: AssetAccessType,
    },
    Modified {
        modifier_id: UserId,
        old_size: usize,
        new_size: usize,
    },
    Deleted {
        deleter_id: UserId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetAccessType {
    Read,
    Download,
    Upload,
    Update,
}

/// Network-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub event_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_data: NetworkEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkEventData {
    ClientConnected {
        client_ip: String,
        user_agent: String,
        protocol_version: String,
    },
    ClientDisconnected {
        client_ip: String,
        reason: String,
        duration: std::time::Duration,
    },
    PacketReceived {
        packet_type: String,
        size: usize,
        source_ip: String,
    },
    PacketSent {
        packet_type: String,
        size: usize,
        destination_ip: String,
    },
    ProtocolError {
        error_type: String,
        error_message: String,
        client_ip: String,
    },
}

/// System-related events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_data: SystemEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEventData {
    ServerStarted {
        version: String,
        startup_duration: std::time::Duration,
    },
    ServerStopped {
        reason: String,
        uptime: std::time::Duration,
    },
    ServiceStarted {
        service_name: String,
    },
    ServiceStopped {
        service_name: String,
        reason: String,
    },
    PerformanceAlert {
        metric_name: String,
        current_value: f64,
        threshold: f64,
    },
    Error {
        component: String,
        error_message: String,
        error_code: Option<i32>,
    },
    Warning {
        component: String,
        warning_message: String,
    },
}

/// Event builder for convenient event creation
pub struct EventBuilder;

impl EventBuilder {
    /// Create a new user login event
    pub fn user_login(
        user_id: UserId,
        session_id: uuid::Uuid,
        client_info: String,
        region_id: Option<RegionId>,
    ) -> MutseaEvent {
        MutseaEvent::User(UserEvent {
            event_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            user_id,
            region_id,
            event_data: UserEventData::Login {
                session_id,
                client_info,
            },
        })
    }

    /// Create a new user movement event
    pub fn user_movement(
        user_id: UserId,
        old_position: Vector3,
        new_position: Vector3,
        velocity: Vector3,
        region_id: Option<RegionId>,
    ) -> MutseaEvent {
        MutseaEvent::User(UserEvent {
            event_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            user_id,
            region_id,
            event_data: UserEventData::Movement {
                old_position,
                new_position,
                velocity,
            },
        })
    }

    /// Create a new chat event
    pub fn user_chat(
        user_id: UserId,
        message: String,
        chat_type: ChatType,
        channel: i32,
        region_id: Option<RegionId>,
    ) -> MutseaEvent {
        MutseaEvent::User(UserEvent {
            event_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            user_id,
            region_id,
            event_data: UserEventData::Chat {
                message,
                chat_type,
                channel,
            },
        })
    }

    /// Create a new object created event
    pub fn object_created(
        object_id: ObjectId,
        creator_id: UserId,
        position: Vector3,
        object_type: String,
        region_id: RegionId,
    ) -> MutseaEvent {
        MutseaEvent::Object(ObjectEvent {
            event_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            object_id,
            region_id,
            event_data: ObjectEventData::Created {
                creator_id,
                position,
                object_type,
            },
        })
    }

    /// Create a new asset created event
    pub fn asset_created(
        asset_id: AssetId,
        creator_id: UserId,
        asset_type: crate::AssetType,
        size: usize,
    ) -> MutseaEvent {
        MutseaEvent::Asset(AssetEvent {
            event_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            asset_id,
            event_data: AssetEventData::Created {
                creator_id,
                asset_type,
                size,
            },
        })
    }

    /// Create a new region started event
    pub fn region_started(
        region_id: RegionId,
        startup_duration: std::time::Duration,
    ) -> MutseaEvent {
        MutseaEvent::Region(RegionEvent {
            event_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            region_id,
            event_data: RegionEventData::Started { startup_duration },
        })
    }

    /// Create a new system error event
    pub fn system_error(
        component: String,
        error_message: String,
        error_code: Option<i32>,
    ) -> MutseaEvent {
        MutseaEvent::System(SystemEvent {
            event_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_data: SystemEventData::Error {
                component,
                error_message,
                error_code,
            },
        })
    }
}

/// Event filter for subscribing to specific event types
#[derive(Debug, Clone)]
pub struct EventFilter {
    pub event_types: Option<Vec<String>>,
    pub user_ids: Option<Vec<UserId>>,
    pub region_ids: Option<Vec<RegionId>>,
    pub object_ids: Option<Vec<ObjectId>>,
    pub asset_ids: Option<Vec<AssetId>>,
}

impl EventFilter {
    /// Create a new empty filter (matches all events)
    pub fn new() -> Self {
        Self {
            event_types: None,
            user_ids: None,
            region_ids: None,
            object_ids: None,
            asset_ids: None,
        }
    }

    /// Filter by event types
    pub fn with_event_types(mut self, event_types: Vec<String>) -> Self {
        self.event_types = Some(event_types);
        self
    }

    /// Filter by user IDs
    pub fn with_user_ids(mut self, user_ids: Vec<UserId>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    /// Filter by region IDs
    pub fn with_region_ids(mut self, region_ids: Vec<RegionId>) -> Self {
        self.region_ids = Some(region_ids);
        self
    }

    /// Check if an event matches this filter
    pub fn matches(&self, event: &MutseaEvent) -> bool {
        // Check event type filter
        if let Some(ref types) = self.event_types {
            if !types.contains(&event.event_type().to_string()) {
                return false;
            }
        }

        // Check user ID filter
        if let Some(ref user_ids) = self.user_ids {
            match event {
                MutseaEvent::User(e) => {
                    if !user_ids.contains(&e.user_id) {
                        return false;
                    }
                }
                MutseaEvent::Object(e) => match &e.event_data {
                    ObjectEventData::Created { creator_id, .. }
                    | ObjectEventData::Destroyed {
                        destroyer_id: creator_id,
                    }
                    | ObjectEventData::Moved {
                        mover_id: creator_id,
                        ..
                    }
                    | ObjectEventData::Rotated {
                        rotator_id: creator_id,
                        ..
                    }
                    | ObjectEventData::Scaled {
                        scaler_id: creator_id,
                        ..
                    }
                    | ObjectEventData::Touched {
                        toucher_id: creator_id,
                        ..
                    } => {
                        if !user_ids.contains(creator_id) {
                            return false;
                        }
                    }
                    _ => {}
                },
                MutseaEvent::Asset(e) => match &e.event_data {
                    AssetEventData::Created { creator_id, .. }
                    | AssetEventData::Accessed {
                        accessor_id: creator_id,
                        ..
                    }
                    | AssetEventData::Modified {
                        modifier_id: creator_id,
                        ..
                    }
                    | AssetEventData::Deleted {
                        deleter_id: creator_id,
                    } => {
                        if !user_ids.contains(creator_id) {
                            return false;
                        }
                    }
                },
                _ => {}
            }
        }

        // Check region ID filter
        if let Some(ref region_ids) = self.region_ids {
            match event {
                MutseaEvent::User(e) => {
                    if let Some(region_id) = e.region_id {
                        if !region_ids.contains(&region_id) {
                            return false;
                        }
                    }
                }
                MutseaEvent::Region(e) => {
                    if !region_ids.contains(&e.region_id) {
                        return false;
                    }
                }
                MutseaEvent::Object(e) => {
                    if !region_ids.contains(&e.region_id) {
                        return false;
                    }
                }
                _ => {}
            }
        }

        // Check object ID filter
        if let Some(ref object_ids) = self.object_ids {
            if let MutseaEvent::Object(e) = event {
                if !object_ids.contains(&e.object_id) {
                    return false;
                }
            }
        }

        // Check asset ID filter
        if let Some(ref asset_ids) = self.asset_ids {
            if let MutseaEvent::Asset(e) = event {
                if !asset_ids.contains(&e.asset_id) {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new()
    }
}
