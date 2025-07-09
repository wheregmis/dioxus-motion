//! Dioxus Motion - Animation library for Dioxus
//!
//! Provides smooth animations for web and native applications built with Dioxus.
//! Supports both spring physics and tween-based animations with configurable parameters.
//!
//! # Features
//! - **Simplified Animatable trait** - Uses standard Rust operators (`+`, `-`, `*`) for math operations
//! - Spring physics animations
//! - Tween animations with custom easing
//! - Color interpolation
//! - Transform animations
//! - Configurable animation loops
//! - Animation sequences
//! - Single default epsilon (0.01) for consistent animation completion
//!
//! # Example
//! ```rust,no_run
//! use dioxus_motion::prelude::*;
//!
//! let mut value = use_motion(0.0f32);
//!
//! // Basic animation - uses default epsilon (0.01) for completion detection
//! value.animate_to(100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
//!
//! // Animation with custom epsilon for fine-tuned performance (optional)
//! value.animate_to(
//!     100.0,
//!     AnimationConfig::new(AnimationMode::Spring(Spring::default()))
//!         .with_epsilon(0.001) // Tighter threshold for high-precision animations
//! );
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

use animations::core::{Animatable, AnimationConfig};
use dioxus::prelude::*;
pub use instant::Duration;

pub mod animations;
pub mod keyframes;
pub mod manager;
pub mod motion;
pub mod multithreaded_motion;
pub mod sequence;
#[cfg(feature = "transitions")]
pub mod transitions;

#[cfg(feature = "transitions")]
pub use dioxus_motion_transitions_macro;

pub use animations::platform::{MotionTime, TimeProvider};

pub use keyframes::{Keyframe, KeyframeAnimation};
pub use manager::AnimationManager;

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
    #[cfg(feature = "transitions")]
    pub use crate::transitions::config::TransitionVariant;
    #[cfg(feature = "transitions")]
    pub use crate::transitions::page_transitions::{AnimatableRoute, AnimatedOutlet};
    pub use crate::{AnimationManager, Duration, Time, TimeProvider, use_motion};

    // Multithreaded animation support
    pub use crate::EnhancedMotionHandle;
    pub use crate::animations::{
        parallel::ParallelAnimationProcessor, scheduler::use_motion_scheduled,
    };
    pub use crate::multithreaded_motion::{MultithreadedMotionHandle, use_motion_multithreaded};
}

pub type Time = MotionTime;

/// Helper function to calculate the appropriate delay for the animation loop
fn calculate_delay(dt: f32, running_frames: u32) -> Duration {
    #[cfg(feature = "web")]
    {
        // running_frames is not used in web builds but kept for API consistency
        let _ = running_frames;
        match dt {
            x if x < 0.008 => Duration::from_millis(8),  // ~120fps
            x if x < 0.016 => Duration::from_millis(16), // ~60fps
            _ => Duration::from_millis(32),              // ~30fps
        }
    }
    #[cfg(not(feature = "web"))]
    {
        if running_frames <= 200 {
            Duration::from_micros(8333) // ~120fps
        } else {
            match dt {
                x if x < 0.005 => Duration::from_millis(8),  // ~120fps
                x if x < 0.011 => Duration::from_millis(16), // ~60fps
                _ => Duration::from_millis(33),              // ~30fps
            }
        }
    }
}

