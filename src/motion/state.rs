//! Dioxus Motion state module
//!
//! Defines motion state and logic for applying animation targets to Dioxus state.

use crate::animations::transform::Transform;
use crate::animations::transition::{
    AnimationTarget, EasingFunction, TransitionConfig, TransitionType,
};
use dioxus::prelude::*;

/// Motion component state
#[derive(Clone)]
pub struct MotionState {
    // Signals with both read and write access
    pub opacity: Signal<f32>,
    pub transform: Signal<Transform>,
    pub background_color: Signal<String>,
    pub is_hovering: Signal<bool>,
    pub is_tapping: Signal<bool>,
    // Transition properties
    pub transition_duration: Signal<f32>,
    pub transition_timing_function: Signal<String>,
    pub transition_delay: Signal<f32>,
}

impl MotionState {
    pub fn new() -> Self {
        Self {
            opacity: use_signal(|| 1.0),
            transform: use_signal(Transform::identity),
            background_color: use_signal(String::new),
            is_hovering: use_signal(|| false),
            is_tapping: use_signal(|| false),
            // Initialize transition properties with defaults
            transition_duration: use_signal(|| 0.3),
            transition_timing_function: use_signal(|| "ease".to_string()),
            transition_delay: use_signal(|| 0.0),
        }
    }
}

/// Apply animation target to motion state
#[track_caller]
pub fn apply_animation_target(
    target: &AnimationTarget,
    state: &mut MotionState,
    transition_config: Option<&TransitionConfig>,
) {
    // Get transition configuration
    let transition = target.transition.as_ref().or(transition_config);

    // Apply opacity animation if specified
    if let Some(opacity) = target.opacity {
        // Set directly
        state.opacity.set(opacity);
    }

    // Apply transform animations if specified
    // Get current transform to preserve unspecified values without subscribing
    let current = *state.transform.peek();
    let mut new_transform = current;
    let mut transform_changed = false;

    if let Some(x) = target.x {
        new_transform.x = x;
        transform_changed = true;
    }

    if let Some(y) = target.y {
        new_transform.y = y;
        transform_changed = true;
    }

    if let Some(scale) = target.scale {
        new_transform.scale = scale;
        transform_changed = true;
    }

    if let Some(rotate) = target.rotate {
        // Convert degrees to radians
        new_transform.rotation = rotate * std::f32::consts::PI / 180.0;
        transform_changed = true;
    }

    if transform_changed {
        // Set directly
        state.transform.set(new_transform);
    }

    // Apply background color animation if specified
    if let Some(bg_color) = &target.background_color {
        // Set directly
        state.background_color.set(bg_color.clone());
    }

    // Update the transition CSS property based on the transition config
    if let Some(config) = transition {
        // Set the transition duration based on the config
        if let Some(duration) = config.duration {
            state.transition_duration.set(duration);
        } else {
            // Default duration
            let default_duration = match config.type_ {
                TransitionType::Tween => 0.3,  // Default tween duration
                TransitionType::Spring => 0.5, // Approximate spring duration
            };
            state.transition_duration.set(default_duration);
        }

        // Set the transition timing function based on the config
        let timing_function = match config.type_ {
            TransitionType::Tween => {
                // Use the standard CSS timing functions when possible
                match config.ease.as_ref().unwrap_or(&EasingFunction::EaseInOut) {
                    EasingFunction::Linear => "linear".to_string(),
                    EasingFunction::EaseIn => "ease-in".to_string(),
                    EasingFunction::EaseOut => "ease-out".to_string(),
                    EasingFunction::EaseInOut => "ease-in-out".to_string(),
                    // For other easing functions, use the CSS variables defined in the CSS
                    // These are more standardized and will work better across browsers
                    EasingFunction::CircIn => "cubic-bezier(0.6, 0.04, 0.98, 0.335)".to_string(),
                    EasingFunction::CircOut => "cubic-bezier(0.075, 0.82, 0.165, 1)".to_string(),
                    EasingFunction::CircInOut => {
                        "cubic-bezier(0.785, 0.135, 0.15, 0.86)".to_string()
                    }
                    EasingFunction::BackIn => "cubic-bezier(0.6, -0.28, 0.735, 0.045)".to_string(),
                    EasingFunction::BackOut => {
                        "cubic-bezier(0.175, 0.885, 0.32, 1.275)".to_string()
                    }
                    EasingFunction::BackInOut => {
                        "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string()
                    }
                    // For complex easings, use simpler approximations that work well in CSS
                    EasingFunction::ElasticIn => "cubic-bezier(0.7, 0, 0.84, 0)".to_string(),
                    EasingFunction::ElasticOut => "cubic-bezier(0.16, 1, 0.3, 1)".to_string(),
                    EasingFunction::ElasticInOut => "cubic-bezier(0.87, 0, 0.13, 1)".to_string(),
                    EasingFunction::BounceIn => "cubic-bezier(0.7, 0, 0.84, 0)".to_string(),
                    EasingFunction::BounceOut => "cubic-bezier(0.16, 1, 0.3, 1)".to_string(),
                    EasingFunction::BounceInOut => "cubic-bezier(0.87, 0, 0.13, 1)".to_string(),
                }
            }
            TransitionType::Spring => {
                // Approximate spring physics with a custom cubic-bezier
                // These values create a slightly bouncy effect
                // We only use damping for now, but could use stiffness in the future
                let _stiffness = config.stiffness.unwrap_or(100.0);
                let damping = config.damping.unwrap_or(10.0);

                // Adjust cubic-bezier based on spring parameters
                // Higher stiffness = faster initial acceleration
                // Higher damping = less overshoot
                if damping > 20.0 {
                    // Overdamped - no overshoot
                    "cubic-bezier(0.33, 1, 0.68, 1)".to_string()
                } else if damping > 10.0 {
                    // Slightly underdamped - small overshoot
                    "cubic-bezier(0.34, 1.56, 0.64, 1)".to_string()
                } else {
                    // Very underdamped - significant overshoot
                    "cubic-bezier(0.22, 1.84, 0.88, 0.77)".to_string()
                }
            }
        };
        state.transition_timing_function.set(timing_function);

        // Set the transition delay if specified
        if let Some(delay) = config.delay {
            state.transition_delay.set(delay);
        } else {
            state.transition_delay.set(0.0);
        }
    } else {
        // Default transition (fast with slight easing)
        state.transition_duration.set(0.2);
        state.transition_timing_function.set("ease".to_string());
        state.transition_delay.set(0.0);
    }
}

