//! Mathematical utilities and constants for Mutsea

use crate::{Vector3, Quaternion};
use serde::{Deserialize, Serialize};

/// Mathematical constants
pub mod constants {
    pub const PI: f32 = std::f32::consts::PI;
    pub const TWO_PI: f32 = 2.0 * PI;
    pub const HALF_PI: f32 = PI * 0.5;
    pub const DEG_TO_RAD: f32 = PI / 180.0;
    pub const RAD_TO_DEG: f32 = 180.0 / PI;
    pub const EPSILON: f32 = 1e-6;
}

/// 4x4 transformation matrix
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    
    /// Create a translation matrix
    pub fn translation(translation: Vector3) -> Self {
        Matrix4 {
            m: [
                [1.0, 0.0, 0.0, translation.x],
                [0.0, 1.0, 0.0, translation.y],
                [0.0, 0.0, 1.0, translation.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a rotation matrix from quaternion
    pub fn rotation(rotation: Quaternion) -> Self {
        let q = rotation.normalize();
        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;
        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;
        
        Matrix4 {
            m: [
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy), 0.0],
                [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx), 0.0],
                [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a scale matrix
    pub fn scale(scale: Vector3) -> Self {
        Matrix4 {
            m: [
                [scale.x, 0.0, 0.0, 0.0],
                [0.0, scale.y, 0.0, 0.0],
                [0.0, 0.0, scale.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    
    /// Create a transformation matrix from translation, rotation, and scale
    pub fn transform(translation: Vector3, rotation: Quaternion, scale: Vector3) -> Self {
        let t = Self::translation(translation);
        let r = Self::rotation(rotation);
        let s = Self::scale(scale);
        
        t * r * s
    }
    
    /// Matrix multiplication
    pub fn multiply(&self, other: &Matrix4) -> Matrix4 {
        let mut result = Matrix4::IDENTITY;
        
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = 0.0;
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        
        result
    }
    
    /// Transform a point by this matrix
    pub fn transform_point(&self, point: Vector3) -> Vector3 {
        Vector3 {
            x: self.m[0][0] * point.x + self.m[0][1] * point.y + self.m[0][2] * point.z + self.m[0][3],
            y: self.m[1][0] * point.x + self.m[1][1] * point.y + self.m[1][2] * point.z + self.m[1][3],
            z: self.m[2][0] * point.x + self.m[2][1] * point.y + self.m[2][2] * point.z + self.m[2][3],
        }
    }
    
    /// Transform a direction by this matrix (ignores translation)
    pub fn transform_direction(&self, direction: Vector3) -> Vector3 {
        Vector3 {
            x: self.m[0][0] * direction.x + self.m[0][1] * direction.y + self.m[0][2] * direction.z,
            y: self.m[1][0] * direction.x + self.m[1][1] * direction.y + self.m[1][2] * direction.z,
            z: self.m[2][0] * direction.x + self.m[2][1] * direction.y + self.m[2][2] * direction.z,
        }
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Matrix4;
    
    fn mul(self, other: Matrix4) -> Matrix4 {
        self.multiply(&other)
    }
}

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

impl BoundingBox {
    /// Create a new bounding box
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }
    
    /// Create a bounding box from center and size
    pub fn from_center_size(center: Vector3, size: Vector3) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }
    
    /// Get the center of the bounding box
    pub fn center(&self) -> Vector3 {
        (self.min + self.max) * 0.5
    }
    
    /// Get the size of the bounding box
    pub fn size(&self) -> Vector3 {
        self.max - self.min
    }
    
    /// Check if a point is inside the bounding box
    pub fn contains_point(&self, point: Vector3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
    
    /// Check if this bounding box intersects with another
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.max.x >= other.min.x && self.min.x <= other.max.x &&
        self.max.y >= other.min.y && self.min.y <= other.max.y &&
        self.max.z >= other.min.z && self.min.z <= other.max.z
    }
    
    /// Expand the bounding box to include a point
    pub fn expand_to_include(&mut self, point: Vector3) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);
    }
    
    /// Expand the bounding box to include another bounding box
    pub fn expand_to_include_box(&mut self, other: &BoundingBox) {
        self.expand_to_include(other.min);
        self.expand_to_include(other.max);
    }
}

/// Ray for collision detection and picking
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    /// Create a new ray
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }
    
    /// Get a point on the ray at parameter t
    pub fn point_at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
    
    /// Test intersection with a bounding box
    pub fn intersects_box(&self, bbox: &BoundingBox) -> Option<f32> {
        let inv_dir = Vector3 {
            x: if self.direction.x.abs() < constants::EPSILON { f32::INFINITY } else { 1.0 / self.direction.x },
            y: if self.direction.y.abs() < constants::EPSILON { f32::INFINITY } else { 1.0 / self.direction.y },
            z: if self.direction.z.abs() < constants::EPSILON { f32::INFINITY } else { 1.0 / self.direction.z },
        };
        
        let t1 = (bbox.min.x - self.origin.x) * inv_dir.x;
        let t2 = (bbox.max.x - self.origin.x) * inv_dir.x;
        let t3 = (bbox.min.y - self.origin.y) * inv_dir.y;
        let t4 = (bbox.max.y - self.origin.y) * inv_dir.y;
        let t5 = (bbox.min.z - self.origin.z) * inv_dir.z;
        let t6 = (bbox.max.z - self.origin.z) * inv_dir.z;
        
        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));
        
        if tmax < 0.0 || tmin > tmax {
            None
        } else {
            Some(if tmin < 0.0 { tmax } else { tmin })
        }
    }
}

/// Utility functions
pub mod utils {
    use super::*;
    
    /// Linear interpolation between two values
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
    
    /// Linear interpolation between two vectors
    pub fn lerp_vector3(a: Vector3, b: Vector3, t: f32) -> Vector3 {
        Vector3 {
            x: lerp(a.x, b.x, t),
            y: lerp(a.y, b.y, t),
            z: lerp(a.z, b.z, t),
        }
    }
    
    /// Spherical linear interpolation between two quaternions
    pub fn slerp_quaternion(a: Quaternion, b: Quaternion, t: f32) -> Quaternion {
        let dot = a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
        
        let b = if dot < 0.0 {
            Quaternion { x: -b.x, y: -b.y, z: -b.z, w: -b.w }
        } else {
            b
        };
        
        let dot = dot.abs();
        
        if dot > 0.9995 {
            // Linear interpolation for very close quaternions
            Quaternion {
                x: lerp(a.x, b.x, t),
                y: lerp(a.y, b.y, t),
                z: lerp(a.z, b.z, t),
                w: lerp(a.w, b.w, t),
            }.normalize()
        } else {
            let theta_0 = dot.acos();
            let theta = theta_0 * t;
            let sin_theta = theta.sin();
            let sin_theta_0 = theta_0.sin();
            
            let s0 = (theta_0 - theta).cos() / sin_theta_0;
            let s1 = sin_theta / sin_theta_0;
            
            Quaternion {
                x: a.x * s0 + b.x * s1,
                y: a.y * s0 + b.y * s1,
                z: a.z * s0 + b.z * s1,
                w: a.w * s0 + b.w * s1,
            }
        }
    }
    
    /// Clamp a value between min and max
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    
    /// Check if two floating point values are approximately equal
    pub fn approximately_equal(a: f32, b: f32) -> bool {
        (a - b).abs() < constants::EPSILON
    }
    
    /// Convert degrees to radians
    pub fn deg_to_rad(degrees: f32) -> f32 {
        degrees * constants::DEG_TO_RAD
    }
    
    /// Convert radians to degrees
    pub fn rad_to_deg(radians: f32) -> f32 {
        radians * constants::RAD_TO_DEG
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector3_operations() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        
        let sum = a + b;
        assert_eq!(sum, Vector3::new(5.0, 7.0, 9.0));
        
        let diff = b - a;
        assert_eq!(diff, Vector3::new(3.0, 3.0, 3.0));
        
        let scaled = a * 2.0;
        assert_eq!(scaled, Vector3::new(2.0, 4.0, 6.0));
        
        let dot = a.dot(&b);
        assert_eq!(dot, 32.0);
    }
    
    #[test]
    fn test_quaternion_normalize() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let normalized = q.normalize();
        
        let length_squared = normalized.x * normalized.x + 
                           normalized.y * normalized.y + 
                           normalized.z * normalized.z + 
                           normalized.w * normalized.w;
        
        assert!((length_squared - 1.0).abs() < constants::EPSILON);
    }
    
    #[test]
    fn test_bounding_box_contains() {
        let bbox = BoundingBox::new(
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(1.0, 1.0, 1.0)
        );
        
        assert!(bbox.contains_point(Vector3::ZERO));
        assert!(bbox.contains_point(Vector3::new(0.5, 0.5, 0.5)));
        assert!(!bbox.contains_point(Vector3::new(2.0, 0.0, 0.0)));
    }
    
    #[test]
    fn test_matrix4_transform() {
        let translation = Vector3::new(1.0, 2.0, 3.0);
        let matrix = Matrix4::translation(translation);
        
        let point = Vector3::ZERO;
        let transformed = matrix.transform_point(point);
        
        assert_eq!(transformed, translation);
    }
}