use dioxus::prelude::*;
use super::base::{MotionComponentProps, setup_motion_component};

/// Trait for HTML elements that can be animated
pub trait MotionElement {
    /// Create a motion component from props
    fn create(props: MotionComponentProps) -> Element;
}

/// Motion Div component
pub struct DivElement;

impl MotionElement for DivElement {
    #[component]
    fn create(props: MotionComponentProps) -> Element {
        let setup_result = setup_motion_component(&props);
        
        rsx! {
            div {
                id: props.id.clone(),
                class: props.class.clone(),
                style: setup_result.combined_style,
                "data-layout": if props.layout { "true" } else { "false" },
                onclick: move |e| {
                    if let Some(handler) = &props.onclick {
                        handler.call(e)
                    }
                },
                onmouseenter: setup_result.on_mouse_enter,
                onmouseleave: setup_result.on_mouse_leave,
                onmousedown: setup_result.on_mouse_down,
                onmouseup: setup_result.on_mouse_up,
                {props.children}
            }
        }
    }
}

/// Motion Span component
pub struct SpanElement;

impl MotionElement for SpanElement {
    #[component]
    fn create(props: MotionComponentProps) -> Element {
        let setup_result = setup_motion_component(&props);
        
        rsx! {
            span {
                id: props.id.clone(),
                class: props.class.clone(),
                style: setup_result.combined_style,
                "data-layout": if props.layout { "true" } else { "false" },
                onclick: move |e| {
                    if let Some(handler) = &props.onclick {
                        handler.call(e)
                    }
                },
                onmouseenter: setup_result.on_mouse_enter,
                onmouseleave: setup_result.on_mouse_leave,
                onmousedown: setup_result.on_mouse_down,
                onmouseup: setup_result.on_mouse_up,
                {props.children}
            }
        }
    }
}

/// Motion Button component
pub struct ButtonElement;

impl MotionElement for ButtonElement {
    #[component]
    fn create(props: MotionComponentProps) -> Element {
        let setup_result = setup_motion_component(&props);
        
        rsx! {
            button {
                id: props.id.clone(),
                class: props.class.clone(),
                style: setup_result.combined_style,
                "data-layout": if props.layout { "true" } else { "false" },
                onclick: move |e| {
                    if let Some(handler) = &props.onclick {
                        handler.call(e)
                    }
                },
                onmouseenter: setup_result.on_mouse_enter,
                onmouseleave: setup_result.on_mouse_leave,
                onmousedown: setup_result.on_mouse_down,
                onmouseup: setup_result.on_mouse_up,
                {props.children}
            }
        }
    }
}

// Export component functions
#[component]
pub fn Div(props: MotionComponentProps) -> Element {
    DivElement::create(props)
}

#[component]
pub fn Span(props: MotionComponentProps) -> Element {
    SpanElement::create(props)
}

#[component]
pub fn Button(props: MotionComponentProps) -> Element {
    ButtonElement::create(props)
}
