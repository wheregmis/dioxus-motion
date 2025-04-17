//! Transform module for 2D transformations
//!
//! Provides a Transform type that can be animated, supporting:
//! - Translation (x, y)
//! - Scale
//! - Rotation
//!
//! Uses radians for rotation and supports smooth interpolation.
//! Includes SIMD optimizations when the "simd" feature is enabled.

use crate::Animatable;
use crate::animations::utils::simd;
use wide::f32x4;

/// Represents a 2D transformation with translation, scale, and rotation
///
/// # Examples
/// ```rust
/// use dioxus_motion::prelude::Transform;
/// use std::f32::consts::PI;
/// let transform = Transform::new(100.0, 50.0, 1.5, PI/4.0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    /// X translation component
    pub x: f32,
    /// Y translation component
    pub y: f32,
    /// Uniform scale factor
    pub scale: f32,
    /// Rotation in radians
    pub rotation: f32,
}

impl Transform {
    /// Creates a new transform with specified parameters
    pub fn new(x: f32, y: f32, scale: f32, rotation: f32) -> Self {
        Self {
            x,
            y,
            scale,
            rotation,
        }
    }

    /// Creates an identity transform (no transformation)
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            rotation: 0.0,
        }
    }
}

/// Implementation of Animatable for f32 primitive type
/// Enables direct animation of float values
impl Animatable for f32 {
    fn zero() -> Self {
        0.0
    }

    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        self.abs()
    }

    fn scale(&self, factor: f32) -> Self {
        self * factor
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        self + (target - self) * t
    }
}

/// Implementation of Animatable for Transform
/// Provides smooth interpolation between transform states
impl Animatable for Transform {
    /// Returns self as a Transform reference
    fn as_transform(&self) -> Option<&Transform> {
        Some(self)
    }

    /// Creates a new Transform from a Transform reference
    fn from_transform(transform: &Transform) -> Self {
        *transform
    }
    /// Creates a zero transform (all components 0)
    fn zero() -> Self {
        Transform::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Minimum meaningful difference between transforms
    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        // Pack the transform components into a SIMD vector
        let v = f32x4::from([self.x, self.y, self.scale, self.rotation]);
        simd::magnitude_f32x4(v)
    }

    fn scale(&self, factor: f32) -> Self {
        // Pack the transform components into a SIMD vector
        let v = f32x4::from([self.x, self.y, self.scale, self.rotation]);
        let result = v * f32x4::splat(factor);
        let result_array = result.to_array();
        Transform::new(
            result_array[0],
            result_array[1],
            result_array[2],
            result_array[3],
        )
    }

    fn add(&self, other: &Self) -> Self {
        // Pack the transform components into SIMD vectors
        let v1 = f32x4::from([self.x, self.y, self.scale, self.rotation]);
        let v2 = f32x4::from([other.x, other.y, other.scale, other.rotation]);
        let result = v1 + v2;
        let result_array = result.to_array();
        Transform::new(
            result_array[0],
            result_array[1],
            result_array[2],
            result_array[3],
        )
    }

    fn sub(&self, other: &Self) -> Self {
        // Pack the transform components into SIMD vectors
        let v1 = f32x4::from([self.x, self.y, self.scale, self.rotation]);
        let v2 = f32x4::from([other.x, other.y, other.scale, other.rotation]);
        let result = v1 - v2;
        let result_array = result.to_array();
        Transform::new(
            result_array[0],
            result_array[1],
            result_array[2],
            result_array[3],
        )
    }

    /// Interpolates between two transforms
    /// Handles rotation specially to ensure shortest path
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        // Special handling for rotation to ensure shortest path
        let mut rotation_diff = target.rotation - self.rotation;
        if rotation_diff > std::f32::consts::PI {
            rotation_diff -= 2.0 * std::f32::consts::PI;
        } else if rotation_diff < -std::f32::consts::PI {
            rotation_diff += 2.0 * std::f32::consts::PI;
        }

        // Pack the transform components into SIMD vectors
        // We handle rotation separately due to the special path calculation
        let v1 = f32x4::from([self.x, self.y, self.scale, self.rotation]);
        let v2 = f32x4::from([
            target.x,
            target.y,
            target.scale,
            self.rotation + rotation_diff,
        ]);

        // Use SIMD lerp function
        let result = simd::lerp_f32x4(v1, v2, t);
        let result_array = result.to_array();

        Transform::new(
            result_array[0],
            result_array[1],
            result_array[2],
            result_array[3],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_transform_new() {
        let transform = Transform::new(100.0, 50.0, 1.5, PI / 4.0);
        assert_eq!(transform.x, 100.0);
        assert_eq!(transform.y, 50.0);
        assert_eq!(transform.scale, 1.5);
        assert!((transform.rotation - PI / 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_transform_default() {
        let transform = Transform::identity();
        assert_eq!(transform.x, 0.0);
        assert_eq!(transform.y, 0.0);
        assert_eq!(transform.scale, 1.0);
        assert_eq!(transform.rotation, 0.0);
    }

    #[test]
    fn test_transform_lerp() {
        let start = Transform::new(0.0, 0.0, 1.0, 0.0);
        let end = Transform::new(100.0, 100.0, 2.0, PI);
        let mid = start.interpolate(&end, 0.5);

        assert_eq!(mid.x, 50.0);
        assert_eq!(mid.y, 50.0);
        assert_eq!(mid.scale, 1.5);
        assert!((mid.rotation - PI / 2.0).abs() < f32::EPSILON);
    }
}
