//! Login service implementation

use crate::{ProtocolError, ProtocolResult, http::LoginRequest, LoginResponse};
use mutsea_core::{UserId, UserAccount};
use std::collections::HashMap;
use uuid::Uuid;

/// Login service for handling user authentication
pub struct LoginService {
    // In a real implementation, this would connect to user services
    test_users: HashMap<String, TestUser>,
}

/// Test user for development
#[derive(Debug, Clone)]
struct TestUser {
    first_name: String,
    last_name: String,
    password: String,
    user_id: UserId,
}

impl LoginService {
    /// Create a new login service
    pub fn new() -> Self {
        let mut service = Self {
            test_users: HashMap::new(),
        };
        
        // Add a test user for development
        service.add_test_user(
            "Test".to_string(),
            "User".to_string(),
            "password".to_string(),
        );
        
        service
    }
    
    /// Add a test user
    pub fn add_test_user(&mut self, first_name: String, last_name: String, password: String) {
        let key = format!("{} {}", first_name, last_name);
        let user = TestUser {
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            password,
            user_id: UserId::new(),
        };
        self.test_users.insert(key, user);
    }
    
    /// Process login request
    pub fn process_login(&self, request: &LoginRequest) -> ProtocolResult<LoginResponse> {
        let user_key = format!("{} {}", request.first, request.last);
        
        if let Some(user) = self.test_users.get(&user_key) {
            if user.password == request.passwd {
                // Successful login
                let session_id = Uuid::new_v4();
                let secure_session_id = Uuid::new_v4();
                let circuit_code = rand::random::<u32>();
                
                let seed_capability = format!(
                    "http://localhost:8080/caps/{}/",
                    Uuid::new_v4()
                );
                
                Ok(LoginResponse::success(
                    session_id,
                    secure_session_id,
                    user.user_id,
                    user.first_name.clone(),
                    user.last_name.clone(),
                    mutsea_core::RegionId::new(),
                    "127.0.0.1".to_string(),
                    9000,
                    circuit_code,
                    seed_capability,
                ))
            } else {
                Ok(LoginResponse::failure("Invalid password".to_string()))
            }
        } else {
            Ok(LoginResponse::failure("User not found".to_string()))
        }
    }
    
    /// Validate session
    pub fn validate_session(&self, session_id: &str, user_id: &UserId) -> bool {
        // In a real implementation, this would validate against stored sessions
        // For now, just return true for any non-empty session
        !session_id.is_empty()
    }
    
    /// Create guest user session
    pub fn create_guest_session(&self) -> ProtocolResult<LoginResponse> {
        let session_id = Uuid::new_v4();
        let secure_session_id = Uuid::new_v4();
        let circuit_code = rand::random::<u32>();
        let user_id = UserId::new();
        
        let seed_capability = format!(
            "http://localhost:8080/caps/{}/",
            Uuid::new_v4()
        );
        
        Ok(LoginResponse::success(
            session_id,
            secure_session_id,
            user_id,
            "Guest".to_string(),
            "User".to_string(),
            mutsea_core::RegionId::new(),
            "127.0.0.1".to_string(),
            9000,
            circuit_code,
            seed_capability,
        ))
    }
    
    /// Get user information by name
    pub fn get_user_by_name(&self, first_name: &str, last_name: &str) -> Option<UserId> {
        let user_key = format!("{} {}", first_name, last_name);
        self.test_users.get(&user_key).map(|user| user.user_id)
    }
    
    /// List available test users
    pub fn list_test_users(&self) -> Vec<String> {
        self.test_users.keys().cloned().collect()
    }
}

impl Default for LoginService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::http::LoginRequest;
    use super::*;
    
    #[test]
    fn test_login_service() {
        let service = LoginService::new();
        
        let request = LoginRequest {
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
        
        let response = service.process_login(&request).unwrap();
        assert_eq!(response.login, "true");
        assert_eq!(response.first_name, "Test");
        assert_eq!(response.last_name, "User");
    }
    
    #[test]
    fn test_invalid_login() {
        let service = LoginService::new();
        
        let request = LoginRequest {
            first: "Invalid".to_string(),
            last: "User".to_string(),
            passwd: "wrongpassword".to_string(),
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
        
        let response = service.process_login(&request).unwrap();
        assert_eq!(response.login, "false");
    }
}