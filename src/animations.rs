//! Animation module providing core animation functionality
//!
//! This module contains traits and types for implementing animations in Dioxus Motion.
//! It provides support for both tweening and spring-based animations with configurable
//! parameters.

use crate::{spring::Spring, tween::Tween};
use instant::Duration;

/// A trait for types that can be animated
///
/// Types implementing this trait can be used with both tween and spring animations.
/// The trait provides basic mathematical operations needed for interpolation and
/// physics calculations.
pub trait Animatable: Copy + 'static {
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
    Times(u32),
}

/// Type alias for animation completion callback
pub type OnComplete = Box<dyn FnMut() + 'static>;

/// Configuration for an animation
#[derive(Default)]
pub struct AnimationConfig {
    /// The type of animation (Tween or Spring)
    pub mode: AnimationMode,
    /// How the animation should loop
    pub loop_mode: Option<LoopMode>,
    /// Delay before animation starts
    pub delay: Duration,
    /// Callback when animation completes
    pub on_complete: Option<OnComplete>,
}

impl AnimationConfig {
    /// Creates a new animation configuration with specified mode
    ///
    /// # Example
    /// ```rust
    /// let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
    /// ```
    pub fn new(mode: AnimationMode) -> Self {
        Self {
            mode,
            loop_mode: None,
            delay: Duration::default(),
            on_complete: None,
        }
    }

    /// Sets the loop mode for the animation
    ///
    /// # Example
    /// ```rust
    /// let config = AnimationConfig::new(mode)
    ///     .with_loop(LoopMode::Infinite);
    /// ```
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
        F: FnMut() + 'static,
    {
        self.on_complete = Some(Box::new(f));
        self
    }
}
