//! mutsea-protocol/src/constants.rs
//! Protocol constants for OpenSim LLUDP compatibility

/// Maximum UDP packet size
pub const MAX_PACKET_SIZE: usize = 1200;

/// LLUDP header size
pub const LLUDP_HEADER_SIZE: usize = 10;

/// Maximum payload size
pub const MAX_PAYLOAD_SIZE: usize = MAX_PACKET_SIZE - LLUDP_HEADER_SIZE;

/// Packet flags
pub mod flags {
    pub const RELIABLE: u8 = 0x40;
    pub const RESENT: u8 = 0x20;
    pub const ACK: u8 = 0x10;
    pub const ZEROCODED: u8 = 0x80;
    pub const APPENDED_ACKS: u8 = 0x01;
}

/// Packet types for OpenSim compatibility
pub mod packet_types {
    // Control packets
    pub const PACKET_ACK: u8 = 0xFF;
    pub const START_PING_CHECK: u8 = 0x01;
    pub const COMPLETE_PING_CHECK: u8 = 0x02;
    
    // Authentication and session
    pub const USE_CIRCUIT_CODE: u32 = 3;
    pub const LOGOUT_REQUEST: u32 = 252;
    pub const KICK_USER: u8 = 0xFE;
    
    // Agent management
    pub const AGENT_UPDATE: u32 = 4;
    pub const AGENT_ANIMATION: u32 = 20;
    pub const COMPLETE_AGENT_MOVEMENT: u32 = 249;
    pub const ESTABLISH_AGENT_COMMUNICATION: u8 = 0xFC;
    
    // Region and world
    pub const REGION_HANDSHAKE: u8 = 0x94;
    pub const REGION_HANDSHAKE_REPLY: u32 = 149;
    pub const ENABLE_SIMULATOR: u32 = 151;
    pub const DISABLE_SIMULATOR: u32 = 152;
    
    // Objects and terrain
    pub const OBJECT_UPDATE: u8 = 0x0C;
    pub const OBJECT_UPDATE_CACHED: u32 = 14;
    pub const OBJECT_UPDATE_COMPRESSED: u32 = 15;
    pub const KILL_OBJECT: u32 = 78;
    pub const TERRAIN_PATCH: u32 = 87;
    
    // Chat and communication
    pub const CHAT_FROM_VIEWER: u32 = 80;
    pub const CHAT_FROM_SIMULATOR: u8 = 0x50;
    pub const INSTANT_MESSAGE: u32 = 254;
    
    // Assets and inventory
    pub const REQUEST_IMAGE: u32 = 21;
    pub const IMAGE_DATA: u32 = 22;
    pub const IMAGE_PACKET: u32 = 23;
    pub const TRANSFER_REQUEST: u32 = 116;
    pub const TRANSFER_INFO: u32 = 117;
    pub const TRANSFER_PACKET: u32 = 118;
    
    // Physics and movement
    pub const SET_FOLLOW_CAM_PROPERTIES: u32 = 319;
    pub const CLEAR_FOLLOW_CAM_PROPERTIES: u32 = 320;
    pub const REQUEST_PAYAMOUNT: u32 = 303;
    
    // Economy
    pub const MONEY_BALANCE_REQUEST: u32 = 241;
    pub const MONEY_BALANCE_REPLY: u32 = 242;
    pub const PAY_MONEY_REQUEST: u32 = 243;
    pub const PAY_MONEY_REPLY: u32 = 244;
    
    // Group management
    pub const GROUP_MEMBERSHIP_DATA: u32 = 357;
    pub const GROUP_ACTIVE_PROPOSALS: u32 = 358;
    pub const GROUP_VOTES_HISTORY: u32 = 359;
    
    // Parcel and estate
    pub const PARCEL_INFO_REQUEST: u32 = 434;
    pub const PARCEL_INFO_REPLY: u32 = 435;
    pub const PARCEL_PROPERTIES_REQUEST: u32 = 436;
    pub const PARCEL_PROPERTIES: u32 = 437;
    
