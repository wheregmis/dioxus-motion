use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};

#[derive(Props, PartialEq, Clone)]
pub struct MotionProps {
    // Existing props
    #[props(default = Transform::default())]
    pub initial: Transform,
    pub animate: Transform,

    // Hover states
    #[props(default = None)]
    pub whilehover: Option<Transform>,

    // Tap/Click states
    #[props(default = None)]
    pub whiletap: Option<Transform>,

    // Variants for different states
    #[props(default = None)]
    pub variants: Option<HashMap<&'static str, Transform>>,

    // Custom transitions per property
    #[props(default = None)]
    pub transition: Option<TransitionConfig>,

    // Drag constraints
    #[props(default = false)]
    pub drag: bool,
    #[props(default = None)]
    pub dragconstraints: Option<DragConstraints>,

    // Animation controls
    #[props(default = None)]
    pub onanimationstart: Option<EventHandler>,
    #[props(default = None)]
    pub onanimationend: Option<EventHandler>,

    // Spring animation config
    #[props(default = Spring::default())]
    pub spring: Spring,

    // Style props
    #[props(default = "")]
    pub class: &'static str,
    #[props(default = "")]
    pub style: &'static str,
    #[props(default)]
    pub children: Element,
}

#[derive(Clone, PartialEq)]
pub struct TransitionConfig {
    pub duration: Option<Duration>,
    pub delay: Option<Duration>,
    pub easing: fn(f32, f32, f32, f32) -> f32,
}

#[derive(Clone, PartialEq)]
pub struct DragConstraints {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

#[component]
pub fn MotionDiv(props: MotionProps) -> Element {
    let mut transform = use_transform_animation(
        props.initial,
        props.animate,
        AnimationMode::Spring(props.spring),
    );
    let mut hover_state = use_signal(|| false);
    let mut tap_state = use_signal(|| false);

    // Handle hover animations
    let on_hover = move |_| {
        if let Some(hover_transform) = props.whilehover {
            *hover_state.write() = true;
            transform.animate_to(hover_transform);
        }
    };

    let on_hover_end = move |_| {
        if props.whilehover.is_some() {
            *hover_state.write() = false;
            transform.animate_to(props.animate);
        }
    };

    rsx! {
        div {
            class: "{props.class}",
            style: "{transform.style()} {props.style}",
            onmounted: move |_| transform.start(),
            onmouseenter: on_hover,
            onmouseleave: on_hover_end,
            onmousedown: move |_| {
                if props.whiletap.is_some() {
                    *tap_state.write() = true;
                    transform.animate_to(props.whiletap.unwrap());
                }
            },
            onmouseup: move |_| {
                if props.whiletap.is_some() {
                    *tap_state.write() = false;
                    transform.animate_to(props.animate);
                }
            },
            draggable: props.drag,
            {props.children}
        }
    }
}
