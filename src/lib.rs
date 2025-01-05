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
    pub use crate::AnimationManager;
    pub use crate::Duration;
    pub use crate::Time;
    pub use crate::TimeProvider;
}

pub type Time = MotionTime;

pub trait AnimationManager<T: Animatable>: Clone + Copy {
    fn new(initial: T) -> Self;
    fn animate_to(&mut self, target: T, config: AnimationConfig);
    fn update(&mut self, dt: f32) -> bool;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn stop(&mut self);
    fn delay(&mut self, duration: Duration); // Add delay function
}

pub struct AnimationState<T: Animatable> {
    current: T,
    target: T,
    initial: T,
    velocity: T, // Changed to generic type for better precision
    config: AnimationConfig,
    running: bool,
    elapsed: Duration,
    delay_elapsed: Duration, // Add delay tracking
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
            delay_elapsed: Duration::default(), // Initialize delay tracking
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
        self.delay_elapsed = Duration::default(); // Reset delay tracking
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

        // Handle initial delay
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
            if let Some(ref mut f) = self.config.on_complete {
                f();
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
pub fn use_motion<T: Animatable>(initial: T) -> impl AnimationManager<T> {
    let mut state = AnimationSignal(use_signal(|| AnimationState::new(initial)));

    const TARGET_FPS: f32 = 90.0; // Until we have dynamic frame rate support
    let frame_time: Duration = Duration::from_secs_f32(1.0 / TARGET_FPS);

    const IDLE_DELAY: Duration = Duration::from_millis(70);

    use_future(move || async move {
        let mut last_frame = Time::now();

        loop {
            let frame_start = Time::now();
            let dt = frame_start.duration_since(last_frame).as_secs_f32();
            last_frame = frame_start;

            if !state.is_running() {
                Time::delay(IDLE_DELAY).await;
                continue;
            }

            if state.update(dt) {
                Time::delay(frame_time).await;
            }

            Time::delay(IDLE_DELAY).await;
        }
    });

    state
}
