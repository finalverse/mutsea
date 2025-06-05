//! mutsea-network/src/lludp_server/handler_object.rs
//! Object and primitive management handler

use crate::NetworkResult;
use mutsea_core::{Vector3, Quaternion, ObjectId, UserId};
use mutsea_protocol::{Packet, constants::packet_types};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{debug, warn, info};

use super::{CircuitInfo, ServerStats};

/// Object handler for managing scene objects and primitives
#[derive(Clone)]
pub struct ObjectHandler;

/// Object information in the scene
#[derive(Debug, Clone)]
pub struct SceneObjectInfo {
    pub object_id: ObjectId,
    pub local_id: u32,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    pub velocity: Vector3,
    pub owner_id: UserId,
    pub creator_id: UserId,
    pub parent_id: Option<ObjectId>,
    pub material: u8,
    pub flags: u32,
    pub created_at: Instant,
    pub last_updated: Instant,
}

/// Object update type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectUpdateType {
    Full,
    Terse,
    Compressed,
    Cached,
}

/// Object selection data
#[derive(Debug, Clone)]
pub struct ObjectSelectData {
    pub object_ids: Vec<ObjectId>,
}

impl ObjectHandler {
    pub fn new() -> Self {
        Self
    }

    /// Handle ObjectSelect message
    pub async fn handle_object_select(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 21 { // Minimum size for ObjectSelect
            warn!("ObjectSelect packet too short from {}", addr);
            return Ok(());
        }

        // Find circuit by address
        let circuit_code = {
            let circuits_guard = circuits.read().await;
            circuits_guard.iter()
                .find(|(_, circuit)| circuit.address == addr)
                .map(|(code, _)| *code)
        };

        let Some(circuit_code) = circuit_code else {
            warn!("No circuit found for address {}", addr);
            return Ok(());
        };

        // Parse object selection
        let selection_data = self.parse_object_select(&packet.payload)?;
        debug!("Object select from circuit {}: {} objects", 
               circuit_code, selection_data.object_ids.len());

        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }
        drop(circuits_guard);

        // Send object properties for selected objects
        for object_id in &selection_data.object_ids {
            self.send_object_properties(socket, addr, *object_id).await?;
        }

