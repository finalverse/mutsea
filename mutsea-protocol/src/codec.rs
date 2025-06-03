//! Protocol encoding and decoding utilities

use crate::{ProtocolError, ProtocolResult};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use mutsea_core::{Vector3, Quaternion};
use std::io::{Cursor, Read, Write};
use uuid::Uuid;

/// Protocol codec for encoding and decoding data types
pub struct ProtocolCodec;

impl ProtocolCodec {
    /// Encode a UUID
    pub fn encode_uuid(uuid: &Uuid) -> Vec<u8> {
        uuid.as_bytes().to_vec()
    }
    
    /// Decode a UUID
    pub fn decode_uuid(data: &[u8]) -> ProtocolResult<Uuid> {
        if data.len() < 16 {
            return Err(ProtocolError::Decoding("UUID requires 16 bytes".to_string()));
        }
        
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&data[..16]);
        Ok(Uuid::from_bytes(bytes))
    }

    /// Encode a Vector3
    pub fn encode_vector3(vector: &Vector3) -> ProtocolResult<Vec<u8>> {
        let mut buffer = Vec::with_capacity(12);
        buffer.write_f32::<LittleEndian>(vector.x)?;
        buffer.write_f32::<LittleEndian>(vector.y)?;
        buffer.write_f32::<LittleEndian>(vector.z)?;
        Ok(buffer)
    }
    
    /// Decode a Vector3
    pub fn decode_vector3(data: &[u8]) -> ProtocolResult<Vector3> {
        if data.len() < 12 {
            return Err(ProtocolError::Decoding("Vector3 requires 12 bytes".to_string()));
        }
        
        let mut cursor = Cursor::new(data);
        let x = cursor.read_f32::<LittleEndian>()?;
        let y = cursor.read_f32::<LittleEndian>()?;
        let z = cursor.read_f32::<LittleEndian>()?;
        
        Ok(Vector3::new(x, y, z))
    }
    
    /// Encode a Quaternion
    pub fn encode_quaternion(quat: &Quaternion) -> ProtocolResult<Vec<u8>> {
        let mut buffer = Vec::with_capacity(16);
        buffer.write_f32::<LittleEndian>(quat.x)?;
        buffer.write_f32::<LittleEndian>(quat.y)?;
        buffer.write_f32::<LittleEndian>(quat.z)?;
        buffer.write_f32::<LittleEndian>(quat.w)?;
        Ok(buffer)
    }
    
    /// Decode a Quaternion
    pub fn decode_quaternion(data: &[u8]) -> ProtocolResult<Quaternion> {
        if data.len() < 16 {
            return Err(ProtocolError::Decoding("Quaternion requires 16 bytes".to_string()));
        }
        
        let mut cursor = Cursor::new(data);
        let x = cursor.read_f32::<LittleEndian>()?;
        let y = cursor.read_f32::<LittleEndian>()?;
        let z = cursor.read_f32::<LittleEndian>()?;
        let w = cursor.read_f32::<LittleEndian>()?;
        
        Ok(Quaternion::new(x, y, z, w))
    }
    
    /// Encode a string
    pub fn encode_string(s: &str) -> ProtocolResult<Vec<u8>> {
        let bytes = s.as_bytes();
        let mut buffer = Vec::with_capacity(bytes.len() + 1);
        buffer.write_u8(bytes.len() as u8)?;
        buffer.extend_from_slice(bytes);
        Ok(buffer)
    }
    
    /// Decode a string
    pub fn decode_string(data: &[u8]) -> ProtocolResult<String> {
        if data.is_empty() {
            return Err(ProtocolError::Decoding("String length byte missing".to_string()));
        }
        
        let length = data[0] as usize;
        if data.len() < length + 1 {
            return Err(ProtocolError::Decoding("String data incomplete".to_string()));
        }
        
        let string_data = &data[1..length + 1];
        String::from_utf8(string_data.to_vec())
            .map_err(|e| ProtocolError::Decoding(format!("Invalid UTF-8: {}", e)))
    }
    
    /// Encode a variable length string
    pub fn encode_variable_string(s: &str) -> ProtocolResult<Vec<u8>> {
        let bytes = s.as_bytes();
        let mut buffer = Vec::with_capacity(bytes.len() + 2);
        buffer.write_u16::<LittleEndian>(bytes.len() as u16)?;
        buffer.extend_from_slice(bytes);
        Ok(buffer)
    }
    
    /// Decode a variable length string
    pub fn decode_variable_string(data: &[u8]) -> ProtocolResult<String> {
        if data.len() < 2 {
            return Err(ProtocolError::Decoding("Variable string length bytes missing".to_string()));
        }
        
        let mut cursor = Cursor::new(data);
        let length = cursor.read_u16::<LittleEndian>()? as usize;
        
        if data.len() < length + 2 {
            return Err(ProtocolError::Decoding("Variable string data incomplete".to_string()));
        }
        
        let string_data = &data[2..length + 2];
        String::from_utf8(string_data.to_vec())
            .map_err(|e| ProtocolError::Decoding(format!("Invalid UTF-8: {}", e)))
    }
    
    /// Encode binary data with length prefix
    pub fn encode_binary(data: &[u8]) -> ProtocolResult<Vec<u8>> {
        let mut buffer = Vec::with_capacity(data.len() + 4);
        buffer.write_u32::<LittleEndian>(data.len() as u32)?;
        buffer.extend_from_slice(data);
        Ok(buffer)
    }
    
    /// Decode binary data with length prefix
    pub fn decode_binary(data: &[u8]) -> ProtocolResult<Vec<u8>> {
        if data.len() < 4 {
            return Err(ProtocolError::Decoding("Binary length bytes missing".to_string()));
        }
        
        let mut cursor = Cursor::new(data);
        let length = cursor.read_u32::<LittleEndian>()? as usize;
        
        if data.len() < length + 4 {
            return Err(ProtocolError::Decoding("Binary data incomplete".to_string()));
        }
        
        Ok(data[4..length + 4].to_vec())
    }
    
    /// Calculate CRC32 checksum
    pub fn calculate_crc32(data: &[u8]) -> u32 {
        crc32fast::hash(data)
    }
    
    /// Verify CRC32 checksum
    pub fn verify_crc32(data: &[u8], expected: u32) -> bool {
        Self::calculate_crc32(data) == expected
    }
}

