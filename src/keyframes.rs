use crate::Duration;
use crate::animations::core::Animatable;

pub type EasingFn = fn(f32, f32, f32, f32) -> f32;

#[derive(Debug, thiserror::Error)]
pub enum KeyframeError {
    #[error("Keyframe offsets cannot be NaN")]
    InvalidOffset,
    #[error("Keyframe values must have finite numerical components")]
    NonFiniteValue,
}

#[derive(Clone)]
pub struct Keyframe<T: Animatable> {
    pub value: T,
    pub offset: f32,
    pub easing: Option<EasingFn>,
}

#[derive(Clone)]
pub struct KeyframeAnimation<T: Animatable> {
    keyframes: Vec<Keyframe<T>>,
    pub duration: Duration,
}

impl<T: Animatable> KeyframeAnimation<T> {
    pub fn new(duration: Duration) -> Self {
        Self {
            keyframes: Vec::new(),
            duration,
        }
    }

    /// Returns keyframes in offset order. Add frames through `add_keyframe`
    /// so offsets remain validated and ordered.
    ///
    /// ```compile_fail
    /// use dioxus_motion::KeyframeAnimation;
    /// use std::time::Duration;
    /// let mut animation = KeyframeAnimation::<f32>::new(Duration::from_secs(1));
    /// animation.keyframes().clear();
    /// ```
    pub fn keyframes(&self) -> &[Keyframe<T>] {
        &self.keyframes
    }

    /// Adds a frame, keeping equal offsets in insertion order.
    /// Completed playback always lands on the last frame, including duplicate offsets at 1.0.
    pub fn add_keyframe(
        mut self,
        value: T,
        offset: f32,
        easing: Option<EasingFn>,
    ) -> Result<Self, KeyframeError> {
        if offset.is_nan() {
            return Err(KeyframeError::InvalidOffset);
        }
        if !value.is_finite() {
            return Err(KeyframeError::NonFiniteValue);
        }
        let offset = offset.clamp(0.0, 1.0);
        let index = self
            .keyframes
            .partition_point(|frame| frame.offset <= offset);
        self.keyframes.insert(
            index,
            Keyframe {
                value,
                offset,
                easing,
            },
        );
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonfinite_values_are_rejected_before_insertion() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(matches!(
                KeyframeAnimation::new(Duration::from_secs(1)).add_keyframe(value, 0.5, None),
                Err(KeyframeError::NonFiniteValue)
            ));
        }
        assert!(
            KeyframeAnimation::new(Duration::from_secs(1))
                .add_keyframe(f32::MAX, 0.5, None)
                .is_ok()
        );
    }

    #[test]
    fn offsets_are_validated_clamped_and_stably_ordered() {
        let mut animation = KeyframeAnimation::new(Duration::from_secs(1));
        for (value, offset) in [(1.0f32, 0.5), (2.0, 2.0), (3.0, -1.0), (4.0, 0.5)] {
            animation = animation
                .add_keyframe(value, offset, None)
                .expect("valid offset");
        }
        let frames = animation.keyframes();
        assert_eq!(
            frames.iter().map(|frame| frame.value).collect::<Vec<_>>(),
            [3.0, 1.0, 4.0, 2.0]
        );
        assert_eq!(
            frames.iter().map(|frame| frame.offset).collect::<Vec<_>>(),
            [0.0, 0.5, 0.5, 1.0]
        );
        assert!(matches!(
            KeyframeAnimation::new(Duration::from_secs(1)).add_keyframe(0.0f32, f32::NAN, None),
            Err(KeyframeError::InvalidOffset),
        ));
    }
}
