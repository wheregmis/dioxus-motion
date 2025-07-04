//! Transform module for 2D transformations
//!
//! Provides a Transform type that can be animated, supporting:
//! - Translation (x, y)
//! - Scale
//! - Rotation
//!
//! Uses radians for rotation and supports smooth interpolation.

use crate::Animatable;
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
    /// Creates a zero transform (all components 0)
    fn zero() -> Self {
        Transform::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Minimum meaningful difference between transforms
    fn epsilon() -> f32 {
        0.01
    }

    /// Calculates the magnitude of the transform
    fn magnitude(&self) -> f32 {
        (self.x * self.x
            + self.y * self.y
            + self.scale * self.scale
            + self.rotation * self.rotation)
            .sqrt()
    }

    /// Scales all transform components by a factor
    fn scale(&self, factor: f32) -> Self {
        Transform::new(
            self.x * factor,
            self.y * factor,
            self.scale * factor,
            self.rotation * factor,
        )
    }

    /// Adds two transforms component-wise
    fn add(&self, other: &Self) -> Self {
        Transform::new(
            self.x + other.x,
            self.y + other.y,
            self.scale + other.scale,
            self.rotation + other.rotation,
        )
    }

    /// Subtracts two transforms component-wise
    fn sub(&self, other: &Self) -> Self {
        Transform::new(
            self.x - other.x,
            self.y - other.y,
            self.scale - other.scale,
            self.rotation - other.rotation,
        )
    }

    /// Interpolates between two transforms
    /// Handles rotation specially to ensure shortest path
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        // SIMD for x, y, scale; handle rotation separately for shortest path
        let a = [self.x, self.y, self.scale, 0.0];
        let b = [target.x, target.y, target.scale, 0.0];
        let va = f32x4::new(a);
        let vb = f32x4::new(b);
        let vt = f32x4::splat(t.clamp(0.0, 1.0));
        let result = va + (vb - va) * vt;
        let out = result.to_array();

        // Rotation: shortest path
        let mut rotation_diff = target.rotation - self.rotation;
        if rotation_diff > std::f32::consts::PI {
            rotation_diff -= 2.0 * std::f32::consts::PI;
        } else if rotation_diff < -std::f32::consts::PI {
            rotation_diff += 2.0 * std::f32::consts::PI;
        }
        let rotation = self.rotation + rotation_diff * t;

        Transform::new(out[0], out[1], out[2], rotation)
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