/// Message encoder for structured data
pub struct MessageEncoder {
    buffer: Vec<u8>,
}

impl MessageEncoder {
    /// Create a new message encoder
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
    
    /// Write a byte
    pub fn write_u8(&mut self, value: u8) -> ProtocolResult<()> {
        self.buffer.write_u8(value)?;
        Ok(())
    }
    
    /// Write a 16-bit unsigned integer
    pub fn write_u16(&mut self, value: u16) -> ProtocolResult<()> {
        self.buffer.write_u16::<LittleEndian>(value)?;
        Ok(())
    }
    
    /// Write a 32-bit unsigned integer
    pub fn write_u32(&mut self, value: u32) -> ProtocolResult<()> {
        self.buffer.write_u32::<LittleEndian>(value)?;
        Ok(())
    }
    
    /// Write a 64-bit unsigned integer
    pub fn write_u64(&mut self, value: u64) -> ProtocolResult<()> {
        self.buffer.write_u64::<LittleEndian>(value)?;
        Ok(())
    }
    
    /// Write a 32-bit float
    pub fn write_f32(&mut self, value: f32) -> ProtocolResult<()> {
        self.buffer.write_f32::<LittleEndian>(value)?;
        Ok(())
    }
    
    /// Write a 64-bit float
    pub fn write_f64(&mut self, value: f64) -> ProtocolResult<()> {
        self.buffer.write_f64::<LittleEndian>(value)?;
        Ok(())
    }
    
    /// Write a UUID
    pub fn write_uuid(&mut self, uuid: &Uuid) -> ProtocolResult<()> {
        self.buffer.extend_from_slice(uuid.as_bytes());
        Ok(())
    }
    
    /// Write a Vector3
    pub fn write_vector3(&mut self, vector: &Vector3) -> ProtocolResult<()> {
        self.write_f32(vector.x)?;
        self.write_f32(vector.y)?;
        self.write_f32(vector.z)?;
        Ok(())
    }
    
    /// Write a Quaternion
    pub fn write_quaternion(&mut self, quat: &Quaternion) -> ProtocolResult<()> {
        self.write_f32(quat.x)?;
        self.write_f32(quat.y)?;
        self.write_f32(quat.z)?;
        self.write_f32(quat.w)?;
        Ok(())
    }
    
    /// Write a string
    pub fn write_string(&mut self, s: &str) -> ProtocolResult<()> {
        let bytes = s.as_bytes();
        self.write_u8(bytes.len() as u8)?;
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }
    
    /// Write a variable length string
    pub fn write_variable_string(&mut self, s: &str) -> ProtocolResult<()> {
        let bytes = s.as_bytes();
        self.write_u16(bytes.len() as u16)?;
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }
    
    /// Write binary data
    pub fn write_binary(&mut self, data: &[u8]) -> ProtocolResult<()> {
        self.write_u32(data.len() as u32)?;
        self.buffer.extend_from_slice(data);
        Ok(())
    }
    
    /// Get the encoded data
    pub fn finish(self) -> Vec<u8> {
        self.buffer
    }
    
    /// Get the current size
    pub fn size(&self) -> usize {
        self.buffer.len()
    }
}