        Ok(())
    }

    /// Parse object select packet
    fn parse_object_select(&self, payload: &[u8]) -> NetworkResult<ObjectSelectData> {
        let mut offset = 1; // Skip message ID

        // AgentData block
        let _agent_id = &payload[offset..offset + 16];
        offset += 16;
        
        let _session_id = &payload[offset..offset + 16];
        offset += 16;

        // ObjectData block count
        let object_count = payload[offset];
        offset += 1;

        let mut object_ids = Vec::new();
        for _ in 0..object_count {
            if offset + 4 <= payload.len() {
                let local_id = u32::from_le_bytes([
                    payload[offset], payload[offset + 1], 
                    payload[offset + 2], payload[offset + 3]
                ]);
                offset += 4;
                
                // Convert local ID to ObjectId (simplified)
                object_ids.push(ObjectId::new());
            }
        }

        Ok(ObjectSelectData { object_ids })
    }

    /// Send object properties response
    async fn send_object_properties(
        &self,
        socket: &UdpSocket,
        addr: SocketAddr,
        object_id: ObjectId,
    ) -> NetworkResult<()> {
        let mut payload = Vec::new();
        payload.push(packet_types::OBJECT_PROPERTIES as u8);

        // ObjectData block
        payload.extend_from_slice(object_id.as_uuid().as_bytes());
        
        // Object name (variable string)
        let name = "Default Object";
        let name_bytes = name.as_bytes();
        payload.push(name_bytes.len() as u8);
        payload.extend_from_slice(name_bytes);
        
        // Object description (variable string)
        let description = "A basic object";
        let desc_bytes = description.as_bytes();
        payload.push(desc_bytes.len() as u8);
        payload.extend_from_slice(desc_bytes);
        
        // Creator ID
        payload.extend_from_slice(UserId::new().as_uuid().as_bytes());
        
        // Owner ID
        payload.extend_from_slice(UserId::new().as_uuid().as_bytes());
        
        // Group ID
        payload.extend_from_slice(uuid::Uuid::nil().as_bytes());

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize ObjectProperties: {}", e)))?;

        socket.send_to(&packet_data, addr).await?;
        debug!("Sent ObjectProperties for {} to {}", object_id, addr);
        Ok(())
    }

    /// Handle ObjectDeselect message
    pub async fn handle_object_deselect(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        // Find circuit by address
        let circuit_code = {
            let circuits_guard = circuits.read().await;
            circuits_guard.iter()
                .find(|(_, circuit)| circuit.address == addr)
                .map(|(code, _)| *code)
        };

        let Some(circuit_code) = circuit_code else {
            warn!("No circuit found for address {}", addr);
            return Ok(());
        };

        debug!("Object deselect from circuit {}", circuit_code);

        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }

        Ok(())
    }

    /// Send object update to clients
    pub async fn send_object_update(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        object: &SceneObjectInfo,
        update_type: ObjectUpdateType,
        range: f32,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<usize> {
        let circuits_guard = circuits.read().await;
        let mut broadcast_count = 0;

        // Create object update packet
        let packet_data = match update_type {
            ObjectUpdateType::Full => self.create_full_object_update(object)?,
            ObjectUpdateType::Terse => self.create_terse_object_update(object)?,
            ObjectUpdateType::Compressed => self.create_compressed_object_update(object)?,
            ObjectUpdateType::Cached => self.create_cached_object_update(object)?,
        };

        // Send to nearby circuits
        for circuit in circuits_guard.values() {
            if circuit.authenticated {
                let distance = (circuit.position - object.position).length();
                if distance <= range {
                    if let Err(e) = socket.send_to(&packet_data, circuit.address).await {
                        warn!("Failed to send object update to circuit {}: {}", 
                              circuit.circuit_code, e);
                    } else {
                        broadcast_count += 1;
                    }
                }
            }
        }

        // Update stats
        if broadcast_count > 0 {
            let mut stats_guard = stats.write().await;
            stats_guard.packets_sent += broadcast_count as u64;
            stats_guard.bytes_sent += (packet_data.len() * broadcast_count) as u64;
        }

        debug!("Sent {} object update for {} to {} circuits", 
               format!("{:?}", update_type).to_lowercase(), object.object_id, broadcast_count);
        Ok(broadcast_count)
    }

    /// Create full object update packet
    fn create_full_object_update(&self, object: &SceneObjectInfo) -> NetworkResult<Vec<u8>> {
        let mut payload = Vec::new();
        payload.push(packet_types::OBJECT_UPDATE as u8);

        // RegionData block
        payload.extend_from_slice(&0u64.to_le_bytes()); // RegionHandle
        payload.extend_from_slice(&0u16.to_le_bytes()); // TimeDilation

        // ObjectData block
        payload.push(1); // Object count

        // Object update data
        payload.extend_from_slice(&object.local_id.to_le_bytes());
        payload.push(0); // State (full update)
        payload.extend_from_slice(object.object_id.as_uuid().as_bytes());
        payload.push(0); // CRC - simplified
        payload.push(object.material);
        payload.push(0); // ClickAction
        payload.extend_from_slice(&object.scale.x.to_le_bytes());
        payload.extend_from_slice(&object.scale.y.to_le_bytes());
        payload.extend_from_slice(&object.scale.z.to_le_bytes());
        payload.extend_from_slice(&object.position.x.to_le_bytes());
        payload.extend_from_slice(&object.position.y.to_le_bytes());
        payload.extend_from_slice(&object.position.z.to_le_bytes());
        payload.extend_from_slice(&object.rotation.x.to_le_bytes());
        payload.extend_from_slice(&object.rotation.y.to_le_bytes());
        payload.extend_from_slice(&object.rotation.z.to_le_bytes());
        payload.extend_from_slice(&object.rotation.w.to_le_bytes());
        payload.extend_from_slice(&object.velocity.x.to_le_bytes());
        payload.extend_from_slice(&object.velocity.y.to_le_bytes());
        payload.extend_from_slice(&object.velocity.z.to_le_bytes());

        let packet = Packet::reliable(1, payload);
        packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize full object update: {}", e)))
    }

    /// Create terse object update packet (position/rotation only)
    fn create_terse_object_update(&self, object: &SceneObjectInfo) -> NetworkResult<Vec<u8>> {
        let mut payload = Vec::new();
        payload.push(packet_types::IMPROVED_TERSE_OBJECT_UPDATE as u8);

        // RegionData block
        payload.extend_from_slice(&0u64.to_le_bytes()); // RegionHandle
        payload.extend_from_slice(&0u16.to_le_bytes()); // TimeDilation

        // ObjectData block
        payload.push(1); // Object count
        payload.extend_from_slice(&object.local_id.to_le_bytes());
        payload.push(0); // State
        
        // Terse data (position, rotation, velocity)
        payload.extend_from_slice(&object.position.x.to_le_bytes());
        payload.extend_from_slice(&object.position.y.to_le_bytes());
        payload.extend_from_slice(&object.position.z.to_le_bytes());
        payload.extend_from_slice(&object.velocity.x.to_le_bytes());
        payload.extend_from_slice(&object.velocity.y.to_le_bytes());
        payload.extend_from_slice(&object.velocity.z.to_le_bytes());
        payload.extend_from_slice(&object.rotation.x.to_le_bytes());
        payload.extend_from_slice(&object.rotation.y.to_le_bytes());
        payload.extend_from_slice(&object.rotation.z.to_le_bytes());
        payload.extend_from_slice(&object.rotation.w.to_le_bytes());

        let packet = Packet::reliable(1, payload);
        packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize terse object update: {}", e)))
    }

    /// Create compressed object update packet
    fn create_compressed_object_update(&self, object: &SceneObjectInfo) -> NetworkResult<Vec<u8>> {
        // For now, use the same as full update
        // In a real implementation, this would use compression
        self.create_full_object_update(object)
    }

    /// Create cached object update packet
    fn create_cached_object_update(&self, object: &SceneObjectInfo) -> NetworkResult<Vec<u8>> {
        let mut payload = Vec::new();
        payload.push(packet_types::OBJECT_UPDATE_CACHED as u8);

        // RegionData block
        payload.extend_from_slice(&0u64.to_le_bytes()); // RegionHandle

        // ObjectData block
        payload.push(1); // Object count
        payload.extend_from_slice(&object.local_id.to_le_bytes());
        payload.extend_from_slice(&0u32.to_le_bytes()); // Cache ID

        let packet = Packet::reliable(1, payload);
        packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize cached object update: {}", e)))
    }

    /// Handle object grab
    pub async fn handle_object_grab(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        // Find circuit by address
        let circuit_code = {
            let circuits_guard = circuits.read().await;
            circuits_guard.iter()
                .find(|(_, circuit)| circuit.address == addr)
                .map(|(code, _)| *code)
        };

        let Some(circuit_code) = circuit_code else {
            warn!("No circuit found for address {}", addr);
            return Ok(());
        };

        debug!("Object grab from circuit {}", circuit_code);

        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }

        // TODO: Process object grab logic
        Ok(())
    }

    /// Kill/remove object from scene
    pub async fn kill_object(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        socket: &UdpSocket,
        object_id: ObjectId,
        local_id: u32,
        stats: &Arc<RwLock<ServerStats>>,
    ) -> NetworkResult<usize> {
        let circuits_guard = circuits.read().await;
        let mut broadcast_count = 0;

        let mut payload = Vec::new();
        payload.push(packet_types::KILL_OBJECT as u8);

        // ObjectData block
        payload.push(1); // Object count
        payload.extend_from_slice(&local_id.to_le_bytes());

        let packet = Packet::reliable(1, payload);
        let packet_data = packet.serialize()
            .map_err(|e| crate::NetworkError::Protocol(format!("Failed to serialize KillObject: {}", e)))?;

        // Send to all authenticated circuits
        for circuit in circuits_guard.values() {
            if circuit