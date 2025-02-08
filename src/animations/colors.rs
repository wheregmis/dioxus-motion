//! Color module for animation support
//!
//! Provides RGBA color representation and animation interpolation.
//! Supports both normalized (0.0-1.0) and byte (0-255) color values.

use crate::animations::utils::Animatable;

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
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }
}

/// Implementation of animation interpolation for Color
impl Animatable for Color {
    /// Creates a fully transparent black color
    fn zero() -> Self {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Minimum difference between color components
    fn epsilon() -> f32 {
        0.00001 // Increased precision for smoother transitions
    }

    /// Calculates color vector magnitude
    fn magnitude(&self) -> f32 {
        // Weighted magnitude calculation for better precision
        let r_diff = self.r;
        let g_diff = self.g;
        let b_diff = self.b;
        let a_diff = self.a;

        (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff + a_diff * a_diff).sqrt()
    }

    /// Scales color components by a factor
    fn scale(&self, factor: f32) -> Self {
        Color::new(
            self.r * factor,
            self.g * factor,
            self.b * factor,
            self.a * factor,
        )
    }

    /// Adds two colors component-wise
    fn add(&self, other: &Self) -> Self {
        Color::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
            self.a + other.a,
        )
    }

    /// Subtracts two colors component-wise
    fn sub(&self, other: &Self) -> Self {
        Color::new(
            self.r - other.r,
            self.g - other.g,
            self.b - other.b,
            self.a - other.a,
        )
    }

    /// Linearly interpolates between two colors
    ///
    /// # Parameters
    /// * `target` - Target color to interpolate towards
    /// * `t` - Interpolation factor (0.0-1.0)
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);

        // Direct linear interpolation that works for both increasing and decreasing values
        let r = self.r * (1.0 - t) + target.r * t;
        let g = self.g * (1.0 - t) + target.g * t;
        let b = self.b * (1.0 - t) + target.b * t;
        let a = self.a * (1.0 - t) + target.a * t;

        Color::new(r, g, b, a)
    }
}
