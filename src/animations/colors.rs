//! Color module for animation support
//!
//! Provides RGBA color representation and animation interpolation.
//! Supports both normalized (0.0-1.0) and byte (0-255) color values.

use crate::animations::core::Animatable;
use wide::f32x4;

/// Represents an RGBA color with normalized components
///
/// Each component (r,g,b,a) is stored as a float between 0.0 and 1.0
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    /// Red component (0.0-1.0)
    pub r: f32,
    /// Green component (0.0-1.0)
    pub g: f32,
    /// Blue component (0.0-1.0)
    pub b: f32,
    /// Alpha component (0.0-1.0)
    pub a: f32,
}

impl Color {
    /// Creates a new color with normalized components
    ///
    /// # Examples
    /// ```
    /// use dioxus_motion::prelude::Color;
    /// let color = Color::new(1.0, 0.5, 0.0, 1.0); // Orange color
    /// ```
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Creates a color from 8-bit RGBA values
    ///
    /// # Examples
    /// ```
    /// use dioxus_motion::prelude::Color;
    /// let color = Color::from_rgba(255, 128, 0, 255); // Orange color
    /// ```
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Converts color to 8-bit RGBA values
    ///
    /// # Returns
    /// Tuple of (r,g,b,a) with values from 0-255
    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0 + 0.5) as u8,
            (self.g * 255.0 + 0.5) as u8,
            (self.b * 255.0 + 0.5) as u8,
            (self.a * 255.0 + 0.5) as u8,
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0.0, 0.0, 0.0, 1.0) // Black with full opacity
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color::new(
            (self.r + other.r).clamp(0.0, 1.0),
            (self.g + other.g).clamp(0.0, 1.0),
            (self.b + other.b).clamp(0.0, 1.0),
            (self.a + other.a).clamp(0.0, 1.0),
        )
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color::new(
            (self.r - other.r).clamp(0.0, 1.0),
            (self.g - other.g).clamp(0.0, 1.0),
            (self.b - other.b).clamp(0.0, 1.0),
            (self.a - other.a).clamp(0.0, 1.0),
        )
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Color::new(
            (self.r * factor).clamp(0.0, 1.0),
            (self.g * factor).clamp(0.0, 1.0),
            (self.b * factor).clamp(0.0, 1.0),
            (self.a * factor).clamp(0.0, 1.0),
        )
    }
}

/// Implementation of Animatable for Color
/// Much simpler with the new trait design - uses standard operators
impl Animatable for Color {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let a = [self.r, self.g, self.b, self.a];
        let b = [target.r, target.g, target.b, target.a];
        let va = f32x4::new(a);
        let vb = f32x4::new(b);
        let vt = f32x4::splat(t.clamp(0.0, 1.0));
        let result = va + (vb - va) * vt;
        let out = result.to_array();
        Color::new(out[0], out[1], out[2], out[3])
    }

    fn magnitude(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b + self.a * self.a).sqrt()
    }

    fn epsilon() -> f32 {
        0.005 // Smaller epsilon for Color since components are in 0-1 range
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(1.0, 0.5, 0.0, 1.0);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.5);
        assert_eq!(color.b, 0.0);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_color_from_rgba() {
        let color = Color::from_rgba(255, 128, 0, 255);
        assert!((color.r - 1.0).abs() < f32::EPSILON);
        assert!((color.g - 0.5019608).abs() < 0.000001);
        assert!((color.b - 0.0).abs() < f32::EPSILON);
        assert!((color.a - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_color_lerp() {
        let start = Color::new(0.0, 0.0, 0.0, 1.0);
        let end = Color::new(1.0, 1.0, 1.0, 1.0);
        let mid = start.interpolate(&end, 0.5);

        assert!((mid.r - 0.5).abs() < f32::EPSILON);
        assert!((mid.g - 0.5).abs() < f32::EPSILON);
        assert!((mid.b - 0.5).abs() < f32::EPSILON);
        assert!((mid.a - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_color_to_rgba() {
        let color = Color::new(1.0, 0.5, 0.0, 1.0);
        let (r, g, b, a) = color.to_rgba();
        assert_eq!(r, 255);
        assert_eq!(g, 128);
        assert_eq!(b, 0);
        assert_eq!(a, 255);
    }
}
