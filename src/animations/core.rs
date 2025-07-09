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
/// Only requires implementing interpolation and magnitude calculation.
pub trait Animatable:
    Copy
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

    /// Returns the epsilon threshold for this type
    /// Default implementation provides a reasonable value for most use cases
    fn epsilon() -> f32 {
        0.01 // Single default epsilon for simplicity
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
    pub on_complete: Option<Arc<Mutex<dyn FnMut() + Send + Sync>>>,
    /// Custom epsilon threshold for animation completion detection
    /// If None, uses the type's default epsilon from Animatable::epsilon()
    pub epsilon: Option<f32>,
}

impl AnimationConfig {
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
