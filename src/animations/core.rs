//! Core animation types and traits for Dioxus Motion
//!
//! This module contains the fundamental traits and types for implementing animations in Dioxus Motion.
//! It provides support for both tweening and spring-based animations with configurable parameters.

use std::sync::{Arc, Mutex};

use crate::animations::{spring::Spring, tween::Tween};
use instant::Duration;

/// A simplified trait for types that can be animated
///
/// This trait leverages standard Rust operator traits for mathematical operations,
/// reducing boilerplate and making implementations more intuitive.
/// Requires interpolation, magnitude calculation, component validation, and exact
/// value equality through `PartialEq`. Equality must include discrete properties
/// and units so reactive views observe every changed value; use epsilon only for convergence.
///
/// Arithmetic operates on values, displacements, forces, and velocities.
/// Preserve signed components and avoid clamping intermediate arithmetic.
/// `Self::default() * 0.0` must produce an additive zero value; `Default`
/// itself may represent a display value such as an identity transform.
pub trait Animatable:
    Clone
    + PartialEq
    + 'static
    + Default
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<f32, Output = Self>
{
    /// Interpolates between self and target using t (0.0 to 1.0)
    fn interpolate(&self, target: &Self, t: f32) -> Self;

    /// Calculates the magnitude/distance from zero
    /// Used for determining animation completion
    fn magnitude(&self) -> f32;

    /// Returns true only when every numerical component is finite.
    /// Check components directly; a magnitude can overflow for a finite vector.
    fn is_finite(&self) -> bool;

    /// Whether shared components have compatible units and shapes for spring arithmetic.
    /// Ordinary numerical vectors are compatible with every value of the same type.
    fn is_spring_compatible(&self, _target: &Self) -> bool {
        true
    }

    /// Returns the epsilon threshold for this type
    /// Default implementation provides a reasonable value for most use cases
    fn epsilon() -> f32 {
        0.01 // Single default epsilon for simplicity
    }
}