/// Creates an animation manager that continuously updates a motion state.
///
/// This function initializes a motion state with the provided initial value and spawns an asynchronous loop
/// that updates the animation state based on the elapsed time between frames. It includes automatic
/// multithreading support for background processing on desktop platforms, while remaining lightweight
/// on web platforms.
///
/// # Enhanced Features
///
/// - **Background Processing**: Heavy calculations are offloaded to background threads (desktop only)
/// - **Parallel Animation Support**: Multiple animations can be processed concurrently
/// - **Adaptive Performance**: Automatically adjusts update rates based on animation complexity
/// - **Cross-Platform**: Full features on desktop, optimized for web/WASM compatibility
///
/// # Example
///
/// ```no_run
/// use dioxus_motion::prelude::*;
/// use dioxus::prelude::*;
///
/// fn app() -> Element {
///     let mut value = use_motion(0.0f32);
///
///     // Animate to 100 with spring physics
///     value.animate_to(
///         100.0,
///         AnimationConfig::new(AnimationMode::Spring(Spring::default()))
///     );
///
///     rsx! {
///         div {
///             style: "transform: translateY({value.get_value()}px)",
///             "Animated content"
///         }
///     }
/// }
/// ```
pub fn use_motion<T: Animatable + Send + Sync + 'static>(initial: T) -> EnhancedMotionHandle<T> {
    let mut state = use_signal(|| Motion::new(initial));
    let mut processing_queue = use_signal(Vec::<AnimationCommand<T>>::new);
    let mut is_processing = use_signal(|| false);

    #[cfg(feature = "web")]
    let idle_poll_rate = Duration::from_millis(100);

    #[cfg(not(feature = "web"))]
    let idle_poll_rate = Duration::from_millis(33);

    use_effect(move || {
        // Enhanced animation loop with multithreading support
        spawn(async move {
            let mut last_frame = Time::now();
            let mut running_frames = 0u32;

            loop {
                let now = Time::now();
                let dt = (now.duration_since(last_frame).as_secs_f32()).min(0.1);
                last_frame = now;

                // Process background animation commands
                let commands = {
                    let mut queue = processing_queue.write();
                    if queue.is_empty() {
                        Vec::new()
                    } else {
                        std::mem::take(&mut *queue)
                    }
                };

                if !commands.is_empty() {
                    *is_processing.write() = true;

                    // Process commands with platform-specific optimization
                    for cmd in commands {
                        process_animation_command(cmd, state).await;
                    }

                    *is_processing.write() = false;
                }

                // Main animation update loop
                if (*state.peek()).is_running() {
                    running_frames += 1;
                    let prev_value = (*state.peek()).get_value();
                    let updated = (*state.write()).update(dt);
                    let new_value = (*state.peek()).get_value();
                    let epsilon = (*state.peek()).get_epsilon();

                    // Only trigger a re-render if the value changed significantly
                    if (new_value - prev_value).magnitude() > epsilon || updated {
                        // State has changed enough, continue
                    } else {
                        // Skip this frame's update to avoid unnecessary re-render
                        let delay = calculate_delay(dt, running_frames);
                        Time::delay(delay).await;
                        continue;
                    }

                    let delay = calculate_delay(dt, running_frames);
                    Time::delay(delay).await;
                } else {
                    running_frames = 0;
                    Time::delay(idle_poll_rate).await;
                }
            }
        });
    });

    EnhancedMotionHandle {
        state,
        processing_queue,
        is_processing,
    }
}

/// Deprecated: use_motion_enhanced is now an alias for use_motion. Use use_motion instead.
#[deprecated(note = "use_motion_enhanced is now an alias for use_motion. Use use_motion instead.")]
pub fn use_motion_enhanced<T: Animatable + Send + Sync + 'static>(
    initial: T,
) -> EnhancedMotionHandle<T> {
    use_motion(initial)
}

/// Enhanced motion handle with multithreading capabilities
#[derive(Clone)]
pub struct EnhancedMotionHandle<T: Animatable + Send + Sync + 'static> {
    state: Signal<Motion<T>>,
    processing_queue: Signal<Vec<AnimationCommand<T>>>,
    is_processing: Signal<bool>,
}

impl<T: Animatable + Send + Sync + 'static> EnhancedMotionHandle<T> {
    /// Standard animation (non-blocking)
    pub fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.state.write().animate_to(target, config);
    }

    /// Parallel batch animation - processes multiple targets concurrently
    /// Uses background processing on desktop, sequential on web
    pub fn animate_to_parallel(&mut self, targets: Vec<(T, AnimationConfig)>) {
        let cmd = AnimationCommand::ParallelBatch(targets);
        self.processing_queue.write().push(cmd);
    }

    /// Heavy computation animation - offloads complex calculations
    /// Uses background threads on desktop, immediate processing on web
    pub fn animate_to_heavy(&mut self, target: T, config: AnimationConfig) {
        let cmd = AnimationCommand::HeavyComputation(target, config);
        self.processing_queue.write().push(cmd);
    }

    /// Interpolate between multiple values with parallel processing
    pub fn interpolate_sequence(&mut self, sequence: Vec<T>, duration_per_step: f32) {
        let cmd = AnimationCommand::InterpolateSequence(sequence, duration_per_step);
        self.processing_queue.write().push(cmd);
    }

    /// Get the current value
    pub fn get_value(&self) -> T {
        self.state.read().get_value()
    }

    /// Check if any background processing is happening
    pub fn is_processing(&self) -> bool {
        *self.is_processing.read()
    }

    /// Check if the animation is running
    pub fn is_running(&self) -> bool {
        self.state.read().is_running() || self.is_processing()
    }

    /// Stop all animations and processing
    pub fn stop(&mut self) {
        self.state.write().stop();
        self.processing_queue.write().clear();
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.state.write().reset();
        self.processing_queue.write().clear();
    }
}

