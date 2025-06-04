// mutsea-protocol/src/login.rs
//! Login service implementation
//! Unified login service with full OpenSim compatibility

use crate::{ProtocolError, ProtocolResult};
use mutsea_core::{UserId, UserAccount};
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Unified login service supporting both HTTP (8080) and LLUDP (9000) protocols
pub struct LoginService {
    test_users: RwLock<HashMap<String, TestUser>>,
    active_sessions: RwLock<HashMap<String, SessionInfo>>,
}

/// Test user for development and testing
#[derive(Debug, Clone)]
struct TestUser {
    first_name: String,
    last_name: String,
    password: String,
    user_id: UserId,
    email: Option<String>,
    is_admin: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Active session information
#[derive(Debug, Clone)]
struct SessionInfo {
    session_id: String,
    user_id: UserId,
    agent_id: UserId,
    created_at: chrono::DateTime<chrono::Utc>,
    last_activity: chrono::DateTime<chrono::Utc>,
}

/// Parsed login request structure for XMLRPC compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedLoginRequest {
    pub first: String,
    pub last: String,
    pub passwd: String,
    pub start: String,
    pub channel: String,
    pub version: String,
    pub platform: String,
    pub mac: String,
    pub id0: String,
    pub agree_to_tos: String,
    pub read_critical: String,
    pub viewer_digest: String,
    pub options: Vec<String>,
}

/// OpenSim-compatible login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSimLoginResponse {
    pub login: String,
    pub reason: String,
    pub session_id: Option<String>,
    pub secure_session_id: Option<String>,
    pub agent_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub start_location: Option<String>,
    pub look_at: Option<String>,
    pub seed_capability: Option<String>,
    pub agent_access: Option<String>,
    pub agent_access_max: Option<String>,
    pub inventory_host: Option<String>,
    pub sim_ip: Option<String>,
    pub sim_port: Option<i32>,
    pub region_x: Option<i32>,
    pub region_y: Option<i32>,
    pub circuit_code: Option<i32>,
    pub home: Option<String>,
    pub message: String,
    pub seconds_since_epoch: i64,
    #[serde(default)]
    pub event_categories: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub event_notifications: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub classified_categories: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub ui_config: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub global_textures: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub login_flags: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub inventory_skeleton: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub inventory_lib_skeleton: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub inventory_lib_owner: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub buddy_list: Vec<HashMap<String, String>>,
}

impl LoginService {
    /// Create a new unified login service
    pub fn new() -> Self {
        Self {
            test_users: RwLock::new(HashMap::new()),
            active_sessions: RwLock::new(HashMap::new()),
        }
    }

    /// Add a test user
    pub fn add_test_user(&self, first_name: String, last_name: String, password: String) {
        let key = format!("{} {}", first_name, last_name);
        let user = TestUser {
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            password,
            user_id: UserId::new(),
            email: None,
            is_admin: false,
            created_at: chrono::Utc::now(),
        };

        self.test_users.write().unwrap().insert(key, user);
        tracing::info!("Added test user: {} {}", first_name, last_name);
    }

    /// Process OpenSim-compatible login request
    pub fn authenticate(&self, request: &ParsedLoginRequest) -> ProtocolResult<OpenSimLoginResponse> {
        let user_key = format!("{} {}", request.first, request.last);

        let users = self.test_users.read().unwrap();
        if let Some(user) = users.get(&user_key) {
            if user.password == request.passwd {
                // Successful login
                let session_id = Uuid::new_v4();
                let secure_session_id = Uuid::new_v4();
                let circuit_code = rand::random::<u32>();

                // Store session for validation
                let session_info = SessionInfo {
                    session_id: session_id.to_string(),
                    user_id: user.user_id,
                    agent_id: user.user_id, // Using same ID for simplicity
                    created_at: chrono::Utc::now(),
                    last_activity: chrono::Utc::now(),
                };
                
                self.active_sessions.write().unwrap().insert(session_id.to_string(), session_info);

                let seed_capability = format!(
                    "http://127.0.0.1:8080/caps/{}/",
                    Uuid::new_v4()
                );

                Ok(OpenSimLoginResponse::success(
                    session_id,
                    secure_session_id,
                    user.user_id,
                    user.first_name.clone(),
                    user.last_name.clone(),
                    mutsea_core::RegionId::new(),
                    "127.0.0.1".to_string(),
                    9000, // LLUDP port
                    circuit_code,
                    seed_capability,
                ))
            } else {
                Ok(OpenSimLoginResponse::failure("Invalid password".to_string()))
            }
        } else {
            Ok(OpenSimLoginResponse::failure("User not found".to_string()))
        }
    }

