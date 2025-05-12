//! Dioxus Motion base module
//!
//! Defines motion props and setup logic for Dioxus motion components.
use super::state::{MotionState, apply_animation_target, generate_style_string};
use crate::animations::transition::{AnimationTarget, TransitionConfig, Variants};
use dioxus::prelude::*;

/// Common motion props for all components
#[derive(Props, Clone, PartialEq)]
pub struct MotionComponentProps {
    // Common motion props
    #[props(default)]
    pub initial: Option<AnimationTarget>,

    #[props(default)]
    pub animate: Option<AnimationTarget>,

    #[props(default)]
    pub while_hover: Option<AnimationTarget>,

    #[props(default)]
    pub while_tap: Option<AnimationTarget>,

    #[props(default)]
    pub while_in_view: Option<AnimationTarget>,

    #[props(default)]
    pub exit: Option<AnimationTarget>,

    #[props(default)]
    pub variants: Option<Variants>,

    #[props(default)]
    pub initial_variant: Option<String>,

    #[props(default)]
    pub animate_variant: Option<String>,

    #[props(default)]
    pub transition: Option<TransitionConfig>,

    // Configuration props removed

    // HTML element specific props
    #[props(default)]
    pub id: Option<String>,

    #[props(default)]
    pub class: Option<String>,

    #[props(default)]
    pub style: Option<String>,

    // Event handlers
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    // Global attributes extension with element-specific attributes
    #[props(extends = GlobalAttributes, extends = button, extends = img, extends = a, extends = input)]
    pub attributes: Vec<Attribute>,

    // Children
    pub children: Element,
}

/// Result of setting up a motion component
pub struct MotionSetupResult {
    pub combined_style: String,
    pub on_mouse_enter: EventHandler<MouseEvent>,
    pub on_mouse_leave: EventHandler<MouseEvent>,
    pub on_mouse_down: EventHandler<MouseEvent>,
    pub on_mouse_up: EventHandler<MouseEvent>,
}

