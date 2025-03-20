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
use std::sync::Arc;

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
#[derive(Clone)]
pub struct AnimationSequence<T: Animatable> {
    steps: Vec<AnimationStep<T>>,
    current_step: u8,
    on_complete: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
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

    pub fn on_complete<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_complete = Some(Arc::new(f));
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
        let dt = dt.min(0.064); // Cap maximum time step for stability

        // Pre-calculate common values
        let delta = self.target.sub(&self.current);
        let delta_magnitude = delta.magnitude();
        let velocity_magnitude = self.velocity.magnitude();

        // Early exit conditions with epsilon comparison
        let epsilon = T::epsilon();
        if delta_magnitude < epsilon && velocity_magnitude < epsilon {
            self.current = self.target;
            self.velocity = T::zero();
            return SpringState::Completed;
        }

        // Semi-implicit Euler integration for better stability
        // Calculate acceleration first
        let force = delta.scale(spring.stiffness);
        let damping = self.velocity.scale(spring.damping);
        let acceleration = force.sub(&damping).scale(1.0 / spring.mass);

        // Update velocity first
        self.velocity = self.velocity.add(&acceleration.scale(dt));

        // Then update position using new velocity
        self.current = self.current.add(&self.velocity.scale(dt));

        // Check for completion
        let new_delta = self.target.sub(&self.current);
        let new_delta_magnitude = new_delta.magnitude();

        if self.velocity.magnitude() < epsilon && new_delta_magnitude < epsilon {
            self.current = self.target;
            self.velocity = T::zero();
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
    pending_updates: Signal<Vec<PendingUpdate<T>>>,
}

#[derive(Clone)]
enum PendingUpdate<T: Animatable> {
    AnimateTo(T, AnimationConfig),
    Sequence(AnimationSequence<T>),
    Stop,
    Reset,
    Delay(Duration),
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
            pending_updates: Signal::new(Vec::new()),
        }
    }

    fn process_pending_updates(&mut self) {
        let mut updates = self.pending_updates.write();
        for update in updates.drain(..) {
            match update {
                PendingUpdate::AnimateTo(target, config) => {
                    self.sequence.set(None);
                    self.base.0.write().animate_to(target, config);
                }
                PendingUpdate::Sequence(sequence) => {
                    if !sequence.steps.is_empty() {
                        let mut optimized_steps = Vec::with_capacity(sequence.steps.len());
                        optimized_steps.extend(sequence.steps.into_iter());

                        let first_step = &optimized_steps[0];
                        self.base
                            .0
                            .write()
                            .animate_to(first_step.target, first_step.config.clone());

                        self.sequence.set(Some(SequenceState {
                            sequence: AnimationSequence {
                                steps: optimized_steps,
                                current_step: 0,
                                on_complete: sequence.on_complete,
                            },
                            _current_value: self.base.0.read().get_value(),
                        }));
                    }
                }
                PendingUpdate::Stop => {
                    self.sequence.set(None);
                    self.base.0.write().stop();
                }
                PendingUpdate::Reset => {
                    self.sequence.set(None);
                    self.base.0.write().reset();
                }
                PendingUpdate::Delay(duration) => {
                    self.base.0.write().config.delay = duration;
                }
            }
        }
    }
}

impl<T: Animatable> AnimationManager<T> for MotionState<T> {
    fn new(initial: T) -> Self {
        Self::new(initial)
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.pending_updates
            .write()
            .push(PendingUpdate::AnimateTo(target, config));
    }

    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.pending_updates
            .write()
            .push(PendingUpdate::Sequence(sequence));
    }

    fn update(&mut self, dt: f32) -> bool {
        self.process_pending_updates();

        let mut still_animating = false;
        let mut should_clear_sequence = false;

        if let Some(sequence_state) = &mut *self.sequence.write() {
            let current_step = sequence_state.sequence.current_step;
            let total_steps = sequence_state.sequence.steps.len();

            if !self.base.is_running() {
                match current_step.cmp(&(total_steps as u8 - 1)) {
                    std::cmp::Ordering::Less => {
                        sequence_state.sequence.current_step += 1;
                        let next_step = &sequence_state.sequence.steps[(current_step + 1) as usize];

                        let current_value = self.base.get_value();
                        if current_value.sub(&next_step.target).magnitude() > T::epsilon() {
                            self.pending_updates.write().push(PendingUpdate::AnimateTo(
                                next_step.target,
                                next_step.config.clone(),
                            ));
                            still_animating = true;
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        if let Some(on_complete) = sequence_state.sequence.on_complete.take() {
                            on_complete();
                        }
                        should_clear_sequence = true;
                        still_animating = false;
                        self.pending_updates.write().push(PendingUpdate::Stop);
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
        !self.pending_updates.read().is_empty()
            || self.base.is_running()
            || self.sequence.read().is_some()
    }

    fn reset(&mut self) {
        self.pending_updates.write().push(PendingUpdate::Reset);
    }

    fn stop(&mut self) {
        self.pending_updates.write().push(PendingUpdate::Stop);
    }

    fn delay(&mut self, duration: Duration) {
        self.pending_updates
            .write()
            .push(PendingUpdate::Delay(duration));
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

    use_future(move || async move {
        let mut last_frame = Time::now();
        let mut frame_times = [0.016f32; 10];
        let mut frame_index = 0;

        loop {
            let now = Time::now();
            let dt = now.duration_since(last_frame).as_secs_f32();

            frame_times[frame_index] = dt;
            frame_index = (frame_index + 1) % frame_times.len();

            let avg_frame_time = frame_times.iter().sum::<f32>() / frame_times.len() as f32;

            if state.read().is_running() {
                state.write().update(dt);

                let delay = if avg_frame_time > 0.032 {
                    Duration::from_millis(32)
                } else if avg_frame_time > 0.016 {
                    Duration::from_millis(16)
                } else {
                    Duration::from_millis(8)
                };

                Time::delay(delay).await;
            } else {
                Time::delay(Duration::from_millis(100)).await;
            }

            last_frame = now;
        }
    });

    state
}