    /// Validate session for LLUDP circuit authentication
    pub fn validate_session(&self, session_id: &str, agent_id: &UserId) -> bool {
        if let Ok(sessions) = self.active_sessions.read() {
            if let Some(session_info) = sessions.get(session_id) {
                // Check if agent ID matches and session is still valid
                return session_info.agent_id == *agent_id && 
                       chrono::Utc::now().signed_duration_since(session_info.created_at).num_hours() < 24;
            }
        }
        false
    }

    /// Update session activity
    pub fn update_session_activity(&self, session_id: &str) {
        if let Ok(mut sessions) = self.active_sessions.write() {
            if let Some(session_info) = sessions.get_mut(session_id) {
                session_info.last_activity = chrono::Utc::now();
            }
        }
    }

    /// Remove expired sessions
    pub fn cleanup_expired_sessions(&self) {
        if let Ok(mut sessions) = self.active_sessions.write() {
            let now = chrono::Utc::now();
            sessions.retain(|_, session| {
                now.signed_duration_since(session.created_at).num_hours() < 24
            });
        }
    }

    /// List all users
    pub fn list_users(&self) -> Vec<String> {
        self.test_users.read().unwrap().keys().cloned().collect()
    }

    /// Get user by name
    pub fn get_user_by_name(&self, first_name: &str, last_name: &str) -> Option<UserId> {
        let user_key = format!("{} {}", first_name, last_name);
        self.test_users.read().unwrap().get(&user_key).map(|user| user.user_id)
    }

    /// Get active sessions count
    pub fn get_active_sessions_count(&self) -> usize {
        self.active_sessions.read().unwrap().len()
    }
}

impl ParsedLoginRequest {
    /// Parse XMLRPC login request
    pub fn from_xmlrpc(xml: &str) -> ProtocolResult<Self> {
        // Simple XMLRPC parsing for OpenSim login
        // In a full implementation, use proper XMLRPC parser

        let mut request = Self {
            first: String::new(),
            last: String::new(),
            passwd: String::new(),
            start: "home".to_string(),
            channel: "Mutsea".to_string(),
            version: "1.0.0".to_string(),
            platform: "Unknown".to_string(),
            mac: "00:00:00:00:00:00".to_string(),
            id0: "unknown".to_string(),
            agree_to_tos: "true".to_string(),
            read_critical: "true".to_string(),
            viewer_digest: "unknown".to_string(),
            options: vec!["inventory-root".to_string(), "inventory-skeleton".to_string()],
        };

        // Extract values from XMLRPC (simplified parsing)
        if let Some(first_start) = xml.find("<name>first</name>") {
            if let Some(value_start) = xml[first_start..].find("<value><string>") {
                let start_pos = first_start + value_start + 14;
                if let Some(value_end) = xml[start_pos..].find("</string></value>") {
                    request.first = xml[start_pos..start_pos + value_end].to_string();
                }
            }
        }

        if let Some(last_start) = xml.find("<name>last</name>") {
            if let Some(value_start) = xml[last_start..].find("<value><string>") {
                let start_pos = last_start + value_start + 14;
                if let Some(value_end) = xml[start_pos..].find("</string></value>") {
                    request.last = xml[start_pos..start_pos + value_end].to_string();
                }
            }
        }

        if let Some(passwd_start) = xml.find("<name>passwd</name>") {
            if let Some(value_start) = xml[passwd_start..].find("<value><string>") {
                let start_pos = passwd_start + value_start + 14;
                if let Some(value_end) = xml[start_pos..].find("</string></value>") {
                    request.passwd = xml[start_pos..start_pos + value_end].to_string();
                }
            }
        }

        Ok(request)
    }
}