    // Friends and social
    pub const ONLINE_NOTIFICATION: u32 = 138;
    pub const OFFLINE_NOTIFICATION: u32 = 139;
    pub const FIND_AGENT: u32 = 126;
    pub const TRACK_AGENT: u32 = 127;
    
    // Map and teleport
    pub const MAP_BLOCK_REQUEST: u32 = 86;
    pub const MAP_BLOCK_REPLY: u32 = 153;
    pub const MAP_ITEM_REQUEST: u32 = 260;
    pub const MAP_ITEM_REPLY: u32 = 261;
    pub const TELEPORT_REQUEST: u32 = 85;
    pub const TELEPORT_START: u32 = 72;
    pub const TELEPORT_PROGRESS: u32 = 73;
    pub const TELEPORT_FINISH: u32 = 65;
    pub const TELEPORT_LOCAL: u32 = 74;
    pub const TELEPORT_LANDMARK_REQUEST: u32 = 84;
    
    // Avatar appearance
    pub const AVATAR_APPEARANCE: u32 = 158;
    pub const WEARABLES_REQUEST: u32 = 159;
    pub const USER_INFO_REQUEST: u32 = 160;
    pub const USER_INFO_REPLY: u32 = 161;
    
    // Script and LSL
    pub const SCRIPT_QUESTION: u32 = 102;
    pub const SCRIPT_CONTROL_CHANGE: u32 = 103;
    pub const SCRIPT_DIALOG: u32 = 104;
    pub const SCRIPT_DIALOG_REPLY: u32 = 105;
    pub const LOAD_URL: u32 = 203;
    
    // Voice
    pub const PROVISION_VOICE_ACCOUNT_REQUEST: u32 = 434;
    pub const PROVISION_VOICE_ACCOUNT_REPLY: u32 = 435;
    pub const PARCEL_VOICE_INFO_REQUEST: u32 = 436;
    pub const PARCEL_VOICE_INFO_REPLY: u32 = 437;
    
    // Statistics and monitoring
    pub const SIM_STATS: u32 = 57;
    pub const REQUEST_REGION_INFO: u32 = 58;
    pub const REGION_INFO: u32 = 59;
    
    // Event system
    pub const EVENT_QUEUE_GET: u32 = 460;
    pub const EVENT_QUEUE_ACK: u32 = 461;
    
    // HTTP and capabilities
    pub const HTTP_REQUEST: u32 = 470;
    pub const HTTP_RESPONSE: u32 = 471;
    
    // Media and streaming
    pub const MEDIA_DATA_REQUEST: u32 = 480;
    pub const MEDIA_DATA_REPLY: u32 = 481;
    pub const STREAMING_AUDIO_CONFIG: u32 = 482;
    
    // Physics and collision
    pub const COLLISION_SOUND_TRIGGER: u32 = 490;
    pub const ATTACH_SOUND_TRIGGER: u32 = 491;
    pub const PHYSICS_SHAPE_DATA: u32 = 492;
    
    // Advanced features
    pub const MESH_DATA_REQUEST: u32 = 500;
    pub const MESH_DATA_REPLY: u32 = 501;
    pub const MATERIALS_DATA_REQUEST: u32 = 502;
    pub const MATERIALS_DATA_REPLY: u32 = 503;
    
    // AI and procedural (Mutsea extensions)
    pub const AI_CONTENT_REQUEST: u32 = 0x8000;
    pub const AI_CONTENT_REPLY: u32 = 0x8001;
    pub const PROCEDURAL_TERRAIN_REQUEST: u32 = 0x8002;
    pub const PROCEDURAL_TERRAIN_REPLY: u32 = 0x8003;
    pub const AI_AGENT_BEHAVIOR: u32 = 0x8004;
    pub const WORLD_GENERATION_REQUEST: u32 = 0x8005;
    pub const WORLD_GENERATION_REPLY: u32 = 0x8006;

