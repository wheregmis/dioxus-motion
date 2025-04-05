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
#![deny(clippy::option_if_let_else)] // Prefer map/and_then

use std::{cell::RefCell, sync::Arc};

use animations::utils::{Animatable, AnimationMode};
use dioxus::prelude::*;
pub use instant::Duration;

pub mod animations;
pub mod transitions;

#[cfg(feature = "transitions")]
pub use dioxus_motion_transitions_macro;

pub use animations::platform::{MotionTime, TimeProvider};
use animations::spring::{Spring, SpringState};
use prelude::{AnimationConfig, LoopMode, Transform, Tween};
use smallvec::SmallVec;

// Re-exports
pub mod prelude {
    pub use crate::animations::utils::{AnimationConfig, AnimationMode, LoopMode};
    pub use crate::animations::{
        colors::Color, spring::Spring, transform::Transform, tween::Tween,
    };
    #[cfg(feature = "transitions")]
    pub use crate::dioxus_motion_transitions_macro::MotionTransitions;
    #[cfg(feature = "transitions")]
    pub use crate::transitions::page_transitions::{AnimatableRoute, AnimatedOutlet};
    #[cfg(feature = "transitions")]
    pub use crate::transitions::utils::TransitionVariant;
    pub use crate::{
        use_motion, AnimationManager, AnimationSequence, Duration, Time, TimeProvider,
    };
}

pub type Time = MotionTime;

#[derive(Clone)]
#[allow(dead_code)]
struct AnimationStep<T: Animatable> {
    target: T,
    config: Arc<AnimationConfig>,
    // Add predicted next state for smoother transitions
    predicted_next: Option<T>,
}

// Use a static array instead of Vec for small sequences
type AnimationSteps<T> = SmallVec<[AnimationStep<T>; 8]>;

pub struct AnimationSequence<T: Animatable> {
    steps: AnimationSteps<T>,
    current_step: u8,
    on_complete: Option<Box<dyn FnOnce()>>,
    // Add capacity hint for better allocation
    capacity_hint: u8,
}

impl<T: Animatable> Clone for AnimationSequence<T> {
    fn clone(&self) -> Self {
        Self {
            steps: self.steps.clone(),
            current_step: self.current_step,
            on_complete: None,
            capacity_hint: self.capacity_hint,
        }
    }
}

impl<T: Animatable> AnimationSequence<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: u8) -> Self {
        Self {
            steps: SmallVec::with_capacity(capacity as usize),
            current_step: 0,
            on_complete: None,
            capacity_hint: capacity,
        }
    }

    // Add method to reserve space upfront
    pub fn reserve(&mut self, additional: u8) {
        self.steps.reserve(additional as usize);
    }

    pub fn then(mut self, target: T, config: AnimationConfig) -> Self {
        let predicted_next = self
            .steps
            .last()
            .map(|last_step| last_step.target.interpolate(&target, 0.5));

        self.steps.push(AnimationStep {
            target,
            config: Arc::new(config),
            predicted_next,
        });
        self
    }

    pub fn on_complete<F: FnOnce() + 'static>(mut self, f: F) -> Self {
        self.on_complete = Some(Box::new(f));
        self
    }
}

impl<T: Animatable> Default for AnimationSequence<T> {
    fn default() -> Self {
        Self {
            steps: AnimationSteps::new(),
            current_step: 0,
            on_complete: None,
            capacity_hint: 0,
        }
    }
}

#[derive(Clone)]
pub struct Motion<T: Animatable> {
    initial: T,
    current: T,
    target: T,
    velocity: T,
    running: bool,
    elapsed: Duration,
    delay_elapsed: Duration, // Add this field
    current_loop: u8,
    config: Arc<AnimationConfig>,
    sequence: Option<Arc<AnimationSequence<T>>>,
    reverse: bool, // New field to track direction for alternating animations
    keyframe_animation: Option<Arc<KeyframeAnimation<T>>>,
}

impl<T: Animatable> Motion<T> {
    pub fn new(initial: T) -> Self {
        Self {
            initial,
            current: initial,
            target: initial,
            velocity: T::zero(),
            running: false,
            elapsed: Duration::default(),
            current_loop: 0,
            config: Arc::new(AnimationConfig::default()),
            sequence: None,
            reverse: false,
            delay_elapsed: Duration::default(),
            keyframe_animation: None,
        }
    }