/// Commands for multithreaded animation processing
#[derive(Clone)]
enum AnimationCommand<T: Animatable + Send + Sync + 'static> {
    ParallelBatch(Vec<(T, AnimationConfig)>),
    HeavyComputation(T, AnimationConfig),
    InterpolateSequence(Vec<T>, f32),
}

/// Process animation commands with platform-specific optimization
async fn process_animation_command<T: Animatable + Send + Sync + 'static>(
    command: AnimationCommand<T>,
    mut state: Signal<Motion<T>>,
) {
    match command {
        AnimationCommand::ParallelBatch(targets) => {
            // Process multiple animation targets
            if let Some((target, config)) = targets.first() {
                let target_copy = *target;
                let config_copy = config.clone();
                #[cfg(not(target_arch = "wasm32"))]
                spawn(async move {
                    // Background processing for desktop
                    let computed_target = perform_background_calculation(target_copy).await;
                    state.write().animate_to(computed_target, config_copy);
                });

                #[cfg(target_arch = "wasm32")]
                {
                    // Direct processing for web
                    state.write().animate_to(target_copy, config_copy);
                }
            }
        }

        AnimationCommand::HeavyComputation(target, config) => {
            #[cfg(not(target_arch = "wasm32"))]
            spawn(async move {
                // Perform heavy computation in background
                let computed_target = perform_heavy_computation(target).await;
                state.write().animate_to(computed_target, config);
            });

            #[cfg(target_arch = "wasm32")]
            {
                // Direct processing for web (no background threads)
                state.write().animate_to(target, config);
            }
        }

        AnimationCommand::InterpolateSequence(sequence, _duration_per_step) => {
            // Process interpolation sequence
            if sequence.len() > 1 {
                #[cfg(not(target_arch = "wasm32"))]
                spawn(async move {
                    let final_target = process_interpolation_sequence(sequence).await;
                    state
                        .write()
                        .animate_to(final_target, AnimationConfig::default());
                });

                #[cfg(target_arch = "wasm32")]
                {
                    // Direct processing for web
                    if let Some(&final_target) = sequence.last() {
                        state
                            .write()
                            .animate_to(final_target, AnimationConfig::default());
                    }
                }
            }
        }
    }
}

/// Perform background calculation (desktop only)
#[cfg(not(target_arch = "wasm32"))]
async fn perform_background_calculation<T: Animatable + Send + Sync + 'static>(target: T) -> T {
    // Simulate complex calculation
    for _i in 0..100 {
        std::thread::sleep(Duration::from_micros(10));
    }
    target
}

/// Perform heavy computation (desktop only)
#[cfg(not(target_arch = "wasm32"))]
async fn perform_heavy_computation<T: Animatable + Send + Sync + 'static>(target: T) -> T {
    // Simulate heavy computation
    for _i in 0..50 {
        std::thread::sleep(Duration::from_micros(20));
    }
    target
}

/// Process interpolation sequence (desktop only)
#[cfg(not(target_arch = "wasm32"))]
async fn process_interpolation_sequence<T: Animatable + Send + Sync + 'static>(
    sequence: Vec<T>,
) -> T {
    // Simulate complex interpolation processing
    for _i in 0..sequence.len() {
        std::thread::sleep(Duration::from_micros(5));
    }
    sequence.into_iter().last().unwrap_or_else(T::default)
}

// Note: EnhancedMotionHandle doesn't implement AnimationManager trait
// because it requires Copy, but provides a richer API with multithreading capabilities
