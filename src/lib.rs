//! Dioxus Motion - Animation library for Dioxus
//!
//! Provides smooth animations for web and native applications built with Dioxus.
//! Supports both spring physics and tween-based animations with configurable parameters.
//!
//! # Features
//! - **Simplified Animatable trait** - Uses standard Rust operators (`+`, `-`, `*`) for math operations
//! - **High-performance optimizations** - Automatic memory pooling, state machine dispatch, and resource management
//! - Spring physics animations with optimized integration
//! - Tween animations with custom easing
//! - Color interpolation
//! - Transform animations
//! - Configurable animation loops
//! - Animation sequences with atomic step management
//! - Single default epsilon (0.01) for consistent animation completion
//! - Automatic resource pool management for maximum performance
//!
//! # Example
//! ```rust,no_run
//! use dioxus_motion::prelude::*;
//!
//! // Optional: Configure resource pools for optimal performance (recommended for production)
//! resource_pools::init_high_performance();
//!
//! let mut motion = use_motion_store(0.0f32);
//!
//! // Basic animation - automatically uses all optimizations
//! motion.animate_to(100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
//!
//! // Animation with custom epsilon for fine-tuned performance (optional)
//! motion.animate_to(
//!     100.0,
//!     AnimationConfig::new(AnimationMode::Spring(Spring::default()))
//!         .with_epsilon(0.001) // Tighter threshold for high-precision animations
//! );
//!
//! // Check if animation is running
//! if motion.is_running() {
//!     println!("Animation is active with current value: {}", motion.get_value());
//! }
//! ```
//!
//! # Creating Custom Animatable Types
//!
//! The simplified `Animatable` trait requires only two methods and leverages standard Rust traits:
//!
//! ```rust
//! use dioxus_motion::prelude::*;
//! use dioxus_motion::animations::core::Animatable;
//!
//! #[derive(Debug, Copy, Clone, PartialEq, Default)]
//! struct Point { x: f32, y: f32 }
//!
//! // Implement standard math operators
//! impl std::ops::Add for Point {
//!     type Output = Self;
//!     fn add(self, other: Self) -> Self {
//!         Self { x: self.x + other.x, y: self.y + other.y }
//!     }
//! }
//!
//! impl std::ops::Sub for Point {
//!     type Output = Self;
//!     fn sub(self, other: Self) -> Self {
//!         Self { x: self.x - other.x, y: self.y - other.y }
//!     }
//! }
//!
//! impl std::ops::Mul<f32> for Point {
//!     type Output = Self;
//!     fn mul(self, factor: f32) -> Self {
//!         Self { x: self.x * factor, y: self.y * factor }
//!     }
//! }
//!
//! // Implement Animatable with just two methods
//! impl Animatable for Point {
//!     fn interpolate(&self, target: &Self, t: f32) -> Self {
//!         *self + (*target - *self) * t
//!     }
//!     
//!     fn magnitude(&self) -> f32 {
//!         (self.x * self.x + self.y * self.y).sqrt()
//!     }
//! }
//! ```

#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(unused_variables)]
#![deny(unused_must_use)]
#![deny(unsafe_code)] // Prevent unsafe blocks
#![deny(clippy::unwrap_in_result)] // No unwrap() on Result
// #![deny(clippy::indexing_slicing)] // Prevent unchecked indexing
#![deny(rustdoc::broken_intra_doc_links)] // Check doc links
// #![deny(clippy::arithmetic_side_effects)] // Check for integer overflow
#![deny(clippy::modulo_arithmetic)] // Check modulo operations
#![deny(clippy::option_if_let_else)] // Prefer map/and_then

use animations::core::Animatable;
pub use instant::Duration;

pub mod animations;
pub mod keyframes;
pub mod motion;
pub mod pool;
pub mod sequence;
pub mod store;
#[cfg(feature = "transitions")]
pub mod transitions;

#[cfg(feature = "transitions")]
pub use dioxus_motion_transitions_macro;

pub use animations::platform::{MotionTime, TimeProvider};

pub use keyframes::{Keyframe, KeyframeAnimation};

use motion::Motion;

// Re-exports
pub mod prelude {
    pub use crate::animations::core::{AnimationConfig, AnimationMode, LoopMode};
    pub use crate::animations::{
        colors::Color, spring::Spring, transform::Transform, tween::Tween,
    };
    #[cfg(feature = "transitions")]
    pub use crate::dioxus_motion_transitions_macro::MotionTransitions;
    pub use crate::sequence::AnimationSequence;
    pub use crate::store::{MotionHandle, MotionStore, MotionStoreStoreExt, use_motion_store};
    #[cfg(feature = "transitions")]
    pub use crate::transitions::config::TransitionVariant;
    #[cfg(feature = "transitions")]
    pub use crate::transitions::page_transitions::{
        AnimatableRoute, AnimatedOutlet, AnimatedRouterContext, TransitionVariantResolver,
        use_animated_router,
    };
    pub use crate::{Duration, Time, TimeProvider};

    // Performance optimization exports
    pub use crate::motion::MotionOptimizationStats;
    pub use crate::pool::resource_pools;
    pub use crate::pool::{PoolConfig, PoolStats};
}

pub type Time = MotionTime;
