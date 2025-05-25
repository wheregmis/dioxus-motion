use crate::Duration;
use crate::animations::utils::Animatable;

#[derive(Clone)]
pub struct Keyframe<T: Animatable> {
    pub value: T,
    pub offset: f32,
    pub easing: Option<fn(f32, f32, f32, f32) -> f32>,
}

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
