use animation::AnimationMode;
use dioxus_hooks::{use_coroutine, use_signal, Coroutine};
use dioxus_signals::{Readable, Signal, Writable};

use futures_util::StreamExt;
pub use instant::Duration;
use motion::Motion;
use prelude::AnimationState;

pub mod animation;
pub mod motion;
pub mod platform;
pub mod spring;
pub mod use_transform_motion;

pub mod prelude {
    pub use crate::animation::{AnimationMode, AnimationState};
    pub use crate::motion::Motion;
    pub use crate::spring::Spring;
    pub use crate::use_motion;
    pub use crate::Duration;
    pub use crate::UseMotion;
}

use platform::{TimeProvider, WebTime};

#[cfg(feature = "web")]
pub type Time = WebTime;

#[cfg(not(feature = "web"))]
use platform::DesktopTime;
use use_transform_motion::TransformMotion;

#[cfg(not(feature = "web"))]
pub type Time = DesktopTime;

/// Represents an active motion animation
#[derive(Clone, Copy)]
pub struct UseMotion {
    value: Signal<f32>,
    running_state: Signal<bool>,
    completion_state: Signal<AnimationState>,
    elapsed_time: Signal<Duration>,
    config: Motion,
    channel: Coroutine<()>,
    reverse_state: Signal<bool>,
}

impl UseMotion {
    /// Get the current animated value
    pub fn value(&self) -> f32 {
        *self.value.read()
    }

    /// Start the animation
    pub fn start(&mut self) {
        *self.value.write() = self.config.initial;
        *self.completion_state.write() = AnimationState::Idle;
        *self.running_state.write() = true;
        *self.reverse_state.write() = false;
        self.channel.send(());
    }

    /// Stop the animation
    pub fn stop(&mut self) {
        *self.running_state.write() = false;
        *self.completion_state.write() = AnimationState::Idle;
    }

    pub fn resume(&mut self) {
        *self.running_state.write() = true;
        self.channel.send(());
    }

    pub fn reset(&mut self) {
        *self.value.write() = self.config.initial;
        *self.completion_state.write() = AnimationState::Idle;
        *self.running_state.write() = false;
        *self.elapsed_time.write() = Duration::from_secs(0);
        *self.reverse_state.write() = false;
    }

    pub fn reverse(&mut self) {
        if *self.reverse_state.read() {
            self.config.initial
        } else {
            self.config.target
        };

        self.reverse_state.toggle();
        *self.completion_state.write() = AnimationState::Idle;
        *self.running_state.write() = true;
        self.channel.send(());
    }

    /// Get the current animation state
    pub fn state(&self) -> AnimationState {
        self.completion_state.read().clone()
    }

    /// Check if the animation is currently running
    pub fn is_running(&self) -> bool {
        *self.running_state.read()
    }
}

// Rename existing to be more specific
pub fn use_motion(config: Motion) -> UseMotion {
    let mut value = use_signal(|| config.initial);
    let mut running_state = use_signal(|| false);
    let mut completion_state = use_signal(|| AnimationState::Idle);
    let mut elapsed_time = use_signal(|| Duration::from_secs(0));
    let reverse_state = use_signal(|| false);

    let channel = use_coroutine(move |mut rx| async move {
        while rx.next().await.is_some() {
            Time::delay(config.delay).await;

            match config.mode {
                AnimationMode::Tween(tween) => {
                    let start_time = Time::now();
                    let start_value = *value.peek();
                    let end_value = if *reverse_state.read() {
                        config.initial
                    } else {
                        config.target
                    };
                    let total_change = (end_value - start_value).abs();
                    let total_frames = total_change.ceil() as u64;
                    let initial_elapsed = *elapsed_time.read();

                    completion_state.set(AnimationState::Running);
                    running_state.set(true);

                    while *running_state.read() {
                        let current_elapsed =
                            Time::now().duration_since(start_time) + initial_elapsed;
                        elapsed_time.set(current_elapsed);

                        if current_elapsed >= tween.duration {
                            break;
                        }

                        // Calculate remaining duration
                        let remaining_duration = tween.duration - current_elapsed;

                        let frame_progress = (current_elapsed.as_secs_f64()
                            / tween.duration.as_secs_f64())
                            * total_frames as f64;
                        let current_frame = frame_progress.floor() as u64;
                        let progress = current_frame as f32 / total_frames as f32;
                        let current =
                            (tween.easing)(progress, start_value, end_value - start_value, 1.0);

                        value.set(current);

                        // Calculate frame delay based on remaining duration
                        let frame_delay = if remaining_duration >= Duration::from_secs(2) {
                            Duration::from_secs_f64(
                                remaining_duration.as_secs_f64()
                                    / (total_frames - current_frame) as f64,
                            )
                        } else {
                            Duration::from_millis(16)
                        };

                        Time::delay(frame_delay).await;
                    }

                    // Ensure final value is set
                    if Time::now().duration_since(start_time) + initial_elapsed >= tween.duration {
                        value.set(end_value);
                        elapsed_time.set(Duration::from_secs(0));
                        running_state.set(false);
                        completion_state.set(AnimationState::Completed);

                        if let Some(f) = config.on_complete {
                            f();
                        }
                    }
                }
                AnimationMode::Spring(spring) => {
                    let mut velocity = spring.velocity;
                    let mut current = *value.peek();
                    let target = if *reverse_state.read() {
                        config.initial
                    } else {
                        config.target
                    };

                    completion_state.set(AnimationState::Running);
                    running_state.set(true);

                    while *running_state.read() {
                        let dt = 1.0 / 60.0; // 60 FPS

                        current =
                            Motion::update_spring(current, target, &mut velocity, &spring, dt);

                        value.set(current);

                        // Check if spring has settled
                        if velocity.abs() < 0.01 && (current - target).abs() < 0.01 {
                            break;
                        }

                        Time::delay(Duration::from_millis(5)).await;
                    }

                    // Ensure we reach exact target
                    value.set(target);
                    running_state.set(false);
                    completion_state.set(AnimationState::Completed);
                }
            }
        }
    });

    UseMotion {
        value,
        running_state,
        completion_state,
        elapsed_time,
        config,
        channel,
        reverse_state,
    }
}
