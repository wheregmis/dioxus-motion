use dioxus::prelude::*;
use easer::functions::Easing;
use std::collections::HashMap;

use crate::Duration;
use crate::animations::transform::Transform;
use crate::animations::utils::{AnimationConfig, AnimationMode};
use crate::animations::{spring::Spring, tween::Tween};

/// Animation target for motion components
#[derive(Clone, Debug, PartialEq)]
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

impl Default for AnimationTarget {
    fn default() -> Self {
        Self {
            opacity: None,
            x: None,
            y: None,
            scale: None,
            rotate: None,
            background_color: None,
            transition: None,
        }
    }
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

/// Motion component props
#[derive(Clone, PartialEq)]
pub struct MotionProps<P: Clone + PartialEq> {
    /// Base component props
    pub base_props: P,

    /// Initial animation state
    pub initial: Option<AnimationTarget>,

    /// Target animation state
    pub animate: Option<AnimationTarget>,

    /// Animation state when hovered
    pub while_hover: Option<AnimationTarget>,

    /// Animation state when tapped/clicked
    pub while_tap: Option<AnimationTarget>,

    /// Animation state when in viewport
    pub while_in_view: Option<AnimationTarget>,

    /// Animation state when component exits
    pub exit: Option<AnimationTarget>,

    /// Animation variants
    pub variants: Option<Variants>,

    /// Initial variant name
    pub initial_variant: Option<String>,

    /// Target variant name
    pub animate_variant: Option<String>,

    /// Default transition configuration
    pub transition: Option<TransitionConfig>,

    /// Layout animation flag
    pub layout: Option<bool>,

    /// Layout ID for shared element transitions
    pub layout_id: Option<String>,

    /// Children elements
    pub children: Element,
}

/// Motion component state
#[derive(Clone)]
struct MotionState {
    opacity: Signal<f32>,
    transform: Signal<Transform>,
    background_color: Signal<String>,
    is_hovering: Signal<bool>,
    is_tapping: Signal<bool>,
    is_in_view: Signal<bool>,
}

impl MotionState {
    fn new() -> Self {
        Self {
            opacity: use_signal(|| 1.0),
            transform: use_signal(|| Transform::identity()),
            background_color: use_signal(|| String::new()),
            is_hovering: use_signal(|| false),
            is_tapping: use_signal(|| false),
            is_in_view: use_signal(|| false),
        }
    }
}

/// Apply animation target to motion state
fn apply_animation_target(
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
    // Create a new transform without reading the current one
    let mut new_transform = Transform::identity();
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
        // Set directly without reading
        state.transform.set(new_transform);
    }

    // Apply background color animation if specified
    if let Some(bg_color) = &target.background_color {
        // Set directly without reading
        state.background_color.set(bg_color.clone());
    }
}