impl OpenSimLoginResponse {
    /// Create successful login response
    pub fn success(
        session_id: Uuid,
        secure_session_id: Uuid,
        agent_id: UserId,
        first_name: String,
        last_name: String,
        region_id: mutsea_core::RegionId,
        sim_ip: String,
        sim_port: i32,
        circuit_code: u32,
        seed_capability: String,
    ) -> Self {
        Self {
            login: "true".to_string(),
            reason: "".to_string(),
            session_id: Some(session_id.to_string()),
            secure_session_id: Some(secure_session_id.to_string()),
            agent_id: Some(agent_id.to_string()),
            first_name: Some(first_name),
            last_name: Some(last_name),
            start_location: Some("home".to_string()),
            look_at: Some("[r1,r1,r0]".to_string()),
            seed_capability: Some(seed_capability),
            agent_access: Some("M".to_string()),
            agent_access_max: Some("A".to_string()),
            inventory_host: Some("127.0.0.1".to_string()),
            sim_ip: Some(sim_ip),
            sim_port: Some(sim_port),
            region_x: Some(1000),
            region_y: Some(1000),
            circuit_code: Some(circuit_code as i32),
            home: Some(format!("{{'region_handle':[r{},r{}], 'position':[r128,r128,r21], 'look_at':[r1,r0,r0]}}",
                               1000 * 256, 1000 * 256)),
            message: "Welcome to Mutsea!".to_string(),
            seconds_since_epoch: chrono::Utc::now().timestamp(),
            event_categories: Vec::new(),
            event_notifications: Vec::new(),
            classified_categories: Vec::new(),
            ui_config: Vec::new(),
            global_textures: Vec::new(),
            login_flags: Vec::new(),
            inventory_skeleton: Vec::new(),
            inventory_lib_skeleton: Vec::new(),
            inventory_lib_owner: Vec::new(),
            buddy_list: Vec::new(),
        }
    }

    /// Create failed login response
    pub fn failure(reason: String) -> Self {
        Self {
            login: "false".to_string(),
            reason: reason.clone(),
            session_id: None,
            secure_session_id: None,
            agent_id: None,
            first_name: None,
            last_name: None,
            start_location: None,
            look_at: None,
            seed_capability: None,
            agent_access: None,
            agent_access_max: None,
            inventory_host: None,
            sim_ip: None,
            sim_port: None,
            region_x: None,
            region_y: None,
            circuit_code: None,
            home: None,
            message: reason,
            seconds_since_epoch: chrono::Utc::now().timestamp(),
            event_categories: Vec::new(),
            event_notifications: Vec::new(),
            classified_categories: Vec::new(),
            ui_config: Vec::new(),
            global_textures: Vec::new(),
            login_flags: Vec::new(),
            inventory_skeleton: Vec::new(),
            inventory_lib_skeleton: Vec::new(),
            inventory_lib_owner: Vec::new(),
            buddy_list: Vec::new(),
        }
    }

