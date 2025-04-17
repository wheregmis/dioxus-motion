//! Animation module providing core animation functionality
//!
//! This module contains traits and types for implementing animations in Dioxus Motion.
//! It provides support for both tweening and spring-based animations with configurable
//! parameters.

use std::sync::{Arc, Mutex};

use crate::animations::{spring::Spring, tween::Tween};
use instant::Duration;

/// A trait for types that can be animated
///
/// Types implementing this trait can be used with both tween and spring animations.
/// The trait provides basic mathematical operations needed for interpolation and
/// physics calculations.
///
/// This trait is now Send + Sync compatible to support multithreaded animations.
pub trait Animatable: Copy + Send + Sync + 'static {
    /// Creates a zero value for the type
    fn zero() -> Self;

    /// Returns the smallest meaningful difference between values
    fn epsilon() -> f32;

    /// Calculates the magnitude/length of the value
    fn magnitude(&self) -> f32;

    /// Scales the value by a factor
    fn scale(&self, factor: f32) -> Self;

    /// Adds another value
    fn add(&self, other: &Self) -> Self;

    /// Subtracts another value
    fn sub(&self, other: &Self) -> Self;

    /// Interpolates between self and target using t (0.0 to 1.0)
    fn interpolate(&self, target: &Self, t: f32) -> Self;

    /// Returns self as a Transform if this type is a Transform, or None otherwise
    /// Used for SIMD optimizations
    fn as_transform(&self) -> Option<&crate::prelude::Transform> {
        None
    }

    /// Creates a new instance from a Transform
    /// Used for SIMD optimizations
    fn from_transform(_transform: &crate::prelude::Transform) -> Self {
        // Default implementation returns zero, should be overridden by Transform
        Self::zero()
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopMode {
    /// Play animation once
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

impl Default for LoopMode {
    fn default() -> Self {
        Self::None
    }
}

/// Type alias for animation completion callbacks
///
/// Must be Send + Sync to support multithreaded animations
pub type OnComplete = Arc<Mutex<dyn FnMut() + Send + Sync + 'static>>;
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
    /// Must be Send + Sync for multithreaded animations
    pub on_complete: Option<Arc<Mutex<dyn FnMut() + Send + Sync>>>,
}

impl AnimationConfig {
    /// Creates a new animation configuration with specified mode
    pub fn new(mode: AnimationMode) -> Self {
        Self {
            mode,
            loop_mode: None,
            delay: Duration::default(),
            on_complete: None,
        }
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
        F: FnMut() + Send + Sync + 'static,
    {
        self.on_complete = Some(Arc::new(Mutex::new(f)));
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
                    Some(LoopMode::Times(count)) => base_duration * count.into(),
                    Some(LoopMode::Alternate) => Duration::from_secs(f32::INFINITY as u64),
                    Some(LoopMode::AlternateTimes(count)) => base_duration * (count * 2).into(),
                    Some(LoopMode::None) | None => base_duration,
                }
            }
        }
    }

    /// Execute the completion callback if it exists
    pub fn execute_completion(&mut self) {
        if let Some(on_complete) = &self.on_complete {
            if let Ok(mut callback) = on_complete.lock() {
                callback();
            }
        }
    }
}

pub mod simd {
    use wide::{f32x4, f32x8};

    /// SIMD-optimized linear interpolation for 4 f32 values at once
    #[inline]
    pub fn lerp_f32x4(start: f32x4, end: f32x4, t: f32) -> f32x4 {
        start + (end - start) * f32x4::splat(t)
    }

    /// SIMD-optimized linear interpolation for 8 f32 values at once
    #[inline]
    pub fn lerp_f32x8(start: f32x8, end: f32x8, t: f32) -> f32x8 {
        start + (end - start) * f32x8::splat(t)
    }

    /// SIMD-optimized spring force calculation for 4 f32 values at once
    #[inline]
    pub fn spring_force_f32x4(
        position: f32x4,
        target: f32x4,
        velocity: f32x4,
        stiffness: f32,
        damping: f32,
    ) -> f32x4 {
        let delta = target - position;
        let spring_force = delta * f32x4::splat(stiffness);
        let damping_force = velocity * f32x4::splat(damping);
        spring_force - damping_force
    }

    /// SIMD-optimized magnitude calculation for 4 f32 values
    #[inline]
    pub fn magnitude_f32x4(v: f32x4) -> f32 {
        let squared = v * v;
        let arr = squared.to_array();
        let sum = arr[0] + arr[1] + arr[2] + arr[3];
        sum.sqrt()
    }

    /// SIMD-optimized check if values are close enough to target
    #[inline]
    pub fn is_near_target_f32x4(
        position: f32x4,
        target: f32x4,
        velocity: f32x4,
        epsilon: f32,
    ) -> bool {
        let epsilon_sq = epsilon * epsilon;
        let delta = target - position;
        let delta_sq = delta * delta;
        let velocity_sq = velocity * velocity;

        let delta_arr = delta_sq.to_array();
        let velocity_arr = velocity_sq.to_array();
        let delta_sum = delta_arr[0] + delta_arr[1] + delta_arr[2] + delta_arr[3];
        let velocity_sum = velocity_arr[0] + velocity_arr[1] + velocity_arr[2] + velocity_arr[3];

        delta_sum < epsilon_sq && velocity_sum < epsilon_sq
    }
}
