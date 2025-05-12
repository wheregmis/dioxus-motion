//! Dioxus Motion state module
//!
//! Defines motion state and logic for applying animation targets to Dioxus state.

use crate::animations::transform::Transform;
use crate::animations::transition::{AnimationTarget, TransitionConfig};
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
}

impl MotionState {
    pub fn new() -> Self {
        Self {
            opacity: use_signal(|| 1.0),
            transform: use_signal(Transform::identity),
            background_color: use_signal(String::new),
            is_hovering: use_signal(|| false),
            is_tapping: use_signal(|| false),
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
    // Get transition configuration - we're not using it for now
    // but keeping the code structure for future enhancements
    let _transition = target.transition.as_ref().or(transition_config);

    // Apply opacity animation if specified
    if let Some(opacity) = target.opacity {
        // Set directly without reading
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
        // Set directly without reading again
        state.transform.set(new_transform);
    }

    // Apply background color animation if specified
    if let Some(bg_color) = &target.background_color {
        // Set directly without reading
        state.background_color.set(bg_color.clone());
    }
}

/// Generate style string from motion state
pub fn generate_style_string(state: &MotionState, base_style: Option<&str>) -> String {
    // Use read() to subscribe to signal changes
    let transform = state.transform.read();
    let opacity = state.opacity.read();
    let bg_color = state.background_color.read();

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

    // Add base style if provided
    if let Some(base) = base_style {
        style.push_str(base);
    }

    style
}