/// Euclidean magnitude with the normal f32 path and a wider fallback for
/// squared components that overflow or underflow. No allocation is needed.
#[inline]
pub(crate) fn magnitude(values: impl Iterator<Item = f32> + Clone) -> f32 {
    let squared = values
        .clone()
        .map(|value| value * value)
        .reduce(|sum, value| sum + value)
        .unwrap_or(0.0);
    if squared.is_normal() {
        squared.sqrt()
    } else {
        values
            .map(|value| f64::from(value).powi(2))
            .sum::<f64>()
            .sqrt() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_validation_checks_components_including_css_properties() {
        use crate::prelude::{Color, CssColor, CssValue, IntoCssValue, MotionStyle, Transform};
        for value in [0.0, f32::MAX, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let expected = value.is_finite();
            for index in 0..4 {
                let mut components = [0.0; 4];
                components[index] = value;
                let [r, g, b, a] = components;
                assert_eq!(Color { r, g, b, a }.is_finite(), expected);
                assert_eq!(Transform::new(r, g, b, a).is_finite(), expected);
                assert_eq!(
                    CssValue::Color(CssColor {
                        red: r,
                        green: g,
                        blue: b,
                        alpha: a
                    })
                    .is_finite(),
                    expected
                );
            }
            for css in [
                CssValue::Number(value),
                CssValue::Px(value),
                CssValue::Percent(value),
                CssValue::Vw(value),
                CssValue::Vh(value),
                CssValue::Deg(value),
            ] {
                assert_eq!(css.is_finite(), expected);
                let mut style = MotionStyle::default();
                style.properties.insert("width".into(), css);
                assert_eq!(style.is_finite(), expected);
            }
            for index in 0..16 {
                let mut style = MotionStyle::default();
                let fields = [
                    &mut style.opacity,
                    &mut style.x,
                    &mut style.y,
                    &mut style.z,
                    &mut style.scale,
                    &mut style.scale_x,
                    &mut style.scale_y,
                    &mut style.scale_z,
                    &mut style.rotate,
                    &mut style.rotate_x,
                    &mut style.rotate_y,
                    &mut style.rotate_z,
                    &mut style.skew,
                    &mut style.skew_x,
                    &mut style.skew_y,
                    &mut style.perspective,
                ];
                *fields[index] = value;
                assert_eq!(style.is_finite(), expected, "style field {index}");
            }
        }
        assert!(
            Color {
                r: f32::MAX,
                g: f32::MAX,
                b: f32::MAX,
                a: f32::MAX
            }
            .is_finite()
        );
        assert!(Transform::new(f32::MAX, f32::MAX, f32::MAX, f32::MAX).is_finite());
        let complex = "translateX(2px)".into_css_value("transform");
        assert!(matches!(complex, CssValue::Complex(_)));
        assert!(complex.is_finite());
        assert!(!complex.scale(f32::MAX).is_finite());
        assert!(CssValue::Keyword("inherit".into()).is_finite());
    }

    #[test]
    fn magnitude_preserves_extreme_components_and_euclidean_distance() {
        use crate::prelude::{Color, CssColor, CssValue, MotionStyle, Transform};
        for value in [
            f32::from_bits(1),
            f32::MIN_POSITIVE / 4.0,
            1.0,
            1e20,
            f32::MAX,
        ] {
            for index in 0..4 {
                let mut components = [0.0; 4];
                components[index] = -value;
                assert_eq!(magnitude(components.into_iter()), value);
                let [r, g, b, a] = components;
                assert_eq!(Color { r, g, b, a }.magnitude(), value);
                assert_eq!(Transform::new(r, g, b, a).magnitude(), value);
                assert_eq!(
                    CssValue::Color(CssColor {
                        red: r,
                        green: g,
                        blue: b,
                        alpha: a
                    })
                    .number(),
                    value
                );
            }
            for index in 0..17 {
                let mut style = MotionStyle::default() * 0.0;
                if index == 16 {
                    style
                        .properties
                        .insert("width".into(), CssValue::Px(-value));
                } else {
                    let fields = [
                        &mut style.opacity,
                        &mut style.x,
                        &mut style.y,
                        &mut style.z,
                        &mut style.scale,
                        &mut style.scale_x,
                        &mut style.scale_y,
                        &mut style.scale_z,
                        &mut style.rotate,
                        &mut style.rotate_x,
                        &mut style.rotate_y,
                        &mut style.rotate_z,
                        &mut style.skew,
                        &mut style.skew_x,
                        &mut style.skew_y,
                        &mut style.perspective,
                    ];
                    *fields[index] = -value;
                }
                assert_eq!(style.magnitude(), value, "style field {index}");
            }
        }
        assert_eq!(magnitude([3.0, 4.0].into_iter()), 5.0);
        assert_eq!(magnitude([0.0; 4].into_iter()), 0.0);
        assert!(magnitude([f32::NAN, 0.0].into_iter()).is_nan());
        assert_eq!(magnitude([f32::INFINITY].into_iter()), f32::INFINITY);
        for value in [f32::MIN_POSITIVE / 4.0, f32::MAX / 4.0] {
            assert_eq!(magnitude([value; 4].into_iter()), value * 2.0);
        }
    }

    #[test]
    fn shared_completion_reentry_returns_an_error_instead_of_blocking() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            mpsc,
        };
        let holder = Arc::new(Mutex::new(None::<AnimationConfig>));
        let weak = Arc::downgrade(&holder);
        let nested_result = Arc::new(Mutex::new(None));
        let observed = nested_result.clone();
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        let mut config = AnimationConfig::tween_ms(0).with_on_complete(move || {
            callback_calls.fetch_add(1, Ordering::Relaxed);
            let holder = weak.upgrade().expect("test config holder");
            let mut nested = holder
                .lock()
                .expect("holder mutex")
                .as_ref()
                .expect("shared config")
                .clone();
            *observed.lock().expect("result mutex") = Some(nested.execute_completion());
        });
        *holder.lock().expect("holder mutex") = Some(config.clone());
        let (sender, receiver) = mpsc::channel();
        let mut worker_config = config.clone();
        let worker = std::thread::spawn(move || {
            let _ = sender.send(worker_config.execute_completion());
        });
        assert_eq!(
            receiver
                .recv_timeout(Duration::from_secs(2))
                .expect("reentrant callback must not block"),
            Ok(())
        );
        worker.join().expect("completion worker");
        assert_eq!(
            *nested_result.lock().expect("result mutex"),
            Some(Err(AnimationError::CompletionBusy))
        );
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(config.execute_completion(), Ok(()));
        assert_eq!(calls.load(Ordering::Relaxed), 2);
        assert_eq!(AnimationConfig::tween_ms(0).execute_completion(), Ok(()));
    }

    #[test]
    #[allow(clippy::panic)] // Deliberately poison only this test callback's mutex.
    fn poisoned_completion_is_reported_instead_of_silently_skipped() {
        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let callback_calls = calls.clone();
        let mut config = AnimationConfig::tween_ms(0).with_on_complete(move || {
            callback_calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });
        let callback = config.on_complete.as_ref().expect("test callback").clone();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = callback.lock().expect("callback mutex");
            panic!("poison sentinel");
        }));
        assert_eq!(
            config.execute_completion(),
            Err(AnimationError::CompletionPoisoned)
        );
        assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 0);
    }

    #[test]
    fn configuration_rejects_invalid_numbers_and_accepts_boundaries() {
        for epsilon in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 0.0, -0.0, -1.0] {
            assert_eq!(
                AnimationConfig::tween_ms(1)
                    .with_epsilon(epsilon)
                    .validate(),
                Err(AnimationError::InvalidEpsilon)
            );
        }
        for epsilon in [f32::from_bits(1), 0.125, f32::MAX] {
            assert_eq!(
                AnimationConfig::tween_ms(1)
                    .with_epsilon(epsilon)
                    .validate(),
                Ok(())
            );
        }
        for field in ["stiffness", "damping", "mass"] {
            for (value, coefficient_valid, mass_valid) in [
                (f32::NAN, false, false),
                (f32::INFINITY, false, false),
                (f32::NEG_INFINITY, false, false),
                (-1.0, false, false),
                (-0.0, true, false),
                (0.0, true, false),
                (f32::from_bits(1), true, false),
                (1.0, true, true),
                (f32::MAX, true, true),
            ] {
                let mut spring = Spring::default();
                let valid = match field {
                    "stiffness" => {
                        spring.stiffness = value;
                        coefficient_valid
                    }
                    "damping" => {
                        spring.damping = value;
                        coefficient_valid
                    }
                    _ => {
                        spring.mass = value;
                        mass_valid
                    }
                };
                let expected = if valid {
                    Ok(())
                } else {
                    Err(AnimationError::InvalidSpringParameter(field))
                };
                assert_eq!(
                    AnimationConfig::spring(spring).validate(),
                    expected,
                    "{field}={value}"
                );
            }
        }
    }

    #[test]
    fn tween_ms_creates_tween_config_with_millisecond_duration() {
        let config = AnimationConfig::tween_ms(220);

        assert!(matches!(
            config.mode,
            AnimationMode::Tween(Tween { duration, .. }) if duration == Duration::from_millis(220)
        ));
    }

    #[test]
    fn alternate_duration_covers_entire_u8_range() {
        for count in 0..=u8::MAX {
            let config = AnimationConfig::tween(Duration::from_secs(1))
                .with_loop(LoopMode::AlternateTimes(count));
            assert_eq!(
                config.get_duration(),
                Duration::from_secs(u64::from(count) * 2)
            );
        }
        assert_eq!(
            AnimationConfig::tween(Duration::MAX)
                .with_loop(LoopMode::Times(2))
                .get_duration(),
            Duration::MAX,
        );
    }

    #[test]
    fn spring_creates_spring_config() {
        let spring = Spring::default();
        let config = AnimationConfig::spring(spring);

        assert_eq!(config.mode, AnimationMode::Spring(spring));
    }
}

