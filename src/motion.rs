pub use instant::Duration;

use uuid::Uuid;

use crate::{
    animation::{AnimationMode, Tween},
    spring::Spring,
};

/// Configuration for a motion animation
#[derive(Debug, Clone, Copy)]
pub struct Motion {
    pub id: Uuid,
    pub initial: f32,
    pub target: f32,
    pub duration: Duration,
    pub delay: Duration,
    pub mode: AnimationMode,
    pub on_complete: Option<fn()>,
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
            mode: AnimationMode::default(),
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
        self.mode = AnimationMode::Tween(Tween::new(duration));
        self
    }

    pub fn spring(mut self, config: Spring) -> Self {
        self.mode = AnimationMode::Spring(config);
        self
    }

    pub fn mode(mut self, mode: AnimationMode) -> Self {
        self.mode = mode;
        self
    }

    // Helper method to update spring physics
    pub fn update_spring(
        current: f32,
        target: f32,
        velocity: &mut f32,
        spring: &Spring,
        dt: f32,
    ) -> f32 {
        let force = spring.stiffness * (target - current);
        let damping = spring.damping * *velocity;
        let acceleration = (force - damping) / spring.mass;

        *velocity += acceleration * dt;
        current + *velocity * dt
    }

    /// Set a custom easing function
    pub fn easing(mut self, easing: fn(f32, f32, f32, f32) -> f32) -> Self {
        self.mode = match self.mode {
            AnimationMode::Tween(tween) => AnimationMode::Tween(tween.with_easing(easing)),
            _ => self.mode,
        };
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
