use dioxus_hooks::{use_future, use_signal};
use dioxus_signals::{Readable, Signal, Writable};
pub use instant::Duration;

pub mod animations;
pub mod platform;
pub mod spring;
pub mod tween;

pub use platform::{MotionTime, TimeProvider};
use prelude::Tween;
use spring::Spring;

// Re-exports
pub mod prelude {
    pub use crate::animations::{Color, Transform};
    pub use crate::spring::Spring;
    pub use crate::tween::Tween;
    pub use crate::use_animation;
    pub use crate::AnimationConfig;
    pub use crate::AnimationManager;
    pub use crate::AnimationMode;
    pub use crate::Duration;
    pub use crate::LoopMode;
    pub use crate::Time;
    pub use crate::TimeProvider;
}

pub type Time = MotionTime;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationMode {
    Tween(Tween),
    Spring(Spring),
}

impl Default for AnimationMode {
    fn default() -> Self {
        Self::Tween(Tween::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopMode {
    None,
    Infinite,
    Times(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimationConfig {
    pub mode: AnimationMode,
    pub loop_mode: Option<LoopMode>,
}

impl AnimationConfig {
    pub fn new(mode: AnimationMode) -> Self {
        Self {
            mode,
            loop_mode: None,
        }
    }

    pub fn with_loop(mut self, loop_mode: LoopMode) -> Self {
        self.loop_mode = Some(loop_mode);
        self
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            mode: AnimationMode::default(),
            loop_mode: None,
        }
    }
}

pub trait AnimationManager<T: Animatable>: Clone + Copy {
    fn new(initial: T) -> Self;
    fn animate_to(&mut self, target: T, config: AnimationConfig);
    fn update(&mut self, dt: f32) -> bool;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn stop(&mut self);
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationState<T: Animatable> {
    current: T,
    target: T,
    initial: T,
    velocity: T, // Changed to generic type for better precision
    config: AnimationConfig,
    running: bool,
    elapsed: Duration,
    current_loop: u32,
}

impl<T: Animatable> AnimationState<T> {
    fn new(initial: T) -> Self {
        Self {
            current: initial,
            target: initial,
            velocity: T::zero(), // New method required in Animatable trait
            config: AnimationConfig::default(),
            running: false,
            elapsed: Duration::default(),
            initial,
            current_loop: 0,
        }
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.initial = self.current;
        self.target = target;
        self.config = config;
        self.running = true;
        self.elapsed = Duration::default();
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
        // Limit dt to prevent instability
        let dt = dt.min(0.064);

        let force = self.target.sub(&self.current).scale(spring.stiffness);
        let damping = self.velocity.scale(spring.damping);
        let acceleration = force.sub(&damping).scale(1.0 / spring.mass);

        self.velocity = self.velocity.add(&acceleration.scale(dt));
        self.current = self.current.add(&self.velocity.scale(dt));

        // Check for completion
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
        match self.config.loop_mode.unwrap_or(LoopMode::None) {
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
        }
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
}

pub fn use_animation<T: Animatable>(initial: T) -> impl AnimationManager<T> {
    let mut state = AnimationSignal(use_signal(|| AnimationState::new(initial)));
    let mut running = use_signal(|| false);

    use_future(move || async move {
        loop {
            let frame_delay = Duration::from_secs_f32(1.0 / 90.0);
            if !state.is_running() {
                Time::delay(frame_delay).await;
                continue;
            }

            Time::delay(frame_delay).await;

            if state.update(frame_delay.as_secs_f32()) {
                running.toggle();
            }
        }
    });

    state
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SpringState {
    Active,
    Completed,
}

// Required trait extension
pub trait Animatable: Copy + 'static {
    fn zero() -> Self;
    fn epsilon() -> f32;
    fn magnitude(&self) -> f32;
    fn scale(&self, factor: f32) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn interpolate(&self, target: &Self, t: f32) -> Self;
}
