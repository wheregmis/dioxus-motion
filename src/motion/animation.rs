use easer::functions::Easing;
use std::collections::HashMap;

use crate::Duration;
use crate::animations::transform::Transform;
use crate::animations::utils::{AnimationConfig, AnimationMode};
use crate::animations::{spring::Spring, tween::Tween};

/// Animation target for motion components
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AnimationTarget {
    /// Opacity value (0.0 to 1.0)
    pub opacity: Option<f32>,
    /// X position in pixels
    pub x: Option<f32>,
    /// Y position in pixels
    pub y: Option<f32>,
    /// Scale factor
    pub scale: Option<f32>,
    /// Rotation in degrees
    pub rotate: Option<f32>,
    /// Background color
    pub background_color: Option<String>,
    /// Custom transition configuration
    pub transition: Option<TransitionConfig>,
}

impl AnimationTarget {
    /// Create a new empty animation target
    pub fn new() -> Self {
        Self::default()
    }

    /// Set opacity value
    pub fn opacity(mut self, value: f32) -> Self {
        self.opacity = Some(value);
        self
    }

    /// Set x position
    pub fn x(mut self, value: f32) -> Self {
        self.x = Some(value);
        self
    }

    /// Set y position
    pub fn y(mut self, value: f32) -> Self {
        self.y = Some(value);
        self
    }

    /// Set scale factor
    pub fn scale(mut self, value: f32) -> Self {
        self.scale = Some(value);
        self
    }

    /// Set rotation in degrees
    pub fn rotate(mut self, value: f32) -> Self {
        self.rotate = Some(value);
        self
    }

    /// Set background color
    pub fn background_color(mut self, value: impl Into<String>) -> Self {
        self.background_color = Some(value.into());
        self
    }

    /// Set transition configuration
    pub fn transition(mut self, config: TransitionConfig) -> Self {
        self.transition = Some(config);
        self
    }
    
    /// Create a default target with all properties explicitly set
    pub fn default_reset() -> Self {
        Self::new()
            .x(0.0)
            .y(0.0)
            .scale(1.0)
            .rotate(0.0)
            .opacity(1.0)
    }
}

/// Transition configuration for animations
#[derive(Clone, Debug, PartialEq)]
pub struct TransitionConfig {
    /// Animation type (spring or tween)
    pub type_: TransitionType,
    /// Duration in seconds (for tween animations)
    pub duration: Option<f32>,
    /// Easing function name (for tween animations)
    pub ease: Option<String>,
    /// Spring stiffness (for spring animations)
    pub stiffness: Option<f32>,
    /// Spring damping (for spring animations)
    pub damping: Option<f32>,
    /// Spring mass (for spring animations)
    pub mass: Option<f32>,
    /// Initial velocity (for spring animations)
    pub velocity: Option<f32>,
    /// Delay before animation starts (in seconds)
    pub delay: Option<f32>,
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self {
            type_: TransitionType::Spring,
            duration: None,
            ease: None,
            stiffness: None,
            damping: None,
            mass: None,
            velocity: None,
            delay: None,
        }
    }
}

impl TransitionConfig {
    /// Create a new transition configuration
    pub fn new(type_: TransitionType) -> Self {
        Self {
            type_,
            ..Default::default()
        }
    }

    /// Set duration (for tween animations)
    pub fn duration(mut self, value: f32) -> Self {
        self.duration = Some(value);
        self
    }

    /// Set easing function (for tween animations)
    pub fn ease(mut self, value: impl Into<String>) -> Self {
        self.ease = Some(value.into());
        self
    }

    /// Set spring stiffness (for spring animations)
    pub fn stiffness(mut self, value: f32) -> Self {
        self.stiffness = Some(value);
        self
    }

    /// Set spring damping (for spring animations)
    pub fn damping(mut self, value: f32) -> Self {
        self.damping = Some(value);
        self
    }

    /// Set spring mass (for spring animations)
    pub fn mass(mut self, value: f32) -> Self {
        self.mass = Some(value);
        self
    }

    /// Set initial velocity (for spring animations)
    pub fn velocity(mut self, value: f32) -> Self {
        self.velocity = Some(value);
        self
    }

    /// Set delay before animation starts
    pub fn delay(mut self, value: f32) -> Self {
        self.delay = Some(value);
        self
    }

    /// Convert to AnimationConfig
    pub fn to_animation_config(&self) -> AnimationConfig {
        match self.type_ {
            TransitionType::Spring => {
                let spring = Spring {
                    stiffness: self.stiffness.unwrap_or(100.0),
                    damping: self.damping.unwrap_or(10.0),
                    mass: self.mass.unwrap_or(1.0),
                    velocity: self.velocity.unwrap_or(0.0),
                };
                let mut config = AnimationConfig::new(AnimationMode::Spring(spring));
                if let Some(delay) = self.delay {
                    config = config.with_delay(Duration::from_secs_f32(delay));
                }
                config
            }
            TransitionType::Tween => {
                let duration = Duration::from_secs_f32(self.duration.unwrap_or(0.3));
                let easing = match self.ease.as_deref() {
                    Some("linear") => {
                        easer::functions::Linear::ease_in_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("easeIn") | Some("ease-in") => {
                        easer::functions::Cubic::ease_in as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("easeOut") | Some("ease-out") => {
                        easer::functions::Cubic::ease_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("easeInOut") | Some("ease-in-out") => {
                        easer::functions::Cubic::ease_in_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("circIn") | Some("circ-in") => {
                        easer::functions::Circ::ease_in as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("circOut") | Some("circ-out") => {
                        easer::functions::Circ::ease_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("circInOut") | Some("circ-in-out") => {
                        easer::functions::Circ::ease_in_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("backIn") | Some("back-in") => {
                        easer::functions::Back::ease_in as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("backOut") | Some("back-out") => {
                        easer::functions::Back::ease_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("backInOut") | Some("back-in-out") => {
                        easer::functions::Back::ease_in_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("elasticIn") | Some("elastic-in") => {
                        easer::functions::Elastic::ease_in as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("elasticOut") | Some("elastic-out") => {
                        easer::functions::Elastic::ease_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("elasticInOut") | Some("elastic-in-out") => {
                        easer::functions::Elastic::ease_in_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("bounceIn") | Some("bounce-in") => {
                        easer::functions::Bounce::ease_in as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("bounceOut") | Some("bounce-out") => {
                        easer::functions::Bounce::ease_out as fn(f32, f32, f32, f32) -> f32
                    }
                    Some("bounceInOut") | Some("bounce-in-out") => {
                        easer::functions::Bounce::ease_in_out as fn(f32, f32, f32, f32) -> f32
                    }
                    _ => easer::functions::Cubic::ease_in_out as fn(f32, f32, f32, f32) -> f32,
                };
                let tween = Tween { duration, easing };
                let mut config = AnimationConfig::new(AnimationMode::Tween(tween));
                if let Some(delay) = self.delay {
                    config = config.with_delay(Duration::from_secs_f32(delay));
                }
                config
            }
        }
    }
}

/// Animation type (spring or tween)
#[derive(Clone, Debug, PartialEq)]
pub enum TransitionType {
    /// Physics-based spring animation
    Spring,
    /// Duration-based tween animation
    Tween,
}

/// Variants map for motion components
pub type Variants = HashMap<String, AnimationTarget>;
