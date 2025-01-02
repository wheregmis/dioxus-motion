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

pub trait AnimationManager<T: Copy + 'static>: Clone + Copy {
    fn new(initial: T) -> Self;
    fn animate_to(&mut self, target: T, mode: AnimationMode);
    fn update(&mut self, dt: f32) -> bool;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn loop_animation(&mut self);
    fn loop_times(&mut self, count: u32);
    fn stop_loop(&mut self);
    fn is_looping(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationState<T: Copy + 'static> {
    current: T,
    target: T,
    initial: T,
    velocity: f32,
    mode: AnimationMode,
    running: bool,
    elapsed: Duration,
    loop_count: Option<u32>,
    current_loop: u32,
    is_looping: bool,
}

impl<T: Copy + Into<f32> + From<f32>> AnimationManager<T> for AnimationState<T> {
    fn new(initial: T) -> Self {
        Self {
            current: initial,
            target: initial,
            velocity: 0.0,
            mode: AnimationMode::Spring(Spring::default()),
            running: false,
            elapsed: Duration::default(),
            initial,
            loop_count: None,
            current_loop: 0,
            is_looping: false,
        }
    }

    fn animate_to(&mut self, target: T, mode: AnimationMode) {
        self.target = target;
        self.mode = mode;
        self.running = true;
        self.elapsed = Duration::default();
    }

    fn loop_animation(&mut self) {
        self.is_looping = true;
        self.loop_count = None;
        self.animate_to(self.target, self.mode);
    }

    fn loop_times(&mut self, count: u32) {
        self.is_looping = true;
        self.loop_count = Some(count);
        self.current_loop = 0;
        self.animate_to(self.target, self.mode);
    }

    fn stop_loop(&mut self) {
        self.is_looping = false;
        self.loop_count = None;
        self.current_loop = 0;
    }

    fn is_looping(&self) -> bool {
        self.is_looping
    }

    fn update(&mut self, dt: f32) -> bool {
        if !self.running {
            if self.is_looping {
                if let Some(count) = self.loop_count {
                    if self.current_loop >= count {
                        self.stop_loop();
                        return false;
                    }
                    self.current_loop += 1;
                }
                self.animate_to(self.target, self.mode);
                return true;
            }
            return false;
        }

        match self.mode {
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

                if self.velocity.abs() < 0.01 && (next - target).abs() < 0.01 {
                    self.running = false;
                    self.current = self.target;
                    return false;
                }
            }
            AnimationMode::Tween(tween) => {
                self.elapsed += Duration::from_secs_f32(dt);
                let progress =
                    (self.elapsed.as_secs_f32() / tween.duration.as_secs_f32()).clamp(0.0, 1.0);

                let current: f32 = self.current.into();
                let target: f32 = self.target.into();

                let next = (tween.easing)(progress, current, target - current, 1.0);
                self.current = T::from(next);

                if progress >= 1.0 {
                    self.running = false;
                    self.current = self.target;
                    return false;
                }
            }
        }
        true
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

    fn animate_to(&mut self, target: T, mode: AnimationMode) {
        self.0.write().animate_to(target, mode);
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

    fn loop_animation(&mut self) {
        todo!()
    }

    fn loop_times(&mut self, count: u32) {
        todo!()
    }

    fn stop_loop(&mut self) {
        todo!()
    }

    fn is_looping(&self) -> bool {
        todo!()
    }
}

pub fn use_animation<T: Copy + Into<f32> + From<f32> + 'static>(
    initial: T,
) -> impl AnimationManager<T> {
    let state = AnimationSignal(use_signal(|| AnimationState::new(initial)));
    let state_clone = state.clone();
    let mut running = use_signal(|| false);

    use_future(move || {
        let mut value = state_clone.clone();
        async move {
            loop {
                if !value.is_running() {
                    Time::delay(Duration::from_millis(16)).await;
                    continue;
                }

                let start = Time::now();
                Time::delay(Duration::from_millis(16)).await; // ~60fps
                let dt = start.elapsed().as_secs_f32();

                if value.0.write().update(dt) {
                    // Force a rerender when animation updates
                    running.toggle();
                }
            }
        }
    });

    state
}
