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
//! ```rust
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

use animations::utils::{Animatable, AnimationMode};
use dioxus::prelude::*;
pub use instant::Duration;

pub mod animations;
pub mod transitions;

#[cfg(feature = "transitions")]
pub use dioxus_motion_transitions_macro;

pub use animations::platform::{MotionTime, TimeProvider};
use animations::spring::{Spring, SpringState};
use prelude::{AnimationConfig, LoopMode};

// Re-exports
pub mod prelude {
    pub use crate::animations::colors::Color;
    pub use crate::animations::spring::Spring;
    pub use crate::animations::transform::Transform;
    pub use crate::animations::tween::Tween;
    pub use crate::animations::utils::AnimationConfig;
    pub use crate::animations::utils::AnimationMode;
    pub use crate::animations::utils::LoopMode;
    #[cfg(feature = "transitions")]
    pub use crate::dioxus_motion_transitions_macro::MotionTransitions;
    #[cfg(feature = "transitions")]
    pub use crate::transitions::page_transitions::AnimatableRoute;
    #[cfg(feature = "transitions")]
    pub use crate::transitions::page_transitions::AnimatedOutlet;
    #[cfg(feature = "transitions")]
    pub use crate::transitions::utils::TransitionVariant;
    pub use crate::use_motion;
    pub use crate::AnimationManager;
    pub use crate::AnimationSequence;
    pub use crate::Duration;
    pub use crate::Time;
    pub use crate::TimeProvider;
}

pub type Time = MotionTime;

/// Animation sequence that can chain multiple animations together
pub struct AnimationSequence<T: Animatable> {
    steps: Vec<AnimationStep<T>>,
    current_step: u8,
    on_complete: Option<Box<dyn FnOnce()>>,
}

#[derive(Clone)]
pub struct AnimationStep<T: Animatable> {
    target: T,
    config: AnimationConfig,
}

impl<T: Animatable> Default for AnimationSequence<T> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            on_complete: None,
        }
    }
}

impl<T: Animatable> AnimationSequence<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn then(mut self, target: T, config: AnimationConfig) -> Self {
        self.steps.push(AnimationStep { target, config });
        self
    }

    pub fn on_complete<F: FnOnce() + 'static>(mut self, f: F) -> Self {
        self.on_complete = Some(Box::new(f));
        self
    }
}

