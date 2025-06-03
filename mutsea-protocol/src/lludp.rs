//! LLUDP protocol implementation

use crate::{ProtocolError, ProtocolResult, constants::*};
use std::collections::HashMap;
use std::time::Instant;

/// LLUDP protocol handler
pub struct LLUDPProtocol {
    circuits: HashMap<u32, Circuit>,
    message_templates: MessageTemplateRegistry,
}

/// Circuit information for LLUDP connections
#[derive(Debug, Clone)]
pub struct Circuit {
    pub code: u32,
    pub ip_endpoint: std::net::SocketAddr,
    pub sequence_in: u32,
    pub sequence_out: u32,
    pub pending_acks: Vec<u32>,
    pub last_ping_id: u8,
    pub last_ping_time: Instant,
    pub reliable_packets: HashMap<u32, ReliablePacketData>,
}

/// Reliable packet data waiting for acknowledgment
#[derive(Debug, Clone)]
pub struct ReliablePacketData {
    pub data: Vec<u8>,
    pub timestamp: Instant,
    pub resend_count: u8,
}

/// Message template registry for packet parsing
pub struct MessageTemplateRegistry {
    templates: HashMap<u32, MessageTemplate>,
}

/// Message template definition
#[derive(Debug, Clone)]
pub struct MessageTemplate {
    pub id: u32,
    pub name: String,
    pub frequency: MessageFrequency,
    pub trust: MessageTrust,
    pub blocks: Vec<MessageBlock>,
}

/// Message frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageFrequency {
    Fixed,
    Low,
    Medium,
    High,
}

/// Message trust level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageTrust {
    NotTrusted,
    Trusted,
}

/// Message block definition
#[derive(Debug, Clone)]
pub struct MessageBlock {
    pub name: String,
    pub block_type: BlockType,
    pub fields: Vec<MessageField>,
}

/// Block type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Single,
    Multiple,
    Variable,
}

/// Message field definition
#[derive(Debug, Clone)]
pub struct MessageField {
    pub name: String,
    pub field_type: FieldType,
    pub size: Option<usize>,
}

/// Field type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F32,
    F64,
    UUID,
    Bool,
    Vector3,
    Vector4,
    Quaternion,
    String,
    Fixed(usize),
    Variable,
}

impl LLUDPProtocol {
    /// Create a new LLUDP protocol handler
    pub fn new() -> Self {
        Self {
            circuits: HashMap::new(),
            message_templates: MessageTemplateRegistry::new(),
        }
    }
    
    /// Add a circuit
    pub fn add_circuit(&mut self, circuit: Circuit) {
        self.circuits.insert(circuit.code, circuit);
    }
    
    /// Get circuit by code
    pub fn get_circuit(&self, code: u32) -> Option<&Circuit> {
        self.circuits.get(&code)
    }
    
    /// Get mutable circuit by code
    pub fn get_circuit_mut(&mut self, code: u32) -> Option<&mut Circuit> {
        self.circuits.get_mut(&code)
    }
    
    /// Remove circuit
    pub fn remove_circuit(&mut self, code: u32) -> Option<Circuit> {
        self.circuits.remove(&code)
    }
    
    /// Process reliable packet acknowledgments
    pub fn process_acks(&mut self, circuit_code: u32, acks: &[u32]) -> ProtocolResult<()> {
        if let Some(circuit) = self.get_circuit_mut(circuit_code) {
            for &ack in acks {
                circuit.reliable_packets.remove(&ack);
            }
        }
        Ok(())
    }
    
    /// Get packets that need resending
    pub fn get_packets_for_resend(&mut self, circuit_code: u32, timeout: std::time::Duration, max_resends: u8) -> Vec<u32> {
        let mut packets_to_resend = Vec::new();
        
        if let Some(circuit) = self.get_circuit_mut(circuit_code) {
            let now = Instant::now();
            let mut packets_to_remove = Vec::new();
            
            for (sequence, packet) in &mut circuit.reliable_packets {
                if packet.timestamp.elapsed() > timeout {
                    if packet.resend_count < max_resends {
                        packet.resend_count += 1;
                        packet.timestamp = now;
                        packets_to_resend.push(*sequence);
                    } else {
                        packets_to_remove.push(*sequence);
                    }
                }
            }
            
            // Remove packets that exceeded max resends
            for sequence in packets_to_remove {
                circuit.reliable_packets.remove(&sequence);
            }
        }
        
        packets_to_resend
    }
}

impl Circuit {
    /// Create a new circuit
    pub fn new(code: u32, endpoint: std::net::SocketAddr) -> Self {
        Self {
            code,
            ip_endpoint: endpoint,
            sequence_in: 0,
            sequence_out: 0,
            pending_acks: Vec::new(),
            last_ping_id: 0,
            last_ping_time: Instant::now(),
            reliable_packets: HashMap::new(),
        }
    }
    
    /// Get next outbound sequence number
    pub fn next_sequence_out(&mut self) -> u32 {
        self.sequence_out += 1;
        self.sequence_out
    }
    
