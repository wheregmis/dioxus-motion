use dioxus_hooks::{use_coroutine, use_signal, Coroutine};
use dioxus_signals::{Readable, Signal, Writable};
use easer::functions::{Easing, Linear};
use futures_util::StreamExt;
use instant::Duration;
use uuid::Uuid;

mod platform;

use platform::{DesktopTime, TimeProvider, WebTime};

#[cfg(feature = "web")]
type Time = WebTime;

#[cfg(not(feature = "web"))]
type Time = DesktopTime;

/// Represents the current state of an animation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Running,
    Completed,
}

/// Configuration for a motion animation
#[derive(Debug, Clone, Copy)]
pub struct Motion {
    id: Uuid,
    initial: f32,
    target: f32,
    duration: Duration,
    delay: Duration,
    easing: fn(f32, f32, f32, f32) -> f32,
    on_complete: Option<fn()>,
}

impl Motion {
    /// Create a new Motion with default parameters
    pub fn new(initial: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            initial,
            target: initial,
            duration: Duration::from_millis(300),
            delay: Duration::from_millis(0),
            easing: Linear::ease_in_out,
            on_complete: None,
        }
    }

    /// Set the target value for the animation
    pub fn to(mut self, target: f32) -> Self {
        self.target = target;
        self
    }

    /// Alias for `to` method
    pub fn animate(self, target: f32) -> Self {
        self.to(target)
    }

    /// Set the duration of the animation
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set a custom easing function
    pub fn easing(mut self, easing: fn(f32, f32, f32, f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    /// Set a callback function to be called when animation completes
    pub fn on_complete(mut self, f: fn()) -> Self {
        self.on_complete = Some(f);
        self
    }

    /// Set a delay before the animation starts
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
}

/// Represents an active motion animation
#[derive(Clone, Copy)]
pub struct UseMotion {
    id: Uuid,
    value: Signal<f32>,
    running_state: Signal<bool>,
    completion_state: Signal<AnimationState>,
    config: Motion,
    channel: Coroutine<()>,
}

impl UseMotion {
    /// Get the current animated value
    pub fn value(&self) -> f32 {
        *self.value.read()
    }

    /// Start the animation
    pub fn start(&mut self) {
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

/// Create a new motion animation
pub fn use_motion(config: Motion) -> UseMotion {
    let id = Uuid::new_v4();
    let mut value = use_signal(|| config.initial);
    let mut running_state = use_signal(|| false);
    let mut completion_state = use_signal(|| AnimationState::Idle);

    let channel = use_coroutine(move |mut rx| async move {
        while rx.next().await.is_some() {
            Time::delay(config.delay).await;

            let start_time = Time::now();
            let start_value = *value.peek();
            let end_value = config.target;

            completion_state.set(AnimationState::Running);
            running_state.set(true);

            while *running_state.read() {
                let elapsed = Time::now().duration_since(start_time);

                // Animation completed
                if elapsed >= config.duration {
                    break;
                }

                // Calculate progress and current value
                let progress = elapsed.as_secs_f32() / config.duration.as_secs_f32();
                let current = (config.easing)(progress, start_value, end_value - start_value, 1.0);

                value.set(current);

                // Small delay to control animation frame rate
                Time::delay(Duration::from_millis(16)).await;
            }

            // Ensure final value is set
            value.set(end_value);

            // Update states
            running_state.set(false);
            completion_state.set(AnimationState::Completed);

            // Call completion handler if provided
            if let Some(f) = config.on_complete {
                f();
            }
        }
    });

    UseMotion {
        id,
        value,
        running_state,
        completion_state,
        config,
        channel,
    }
}