/// Defines the type of animation to be used
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationMode {
    /// Tween animation with duration and easing
    Tween(Tween),
    /// Physics-based spring animation
    Spring(Spring),
}

impl Default for AnimationMode {
    fn default() -> Self {
        Self::Tween(Tween::default())
    }
}

/// Defines how the animation should loop
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum LoopMode {
    /// Play animation once
    #[default]
    None,
    /// Loop animation indefinitely
    Infinite,
    /// Loop animation a specific number of times
    Times(u8),
    /// Loop animation back and forth indefinitely
    Alternate,
    /// Loop animation back and forth a specific number of times
    AlternateTimes(u8),
}

pub type OnComplete = Arc<Mutex<dyn FnMut() + Send + 'static>>;

/// Invalid animation setup or a frame result that cannot be represented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum AnimationError {
    #[error(
        "spring values have incompatible units or component shapes; use a tween for discrete transitions"
    )]
    IncompatibleSpringValues,
    #[error("completion callback is already executing or locked")]
    CompletionBusy,
    #[error("completion callback mutex is poisoned")]
    CompletionPoisoned,
    #[error("velocity can only be changed during spring playback")]
    VelocityRequiresSpring,
    #[error("animation {0} contains a nonfinite numerical component")]
    NonFiniteValue(&'static str),
    #[error("animation epsilon must be finite and positive")]
    InvalidEpsilon,
    #[error(
        "spring {0} is invalid: coefficients must be finite and nonnegative, mass and its reciprocal must be finite and positive"
    )]
    InvalidSpringParameter(&'static str),
}