    /// Process inbound sequence number
    pub fn process_sequence_in(&mut self, sequence: u32) -> bool {
        if sequence > self.sequence_in {
            // Add missing sequences to pending acks
            for missing in (self.sequence_in + 1)..sequence {
                self.pending_acks.push(missing);
            }
            self.sequence_in = sequence;
            true
        } else {
            false
        }
    }
    
    /// Add acknowledgment
    pub fn add_ack(&mut self, sequence: u32) {
        if !self.pending_acks.contains(&sequence) {
            self.pending_acks.push(sequence);
        }
    }
    
    /// Get and clear pending acknowledgments
    pub fn take_pending_acks(&mut self) -> Vec<u32> {
        std::mem::take(&mut self.pending_acks)
    }
    
    /// Add reliable packet
    pub fn add_reliable_packet(&mut self, sequence: u32, data: Vec<u8>) {
        let packet_data = ReliablePacketData {
            data,
            timestamp: Instant::now(),
            resend_count: 0,
        };
        self.reliable_packets.insert(sequence, packet_data);
    }
}

impl MessageTemplateRegistry {
    /// Create a new message template registry
    pub fn new() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };
        
        // Add basic message templates
        registry.add_basic_templates();
        registry
    }
    
    /// Add basic message templates for OpenSim compatibility
    fn add_basic_templates(&mut self) {
        // StartPingCheck message
        self.templates.insert(1, MessageTemplate {
            id: 1,
            name: "StartPingCheck".to_string(),
            frequency: MessageFrequency::High,
            trust: MessageTrust::NotTrusted,
            blocks: vec![
                MessageBlock {
                    name: "PingID".to_string(),
                    block_type: BlockType::Single,
                    fields: vec![
                        MessageField {
                            name: "PingID".to_string(),
                            field_type: FieldType::U8,
                            size: None,
                        },
                        MessageField {
                            name: "OldestUnacked".to_string(),
                            field_type: FieldType::U32,
                            size: None,
                        },
                    ],
                },
            ],
        });
        
        // CompletePingCheck message
        self.templates.insert(2, MessageTemplate {
            id: 2,
            name: "CompletePingCheck".to_string(),
            frequency: MessageFrequency::High,
            trust: MessageTrust::NotTrusted,
            blocks: vec![
                MessageBlock {
                    name: "PingID".to_string(),
                    block_type: BlockType::Single,
                    fields: vec![
                        MessageField {
                            name: "PingID".to_string(),
                            field_type: FieldType::U8,
                            size: None,
                        },
                    ],
                },
            ],
        });
        
        // AgentUpdate message
        self.templates.insert(4, MessageTemplate {
            id: 4,
            name: "AgentUpdate".to_string(),
            frequency: MessageFrequency::High,
            trust: MessageTrust::Trusted,
            blocks: vec![
                MessageBlock {
                    name: "AgentData".to_string(),
                    block_type: BlockType::Single,
                    fields: vec![
                        MessageField {
                            name: "AgentID".to_string(),
                            field_type: FieldType::UUID,
                            size: None,
                        },
                        MessageField {
                            name: "SessionID".to_string(),
                            field_type: FieldType::UUID,
                            size: None,
                        },
                        MessageField {
                            name: "BodyRotation".to_string(),
                            field_type: FieldType::Quaternion,
                            size: None,
                        },
                        MessageField {
                            name: "HeadRotation".to_string(),
                            field_type: FieldType::Quaternion,
                            size: None,
                        },
                        MessageField {
                            name: "State".to_string(),
                            field_type: FieldType::U8,
                            size: None,
                        },
                        MessageField {
                            name: "CameraCenter".to_string(),
                            field_type: FieldType::Vector3,
                            size: None,
                        },
                        MessageField {
                            name: "CameraAtAxis".to_string(),
                            field_type: FieldType::Vector3,
                            size: None,
                        },
                        MessageField {
                            name: "CameraLeftAxis".to_string(),
                            field_type: FieldType::Vector3,
                            size: None,
                        },
                        MessageField {
                            name: "CameraUpAxis".to_string(),
                            field_type: FieldType::Vector3,
                            size: None,
                        },
                        MessageField {
                            name: "Far".to_string(),
                            field_type: FieldType::F32,
                            size: None,
                        },
                        MessageField {
                            name: "ControlFlags".to_string(),
                            field_type: FieldType::U32,
                            size: None,
                        },
                        MessageField {
                            name: "Flags".to_string(),
                            field_type: FieldType::U8,
                            size: None,
                        },
                    ],
                },
            ],
        });
    }
    
    /// Get message template by ID
    pub fn get_template(&self, id: u32) -> Option<&MessageTemplate> {
        self.templates.get(&id)
    }
    
    /// Add custom message template
    pub fn add_template(&mut self, template: MessageTemplate) {
        self.templates.insert(template.id, template);
    }
}

impl Default for LLUDPProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MessageTemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}