/// Generate style string from motion state
pub fn generate_style_string(state: &MotionState, base_style: Option<&str>) -> String {
    // Use read() to subscribe to signal changes
    let transform = state.transform.read();
    let opacity = state.opacity.read();
    let bg_color = state.background_color.read();
    let transition_duration = state.transition_duration.read();
    let transition_timing_function = state.transition_timing_function.read();
    let transition_delay = state.transition_delay.read();

    let mut style = String::new();

    // Add transform
    style.push_str(&format!(
        "transform: translate({}px, {}px) scale({}) rotate({}deg); ",
        transform.x,
        transform.y,
        transform.scale,
        transform.rotation * 180.0 / std::f32::consts::PI
    ));

    // Add opacity
    style.push_str(&format!("opacity: {}; ", opacity));

    // Add background color if specified
    if !bg_color.is_empty() {
        style.push_str(&format!("background-color: {}; ", bg_color));
    }

    // Add transition properties
    let transition_props = ["transform", "opacity"];
    let transition_value = transition_props
        .iter()
        .map(|prop| {
            format!(
                "{} {}s {} {}s",
                prop, transition_duration, transition_timing_function, transition_delay
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    style.push_str(&format!("transition: {}; ", transition_value));

    // Add base style if provided
    if let Some(base) = base_style {
        style.push_str(base);
    }

    style
}
