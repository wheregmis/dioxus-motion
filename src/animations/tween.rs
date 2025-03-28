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
/// use dioxus_motion::Duration;
/// use dioxus_motion::prelude::Tween;
/// use easer::functions::Easing;
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

#[cfg(test)]
mod tests {
    use super::*;
    use easer::functions::{Cubic, Easing};

    #[test]
    fn test_tween_new() {
        let tween = Tween {
            duration: Duration::from_secs(1),
            easing: Cubic::ease_in_out,
        };

        assert_eq!(tween.duration, Duration::from_secs(1));
    }

    #[test]
    fn test_tween_interpolation() {
        let tween = Tween {
            duration: Duration::from_secs(1),
            easing: Linear::ease_in_out,
        };

        // Test midpoint
        let progress = 0.5;
        let result = (tween.easing)(progress, 0.0, 1.0, 1.0);
        assert!((result - 0.5).abs() < f32::EPSILON);

        // Test start
        let result = (tween.easing)(0.0, 0.0, 1.0, 1.0);
        assert!((result - 0.0).abs() < f32::EPSILON);

        // Test end
        let result = (tween.easing)(1.0, 0.0, 1.0, 1.0);
        assert!((result - 1.0).abs() < f32::EPSILON);
    }
}
