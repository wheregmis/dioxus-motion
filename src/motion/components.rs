use super::base::{MotionComponentProps, setup_motion_component};
use dioxus::prelude::*;

/// Trait for HTML elements that can be animated
pub trait MotionElement {
    /// Create a motion component from props
    fn create(props: MotionComponentProps) -> Element;
}

/// Helper macro to reduce boilerplate for motion element implementations
macro_rules! impl_motion_element {
    ($element:ident, $tag:ident) => {
        pub struct $element;

        impl MotionElement for $element {
            #[component]
            fn create(props: MotionComponentProps) -> Element {
                let setup_result = setup_motion_component(&props);

                rsx! {
                    $tag {
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
                        ..props.attributes,
                        {props.children}
                    }
                }
            }
        }
    };
}

// Implement common HTML elements
impl_motion_element!(DivElement, div);
impl_motion_element!(SpanElement, span);
impl_motion_element!(ButtonElement, button);
impl_motion_element!(ParagraphElement, p);
impl_motion_element!(ImageElement, img);
impl_motion_element!(AnchorElement, a);
impl_motion_element!(H1Element, h1);
impl_motion_element!(H2Element, h2);
impl_motion_element!(H3Element, h3);
impl_motion_element!(H4Element, h4);
impl_motion_element!(H5Element, h5);
impl_motion_element!(H6Element, h6);
impl_motion_element!(UlElement, ul);
impl_motion_element!(OlElement, ol);
impl_motion_element!(LiElement, li);
impl_motion_element!(SectionElement, section);
impl_motion_element!(ArticleElement, article);
impl_motion_element!(AsideElement, aside);
impl_motion_element!(HeaderElement, header);
impl_motion_element!(FooterElement, footer);
impl_motion_element!(NavElement, nav);
impl_motion_element!(MainElement, main);
impl_motion_element!(FormElement, form);

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

#[component]
pub fn P(props: MotionComponentProps) -> Element {
    ParagraphElement::create(props)
}

#[component]
pub fn Img(props: MotionComponentProps) -> Element {
    ImageElement::create(props)
}

#[component]
pub fn A(props: MotionComponentProps) -> Element {
    AnchorElement::create(props)
}

#[component]
pub fn H1(props: MotionComponentProps) -> Element {
    H1Element::create(props)
}

#[component]
pub fn H2(props: MotionComponentProps) -> Element {
    H2Element::create(props)
}

#[component]
pub fn H3(props: MotionComponentProps) -> Element {
    H3Element::create(props)
}

#[component]
pub fn H4(props: MotionComponentProps) -> Element {
    H4Element::create(props)
}

#[component]
pub fn H5(props: MotionComponentProps) -> Element {
    H5Element::create(props)
}

#[component]
pub fn H6(props: MotionComponentProps) -> Element {
    H6Element::create(props)
}

#[component]
pub fn Ul(props: MotionComponentProps) -> Element {
    UlElement::create(props)
}

#[component]
pub fn Ol(props: MotionComponentProps) -> Element {
    OlElement::create(props)
}

#[component]
pub fn Li(props: MotionComponentProps) -> Element {
    LiElement::create(props)
}

#[component]
pub fn Section(props: MotionComponentProps) -> Element {
    SectionElement::create(props)
}

#[component]
pub fn Article(props: MotionComponentProps) -> Element {
    ArticleElement::create(props)
}

#[component]
pub fn Aside(props: MotionComponentProps) -> Element {
    AsideElement::create(props)
}

#[component]
pub fn Header(props: MotionComponentProps) -> Element {
    HeaderElement::create(props)
}

#[component]
pub fn Footer(props: MotionComponentProps) -> Element {
    FooterElement::create(props)
}

#[component]
pub fn Nav(props: MotionComponentProps) -> Element {
    NavElement::create(props)
}

#[component]
pub fn Main(props: MotionComponentProps) -> Element {
    MainElement::create(props)
}

#[component]
pub fn Form(props: MotionComponentProps) -> Element {
    FormElement::create(props)
}