/// Internal state for an animation
pub struct AnimationState<T: Animatable> {
    current: T,
    target: T,
    initial: T,
    velocity: T,
    config: AnimationConfig,
    running: bool,
    elapsed: Duration,
    delay_elapsed: Duration,
    current_loop: u8,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Animatable> AnimationState<T> {
    fn new(initial: T) -> Self {
        Self {
            current: initial,
            target: initial,
            initial,
            velocity: T::zero(),
            config: AnimationConfig::default(),
            running: false,
            elapsed: Duration::ZERO,
            delay_elapsed: Duration::ZERO,
            current_loop: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.initial = self.current;
        self.target = target;
        self.config = config;
        self.running = true;
        self.elapsed = Duration::ZERO;
        self.delay_elapsed = Duration::ZERO;
        self.velocity = T::zero();
        self.current_loop = 0;
    }

    fn stop(&mut self) {
        self.running = false;
        self.current_loop = 0;
        self.velocity = T::zero();
    }

    fn update(&mut self, dt: f32) -> bool {
        if !self.running {
            return false;
        }

        if self.delay_elapsed < self.config.delay {
            self.delay_elapsed += Duration::from_secs_f32(dt);
            return true;
        }

        let completed = match self.config.mode {
            AnimationMode::Spring(spring) => {
                let spring_result = self.update_spring(spring, dt);
                match spring_result {
                    SpringState::Active => false,
                    SpringState::Completed => true,
                }
            }
            AnimationMode::Tween(tween) => {
                self.elapsed += Duration::from_secs_f32(dt);
                #[allow(clippy::float_arithmetic)]
                let progress =
                    (self.elapsed.as_secs_f32() / tween.duration.as_secs_f32()).clamp(0.0, 1.0);

                let eased_progress = (tween.easing)(progress, 0.0, 1.0, 1.0);
                self.current = self.initial.interpolate(&self.target, eased_progress);

                progress >= 1.0
            }
        };

        if completed {
            self.handle_completion()
        } else {
            true
        }
    }

    fn update_spring(&mut self, spring: Spring, dt: f32) -> SpringState {
        let dt = dt.min(0.064);

        // Cache intermediate calculations
        let delta = self.target.sub(&self.current);
        let delta_magnitude = delta.magnitude();

        // Early exit if we're close enough to target
        if delta_magnitude < T::epsilon() && self.velocity.magnitude() < T::epsilon() {
            self.current = self.target;
            return SpringState::Completed;
        }

        // Optimize force calculation
        let force = delta.scale(spring.stiffness);
        let damping = self.velocity.scale(spring.damping);
        let acceleration = force.sub(&damping).scale(1.0 / spring.mass);

        // Update velocity and position
        self.velocity = self.velocity.add(&acceleration.scale(dt));
        self.current = self.current.add(&self.velocity.scale(dt));

        // Check completion with cached delta magnitude
        if self.velocity.magnitude() < T::epsilon() && delta_magnitude < T::epsilon() {
            self.current = self.target;
            SpringState::Completed
        } else {
            SpringState::Active
        }
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

    fn is_running(&self) -> bool {
        self.running
    }

    fn reset(&mut self) {
        self.running = false;
        self.velocity = T::zero();
        self.elapsed = Duration::default();
        self.current = self.initial;
    }
}

/// Combined Animation Manager trait
pub trait AnimationManager<T: Animatable>: Clone + Copy {
    fn new(initial: T) -> Self;
    fn animate_to(&mut self, target: T, config: AnimationConfig);
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>);
    fn update(&mut self, dt: f32) -> bool;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn stop(&mut self);
    fn delay(&mut self, duration: Duration);
}

#[derive(Clone, Copy)]
struct AnimationSignal<T: Animatable>(Signal<AnimationState<T>>);

impl<T: Animatable> AnimationManager<T> for AnimationSignal<T> {
    fn new(initial: T) -> Self {
        Self(Signal::new(AnimationState::new(initial)))
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.0.write().animate_to(target, config);
    }

    fn animate_sequence(&mut self, _sequence: AnimationSequence<T>) {
        // No-op for base AnimationSignal
    }

    fn update(&mut self, dt: f32) -> bool {
        self.0.write().update(dt)
    }

    fn get_value(&self) -> T {
        self.0.read().get_value()
    }

    fn is_running(&self) -> bool {
        self.0.read().is_running()
    }

    fn reset(&mut self) {
        self.0.write().reset()
    }

    fn stop(&mut self) {
        self.0.write().stop()
    }

    fn delay(&mut self, duration: Duration) {
        self.0.write().config.delay = duration;
    }
}

#[derive(Clone, Copy)]
pub struct MotionState<T: Animatable> {
    base: AnimationSignal<T>,
    sequence: Signal<Option<SequenceState<T>>>,
}

struct SequenceState<T: Animatable> {
    sequence: AnimationSequence<T>,
    _current_value: T,
}

impl<T: Animatable> MotionState<T> {
    fn new(initial: T) -> Self {
        Self {
            base: AnimationSignal::new(initial),
            sequence: Signal::new(None),
        }
    }
}

impl<T: Animatable> AnimationManager<T> for MotionState<T> {
    fn new(initial: T) -> Self {
        Self::new(initial)
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.sequence.set(None);
        self.base.animate_to(target, config);
    }

    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        if sequence.steps.is_empty() {
            return;
        }
        let first_step = &sequence.steps[0];
        self.base
            .animate_to(first_step.target, first_step.config.clone());
        self.sequence.set(Some(SequenceState {
            sequence,
            _current_value: self.base.get_value(),
        }));
    }

    fn update(&mut self, dt: f32) -> bool {
        let mut still_animating = false;
        let mut should_clear_sequence = false;

        if let Some(sequence_state) = &mut *self.sequence.write() {
            let current_step = sequence_state.sequence.current_step;
            let total_steps = sequence_state.sequence.steps.len();

            if !self.base.is_running() {
                match current_step.cmp(&(total_steps as u8 - 1)) {
                    std::cmp::Ordering::Less => {
                        sequence_state.sequence.current_step += 1;
                        let step = &sequence_state.sequence.steps[(current_step + 1) as usize];
                        self.base.animate_to(step.target, step.config.clone());
                        still_animating = true;
                    }
                    std::cmp::Ordering::Equal => {
                        if let Some(on_complete) = sequence_state.sequence.on_complete.take() {
                            on_complete();
                        }
                        should_clear_sequence = true;
                        still_animating = false;
                        self.base.stop();
                    }
                    std::cmp::Ordering::Greater => {}
                }
            } else {
                still_animating = true;
            }
        }

        if should_clear_sequence {
            self.sequence.set(None);
        }

        still_animating |= self.base.update(dt);
        still_animating
    }

    fn get_value(&self) -> T {
        self.base.get_value()
    }

    fn is_running(&self) -> bool {
        self.base.is_running() || self.sequence.read().is_some()
    }

    fn reset(&mut self) {
        self.sequence.set(None);
        self.base.reset();
    }

    fn stop(&mut self) {
        self.sequence.set(None);
        self.base.stop();
    }

    fn delay(&mut self, duration: Duration) {
        self.base.delay(duration);
    }
}

// Signal wrapper implementations
impl<T: Animatable> AnimationManager<T> for Signal<MotionState<T>> {
    fn new(initial: T) -> Self {
        Signal::new(MotionState::new(initial))
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.write().animate_to(target, config);
    }

    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.write().animate_sequence(sequence);
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
        self.write().delay(duration);
    }
}

pub fn use_motion<T: Animatable>(initial: T) -> impl AnimationManager<T> {
    let mut state = use_signal(|| MotionState::new(initial));
    let mut last_frame = Time::now();
    let mut frame_count = 0;
    let mut last_fps_update = Time::now();

    use_future(move || async move {
        loop {
            let now = Time::now();
            let dt = now.duration_since(last_frame).as_secs_f32();

            if state.read().is_running() {
                state.write().update(dt);

                // Adaptive frame timing based on FPS
                frame_count += 1;
                if now.duration_since(last_fps_update).as_secs_f32() >= 1.0 {
                    let fps = frame_count as f32;
                    frame_count = 0;
                    last_fps_update = now;

                    // Adjust delay based on FPS and dt
                    let delay = if fps < 30.0 || dt > 0.15 {
                        Duration::from_millis(16) // ~60fps
                    } else if fps < 45.0 {
                        Duration::from_millis(24) // ~40fps
                    } else {
                        Duration::from_millis(32) // ~30fps
                    };

                    Time::delay(delay).await;
                } else {
                    // Use shorter delay for smoother animations
                    Time::delay(Duration::from_millis(16)).await;
                }
            } else {
                // Longer delay when no animations are running
                Time::delay(Duration::from_millis(100)).await;
            }

            last_frame = now;
        }
    });

    state
}
