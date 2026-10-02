//! Transform module for 2D transformations
//!
//! Provides a Transform type that can be animated, supporting:
//! - Translation (x, y)
//! - Scale
//! - Rotation
//!
//! Uses radians for rotation and supports smooth interpolation.

use crate::animations::core::Animatable;
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

impl Default for Transform {
    fn default() -> Self {
        Transform::identity()
    }
}

impl std::ops::Add for Transform {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Transform::new(
            self.x + other.x,
            self.y + other.y,
            self.scale + other.scale,
            self.rotation + other.rotation,
        )
    }
}

impl std::ops::Sub for Transform {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Transform::new(
            self.x - other.x,
            self.y - other.y,
            self.scale - other.scale,
            self.rotation - other.rotation,
        )
    }
}

impl std::ops::Mul<f32> for Transform {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Transform::new(
            self.x * factor,
            self.y * factor,
            self.scale * factor,
            self.rotation * factor,
        )
    }
}

/// Implementation of Animatable for f32 primitive type
/// Much simpler with the new trait design - leverages standard Rust operators
impl Animatable for f32 {
    fn is_finite(&self) -> bool {
        f32::is_finite(*self)
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        if t == 0.0 {
            return *self;
        }
        if t == 1.0 {
            return *target;
        }
        let delta = target - self;
        if delta.is_finite() {
            self + delta * t
        } else {
            // Finite endpoints can have a difference outside the f32 range.
            (f64::from(*self) + (f64::from(*target) - f64::from(*self)) * f64::from(t)) as f32
        }
    }

    fn magnitude(&self) -> f32 {
        self.abs()
    }

    // Uses default epsilon of 0.01 from the trait
}

/// Implementation of Animatable for Transform
/// Much simpler with the new trait design - uses standard operators
impl Animatable for Transform {
    fn is_finite(&self) -> bool {
        [self.x, self.y, self.scale, self.rotation]
            .into_iter()
            .all(f32::is_finite)
    }

    // Signed remainder preserves the direction of exact +/-PI ties.
    #[allow(clippy::modulo_arithmetic)]
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        // SIMD for x, y, scale; handle rotation separately for shortest path
        let a = [self.x, self.y, self.scale, 0.0];
        let b = [target.x, target.y, target.scale, 0.0];
        let va = f32x4::new(a);
        let vb = f32x4::new(b);
        let progress = t.clamp(0.0, 1.0);
        let delta = vb - va;
        let out = if progress == 0.0 {
            a
        } else if progress == 1.0 {
            b
        } else if delta.is_finite().all() {
            (va + delta * f32x4::splat(progress)).to_array()
        } else {
            [
                self.x.interpolate(&target.x, progress),
                self.y.interpolate(&target.y, progress),
                self.scale.interpolate(&target.scale, progress),
                0.0,
            ]
        };

        // Rotation: shortest path
        let mut rotation_diff = target.rotation - self.rotation;
        let tau = 2.0 * std::f32::consts::PI;
        if !rotation_diff.is_finite() {
            rotation_diff =
                ((f64::from(target.rotation) - f64::from(self.rotation)) % f64::from(tau)) as f32;
        } else if rotation_diff.abs() > tau {
            rotation_diff %= tau;
        }
        if rotation_diff > std::f32::consts::PI {
            rotation_diff -= tau;
        } else if rotation_diff < -std::f32::consts::PI {
            rotation_diff += tau;
        }
        let rotation = self.rotation + rotation_diff * t;

        Transform::new(out[0], out[1], out[2], rotation)
    }

    fn magnitude(&self) -> f32 {
        crate::animations::core::magnitude([self.x, self.y, self.scale, self.rotation].into_iter())
    }

    // Uses default epsilon of 0.01 from the trait - no need for TRANSFORM_EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn interpolation_preserves_finite_extreme_endpoints() {
        let mut bits = 1u32;
        for _ in 0..4096 {
            bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
            let start = f32::from_bits(bits);
            bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
            let end = f32::from_bits(bits);
            if start.is_finite() && end.is_finite() {
                for progress in [0.0, 0.001, 0.1, 0.5, 0.9, 0.999, 1.0] {
                    assert!(start.interpolate(&end, progress).is_finite());
                    let transform = Transform::new(start, start, start, 0.0)
                        .interpolate(&Transform::new(end, end, end, 0.0), progress);
                    assert!(transform.x.is_finite());
                    assert!(transform.y.is_finite());
                    assert!(transform.scale.is_finite());
                }
            }
        }
        for (start, end) in [(f32::MAX, 1.0f32), (-f32::MAX, 1.0), (1.0, f32::MAX)] {
            assert_eq!(start.interpolate(&end, 0.0), start);
            assert_eq!(start.interpolate(&end, 1.0), end);
            let initial = Transform::new(start, start, start, 0.0);
            let target = Transform::new(end, end, end, 0.0);
            assert_eq!(initial.interpolate(&target, 0.0), initial);
            assert_eq!(initial.interpolate(&target, 1.0), target);
        }
        for (start, end) in [(-f32::MAX, f32::MAX), (f32::MAX, -f32::MAX)] {
            for progress in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let expected = (f64::from(start)
                    + (f64::from(end) - f64::from(start)) * f64::from(progress))
                    as f32;
                assert_eq!(start.interpolate(&end, progress), expected);
                let transform = Transform::new(start, start, start, 0.0)
                    .interpolate(&Transform::new(end, end, end, 0.0), progress);
                assert_eq!(transform, Transform::new(expected, expected, expected, 0.0));
            }
        }
    }

    #[test]
    #[allow(clippy::modulo_arithmetic)]
    fn rotation_uses_shortest_path_across_multiple_turns() {
        let tau = 2.0 * PI;
        for start in [0.0, 0.5, -0.5] {
            for target in [
                PI,
                -PI,
                PI + 0.25,
                -PI - 0.25,
                tau * 10.0 + 1.0,
                -tau * 10.0 - 1.0,
            ] {
                let mut difference = (target - start) % tau;
                if difference > PI {
                    difference -= tau;
                } else if difference < -PI {
                    difference += tau;
                }
                for progress in [0.0, 0.25, 0.5, 0.75, 1.0] {
                    let actual = Transform::new(0.0, 0.0, 1.0, start)
                        .interpolate(&Transform::new(0.0, 0.0, 1.0, target), progress)
                        .rotation;
                    assert!(
                        (actual - (start + difference * progress)).abs() < 0.00001,
                        "start={start}, target={target}, progress={progress}, actual={actual}"
                    );
                }
            }
        }
        for start in [-f32::MAX, f32::MAX] {
            for progress in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let actual = Transform::new(0.0, 0.0, 1.0, start)
                    .interpolate(&Transform::new(0.0, 0.0, 1.0, -start), progress)
                    .rotation;
                // A bounded shortest-path displacement is below this angle's precision.
                assert_eq!(actual, start);
            }
        }
    }

    #[test]
    fn transform_arithmetic_updates_every_component() {
        let a = Transform::new(3.0, -3.0, 4.0, -5.0);
        let b = Transform::new(-7.0, 11.0, -13.0, 17.0);
        assert_eq!(a + b, Transform::new(-4.0, 8.0, -9.0, 12.0));
        assert_eq!(a - b, Transform::new(10.0, -14.0, 17.0, -22.0));
        assert_eq!(a * 3.0, Transform::new(9.0, -9.0, 12.0, -15.0));
        assert_eq!(a.magnitude(), 59.0f32.sqrt());
    }

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
