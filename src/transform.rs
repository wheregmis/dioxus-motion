//! Transform module for 2D transformations
//!
//! Provides a Transform type that can be animated, supporting:
//! - Translation (x, y)
//! - Scale
//! - Rotation
//!
//! Uses radians for rotation and supports smooth interpolation.

use crate::Animatable;

/// Represents a 2D transformation with translation, scale, and rotation
///
/// # Examples
/// ```rust
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
        0.001
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
        // Special handling for rotation to ensure shortest path
        let mut rotation_diff = target.rotation - self.rotation;
        if rotation_diff > std::f32::consts::PI {
            rotation_diff -= 2.0 * std::f32::consts::PI;
        } else if rotation_diff < -std::f32::consts::PI {
            rotation_diff += 2.0 * std::f32::consts::PI;
        }

        Transform::new(
            self.x + (target.x - self.x) * t,
            self.y + (target.y - self.y) * t,
            self.scale + (target.scale - self.scale) * t,
            self.rotation + rotation_diff * t,
        )
    }
}
