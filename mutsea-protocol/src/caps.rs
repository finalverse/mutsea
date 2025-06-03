//! Capability system for HTTP services

use crate::{ProtocolError, ProtocolResult, Capability};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Capability manager for handling HTTP capabilities
pub struct CapabilityManager {
    capabilities: HashMap<String, Capability>,
    handlers: HashMap<String, Box<dyn CapabilityHandler>>,
}

/// Capability handler trait
pub trait CapabilityHandler: Send + Sync {
    fn handle_request(&self, data: &[u8]) -> ProtocolResult<Vec<u8>>;
    fn capability_name(&self) -> &str;
}

/// Seed capability response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedCapabilityResponse {
    pub capabilities: HashMap<String, String>,
}

impl CapabilityManager {
    /// Create a new capability manager
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
            handlers: HashMap::new(),
        }
    }
    
    /// Add capability
    pub fn add_capability(&mut self, capability: Capability) {
        self.capabilities.insert(capability.name.clone(), capability);
    }
    
    /// Add capability handler
    pub fn add_handler<H: CapabilityHandler + 'static>(&mut self, handler: H) {
        let name = handler.capability_name().to_string();
        self.handlers.insert(name, Box::new(handler));
    }
    
    /// Get capability by name
    pub fn get_capability(&self, name: &str) -> Option<&Capability> {
        self.capabilities.get(name)
    }
    
    /// Process capability request
    pub fn process_request(&self, capability_name: &str, data: &[u8]) -> ProtocolResult<Vec<u8>> {
        if let Some(handler) = self.handlers.get(capability_name) {
            handler.handle_request(data)
        } else {
            Err(ProtocolError::Generic(format!("Capability not found: {}", capability_name)))
        }
    }
    
    /// Generate seed capability response
    pub fn generate_seed_response(&self, base_url: &str) -> SeedCapabilityResponse {
        let mut capabilities = HashMap::new();
        
        for (name, _) in &self.capabilities {
            let cap_url = format!("{}/caps/{}/{}", base_url, Uuid::new_v4(), name);
            capabilities.insert(name.clone(), cap_url);
        }
        
        SeedCapabilityResponse { capabilities }
    }
    
    /// Register default capabilities
    pub fn register_defaults(&mut self, base_url: &str) {
        // EventQueueGet
        self.add_capability(Capability::new(
            "EventQueueGet".to_string(),
            format!("{}/caps/event_queue", base_url),
        ));
        
        // UploadBakedTexture
        self.add_capability(Capability::new(
            "UploadBakedTexture".to_string(),
            format!("{}/caps/upload_baked", base_url),
        ));
        
        // GetTexture
        self.add_capability(Capability::new(
            "GetTexture".to_string(),
            format!("{}/caps/get_texture", base_url),
        ));
        
        // GetMesh
        self.add_capability(Capability::new(
            "GetMesh".to_string(),
            format!("{}/caps/get_mesh", base_url),
        ));
        
        // FetchInventoryDescendents2
        self.add_capability(Capability::new(
            "FetchInventoryDescendents2".to_string(),
            format!("{}/caps/fetch_inventory", base_url),
        ));
        
        // WebFetchInventoryDescendents
        self.add_capability(Capability::new(
            "WebFetchInventoryDescendents".to_string(),
            format!("{}/caps/web_fetch_inventory", base_url),
        ));
        
        // Add handlers for basic capabilities
        self.add_handler(EventQueueHandler::new());
        self.add_handler(TextureHandler::new());
    }
}

/// Event queue capability handler
pub struct EventQueueHandler;

impl EventQueueHandler {
    pub fn new() -> Self {
        Self
    }
}

impl CapabilityHandler for EventQueueHandler {
    fn handle_request(&self, _data: &[u8]) -> ProtocolResult<Vec<u8>> {
        // Return empty event queue for now
        let response = serde_json::json!({
            "events": [],
            "id": 1
        });
        
        Ok(serde_json::to_vec(&response)
            .map_err(|e| ProtocolError::Generic(format!("JSON serialization error: {}", e)))?)
    }
    
    fn capability_name(&self) -> &str {
        "EventQueueGet"
    }
}

/// Texture capability handler
pub struct TextureHandler;

impl TextureHandler {
    pub fn new() -> Self {
        Self
    }
}

impl CapabilityHandler for TextureHandler {
    fn handle_request(&self, _data: &[u8]) -> ProtocolResult<Vec<u8>> {
        // Return texture not found for now
        let response = serde_json::json!({
            "error": "Texture not found"
        });
        
        Ok(serde_json::to_vec(&response)
            .map_err(|e| ProtocolError::Generic(format!("JSON serialization error: {}", e)))?)
    }
    
    fn capability_name(&self) -> &str {
        "GetTexture"
    }
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for EventQueueHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TextureHandler {
    fn default() -> Self {
        Self::new()
    }
}