    /// Convert to XMLRPC response
    pub fn to_xmlrpc(&self) -> String {
        if self.login == "true" {
            format!(r#"<?xml version="1.0"?>
<methodResponse>
    <params>
        <param>
            <value>
                <struct>
                    <member>
                        <name>login</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>session_id</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>secure_session_id</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>agent_id</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>first_name</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>last_name</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>start_location</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>sim_ip</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>sim_port</name>
                        <value><i4>{}</i4></value>
                    </member>
                    <member>
                        <name>circuit_code</name>
                        <value><i4>{}</i4></value>
                    </member>
                    <member>
                        <name>seed_capability</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>message</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>inventory-skeleton</name>
                        <value><array><data></data></array></value>
                    </member>
                    <member>
                        <name>inventory-lib-skeleton</name>
                        <value><array><data></data></array></value>
                    </member>
                    <member>
                        <name>inventory-lib-owner</name>
                        <value><array><data></data></array></value>
                    </member>
                    <member>
                        <name>buddy-list</name>
                        <value><array><data></data></array></value>
                    </member>
                </struct>
            </value>
        </param>
    </params>
</methodResponse>"#,
                    self.login,
                    self.session_id.as_ref().unwrap_or(&"".to_string()),
                    self.secure_session_id.as_ref().unwrap_or(&"".to_string()),
                    self.agent_id.as_ref().unwrap_or(&"".to_string()),
                    self.first_name.as_ref().unwrap_or(&"".to_string()),
                    self.last_name.as_ref().unwrap_or(&"".to_string()),
                    self.start_location.as_ref().unwrap_or(&"home".to_string()),
                    self.sim_ip.as_ref().unwrap_or(&"127.0.0.1".to_string()),
                    self.sim_port.unwrap_or(9000),
                    self.circuit_code.unwrap_or(0),
                    self.seed_capability.as_ref().unwrap_or(&"".to_string()),
                    self.message
            )
        } else {
            format!(r#"<?xml version="1.0"?>
<methodResponse>
    <params>
        <param>
            <value>
                <struct>
                    <member>
                        <name>login</name>
                        <value><string>false</string></value>
                    </member>
                    <member>
                        <name>reason</name>
                        <value><string>{}</string></value>
                    </member>
                    <member>
                        <name>message</name>
                        <value><string>{}</string></value>
                    </member>
                </struct>
            </value>
        </param>
    </params>
</methodResponse>"#,
                    self.reason,
                    self.message
            )
        }
    }
}

impl Default for LoginService {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export for backward compatibility
pub use LoginService as OpenSimLoginService;
pub use OpenSimLoginResponse as LoginResponse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_service() {
        let service = LoginService::new();
        service.add_test_user("Test".to_string(), "User".to_string(), "password".to_string());

        let request = ParsedLoginRequest {
            first: "Test".to_string(),
            last: "User".to_string(),
            passwd: "password".to_string(),
            start: "home".to_string(),
            channel: "Mutsea".to_string(),
            version: "1.0.0".to_string(),
            platform: "Test".to_string(),
            mac: "00:00:00:00:00:00".to_string(),
            id0: "test".to_string(),
            agree_to_tos: "true".to_string(),
            read_critical: "true".to_string(),
            viewer_digest: "test".to_string(),
            options: vec![],
        };

        let response = service.authenticate(&request).unwrap();
        assert_eq!(response.login, "true");
    }

    #[test]
    fn test_session_validation() {
        let service = LoginService::new();
        service.add_test_user("Test".to_string(), "User".to_string(), "password".to_string());

        let request = ParsedLoginRequest {
            first: "Test".to_string(),
            last: "User".to_string(),
            passwd: "password".to_string(),
            start: "home".to_string(),
            channel: "Mutsea".to_string(),
            version: "1.0.0".to_string(),
            platform: "Test".to_string(),
            mac: "00:00:00:00:00:00".to_string(),
            id0: "test".to_string(),
            agree_to_tos: "true".to_string(),
            read_critical: "true".to_string(),
            viewer_digest: "test".to_string(),
            options: vec![],
        };

        let response = service.authenticate(&request).unwrap();
        if let Some(session_id) = &response.session_id {
            if let Some(agent_id_str) = &response.agent_id {
                let agent_id = UserId::from_uuid(uuid::Uuid::parse_str(agent_id_str).unwrap());
                assert!(service.validate_session(session_id, &agent_id));
            }
        }
    }
}