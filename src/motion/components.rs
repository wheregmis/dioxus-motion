//! Motion component exports for Dioxus Motion
//!
//! This module provides all motion-enabled HTML elements as Dioxus components.
use super::base::{MotionComponentProps, setup_motion_component};
use dioxus::prelude::*;

/// Trait for HTML elements that can be animated
pub trait MotionElement {
    /// Create a motion component from props
    fn create(props: MotionComponentProps) -> Element;
}

/// Macro to generate a motion element struct, trait impl, and export function.
///
/// Usage: motion_component!(DivElement, div, Div);
///
/// To add a new motion component, add a line below with the element struct, tag, and export name.
macro_rules! motion_component {
    ($element:ident, $tag:ident, $export:ident) => {
        pub struct $element;

        impl MotionElement for $element {
            #[component]
            fn create(props: MotionComponentProps) -> Element {
                let setup_result = setup_motion_component(&props);
                rsx! {
                    $tag {
                        id: props.id.clone(),
                        class: props.class.clone(),
                        style: setup_result.combined_style.clone(),
                        onclick: move |e| {
                            if let Some(handler) = &props.onclick {
                                handler.call(e)
                            }
                        },
                        onmouseenter: setup_result.on_mouse_enter,
                        onmouseleave: setup_result.on_mouse_leave,
                        onmousedown: setup_result.on_mouse_down,
                        onmouseup: setup_result.on_mouse_up,
                        ..props.attributes,
                        {props.children}
                    }
                }
            }
        }

        #[component]
        pub fn $export(props: MotionComponentProps) -> Element {
            $element::create(props)
        }
    };
}

// To add a new motion component, add a line below:
motion_component!(DivElement, div, Div);
motion_component!(SpanElement, span, Span);
motion_component!(ButtonElement, button, Button);
motion_component!(ParagraphElement, p, P);
motion_component!(ImageElement, img, Img);
motion_component!(AnchorElement, a, A);
motion_component!(H1Element, h1, H1);
motion_component!(H2Element, h2, H2);
motion_component!(H3Element, h3, H3);
motion_component!(H4Element, h4, H4);
motion_component!(H5Element, h5, H5);
motion_component!(H6Element, h6, H6);
motion_component!(UlElement, ul, Ul);
motion_component!(OlElement, ol, Ol);
motion_component!(LiElement, li, Li);
motion_component!(SectionElement, section, Section);
motion_component!(ArticleElement, article, Article);
motion_component!(AsideElement, aside, Aside);
motion_component!(HeaderElement, header, Header);
motion_component!(FooterElement, footer, Footer);
motion_component!(NavElement, nav, Nav);
motion_component!(MainElement, main, Main);
// Form is included for API completeness, even if not commonly used.
motion_component!(FormElement, form, Form);

// MotionList and MotionStagger components have been removed to simplify the library

// MotionLayout component has been removed as part of the layout feature cleanup
