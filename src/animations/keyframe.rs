use super::utils::Animatable;
pub use instant::Duration;
/// Represents a single keyframe in an animation
/// Represents a single keyframe in an animation
#[derive(Clone)]
pub struct Keyframe<T: Animatable> {
    /// The target value at this keyframe
    pub value: T,
    /// Timing as a percentage (0.0 to 1.0)
    pub offset: f32,
    /// Optional easing function for this specific keyframe
    pub easing: Option<fn(f32, f32, f32, f32) -> f32>,
}
/// Keyframe animation configuration
#[derive(Clone)]
pub struct KeyframeAnimation<T: Animatable> {
    pub keyframes: Vec<Keyframe<T>>,
    pub duration: Duration,
}

impl<T: Animatable> KeyframeAnimation<T> {
    pub fn new(duration: Duration) -> Self {
        Self {
            keyframes: Vec::new(),
            duration,
        }
    }

    pub fn add_keyframe(
        mut self,
        value: T,
        offset: f32,
        easing: Option<fn(f32, f32, f32, f32) -> f32>,
    ) -> Self {
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
