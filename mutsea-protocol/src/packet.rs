//! LLUDP packet structures and parsing

use crate::constants::*;
use crate::ProtocolError;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

/// LLUDP packet header
#[derive(Debug, Clone, PartialEq)]
pub struct PacketHeader {
    pub flags: u8,
    pub sequence: u32,
    pub extra: u8,
}

impl PacketHeader {
    /// Create a new packet header
    pub fn new(flags: u8, sequence: u32) -> Self {
        Self {
            flags,
            sequence,
            extra: 0,
        }
    }

    /// Check if packet is reliable
    pub fn is_reliable(&self) -> bool {
        (self.flags & flags::RELIABLE) != 0
    }

    /// Check if packet is resent
    pub fn is_resent(&self) -> bool {
        (self.flags & flags::RESENT) != 0
    }

    /// Check if packet is an acknowledgment
    pub fn is_ack(&self) -> bool {
        (self.flags & flags::ACK) != 0
    }

    /// Check if packet is zero-coded
    pub fn is_zerocoded(&self) -> bool {
        (self.flags & flags::ZEROCODED) != 0
    }

    /// Check if packet has appended acknowledgments
    pub fn has_appended_acks(&self) -> bool {
        (self.flags & flags::APPENDED_ACKS) != 0
    }

    /// Serialize header to bytes
    pub fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut buffer = Vec::with_capacity(6);
        buffer.write_u8(self.flags)?;
        buffer.write_u32::<BigEndian>(self.sequence)?;
        buffer.write_u8(self.extra)?;
        Ok(buffer)
    }

    /// Deserialize header from bytes
    pub fn deserialize(data: &[u8]) -> Result<(Self, usize), ProtocolError> {
        if data.len() < 6 {
            return Err(ProtocolError::InvalidPacket("Header too short".to_string()));
        }

        let mut cursor = Cursor::new(data);
        let flags = cursor.read_u8()?;
        let sequence = cursor.read_u32::<BigEndian>()?;
        let extra = cursor.read_u8()?;

        Ok((Self { flags, sequence, extra }, 6))
    }
}

/// LLUDP packet structure
#[derive(Debug, Clone)]
pub struct Packet {
    pub header: PacketHeader,
    pub message_id: Option<u32>,
    pub payload: Vec<u8>,
    pub appended_acks: Vec<u32>,
}

impl Packet {
    /// Create a new packet
    pub fn new(flags: u8, sequence: u32, payload: Vec<u8>) -> Self {
        Self {
            header: PacketHeader::new(flags, sequence),
            message_id: None,
            payload,
            appended_acks: Vec::new(),
        }
    }

    /// Create a reliable packet
    pub fn reliable(sequence: u32, payload: Vec<u8>) -> Self {
        Self::new(flags::RELIABLE, sequence, payload)
    }

    /// Create an acknowledgment packet
    pub fn ack(acks: Vec<u32>) -> Self {
        let mut packet = Self::new(flags::ACK, 0, Vec::new());
        packet.appended_acks = acks;
        packet
    }

    /// Add acknowledgments to packet
    pub fn with_acks(mut self, acks: Vec<u32>) -> Self {
        if !acks.is_empty() {
            self.header.flags |= flags::APPENDED_ACKS;
            self.appended_acks = acks;
        }
        self
    }

    /// Set message ID for high frequency packets
    pub fn with_message_id(mut self, message_id: u32) -> Self {
        self.message_id = Some(message_id);
        self
    }

    /// Serialize packet to bytes
    pub fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut buffer = Vec::new();

        // Write header
        buffer.extend_from_slice(&self.header.serialize()?);

        // Write message ID if present
        if let Some(message_id) = self.message_id {
            if message_id <= 255 {
                buffer.write_u8(message_id as u8)?;
            } else if message_id <= 65535 {
                buffer.write_u8(0xFF)?;
                buffer.write_u8((message_id & 0xFF) as u8)?;
                buffer.write_u8(((message_id >> 8) & 0xFF) as u8)?;
            } else {
                buffer.write_u8(0xFF)?;
                buffer.write_u8(0xFF)?;
                buffer.write_u32::<LittleEndian>(message_id)?;
            }
        }

        // Write payload
        if self.header.is_zerocoded() {
            buffer.extend_from_slice(&self.zero_encode(&self.payload)?);
        } else {
            buffer.extend_from_slice(&self.payload);
        }