    // additional package types
    pub const LAYER_DATA: u32 = 23;
    pub const SIMULATOR_FEATURES: u32 = 24;
    pub const CLOUD_DATA: u32 = 25;
    pub const SIMULATOR_SHUTDOWN: u32 = 26;
    pub const OBJECT_PROPERTIES: u32 = 27;
    pub const IMPROVED_TERSE_OBJECT_UPDATE: u32 = 28;
    pub const AGENT_MOVEMENT_COMPLETE: u32 = 249;
    pub const IMAGE_NOT_IN_DATABASE: u32 = 29;
    pub const INVENTORY_DESCENDENTS: u32 = 30;
}

/// Chat types
pub mod chat_types {
    pub const SAY: u8 = 0;
    pub const SHOUT: u8 = 1;
    pub const WHISPER: u8 = 2;
    pub const BROADCAST: u8 = 3;
    pub const START_TYPING: u8 = 4;
    pub const STOP_TYPING: u8 = 5;
    pub const DEBUG: u8 = 6;
    pub const REGION: u8 = 7;
    pub const OWNER: u8 = 8;
    pub const DIRECT: u8 = 9;
}

/// Chat source types
pub mod chat_sources {
    pub const AGENT: u8 = 0;
    pub const OBJECT: u8 = 1;
    pub const SYSTEM: u8 = 2;
}

/// Agent update flags
pub mod agent_flags {
    pub const NONE: u32 = 0;
    pub const FLY: u32 = 1 << 0;
    pub const STOP: u32 = 1 << 1;
    pub const FINISH_ANIM: u32 = 1 << 2;
    pub const STAND_UP: u32 = 1 << 3;
    pub const SIT_ON_GROUND: u32 = 1 << 4;
    pub const MOUSELOOK: u32 = 1 << 5;
    pub const HIDE_TITLE: u32 = 1 << 6;
    pub const MINIMIZE_FLOATERS: u32 = 1 << 7;
}

/// Control flags for agent movement
pub mod control_flags {
    pub const AT_POS: u32 = 1 << 0;
    pub const AT_NEG: u32 = 1 << 1;
    pub const LEFT_POS: u32 = 1 << 2;
    pub const LEFT_NEG: u32 = 1 << 3;
    pub const UP_POS: u32 = 1 << 4;
    pub const UP_NEG: u32 = 1 << 5;
    pub const PITCH_POS: u32 = 1 << 6;
    pub const PITCH_NEG: u32 = 1 << 7;
    pub const YAW_POS: u32 = 1 << 8;
    pub const YAW_NEG: u32 = 1 << 9;
    pub const FAST_AT: u32 = 1 << 10;
    pub const FAST_LEFT: u32 = 1 << 11;
    pub const FAST_UP: u32 = 1 << 12;
    pub const FLY: u32 = 1 << 13;
    pub const STOP: u32 = 1 << 14;
    pub const FINISH_ANIM: u32 = 1 << 15;
    pub const STAND_UP: u32 = 1 << 16;
    pub const SIT_ON_GROUND: u32 = 1 << 17;
    pub const MOUSELOOK: u32 = 1 << 18;
    pub const NUDGE_AT_POS: u32 = 1 << 19;
    pub const NUDGE_AT_NEG: u32 = 1 << 20;
    pub const NUDGE_LEFT_POS: u32 = 1 << 21;
    pub const NUDGE_LEFT_NEG: u32 = 1 << 22;
    pub const NUDGE_UP_POS: u32 = 1 << 23;
    pub const NUDGE_UP_NEG: u32 = 1 << 24;
    pub const TURN_LEFT: u32 = 1 << 25;
    pub const TURN_RIGHT: u32 = 1 << 26;
    pub const AWAY: u32 = 1 << 27;
    pub const LBUTTON_DOWN: u32 = 1 << 28;
    pub const LBUTTON_UP: u32 = 1 << 29;
    pub const ML_LBUTTON_DOWN: u32 = 1 << 30;
    pub const ML_LBUTTON_UP: u32 = 1 << 31;
}

