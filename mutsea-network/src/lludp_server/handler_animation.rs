//! mutsea-network/src/lludp_server/animation_handler.rs
//! Agent animation and appearance handler

use crate::NetworkResult;
use mutsea_protocol::{Packet, constants::packet_types};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{debug, warn};

use super::{CircuitInfo};

/// Animation handler for agent animations and appearance
#[derive(Clone)]
pub struct AnimationHandler;

impl AnimationHandler {
    pub fn new() -> Self {
        Self
    }

    /// Handle AgentAnimation message
    pub async fn handle_agent_animation(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        addr: SocketAddr,
        packet: &Packet,
    ) -> NetworkResult<()> {
        if packet.payload.len() < 33 { // Minimum size for AgentAnimation
            warn!("AgentAnimation packet too short from {}", addr);
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

        debug!("Agent animation from circuit {}", circuit_code);

        // Parse animation data
        let animation_data = self.parse_animation_packet(&packet.payload)?;
        
        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }

        // TODO: Store animation state and broadcast to nearby users
        debug!("Animation update for circuit {}: {} animations", 
               circuit_code, animation_data.animations.len());

        Ok(())
    }

    /// Parse animation packet data
    fn parse_animation_packet(&self, payload: &[u8]) -> NetworkResult<AnimationData> {
        let mut offset = 1; // Skip message ID

        // AgentData block
        let _agent_id = &payload[offset..offset + 16];
        offset += 16;
        
        let _session_id = &payload[offset..offset + 16];
        offset += 16;

        // AnimationList block count
        let animation_count = payload[offset];
        offset += 1;

        let mut animations = Vec::new();
        
        for _ in 0..animation_count {
            if offset + 20 <= payload.len() {
                // Animation ID (UUID - 16 bytes)
                let animation_id = &payload[offset..offset + 16];
                offset += 16;
                
                // Start animation flag
                let start_anim = payload[offset] != 0;
                offset += 1;
                
                // Animation sequence
                let sequence = u32::from_le_bytes([
                    payload[offset], payload[offset + 1], 
                    payload[offset + 2], payload[offset + 3]
                ]);
                offset += 4;

                animations.push(AnimationInfo {
                    animation_id: animation_id.to_vec(),
                    start_anim,
                    sequence,
                });
            }
        }

        Ok(AnimationData { animations })
    }

    /// Handle agent appearance updates
    pub async fn handle_agent_appearance(
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

        debug!("Agent appearance update from circuit {}", circuit_code);

        // Update last activity
        let mut circuits_guard = circuits.write().await;
        if let Some(circuit) = circuits_guard.get_mut(&circuit_code) {
            circuit.last_activity = Instant::now();
        }

        // TODO: Parse and store appearance data
        // For now, just acknowledge the update
        Ok(())
    }

    /// Broadcast animation update to nearby agents
    pub async fn broadcast_animation_update(
        &self,
        circuits: &Arc<RwLock<HashMap<u32, CircuitInfo>>>,
        source_circuit: u32,
        animation_data: &AnimationData,
        range: f32,
    ) -> NetworkResult<usize> {
        let circuits_guard = circuits.read().await;
        
        let Some(source) = circuits_guard.get(&source_circuit) else {
            return Ok(0);
        };
        
        let source_position = source.position;
        let mut broadcast_count = 0;
        
        // Find nearby circuits
        for (circuit_code, circuit) in circuits_guard.iter() {
            if *circuit_code != source_circuit && circuit.authenticated {
                let distance = (circuit.position - source_position).length();
                if distance <= range {
                    // TODO: Send animation update packet to this circuit
                    debug!("Would broadcast animation to circuit {}", circuit_code);
                    broadcast_count += 1;
                }
            }
        }
        
        Ok(broadcast_count)
    }
}

/// Parsed animation data
#[derive(Debug, Clone)]
pub struct AnimationData {
    pub animations: Vec<AnimationInfo>,
}

/// Individual animation information
#[derive(Debug, Clone)]
pub struct AnimationInfo {
    pub animation_id: Vec<u8>, // UUID bytes
    pub start_anim: bool,
    pub sequence: u32,
}

impl Default for AnimationHandler {
    fn default() -> Self {
        Self::new()
    }
}