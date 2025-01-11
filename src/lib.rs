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

use animations::{Animatable, AnimationMode};
use dioxus_hooks::{use_future, use_signal};
use dioxus_signals::{Readable, Signal, Writable};
pub use instant::Duration;

pub mod animations;
pub mod colors;
pub mod platform;
pub mod spring;
pub mod transform;
pub mod tween;

pub use platform::{MotionTime, TimeProvider};
use prelude::{AnimationConfig, LoopMode};
use spring::{Spring, SpringState};

// Re-exports
pub mod prelude {
    pub use crate::animations::AnimationConfig;
    pub use crate::animations::AnimationMode;
    pub use crate::animations::LoopMode;
    pub use crate::colors::Color;
    pub use crate::spring::Spring;
    pub use crate::transform::Transform;
    pub use crate::tween::Tween;
    pub use crate::use_motion;
    pub use crate::AnimationSequence;
    pub use crate::Duration;
    pub use crate::Time;
    pub use crate::TimeProvider;
    pub use crate::{AnimationManager, EnhancedAnimationManager};
}

pub type Time = MotionTime;

/// Animation sequence that can chain multiple animations together
pub struct AnimationSequence<T: Animatable> {
    steps: Vec<AnimationStep<T>>,
    current_step: usize,
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
    current_loop: u32,
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
            elapsed: Duration::default(),
            delay_elapsed: Duration::default(),
            current_loop: 0,
        }
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.initial = self.current;
        self.target = target;
        self.config = config;
        self.running = true;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
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

        let force = self.target.sub(&self.current).scale(spring.stiffness);
        let damping = self.velocity.scale(spring.damping);
        let acceleration = force.sub(&damping).scale(1.0 / spring.mass);

        self.velocity = self.velocity.add(&acceleration.scale(dt));
        self.current = self.current.add(&self.velocity.scale(dt));

        if self.velocity.magnitude() < T::epsilon()
            && self.current.sub(&self.target).magnitude() < T::epsilon()
        {
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

/// Trait for managing animations of a value
pub trait AnimationManager<T: Animatable>: Clone + Copy {
    fn new(initial: T) -> Self;
    fn animate_to(&mut self, target: T, config: AnimationConfig);
    fn update(&mut self, dt: f32) -> bool;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn stop(&mut self);
    fn delay(&mut self, duration: Duration);
}

/// Enhanced AnimationManager trait with sequence capabilities
pub trait EnhancedAnimationManager<T: Animatable>: AnimationManager<T> {
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>);
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
pub struct EnhancedMotionState<T: Animatable> {
    base: AnimationSignal<T>,
    sequence: Signal<Option<SequenceState<T>>>,
}

struct SequenceState<T: Animatable> {
    sequence: AnimationSequence<T>,
    current_value: T,
}

impl<T: Animatable> EnhancedMotionState<T> {
    fn new(initial: T) -> Self {
        Self {
            base: AnimationSignal::new(initial),
            sequence: Signal::new(None),
        }
    }
}

impl<T: Animatable> EnhancedAnimationManager<T> for EnhancedMotionState<T> {
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        if sequence.steps.is_empty() {
            return;
        }

        let first_step = &sequence.steps[0];
        self.base
            .animate_to(first_step.target, first_step.config.clone());

        self.sequence.set(Some(SequenceState {
            sequence,
            current_value: self.base.get_value(),
        }));
    }
}

impl<T: Animatable> AnimationManager<T> for EnhancedMotionState<T> {
    fn new(initial: T) -> Self {
        Self {
            base: AnimationSignal::new(initial),
            sequence: Signal::new(None),
        }
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.sequence.set(None);
        self.base.animate_to(target, config);
    }

    fn update(&mut self, dt: f32) -> bool {
        let mut still_animating = false;
        let mut should_clear_sequence = false;

        if let Some(sequence_state) = &mut *self.sequence.write() {
            let current_step = sequence_state.sequence.current_step;
            let total_steps = sequence_state.sequence.steps.len();

            if !self.base.is_running() {
                match current_step.cmp(&(total_steps - 1)) {
                    std::cmp::Ordering::Less => {
                        sequence_state.sequence.current_step += 1;
                        let step = &sequence_state.sequence.steps[current_step + 1];
                        self.base.animate_to(step.target, step.config.clone());
                        still_animating = true;
                    }
                    std::cmp::Ordering::Equal => {
                        if let Some(on_complete) = sequence_state.sequence.on_complete.take() {
                            on_complete();
                        }
                        should_clear_sequence = true;
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
impl<T: Animatable> EnhancedAnimationManager<T> for Signal<EnhancedMotionState<T>> {
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.write().animate_sequence(sequence);
    }
}

impl<T: Animatable> AnimationManager<T> for Signal<EnhancedMotionState<T>> {
    fn new(initial: T) -> Self {
        Signal::new(EnhancedMotionState::new(initial))
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.write().animate_to(target, config);
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

pub fn use_motion<T: Animatable>(initial: T) -> impl EnhancedAnimationManager<T> {
    let mut state = use_signal(|| EnhancedMotionState::new(initial));

    use_future(move || async move {
        let mut last_frame = Time::now();

        loop {
            let now = Time::now();
            let dt = now.duration_since(last_frame).as_secs_f32();

            if state.read().is_running() {
                state.write().update(dt);
                Time::delay(Duration::from_millis(16)).await;
            } else {
                Time::delay(Duration::from_millis(100)).await;
            }

            last_frame = now;
        }
    });

    state
}