/// Object update types
pub mod object_update_types {
    pub const OUT_FULL: u8 = 0;
    pub const OUT_TERSE: u8 = 1;
    pub const OUT_FULL_COMPRESSED: u8 = 2;
    pub const OUT_FULL_CACHED: u8 = 3;
}

/// Asset types
pub mod asset_types {
    pub const TEXTURE: u8 = 0;
    pub const SOUND: u8 = 1;
    pub const CALLING_CARD: u8 = 2;
    pub const LANDMARK: u8 = 3;
    pub const SCRIPT: u8 = 4;
    pub const CLOTHING: u8 = 5;
    pub const OBJECT: u8 = 6;
    pub const NOTECARD: u8 = 7;
    pub const CATEGORY: u8 = 8;
    pub const ROOT_CATEGORY: u8 = 9;
    pub const LSL_TEXT: u8 = 10;
    pub const LSL_BYTECODE: u8 = 11;
    pub const TEXTURE_TGA: u8 = 12;
    pub const BODYPART: u8 = 13;
    pub const TRASH: u8 = 14;
    pub const SNAPSHOT_CATEGORY: u8 = 15;
    pub const LOST_AND_FOUND: u8 = 16;
    pub const SOUND_WAV: u8 = 17;
    pub const IMAGE_TGA: u8 = 18;
    pub const IMAGE_JPEG: u8 = 19;
    pub const ANIMATION: u8 = 20;
    pub const GESTURE: u8 = 21;
    pub const SIMSTATE: u8 = 22;
    pub const MESH: u8 = 49;
}

/// Inventory types
pub mod inventory_types {
    pub const TEXTURE: u8 = 0;
    pub const SOUND: u8 = 1;
    pub const CALLING_CARD: u8 = 2;
    pub const LANDMARK: u8 = 3;
    pub const SCRIPT: u8 = 4;
    pub const CLOTHING: u8 = 5;
    pub const OBJECT: u8 = 6;
    pub const NOTECARD: u8 = 7;
    pub const CATEGORY: u8 = 8;
    pub const ROOT_CATEGORY: u8 = 9;
    pub const LSL: u8 = 10;
    pub const SNAPSHOT: u8 = 15;
    pub const ATTACHMENT: u8 = 17;
    pub const WEARABLE: u8 = 18;
    pub const ANIMATION: u8 = 19;
    pub const GESTURE: u8 = 20;
    pub const MESH: u8 = 22;
}

/// Region access levels
pub mod sim_access {
    pub const MIN: u8 = 0;
    pub const TRIAL: u8 = 7;
    pub const PG: u8 = 13;
    pub const MATURE: u8 = 21;
    pub const ADULT: u8 = 42;
    pub const DOWN: u8 = 254;
    pub const NON_EXISTENT: u8 = 255;
}

/// Protocol version constants
pub mod versions {
    pub const PROTOCOL_MAJOR: u8 = 2;
    pub const PROTOCOL_MINOR: u8 = 1;
    pub const PROTOCOL_PATCH: u8 = 0;
    pub const PROTOCOL_BUILD: u32 = 0;
}

/// Network timeouts and limits
pub mod timeouts {
    pub const RESEND_TIMEOUT_MS: u64 = 100;
    pub const ACK_TIMEOUT_MS: u64 = 1000;
    pub const PING_INTERVAL_S: u64 = 5;
    pub const CLIENT_TIMEOUT_S: u64 = 60;
    pub const MAX_RESENDS: u8 = 3;
    pub const MAX_PENDING_ACKS: usize = 256;
}

/// Packet size limits
pub mod limits {
    pub const MAX_APPENDED_ACKS: usize = 250;
    pub const MAX_RELIABLE_QUEUE: usize = 128;
    pub const ZEROCODING_THRESHOLD: usize = 256;
    pub const FRAGMENTATION_THRESHOLD: usize = 1000;
}