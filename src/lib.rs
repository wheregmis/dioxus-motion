//! Dioxus Motion - Animation library for Dioxus
//!
//! Provides smooth animations for web and native applications built with Dioxus.
//! Supports both spring physics and tween-based animations with configurable parameters.
//!
//! # Features
//! - Spring physics animations
//! - Tween animations with custom easing
//! - Color interpolation
//! - Transform animations
//! - Configurable animation loops
//! - Animation sequences
//!
//! # Example
//! ```rust,no_run
//! use dioxus_motion::prelude::*;
//!
//! let mut value = use_motion(0.0f32);
//! value.animate_to(100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
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

use animations::utils::Animatable;
use dioxus::prelude::*;
pub use instant::Duration;

pub mod animations;
pub mod keyframes;
pub mod manager;
pub mod motion;
pub mod sequence;
pub mod transitions;

#[cfg(feature = "transitions")]
pub use dioxus_motion_transitions_macro;

pub use animations::platform::{MotionTime, TimeProvider};

pub use keyframes::{Keyframe, KeyframeAnimation};
pub use manager::AnimationManager;

use motion::Motion;

// Re-exports
pub mod prelude {
    pub use crate::animations::utils::{AnimationConfig, AnimationMode, LoopMode};
    pub use crate::animations::{
        colors::Color, spring::Spring, transform::Transform, tween::Tween,
    };
    #[cfg(feature = "transitions")]
    pub use crate::dioxus_motion_transitions_macro::MotionTransitions;
    pub use crate::sequence::AnimationSequence;
    #[cfg(feature = "transitions")]
    pub use crate::transitions::page_transitions::{AnimatableRoute, AnimatedOutlet};
    #[cfg(feature = "transitions")]
    pub use crate::transitions::utils::TransitionVariant;
    pub use crate::{AnimationManager, Duration, Time, TimeProvider, use_motion};
}

pub type Time = MotionTime;

/// Creates an animation manager that continuously updates a motion state.
///
/// This function initializes a motion state with the provided initial value and spawns an asynchronous loop
/// that updates the animation state based on the elapsed time between frames. When the animation is running,
/// it updates the state using the calculated time delta and dynamically adjusts the update interval to optimize CPU usage;
/// when the animation is inactive, it waits longer before polling again.
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
pub fn use_motion<T: Animatable>(initial: T) -> impl AnimationManager<T> {
    let mut state = use_signal(|| Motion::new(initial));

    #[cfg(feature = "web")]
    let idle_poll_rate = Duration::from_millis(100);

    #[cfg(not(feature = "web"))]
    let idle_poll_rate = Duration::from_millis(33);

    use_effect(move || {
        // This executes after rendering is complete
        spawn(async move {
            let mut last_frame = Time::now();
            let mut _running_frames = 0u32;

            loop {
                let now = Time::now();
                let dt = (now.duration_since(last_frame).as_secs_f32()).min(0.1);
                last_frame = now;

                // Only check if running first, then write to the signal
                if (*state.peek()).is_running() {
                    _running_frames += 1;
                    (*state.write()).update(dt);

                    #[cfg(feature = "web")]
                    // Adaptive frame rate
                    let delay = match dt {
                        x if x < 0.008 => Duration::from_millis(8),  // ~120fps
                        x if x < 0.016 => Duration::from_millis(16), // ~60fps
                        _ => Duration::from_millis(32),              // ~30fps
                    };

                    #[cfg(not(feature = "web"))]
                    let delay = match _running_frames {
                        // Higher frame rate for the first ~200 frames for smooth starts
                        0..=200 => Duration::from_micros(8333), // ~120fps
                        _ => match dt {
                            x if x < 0.005 => Duration::from_millis(8),  // ~120fps
                            x if x < 0.011 => Duration::from_millis(16), // ~60fps
                            _ => Duration::from_millis(33),              // ~30fps
                        },
                    };

                    Time::delay(delay).await;
                } else {
                    _running_frames = 0;
                    Time::delay(idle_poll_rate).await;
                }
            }
        });
    });

    state
}