/// Message decoder for structured data
pub struct MessageDecoder<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> MessageDecoder<'a> {
    /// Create a new message decoder
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }
    
    /// Read a byte
    pub fn read_u8(&mut self) -> ProtocolResult<u8> {
        Ok(self.cursor.read_u8()?)
    }
    
    /// Read a 16-bit unsigned integer
    pub fn read_u16(&mut self) -> ProtocolResult<u16> {
        Ok(self.cursor.read_u16::<LittleEndian>()?)
    }
    
    /// Read a 32-bit unsigned integer
    pub fn read_u32(&mut self) -> ProtocolResult<u32> {
        Ok(self.cursor.read_u32::<LittleEndian>()?)
    }
    
    /// Read a 64-bit unsigned integer
    pub fn read_u64(&mut self) -> ProtocolResult<u64> {
        Ok(self.cursor.read_u64::<LittleEndian>()?)
    }
    
    /// Read a 32-bit float
    pub fn read_f32(&mut self) -> ProtocolResult<f32> {
        Ok(self.cursor.read_f32::<LittleEndian>()?)
    }
    
    /// Read a 64-bit float
    pub fn read_f64(&mut self) -> ProtocolResult<f64> {
        Ok(self.cursor.read_f64::<LittleEndian>()?)
    }
    
    /// Read a UUID
    pub fn read_uuid(&mut self) -> ProtocolResult<Uuid> {
        let mut bytes = [0u8; 16];
        self.cursor.read_exact(&mut bytes)?;
        Ok(Uuid::from_bytes(bytes))
    }
    
    /// Read a Vector3
    pub fn read_vector3(&mut self) -> ProtocolResult<Vector3> {
        let x = self.read_f32()?;
        let y = self.read_f32()?;
        let z = self.read_f32()?;
        Ok(Vector3::new(x, y, z))
    }
    
    /// Read a Quaternion
    pub fn read_quaternion(&mut self) -> ProtocolResult<Quaternion> {
        let x = self.read_f32()?;
        let y = self.read_f32()?;
        let z = self.read_f32()?;
        let w = self.read_f32()?;
        Ok(Quaternion::new(x, y, z, w))
    }
    
    /// Read a string
    pub fn read_string(&mut self) -> ProtocolResult<String> {
        let length = self.read_u8()? as usize;
        let mut bytes = vec![0u8; length];
        self.cursor.read_exact(&mut bytes)?;
        String::from_utf8(bytes)
            .map_err(|e| ProtocolError::Decoding(format!("Invalid UTF-8: {}", e)))
    }
    
    /// Read a variable length string
    pub fn read_variable_string(&mut self) -> ProtocolResult<String> {
        let length = self.read_u16()? as usize;
        let mut bytes = vec![0u8; length];
        self.cursor.read_exact(&mut bytes)?;
        String::from_utf8(bytes)
            .map_err(|e| ProtocolError::Decoding(format!("Invalid UTF-8: {}", e)))
    }
    
    /// Read binary data
    pub fn read_binary(&mut self) -> ProtocolResult<Vec<u8>> {
        let length = self.read_u32()? as usize;
        let mut data = vec![0u8; length];
        self.cursor.read_exact(&mut data)?;
        Ok(data)
    }
    
    /// Get current position
    pub fn position(&self) -> u64 {
        self.cursor.position()
    }
    
    /// Get remaining bytes
    pub fn remaining(&self) -> usize {
        let total = self.cursor.get_ref().len() as u64;
        let pos = self.cursor.position();
        (total - pos) as usize
    }
    
    /// Check if there are more bytes to read
    pub fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }
}

impl Default for MessageEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector3_encoding() {
        let vector = Vector3::new(1.0, 2.0, 3.0);
        let encoded = ProtocolCodec::encode_vector3(&vector).unwrap();
        let decoded = ProtocolCodec::decode_vector3(&encoded).unwrap();
        
        assert_eq!(vector.x, decoded.x);
        assert_eq!(vector.y, decoded.y);
        assert_eq!(vector.z, decoded.z);
    }
    
    #[test]
    fn test_quaternion_encoding() {
        let quat = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        let encoded = ProtocolCodec::encode_quaternion(&quat).unwrap();
        let decoded = ProtocolCodec::decode_quaternion(&encoded).unwrap();
        
        assert_eq!(quat.x, decoded.x);
        assert_eq!(quat.y, decoded.y);
        assert_eq!(quat.z, decoded.z);
        assert_eq!(quat.w, decoded.w);
    }
    
    #[test]
    fn test_string_encoding() {
        let test_string = "Hello, World!";
        let encoded = ProtocolCodec::encode_string(test_string).unwrap();
        let decoded = ProtocolCodec::decode_string(&encoded).unwrap();
        
        assert_eq!(test_string, decoded);
    }
    
    #[test]
    fn test_message_encoder_decoder() {
        let mut encoder = MessageEncoder::new();
        encoder.write_u32(42).unwrap();
        encoder.write_string("test").unwrap();
        encoder.write_f32(3.14).unwrap();
        
        let data = encoder.finish();
        let mut decoder = MessageDecoder::new(&data);
        
        assert_eq!(decoder.read_u32().unwrap(), 42);
        assert_eq!(decoder.read_string().unwrap(), "test");
        assert!((decoder.read_f32().unwrap() - 3.14).abs() < 0.001);
    }
}