pub(crate) fn validate_value<T: Animatable>(
    value: &T,
    role: &'static str,
) -> Result<(), AnimationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(AnimationError::NonFiniteValue(role))
    }
}

pub(crate) fn validate_spring_transition<T: Animatable>(
    initial: &T,
    target: &T,
    mode: AnimationMode,
) -> Result<(), AnimationError> {
    if matches!(mode, AnimationMode::Spring(_)) && !initial.is_spring_compatible(target) {
        Err(AnimationError::IncompatibleSpringValues)
    } else {
        Ok(())
    }
}

/// Configuration for an animation
#[derive(Clone, Default)]
pub struct AnimationConfig {
    /// The type of animation (Tween or Spring)
    pub mode: AnimationMode,
    /// How the animation should loop
    pub loop_mode: Option<LoopMode>,
    /// Delay before animation starts
    pub delay: Duration,
    /// Callback when animation completes
    pub on_complete: Option<Arc<Mutex<dyn FnMut() + Send>>>,
    /// Custom epsilon threshold for animation completion detection
    /// If None, uses the type's default epsilon from Animatable::epsilon()
    pub epsilon: Option<f32>,
}

impl AnimationConfig {
    /// Checks numerical parameters without imposing an application-specific epsilon range.
    /// Animated values and velocities still need to remain representable in their value type.
    pub fn validate(&self) -> Result<(), AnimationError> {
        if let Some(epsilon) = self.epsilon {
            validate_completion_epsilon(epsilon)?;
        }
        if let AnimationMode::Spring(spring) = self.mode {
            for (name, value) in [("stiffness", spring.stiffness), ("damping", spring.damping)] {
                if !value.is_finite() || value < 0.0 {
                    return Err(AnimationError::InvalidSpringParameter(name));
                }
            }
            if !spring.mass.is_finite() || spring.mass <= 0.0 || !(1.0 / spring.mass).is_finite() {
                return Err(AnimationError::InvalidSpringParameter("mass"));
            }
        }
        Ok(())
    }