        // Write appended acknowledgments
        if self.header.has_appended_acks() {
            buffer.write_u8(self.appended_acks.len() as u8)?;
            for ack in &self.appended_acks {
                buffer.write_u32::<BigEndian>(*ack)?;
            }
        }

        Ok(buffer)
    }

    /// Deserialize packet from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self, ProtocolError> {
        if data.len() < 6 {
            return Err(ProtocolError::InvalidPacket("Packet too short".to_string()));
        }

        let (header, mut offset) = PacketHeader::deserialize(data)?;
        let mut message_id = None;
        let mut payload_start = offset;

        // Check if this is a special packet type
        if offset < data.len() {
            let first_byte = data[offset];

            // Parse message ID
            if first_byte != packet_types::PACKET_ACK {
                if first_byte == 0xFF && offset + 1 < data.len() {
                    let second_byte = data[offset + 1];
                    if second_byte == 0xFF && offset + 5 < data.len() {
                        // 4-byte message ID
                        let mut cursor = Cursor::new(&data[offset + 2..]);
                        message_id = Some(cursor.read_u32::<LittleEndian>()?);
                        payload_start = offset + 6;
                    } else {
                        // 2-byte message ID
                        message_id = Some(((data[offset + 2] as u32) << 8) | (second_byte as u32));
                        payload_start = offset + 3;
                    }
                } else {
                    // 1-byte message ID
                    message_id = Some(first_byte as u32);
                    payload_start = offset + 1;
                }
            } else {
                payload_start = offset + 1;
            }
        }

        // Extract payload
        let mut payload_end = data.len();
        let mut appended_acks = Vec::new();

        if header.has_appended_acks() && payload_end > 0 {
            // Find appended acks at the end
            let ack_count_pos = payload_end - 1;
            if ack_count_pos < data.len() {
                let ack_count = data[ack_count_pos] as usize;
                let acks_size = ack_count * 4 + 1; // 4 bytes per ack + count byte

                if payload_end >= acks_size {
                    payload_end -= acks_size;

                    // Parse acknowledgments
                    let acks_start = payload_end;
                    let mut cursor = Cursor::new(&data[acks_start..acks_start + ack_count * 4]);
                    for _ in 0..ack_count {
                        appended_acks.push(cursor.read_u32::<BigEndian>()?);
                    }
                }
            }
        }

        // Extract payload
        let mut payload = if payload_start < payload_end {
            data[payload_start..payload_end].to_vec()
        } else {
            Vec::new()
        };

        // Decode zero-coded payload
        if header.is_zerocoded() && !payload.is_empty() {
            payload = Self::zero_decode(&payload)?;
        }

        Ok(Self {
            header,
            message_id,
            payload,
            appended_acks,
        })
    }

    /// Zero-encode data (compress repeated zero bytes)
    fn zero_encode(&self, data: &[u8]) -> Result<Vec<u8>, ProtocolError> {
        let mut encoded = Vec::new();
        let mut i = 0;

        while i < data.len() {
            if data[i] == 0 {
                // Count consecutive zeros
                let mut zero_count = 0;
                while i + zero_count < data.len() && data[i + zero_count] == 0 && zero_count < 255 {
                    zero_count += 1;
                }

                if zero_count == 1 {
                    // Single zero, encode as 0x00 0x01
                    encoded.push(0x00);
                    encoded.push(0x01);
                } else {
                    // Multiple zeros, encode as 0x00 count
                    encoded.push(0x00);
                    encoded.push(zero_count as u8);
                }
                i += zero_count;
            } else {
                encoded.push(data[i]);
                i += 1;
            }
        }

        Ok(encoded)
    }

    /// Zero-decode data (decompress repeated zero bytes)
    fn zero_decode(data: &[u8]) -> Result<Vec<u8>, ProtocolError> {
        let mut decoded = Vec::new();
        let mut i = 0;

        while i < data.len() {
            if data[i] == 0 {
                if i + 1 < data.len() {
                    let count = data[i + 1] as usize;
                    if count == 0 {
                        return Err(ProtocolError::InvalidPacket("Invalid zero-encoding".to_string()));
                    }
                    // Add 'count' zeros
                    decoded.extend(vec![0; count]);
                    i += 2;
                } else {
                    return Err(ProtocolError::InvalidPacket("Incomplete zero-encoding".to_string()));
                }
            } else {
                decoded.push(data[i]);
                i += 1;
            }
        }

        Ok(decoded)
    }

    /// Get packet size in bytes
    pub fn size(&self) -> usize {
        let mut size = 6; // Header size

        // Message ID size
        if let Some(message_id) = self.message_id {
            if message_id <= 255 {
                size += 1;
            } else if message_id <= 65535 {
                size += 3;
            } else {
                size += 6;
            }
        }

        // Payload size
        if self.header.is_zerocoded() {
            size += self.zero_encode(&self.payload).map(|v| v.len()).unwrap_or(0);
        } else {
            size += self.payload.len();
        }

        // Appended acks size
        if self.header.has_appended_acks() {
            size += 1 + (self.appended_acks.len() * 4);
        }

        size
    }

    /// Check if packet fits in MTU
    pub fn fits_in_mtu(&self) -> bool {
        self.size() <= MAX_PACKET_SIZE
    }
}

