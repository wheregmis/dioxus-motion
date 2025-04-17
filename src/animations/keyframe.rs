use std::cell::RefCell;

use super::utils::Animatable;
use easer::functions::Easing;
pub use instant::Duration;

// Lazily initialized buffer for keyframes to avoid allocations
thread_local! {
    static KEYFRAME_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(16));
}
/// Type alias for easing function to reduce complexity
pub type EasingFunction = fn(f32, f32, f32, f32) -> f32;

/// Represents a single keyframe in an animation
#[derive(Clone)]
pub struct Keyframe<T: Animatable> {
    /// The target value at this keyframe
    pub value: T,
    /// Timing as a percentage (0.0 to 1.0)
    pub offset: f32,
    /// Optional easing function for this specific keyframe
    pub easing: Option<EasingFunction>,
}
/// Keyframe animation configuration
#[derive(Clone)]
pub struct KeyframeAnimation<T: Animatable> {
    pub keyframes: Vec<Keyframe<T>>,
    pub duration: Duration,
}

impl<T: Animatable> KeyframeAnimation<T> {
    pub fn new(duration: Duration) -> Self {
        // Use a pre-allocated vector with capacity for better performance
        Self {
            keyframes: Vec::with_capacity(4), // Most animations have 2-4 keyframes
            duration,
        }
    }

    /// Creates a new keyframe animation with a pre-allocated capacity
    pub fn with_capacity(duration: Duration, capacity: usize) -> Self {
        Self {
            keyframes: Vec::with_capacity(capacity),
            duration,
        }
    }

    pub fn add_keyframe(mut self, value: T, offset: f32, easing: Option<EasingFunction>) -> Self {
        self.keyframes.push(Keyframe {
            value,
            offset: offset.clamp(0.0, 1.0),
            easing,
        });
        self.keyframes.sort_by(|a, b| {
            a.offset
                .partial_cmp(&b.offset)
                .expect("Failed to compare keyframe offsets - possible NaN value")
        });
        self
    }
}

/// Helper function to create a fade animation with keyframes
pub fn create_fade_keyframes<T: Animatable>(
    start: T,
    end: T,
    duration_ms: u64,
) -> KeyframeAnimation<T> {
    let mut animation = KeyframeAnimation::with_capacity(Duration::from_millis(duration_ms), 2);

    // Add start keyframe
    animation = animation.add_keyframe(start, 0.0, Some(easer::functions::Cubic::ease_in_out));

    // Add end keyframe
    animation = animation.add_keyframe(end, 1.0, Some(easer::functions::Cubic::ease_in_out));

    animation
}