    pub(crate) fn validate_for<T: Animatable>(&self) -> Result<(), AnimationError> {
        self.validate()?;
        validate_completion_epsilon(self.epsilon.unwrap_or_else(T::epsilon))
    }

    /// Creates a new animation configuration with specified mode
    pub fn new(mode: AnimationMode) -> Self {
        Self {
            mode,
            loop_mode: None,
            delay: Duration::default(),
            on_complete: None,
            epsilon: None,
        }
    }

    /// Creates a tween animation configuration with the specified duration.
    pub fn tween(duration: Duration) -> Self {
        Self::new(AnimationMode::Tween(Tween::new(duration)))
    }

    /// Creates a tween animation configuration with a millisecond duration.
    pub fn tween_ms(milliseconds: u64) -> Self {
        Self::tween(Duration::from_millis(milliseconds))
    }

    /// Creates a spring animation configuration with the specified spring.
    pub fn spring(spring: Spring) -> Self {
        Self::new(AnimationMode::Spring(spring))
    }

    /// Sets the loop mode for the animation
    pub fn with_loop(mut self, loop_mode: LoopMode) -> Self {
        self.loop_mode = Some(loop_mode);
        self
    }

    /// Sets a delay before the animation starts
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Sets a callback to be called when animation completes
    pub fn with_on_complete<F>(mut self, f: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        self.on_complete = Some(Arc::new(Mutex::new(f)));
        self
    }

    /// Sets a custom epsilon threshold for animation completion detection
    ///
    /// # Arguments
    /// * `epsilon` - The minimum meaningful difference between values for completion detection
    ///
    /// # Examples
    /// ```rust
    /// use dioxus_motion::prelude::*;
    /// let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()))
    ///     .with_epsilon(0.01); // Custom threshold for page transitions
    /// ```
    pub fn with_epsilon(mut self, epsilon: f32) -> Self {
        self.epsilon = Some(epsilon);
        self
    }

    /// Gets the total duration of the animation
    pub fn get_duration(&self) -> Duration {
        match &self.mode {
            AnimationMode::Spring(_) => {
                // Springs don't have a fixed duration, estimate based on typical settling time
                Duration::from_secs_f32(1.0) // You might want to adjust this based on spring parameters
            }
            AnimationMode::Tween(tween) => {
                let base_duration = tween.duration;
                match self.loop_mode {
                    Some(LoopMode::Infinite) => Duration::from_secs(f32::INFINITY as u64),
                    Some(LoopMode::Times(count)) => base_duration.saturating_mul(count.into()),
                    Some(LoopMode::Alternate) => Duration::from_secs(f32::INFINITY as u64),
                    Some(LoopMode::AlternateTimes(count)) => {
                        base_duration.saturating_mul(u32::from(count) * 2)
                    }
                    Some(LoopMode::None) | None => base_duration,
                }
            }
        }
    }

    /// Executes the completion callback without blocking on a busy or poisoned mutex.
    pub fn execute_completion(&mut self) -> Result<(), AnimationError> {
        if let Some(on_complete) = &self.on_complete {
            execute_completion_callback(on_complete)?;
        }
        Ok(())
    }
}

fn validate_completion_epsilon(epsilon: f32) -> Result<(), AnimationError> {
    if epsilon.is_finite() && epsilon > 0.0 {
        Ok(())
    } else {
        Err(AnimationError::InvalidEpsilon)
    }
}

pub(crate) fn execute_completion_callback(on_complete: &OnComplete) -> Result<(), AnimationError> {
    let mut callback = on_complete.try_lock().map_err(|error| match error {
        std::sync::TryLockError::WouldBlock => AnimationError::CompletionBusy,
        std::sync::TryLockError::Poisoned(_) => AnimationError::CompletionPoisoned,
    })?;
    callback();
    Ok(())
}