/// Generate style string from motion state
fn generate_style_string(state: &MotionState, base_style: Option<&str>) -> String {
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

/// Motion component factory
pub fn create_motion_component<P, C>(component_factory: C) -> impl Fn(MotionProps<P>) -> Element
where
    P: Clone + PartialEq + 'static,
    C: Fn(P) -> Element + 'static,
{
    move |props: MotionProps<P>| {
        let mut state = MotionState::new();

        // Apply initial animation if specified
        if let Some(initial) = &props.initial {
            apply_animation_target(initial, &mut state, props.transition.as_ref());
        }

        // Clone props for use in effect
        let animate_clone = props.animate.clone();
        let transition_clone = props.transition.clone();
        let mut state_clone = state.clone();

        // Apply animate target when component mounts
        use_effect(move || {
            if let Some(animate) = &animate_clone {
                apply_animation_target(animate, &mut state_clone, transition_clone.as_ref());
            }
        });

        // Clone props and state for event handlers
        let while_hover_clone = props.while_hover.clone();
        let transition_clone1 = props.transition.clone();
        let mut state_clone1 = state.clone();

        let on_mouse_enter = move |_| {
            state_clone1.is_hovering.set(true);
            if let Some(while_hover) = &while_hover_clone {
                apply_animation_target(while_hover, &mut state_clone1, transition_clone1.as_ref());
            }
        };

        let animate_clone2 = props.animate.clone();
        let transition_clone2 = props.transition.clone();
        let mut state_clone2 = state.clone();

        let on_mouse_leave = move |_| {
            state_clone2.is_hovering.set(false);
            if let Some(animate) = &animate_clone2 {
                apply_animation_target(animate, &mut state_clone2, transition_clone2.as_ref());
            }
        };

        // Handle tap/click animations
        let while_tap_clone = props.while_tap.clone();
        let transition_clone3 = props.transition.clone();
        let mut state_clone3 = state.clone();

        let on_mouse_down = move |_| {
            state_clone3.is_tapping.set(true);
            if let Some(while_tap) = &while_tap_clone {
                apply_animation_target(while_tap, &mut state_clone3, transition_clone3.as_ref());
            }
        };

        let while_hover_clone2 = props.while_hover.clone();
        let animate_clone3 = props.animate.clone();
        let transition_clone4 = props.transition.clone();
        let mut state_clone4 = state.clone();

        let on_mouse_up = move |_| {
            state_clone4.is_tapping.set(false);
            if *state_clone4.is_hovering.read() {
                if let Some(while_hover) = &while_hover_clone2 {
                    apply_animation_target(
                        while_hover,
                        &mut state_clone4,
                        transition_clone4.as_ref(),
                    );
                }
            } else if let Some(animate) = &animate_clone3 {
                apply_animation_target(animate, &mut state_clone4, transition_clone4.as_ref());
            }
        };

        // Create modified props with event handlers and style
        let modified_props = props.base_props.clone();

        // TODO: Add style to modified_props
        // This requires accessing the style property of the base props
        // which is not possible in a generic way without reflection
        // For now, we'll handle this in the specific component implementations

        // Create the component with modified props
        let component = component_factory(modified_props);

        // Wrap the component with event handlers
        rsx! {
            div {
                onmouseenter: on_mouse_enter,
                onmouseleave: on_mouse_leave,
                onmousedown: on_mouse_down,
                onmouseup: on_mouse_up,
                style: generate_style_string(&state, None),
                {component}
            }
        }
    }
}

/// Motion component module
pub mod components {
    use super::*;

    // Define individual motion components for common HTML elements

    #[component]
    pub fn Div(
        // Common motion props
        initial: Option<AnimationTarget>,
        animate: Option<AnimationTarget>,
        while_hover: Option<AnimationTarget>,
        while_tap: Option<AnimationTarget>,
        while_in_view: Option<AnimationTarget>,
        exit: Option<AnimationTarget>,
        variants: Option<Variants>,
        initial_variant: Option<String>,
        animate_variant: Option<String>,
        transition: Option<TransitionConfig>,
        layout: Option<bool>,
        layout_id: Option<String>,

        // HTML element specific props
        id: Option<String>,
        class: Option<String>,
        style: Option<String>,

        // Event handlers
        onclick: Option<EventHandler<MouseEvent>>,

        // Children
        children: Element,
    ) -> Element {
        let state = MotionState::new();

        // Clone props for use in effects
        let initial_clone = initial.clone();
        let animate_clone = animate.clone();
        let transition_clone = transition.clone();
        let mut state_clone = state.clone();

        // Apply initial animation when component mounts
        use_effect(move || {
            // Apply initial animation if specified
            if let Some(initial) = &initial_clone {
                apply_animation_target(initial, &mut state_clone, transition_clone.as_ref());
            }

            // Then apply animate target
            if let Some(animate) = &animate_clone {
                apply_animation_target(animate, &mut state_clone, transition_clone.as_ref());
            }
        });

        // Clone props and state for event handlers
        let while_hover_clone = while_hover.clone();
        let transition_clone1 = transition.clone();
        let mut state_clone1 = state.clone();

        let on_mouse_enter = move |_| {
            state_clone1.is_hovering.set(true);
            if let Some(while_hover) = &while_hover_clone {
                apply_animation_target(while_hover, &mut state_clone1, transition_clone1.as_ref());
            }
        };

        let animate_clone2 = animate.clone();
        let transition_clone2 = transition.clone();
        let mut state_clone2 = state.clone();

        let on_mouse_leave = move |_| {
            state_clone2.is_hovering.set(false);
            if let Some(animate) = &animate_clone2 {
                apply_animation_target(animate, &mut state_clone2, transition_clone2.as_ref());
            }
        };

        // Handle tap/click animations
        let while_tap_clone = while_tap.clone();
        let transition_clone3 = transition.clone();
        let mut state_clone3 = state.clone();

        let on_mouse_down = move |_| {
            state_clone3.is_tapping.set(true);
            if let Some(while_tap) = &while_tap_clone {
                apply_animation_target(while_tap, &mut state_clone3, transition_clone3.as_ref());
            }
        };

        let while_hover_clone2 = while_hover.clone();
        let animate_clone3 = animate.clone();
        let transition_clone4 = transition.clone();
        let mut state_clone4 = state.clone();

        let on_mouse_up = move |_| {
            state_clone4.is_tapping.set(false);
            if *state_clone4.is_hovering.read() {
                if let Some(while_hover) = &while_hover_clone2 {
                    apply_animation_target(
                        while_hover,
                        &mut state_clone4,
                        transition_clone4.as_ref(),
                    );
                }
            } else if let Some(animate) = &animate_clone3 {
                apply_animation_target(animate, &mut state_clone4, transition_clone4.as_ref());
            }
        };

        // Combine base style with motion style
        let combined_style = generate_style_string(&state, style.as_deref());

        // Create the element with all props and event handlers
        rsx! {
            div {
                id,
                class,
                style: combined_style,
                onclick: move |e| {
                    if let Some(handler) = &onclick {
                        handler.call(e)
                    }
                },
                onmouseenter: on_mouse_enter,
                onmouseleave: on_mouse_leave,
                onmousedown: on_mouse_down,
                onmouseup: on_mouse_up,
                {children}
            }
        }
    }

    #[component]
    pub fn Span(
        // Common motion props
        initial: Option<AnimationTarget>,
        animate: Option<AnimationTarget>,
        while_hover: Option<AnimationTarget>,
        while_tap: Option<AnimationTarget>,
        while_in_view: Option<AnimationTarget>,
        exit: Option<AnimationTarget>,
        variants: Option<Variants>,
        initial_variant: Option<String>,
        animate_variant: Option<String>,
        transition: Option<TransitionConfig>,
        layout: Option<bool>,
        layout_id: Option<String>,

        // HTML element specific props
        id: Option<String>,
        class: Option<String>,
        style: Option<String>,

        // Event handlers
        onclick: Option<EventHandler<MouseEvent>>,

        // Children
        children: Element,
    ) -> Element {
        let state = MotionState::new();

        // Clone props for use in effects
        let initial_clone = initial.clone();
        let animate_clone = animate.clone();
        let transition_clone = transition.clone();
        let mut state_clone = state.clone();

        // Apply initial animation when component mounts
        use_effect(move || {
            // Apply initial animation if specified
            if let Some(initial) = &initial_clone {
                apply_animation_target(initial, &mut state_clone, transition_clone.as_ref());
            }

            // Then apply animate target
            if let Some(animate) = &animate_clone {
                apply_animation_target(animate, &mut state_clone, transition_clone.as_ref());
            }
        });

        // Clone props and state for event handlers
        let while_hover_clone = while_hover.clone();
        let transition_clone1 = transition.clone();
        let mut state_clone1 = state.clone();

        let on_mouse_enter = move |_| {
            state_clone1.is_hovering.set(true);
            if let Some(while_hover) = &while_hover_clone {
                apply_animation_target(while_hover, &mut state_clone1, transition_clone1.as_ref());
            }
        };

        let animate_clone2 = animate.clone();
        let transition_clone2 = transition.clone();
        let mut state_clone2 = state.clone();

        let on_mouse_leave = move |_| {
            state_clone2.is_hovering.set(false);
            if let Some(animate) = &animate_clone2 {
                apply_animation_target(animate, &mut state_clone2, transition_clone2.as_ref());
            }
        };

        // Handle tap/click animations
        let while_tap_clone = while_tap.clone();
        let transition_clone3 = transition.clone();
        let mut state_clone3 = state.clone();

        let on_mouse_down = move |_| {
            state_clone3.is_tapping.set(true);
            if let Some(while_tap) = &while_tap_clone {
                apply_animation_target(while_tap, &mut state_clone3, transition_clone3.as_ref());
            }
        };

        let while_hover_clone2 = while_hover.clone();
        let animate_clone3 = animate.clone();
        let transition_clone4 = transition.clone();
        let mut state_clone4 = state.clone();

        let on_mouse_up = move |_| {
            state_clone4.is_tapping.set(false);
            if *state_clone4.is_hovering.read() {
                if let Some(while_hover) = &while_hover_clone2 {
                    apply_animation_target(
                        while_hover,
                        &mut state_clone4,
                        transition_clone4.as_ref(),
                    );
                }
            } else if let Some(animate) = &animate_clone3 {
                apply_animation_target(animate, &mut state_clone4, transition_clone4.as_ref());
            }
        };

        // Combine base style with motion style
        let combined_style = generate_style_string(&state, style.as_deref());

        // Create the element with all props and event handlers
        rsx! {
            span {
                id,
                class,
                style: combined_style,
                onclick: move |e| {
                    if let Some(handler) = &onclick {
                        handler.call(e)
                    }
                },
                onmouseenter: on_mouse_enter,
                onmouseleave: on_mouse_leave,
                onmousedown: on_mouse_down,
                onmouseup: on_mouse_up,
                {children}
            }
        }
    }

    #[component]
    pub fn Button(
        // Common motion props
        initial: Option<AnimationTarget>,
        animate: Option<AnimationTarget>,
        while_hover: Option<AnimationTarget>,
        while_tap: Option<AnimationTarget>,
        while_in_view: Option<AnimationTarget>,
        exit: Option<AnimationTarget>,
        variants: Option<Variants>,
        initial_variant: Option<String>,
        animate_variant: Option<String>,
        transition: Option<TransitionConfig>,
        layout: Option<bool>,
        layout_id: Option<String>,

        // HTML element specific props
        id: Option<String>,
        class: Option<String>,
        style: Option<String>,

        // Event handlers
        onclick: Option<EventHandler<MouseEvent>>,

        // Children
        children: Element,
    ) -> Element {
        let state = MotionState::new();

        // Clone props for use in effects
        let initial_clone = initial.clone();
        let animate_clone = animate.clone();
        let transition_clone = transition.clone();
        let mut state_clone = state.clone();

        // Apply initial animation when component mounts
        use_effect(move || {
            // Apply initial animation if specified
            if let Some(initial) = &initial_clone {
                apply_animation_target(initial, &mut state_clone, transition_clone.as_ref());
            }

            // Then apply animate target
            if let Some(animate) = &animate_clone {
                apply_animation_target(animate, &mut state_clone, transition_clone.as_ref());
            }
        });

        // Clone props and state for event handlers
        let while_hover_clone = while_hover.clone();
        let transition_clone1 = transition.clone();
        let mut state_clone1 = state.clone();

        let on_mouse_enter = move |_| {
            state_clone1.is_hovering.set(true);
            if let Some(while_hover) = &while_hover_clone {
                apply_animation_target(while_hover, &mut state_clone1, transition_clone1.as_ref());
            }
        };

        let animate_clone2 = animate.clone();
        let transition_clone2 = transition.clone();
        let mut state_clone2 = state.clone();

        let on_mouse_leave = move |_| {
            state_clone2.is_hovering.set(false);
            if let Some(animate) = &animate_clone2 {
                apply_animation_target(animate, &mut state_clone2, transition_clone2.as_ref());
            }
        };

        // Handle tap/click animations
        let while_tap_clone = while_tap.clone();
        let transition_clone3 = transition.clone();
        let mut state_clone3 = state.clone();

        let on_mouse_down = move |_| {
            state_clone3.is_tapping.set(true);
            if let Some(while_tap) = &while_tap_clone {
                apply_animation_target(while_tap, &mut state_clone3, transition_clone3.as_ref());
            }
        };

        let while_hover_clone2 = while_hover.clone();
        let animate_clone3 = animate.clone();
        let transition_clone4 = transition.clone();
        let mut state_clone4 = state.clone();

        let on_mouse_up = move |_| {
            state_clone4.is_tapping.set(false);
            if *state_clone4.is_hovering.read() {
                if let Some(while_hover) = &while_hover_clone2 {
                    apply_animation_target(
                        while_hover,
                        &mut state_clone4,
                        transition_clone4.as_ref(),
                    );
                }
            } else if let Some(animate) = &animate_clone3 {
                apply_animation_target(animate, &mut state_clone4, transition_clone4.as_ref());
            }
        };

        // Combine base style with motion style
        let combined_style = generate_style_string(&state, style.as_deref());

        // Create the element with all props and event handlers
        rsx! {
            button {
                id,
                class,
                style: combined_style,
                onclick: move |e| {
                    if let Some(handler) = &onclick {
                        handler.call(e)
                    }
                },
                onmouseenter: on_mouse_enter,
                onmouseleave: on_mouse_leave,
                onmousedown: on_mouse_down,
                onmouseup: on_mouse_up,
                {children}
            }
        }
    }
}

// Re-export motion components
pub mod motion {
    pub use super::components::{Button as button, Div as div, Span as span};
}
