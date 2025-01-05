//! Tween animation module
//!
//! Provides time-based animation with customizable easing functions.
//! Supports duration and interpolation control for smooth animations.

use easer::functions::{Easing, Linear};
pub use instant::Duration;

/// Configuration for tween-based animations
///
/// # Examples
/// ```rust
/// let tween = Tween::new(Duration::from_secs(1))
///     .with_easing(easer::functions::Cubic::ease_in_out);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tween {
    /// Duration of the animation
    pub duration: Duration,
    /// Easing function for interpolation
    pub easing: fn(f32, f32, f32, f32) -> f32,
}

/// Default tween configuration with 300ms duration and linear easing
impl Default for Tween {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            easing: Linear::ease_in_out,
        }
    }
}

impl Tween {
    /// Creates a new tween with specified duration and linear easing
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            easing: Linear::ease_in_out,
        }
    }

    /// Sets the easing function for the animation
    ///
    /// # Arguments
    /// * `easing` - Function that takes (t, b, c, d) and returns interpolated value
    pub fn with_easing(mut self, easing: fn(f32, f32, f32, f32) -> f32) -> Self {
        self.easing = easing;
        self
    }
}
