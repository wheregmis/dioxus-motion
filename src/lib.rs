use dioxus_hooks::{use_future, use_signal};
use dioxus_signals::{Readable, Signal, Writable};

pub use instant::Duration;

pub use platform::{MotionTime, TimeProvider};
use prelude::Tween;
use spring::Spring;

pub mod tween;

pub mod animations;
pub mod platform;
pub mod spring;

pub mod prelude {

    pub use crate::animations::{Color, Transform};
    pub use crate::spring::Spring;
    pub use crate::tween::Tween;
    pub use crate::use_animation;
    pub use crate::AnimationConfig;
    pub use crate::AnimationManager;
    pub use crate::AnimationMode;
    pub use crate::Duration;
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

pub trait AnimationManager<T: Copy + 'static>: Clone + Copy {
    fn new(initial: T) -> Self;
    fn animate_to(&mut self, target: T, config: AnimationConfig);
    fn update(&mut self, dt: f32) -> bool;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn stop(&mut self);
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationState<T: Copy + 'static> {
    current: T,
    target: T,
    initial: T,
    velocity: f32,
    config: AnimationConfig,
    running: bool,
    elapsed: Duration,
    current_loop: u32,
}

impl<T: Copy + Into<f32> + From<f32>> AnimationManager<T> for AnimationState<T> {
    fn new(initial: T) -> Self {
        Self {
            current: initial,
            target: initial,
            velocity: 0.0,
            config: AnimationConfig::default(),
            running: false,
            elapsed: Duration::default(),
            initial,
            current_loop: 0,
        }
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.target = target;
        self.config = config;
        self.running = true;
        self.elapsed = Duration::default();
        self.velocity = 0.0;
        self.current_loop = 0;
    }

    fn stop(&mut self) {
        self.running = false;
        self.current_loop = 0;
    }

    fn update(&mut self, dt: f32) -> bool {
        if !self.running {
            return false;
        }

        let completed = match self.config.mode {
            AnimationMode::Spring(spring) => {
                let current: f32 = self.current.into();
                let target: f32 = self.target.into();

                let next = update_spring(
                    current,
                    target,
                    &mut self.velocity,
                    &spring,
                    dt.min(0.064), // Cap dt to prevent large jumps
                );

                self.current = T::from(next);

                self.velocity.abs() < 0.01 && (next - target).abs() < 0.01
            }
            AnimationMode::Tween(tween) => {
                self.elapsed += Duration::from_secs_f32(dt);

                // Calculate progress based on actual duration
                let progress = (self.elapsed.as_micros() as f32
                    / tween.duration.as_micros() as f32)
                    .clamp(0.0, 1.0);

                let current: f32 = self.current.into();
                let target: f32 = self.target.into();

                // Apply easing function
                let next = (tween.easing)(progress, current, target - current, 1.0);

                self.current = T::from(next);

                // Check if we've reached the end of the duration
                progress >= 1.0
            }
        };

        if completed {
            match self.config.loop_mode.unwrap_or(LoopMode::None) {
                LoopMode::None => {
                    self.running = false;
                    false
                }
                LoopMode::Infinite => {
                    self.current = self.initial;
                    self.elapsed = Duration::default();
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
                        true
                    }
                }
            }
        } else {
            true
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
        self.velocity = 0.0;
        self.elapsed = Duration::default();
    }
}

fn update_spring(current: f32, target: f32, velocity: &mut f32, spring: &Spring, dt: f32) -> f32 {
    let force = spring.stiffness * (target - current);
    let damping = spring.damping * *velocity;
    let acceleration = (force - damping) / spring.mass;

    *velocity += acceleration * dt;
    current + *velocity * dt
}

#[derive(Clone, Copy)]
struct AnimationSignal<T: Copy + 'static>(Signal<AnimationState<T>>);

impl<T: Copy + Into<f32> + From<f32>> AnimationManager<T> for AnimationSignal<T> {
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

pub fn use_animation<T: Copy + Into<f32> + From<f32> + 'static>(
    initial: T,
) -> impl AnimationManager<T> {
    let mut state = AnimationSignal(use_signal(|| AnimationState::new(initial)));

    let mut running = use_signal(|| false);

    use_future(move || {
        async move {
            loop {
                let frame_delay = Duration::from_secs_f32(1.0 / 90.0);
                if !state.is_running() {
                    Time::delay(frame_delay).await;
                    continue;
                }
                Time::delay(frame_delay).await; // ~60fps

                if state.0.write().update(frame_delay.as_secs_f32()) {
                    // Force a rerender when animation updates
                    running.toggle();
                }
            }
        }
    });

    state
}