/// Setup motion state and event handlers for a component
#[track_caller]
pub fn setup_motion_component(props: &MotionComponentProps) -> MotionSetupResult {
    // Create the motion state
    let state = MotionState::new();

    // Process variants if provided
    let initial_target = if let (Some(variants), Some(variant_name)) =
        (props.variants.as_ref(), props.initial_variant.as_ref())
    {
        variants
            .get(variant_name)
            .cloned()
            .or(props.initial.clone())
    } else {
        props.initial.clone()
    };

    let animate_target = if let (Some(variants), Some(variant_name)) =
        (props.variants.as_ref(), props.animate_variant.as_ref())
    {
        variants
            .get(variant_name)
            .cloned()
            .or(props.animate.clone())
    } else {
        props.animate.clone()
    };

    // Create a signal for the state to use in the effect
    let mut state_signal = use_signal(|| state.clone());

    // Apply initial and animate targets in an effect to avoid render-time signal writes
    let initial_target_clone = initial_target.clone();
    let animate_target_clone = animate_target.clone();
    let transition_clone = props.transition.clone();

    use_effect(move || {
        // Apply initial animation in effect
        if let Some(initial) = &initial_target_clone {
            state_signal.with_mut(|state_value| {
                apply_animation_target(initial, state_value, transition_clone.as_ref());
            });
        }

        // Apply animate target in effect
        if let Some(animate) = &animate_target_clone {
            state_signal.with_mut(|state_value| {
                apply_animation_target(animate, state_value, transition_clone.as_ref());
            });
        }
    });

    // Create a single effect to watch for animate_variant changes
    let animate_variant = props.animate_variant.clone();
    let variants = props.variants.clone();
    let transition = props.transition.clone();

    // Create a signal for the state to use in the effect
    let state_signal = use_signal(|| state.clone());

    // Create a signal to track the previous variant
    let prev_variant = use_signal(|| None::<String>);

    // Apply variant animation in an effect to avoid render-time signal writes
    let animate_variant_init = animate_variant.clone();
    let variants_init = variants.clone();
    let transition_init = transition.clone();
    let mut state_signal_init = state_signal;

    use_effect(move || {
        if let Some(variant_name) = &animate_variant_init {
            if let Some(variants_map) = variants_init.as_ref() {
                if let Some(target) = variants_map.get(variant_name) {
                    state_signal_init.with_mut(|state_value| {
                        apply_animation_target(target, state_value, transition_init.as_ref());
                    });
                }
            }
        }
    });

    // Create an effect to watch for animate_variant changes
    let animate_variant_clone = animate_variant.clone();
    let variants_clone = variants.clone();
    let transition_clone = transition.clone();
    let mut state_signal_clone = state_signal;
    let mut prev_variant_clone = prev_variant;

    use_effect(move || {
        if let Some(variant_name) = &animate_variant_clone {
            // Only update if the variant has changed
            if prev_variant_clone() != Some(variant_name.clone()) {
                prev_variant_clone.set(Some(variant_name.clone()));

                // Get the animation target based on the variant
                if let Some(variants_map) = variants_clone.as_ref() {
                    if let Some(target) = variants_map.get(variant_name) {
                        // Update the state
                        state_signal_clone.with_mut(|state_value| {
                            apply_animation_target(target, state_value, transition_clone.as_ref());
                        });
                    }
                }
            }
        }
    });

    // Event handlers
    let on_mouse_enter = {
        let mut state_clone = state.clone();
        let while_hover = props.while_hover.clone();
        let transition = props.transition.clone();

        move |_| {
            state_clone.is_hovering.set(true);

            if let Some(hover_target) = &while_hover {
                apply_animation_target(hover_target, &mut state_clone, transition.as_ref());
            }
        }
    };

    let on_mouse_leave = {
        let mut state_clone = state.clone();
        let animate = animate_target.clone();
        let transition = props.transition.clone();

        move |_| {
            state_clone.is_hovering.set(false);

            // Always reset to the animate target when mouse leaves
            if let Some(animate_target) = &animate {
                apply_animation_target(animate_target, &mut state_clone, transition.as_ref());
            } else {
                // If no animate target is provided, reset to default values
                // Explicitly reset all transform properties to ensure hover effects are removed
                let default_target = AnimationTarget::default_reset();
                apply_animation_target(&default_target, &mut state_clone, transition.as_ref());
            }
        }
    };

    // Handle tap/click animations
    let on_mouse_down = {
        let mut state_clone = state.clone();
        let while_tap = props.while_tap.clone();
        let transition = props.transition.clone();

        move |_| {
            state_clone.is_tapping.set(true);

            if let Some(tap_target) = &while_tap {
                apply_animation_target(tap_target, &mut state_clone, transition.as_ref());
            }
        }
    };

    let on_mouse_up = {
        let mut state_clone = state.clone();
        let while_hover = props.while_hover.clone();
        let animate = animate_target.clone();
        let transition = props.transition.clone();

        move |_| {
            let is_hovering = *state_clone.is_hovering.read();
            state_clone.is_tapping.set(false);

            if is_hovering {
                // If still hovering, apply hover effect
                if let Some(hover_target) = &while_hover {
                    apply_animation_target(hover_target, &mut state_clone, transition.as_ref());
                }
            } else {
                // If not hovering, reset to animate target or default
                if let Some(animate_target) = &animate {
                    apply_animation_target(animate_target, &mut state_clone, transition.as_ref());
                } else {
                    // If no animate target is provided, reset to default values
                    // Explicitly reset all transform properties to ensure hover effects are removed
                    let default_target = AnimationTarget::default_reset();
                    apply_animation_target(&default_target, &mut state_clone, transition.as_ref());
                }
            }
        }
    };

    // Combine base style with motion style
    let combined_style = generate_style_string(&state, props.style.as_deref());

    MotionSetupResult {
        combined_style,
        on_mouse_enter: EventHandler::new(on_mouse_enter),
        on_mouse_leave: EventHandler::new(on_mouse_leave),
        on_mouse_down: EventHandler::new(on_mouse_down),
        on_mouse_up: EventHandler::new(on_mouse_up),
    }
}