    pub fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.sequence = None;
        self.initial = self.current;
        self.target = target;
        self.config = Arc::new(config);
        self.running = true;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::zero();
        self.current_loop = 0;
    }

    pub fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        if let Some(first_step) = sequence.steps.first() {
            // This approach doesn't correctly initialize the sequence state
            self.animate_to(first_step.target, (*first_step.config).clone());

            // Start with current_step as 0 instead of -1 to fix indexing
            let mut new_sequence = sequence;
            new_sequence.current_step = 0;
            self.sequence = Some(Arc::new(new_sequence));
        }
    }

    pub fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        self.keyframe_animation = Some(Arc::new(animation));
        self.running = true;
        self.elapsed = Duration::default();
        self.velocity = T::zero();
    }

    pub fn value(&self) -> T {
        self.current
    }

    pub fn is_running(&self) -> bool {
        self.running || self.sequence.is_some() || self.keyframe_animation.is_some()
    }

    pub fn reset(&mut self) {
        self.stop();
        self.current = self.initial;
        self.elapsed = Duration::default();
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.current_loop = 0;
        self.velocity = T::zero();
        self.sequence = None;
        self.keyframe_animation = None;
    }

    pub fn delay(&mut self, duration: Duration) {
        let mut config = (*self.config).clone();
        config.delay = duration;
        self.config = Arc::new(config);
    }

    fn update(&mut self, dt: f32) -> bool {
        if !self.running && self.sequence.is_none() && self.keyframe_animation.is_none() {
            return false;
        }

        if let Some(sequence) = &self.sequence {
            if !self.running {
                // Current animation has completed, move to next step
                let current_step = sequence.current_step;
                let total_steps = sequence.steps.len();

                // Check if there are more steps to animate
                if current_step < (total_steps - 1) as u8 {
                    // Changed this condition
                    // Move to the next step
                    let mut new_sequence = (**sequence).clone();
                    new_sequence.current_step = current_step + 1;
                    let next_step = current_step + 1;

                    // Get the next step
                    let step = &sequence.steps[next_step as usize];
                    let target = step.target;
                    let config = (*step.config).clone();
                    self.sequence = Some(Arc::new(new_sequence));

                    // Start the next animation
                    self.initial = self.current; // Start from current position
                    self.target = target;
                    self.config = Arc::new(config);
                    self.running = true;
                    self.elapsed = Duration::default();
                    self.delay_elapsed = Duration::default();
                    self.velocity = T::zero();

                    return true;
                } else {
                    // Sequence complete - we've reached the last step
                    let mut sequence_clone = (**sequence).clone();
                    if let Some(on_complete) = sequence_clone.on_complete.take() {
                        on_complete();
                    }
                    self.sequence = None;
                    self.stop();
                    return false;
                }
            }
        }

        if let Some(_animation) = &self.keyframe_animation {
            return self.update_keyframes(dt);
        }

        // Skip updates for imperceptible changes
        const MIN_DELTA: f32 = 1.0 / 240.0; // ~4ms
        if dt < MIN_DELTA {
            return true;
        }

        if self.delay_elapsed < self.config.delay {
            self.delay_elapsed += Duration::from_secs_f32(dt);
            return true;
        }

        let completed = match self.config.mode {
            AnimationMode::Spring(spring) => {
                let spring_result = self.update_spring(spring, dt);
                matches!(spring_result, SpringState::Completed)
            }
            AnimationMode::Tween(tween) => self.update_tween(tween, dt),
        };

        if completed {
            self.handle_completion()
        } else {
            true
        }
    }

    #[cfg(feature = "web")]
    fn update_spring(&mut self, spring: Spring, dt: f32) -> SpringState {
        const VELOCITY_THRESHOLD: f32 = 0.001;
        const POSITION_THRESHOLD: f32 = 0.001;

        // Cache frequently accessed values
        let stiffness = spring.stiffness;
        let damping = spring.damping;
        let mass_inv = 1.0 / spring.mass;

        // Use fixed timestep for better stability
        const FIXED_DT: f32 = 1.0 / 120.0;
        let steps = ((dt / FIXED_DT) as usize).max(1);
        let step_dt = dt / steps as f32;

        for _ in 0..steps {
            let delta = self.target.sub(&self.current);

            // Early exit if movement is negligible
            if delta.magnitude() < POSITION_THRESHOLD
                && self.velocity.magnitude() < VELOCITY_THRESHOLD
            {
                self.current = self.target;
                self.velocity = T::zero();
                return SpringState::Completed;
            }

            let force = delta.scale(stiffness);
            let damping_force = self.velocity.scale(damping);

            // Fused multiply-add for better performance
            self.velocity = self
                .velocity
                .add(&(force.sub(&damping_force)).scale(mass_inv * step_dt));
            self.current = self.current.add(&self.velocity.scale(step_dt));
        }

        self.check_spring_completion()
    }

    #[cfg(not(feature = "web"))]
    fn update_spring(&mut self, spring: Spring, dt: f32) -> SpringState {
        // RK4 integration for better accuracy
        let stiffness = spring.stiffness;
        let damping = spring.damping;
        let mass_inv = 1.0 / spring.mass;

        // State vector: [position, velocity]
        struct State<T> {
            pos: T,
            vel: T,
        }

        // Compute derivatives for RK4
        let derive = |state: &State<T>| -> State<T> {
            let delta = self.target.sub(&state.pos);
            let force = delta.scale(stiffness);
            let damping_force = state.vel.scale(damping);
            let acc = (force.sub(&damping_force)).scale(mass_inv);

            State {
                pos: state.vel.clone(),
                vel: acc,
            }
        };

        let mut state = State {
            pos: self.current.clone(),
            vel: self.velocity.clone(),
        };

        // Perform RK4 integration
        let k1 = derive(&state);
        let k2 = derive(&State {
            pos: state.pos.add(&k1.pos.scale(dt * 0.5)),
            vel: state.vel.add(&k1.vel.scale(dt * 0.5)),
        });
        let k3 = derive(&State {
            pos: state.pos.add(&k2.pos.scale(dt * 0.5)),
            vel: state.vel.add(&k2.vel.scale(dt * 0.5)),
        });
        let k4 = derive(&State {
            pos: state.pos.add(&k3.pos.scale(dt)),
            vel: state.vel.add(&k3.vel.scale(dt)),
        });

        const SIXTH: f32 = 1.0 / 6.0;

        // Update position and velocity
        self.current = state.pos.add(
            &(k1.pos
                .add(&k2.pos.scale(2.0))
                .add(&k3.pos.scale(2.0))
                .add(&k4.pos))
            .scale(dt * SIXTH),
        );

        self.velocity = state.vel.add(
            &(k1.vel
                .add(&k2.vel.scale(2.0))
                .add(&k3.vel.scale(2.0))
                .add(&k4.vel))
            .scale(dt * SIXTH),
        );

        self.check_spring_completion()
    }

    // Helper method for spring completion check (shared between both implementations)
    #[inline(always)]
    fn check_spring_completion(&mut self) -> SpringState {
        const EPSILON: f32 = 0.001;
        const EPSILON_SQ: f32 = EPSILON * EPSILON;

        let velocity_sq = self.velocity.magnitude().powi(2);
        let delta = self.target.sub(&self.current);
        let delta_sq = delta.magnitude().powi(2);

        if velocity_sq < EPSILON_SQ && delta_sq < EPSILON_SQ {
            self.current = self.target;
            self.velocity = T::zero();
            SpringState::Completed
        } else {
            SpringState::Active
        }
    }

    fn update_tween(&mut self, tween: Tween, dt: f32) -> bool {
        // Use raw float operations instead of Duration for better performance
        let elapsed_secs = self.elapsed.as_secs_f32() + dt;
        self.elapsed = Duration::from_secs_f32(elapsed_secs);

        // Avoid division by caching duration reciprocal
        let duration_secs = tween.duration.as_secs_f32();
        let progress = if duration_secs == 0.0 {
            1.0
        } else {
            (elapsed_secs * (1.0 / duration_secs)).min(1.0)
        };

        // Skip interpolation if we're at the start or end
        if progress <= 0.0 {
            self.current = self.initial;
            return false;
        } else if progress >= 1.0 {
            self.current = self.target;
            return true;
        }

        // Cache easing result and avoid unnecessary parameters
        let eased_progress = (tween.easing)(progress, 0.0, 1.0, 1.0);

        // Fast path for common cases
        match eased_progress {
            0.0 => self.current = self.initial,
            1.0 => self.current = self.target,
            _ => self.current = self.initial.interpolate(&self.target, eased_progress),
        }

        progress >= 1.0
    }

    fn handle_completion(&mut self) -> bool {
        let should_continue = match self.config.loop_mode.unwrap_or(LoopMode::None) {
            LoopMode::None => {
                self.running = false;
                false
            }
            LoopMode::Infinite => {
                self.current = self.initial;
                self.elapsed = Duration::default();
                self.velocity = T::zero();
                true
            }
            LoopMode::Times(count) => {
                self.current_loop += 1;
                if self.current_loop >= count {
                    self.stop();
                    false
                } else {
                    self.current = self.initial;
                    self.elapsed = Duration::default();
                    self.velocity = T::zero();
                    true
                }
            }
            LoopMode::Alternate => {
                self.reverse = !self.reverse;
                if self.reverse {
                    std::mem::swap(&mut self.initial, &mut self.target);
                }
                self.elapsed = Duration::default();
                self.velocity = T::zero();
                true
            }
            LoopMode::AlternateTimes(count) => {
                self.current_loop += 1;
                if self.current_loop >= count * 2 {
                    self.stop();
                    false
                } else {
                    self.reverse = !self.reverse;
                    if self.reverse {
                        std::mem::swap(&mut self.initial, &mut self.target);
                    }
                    self.elapsed = Duration::default();
                    self.velocity = T::zero();
                    true
                }
            }
        };

        if !should_continue {
            if let Some(ref f) = self.config.on_complete {
                if let Ok(mut guard) = f.lock() {
                    guard();
                }
            }
        }

        should_continue
    }

    fn get_value(&self) -> T {
        self.current
    }

    fn update_keyframes(&mut self, dt: f32) -> bool {
        if let Some(animation) = &self.keyframe_animation {
            let progress =
                (self.elapsed.as_secs_f32() / animation.duration.as_secs_f32()).clamp(0.0, 1.0);

            // Find the current keyframe pair
            let (start, end) = animation
                .keyframes
                .windows(2)
                .find(|w| progress >= w[0].offset && progress <= w[1].offset)
                .map(|w| (&w[0], &w[1]))
                .unwrap_or_else(|| {
                    // Handle edge cases
                    if progress <= animation.keyframes[0].offset {
                        let first = &animation.keyframes[0];
                        (first, first)
                    } else {
                        let last = animation
                            .keyframes
                            .last()
                            .expect("Keyframe animation must contain at least one keyframe");
                        (last, last)
                    }
                });

            // Calculate local progress between keyframes
            let local_progress = if start.offset == end.offset {
                1.0
            } else {
                (progress - start.offset) / (end.offset - start.offset)
            };

            // Apply easing if specified
            let eased_progress = end
                .easing
                .map_or(local_progress, |ease| (ease)(local_progress, 0.0, 1.0, 1.0));

            // Interpolate between keyframes
            self.current = start.value.interpolate(&end.value, eased_progress);

            // Update elapsed time
            self.elapsed += Duration::from_secs_f32(dt);

            // Check for completion
            if progress >= 1.0 {
                self.handle_completion()
            } else {
                true
            }
        } else {
            false
        }
    }
}