/// Packet acknowledgment
#[derive(Debug, Clone, PartialEq)]
pub struct PacketAck {
    pub sequence: u32,
    pub timestamp: std::time::Instant,
}

impl PacketAck {
    /// Create a new packet acknowledgment
    pub fn new(sequence: u32) -> Self {
        Self {
            sequence,
            timestamp: std::time::Instant::now(),
        }
    }

    /// Check if acknowledgment has timed out
    pub fn is_timed_out(&self, timeout: std::time::Duration) -> bool {
        self.timestamp.elapsed() > timeout
    }
}

/// Reliable packet waiting for acknowledgment
#[derive(Debug, Clone)]
pub struct ReliablePacket {
    pub packet: Packet,
    pub timestamp: std::time::Instant,
    pub resend_count: u8,
}

impl ReliablePacket {
    /// Create a new reliable packet
    pub fn new(packet: Packet) -> Self {
        Self {
            packet,
            timestamp: std::time::Instant::now(),
            resend_count: 0,
        }
    }

    /// Check if packet should be resent
    pub fn should_resend(&self, timeout: std::time::Duration, max_resends: u8) -> bool {
        self.resend_count < max_resends && self.timestamp.elapsed() > timeout
    }

    /// Mark packet for resend
    pub fn mark_resent(&mut self) {
        self.resend_count += 1;
        self.timestamp = std::time::Instant::now();
        self.packet.header.flags |= flags::RESENT;
    }

    /// Check if packet has exceeded max resends
    pub fn has_exceeded_max_resends(&self, max_resends: u8) -> bool {
        self.resend_count >= max_resends
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_header_serialization() {
        let header = PacketHeader::new(flags::RELIABLE | flags::ZEROCODED, 12345);
        let serialized = header.serialize().unwrap();
        let (deserialized, size) = PacketHeader::deserialize(&serialized).unwrap();

        assert_eq!(header, deserialized);
        assert_eq!(size, 6);
        assert!(deserialized.is_reliable());
        assert!(deserialized.is_zerocoded());
    }

    #[test]
    fn test_packet_serialization() {
        let payload = b"Hello, World!".to_vec();
        let packet = Packet::reliable(12345, payload.clone());

        let serialized = packet.serialize().unwrap();
        let deserialized = Packet::deserialize(&serialized).unwrap();

        assert_eq!(packet.header.sequence, deserialized.header.sequence);
        assert_eq!(packet.payload, deserialized.payload);
        assert!(deserialized.header.is_reliable());
    }

    #[test]
    fn test_zero_encoding() {
        let data = vec![1, 2, 0, 0, 0, 3, 4, 0, 5];
        let packet = Packet::new(flags::ZEROCODED, 1, data.clone());

        let encoded = packet.zero_encode(&data).unwrap();
        let decoded = Packet::zero_decode(&encoded).unwrap();

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_packet_with_acks() {
        let payload = b"Test".to_vec();
        let acks = vec![100, 200, 300];
        let packet = Packet::reliable(12345, payload).with_acks(acks.clone());

        let serialized = packet.serialize().unwrap();
        let deserialized = Packet::deserialize(&serialized).unwrap();

        assert_eq!(packet.appended_acks, deserialized.appended_acks);
        assert!(deserialized.header.has_appended_acks());
    }

    #[test]
    fn test_reliable_packet_resend() {
        let packet = Packet::reliable(12345, b"Test".to_vec());
        let mut reliable = ReliablePacket::new(packet);

        assert_eq!(reliable.resend_count, 0);
        assert!(!reliable.packet.header.is_resent());

        reliable.mark_resent();

        assert_eq!(reliable.resend_count, 1);
        assert!(reliable.packet.header.is_resent());
    }
}