/// Combined Animation Manager trait
pub trait AnimationManager<T: Animatable>: Clone + Copy {
    fn new(initial: T) -> Self;
    fn animate_to(&mut self, target: T, config: AnimationConfig);
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>);
    fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>);
    fn update(&mut self, dt: f32) -> bool;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn stop(&mut self);
    fn delay(&mut self, duration: Duration);
}

impl<T: Animatable> AnimationManager<T> for Signal<Motion<T>> {
    fn new(initial: T) -> Self {
        Signal::new(Motion::new(initial))
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.write().animate_to(target, config);
    }

    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        if let Some(first_step) = sequence.steps.first() {
            let mut state = self.write();
            state.animate_to(first_step.target, (*first_step.config).clone());
            state.sequence = Some(sequence.into());
        }
    }

    fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        self.write().animate_keyframes(animation);
    }

    fn update(&mut self, dt: f32) -> bool {
        self.write().update(dt)
    }

    fn get_value(&self) -> T {
        self.read().get_value()
    }

    fn is_running(&self) -> bool {
        self.read().is_running()
    }

    fn reset(&mut self) {
        self.write().reset();
    }

    fn stop(&mut self) {
        self.write().stop();
    }

    fn delay(&mut self, duration: Duration) {
        let mut state = self.write();
        let mut config = (*state.config).clone();
        config.delay = duration;
        state.config = Arc::new(config);
    }
}

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
                if state.peek().is_running() {
                    _running_frames += 1;
                    state.write().update(dt);

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

// Reuse allocations for common operations
thread_local! {
    static TRANSFORM_BUFFER: RefCell<Vec<Transform>> = RefCell::new(Vec::with_capacity(32));
    static SPRING_BUFFER: RefCell<Vec<SpringState>> = RefCell::new(Vec::with_capacity(16));
}

/// Represents a single keyframe in an animation
/// Represents a single keyframe in an animation
#[derive(Clone)]
pub struct Keyframe<T: Animatable> {
    /// The target value at this keyframe
    value: T,
    /// Timing as a percentage (0.0 to 1.0)
    offset: f32,
    /// Optional easing function for this specific keyframe
    easing: Option<fn(f32, f32, f32, f32) -> f32>,
}
/// Keyframe animation configuration
#[derive(Clone)]
pub struct KeyframeAnimation<T: Animatable> {
    keyframes: Vec<Keyframe<T>>,
    duration: Duration,
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
