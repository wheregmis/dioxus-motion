use crate::old_showcase::components::{
    AnimatedCounter, AnimatedFlower, AnimatedMenuItem, BouncingText, Card3DFlip, InteractiveCube,
    MorphingShape, PathAnimation, ProgressBar, PulseEffect, RotatingButton, SwingingCube,
    TransformAnimationShowcase, TypewriterEffect, ValueAnimationShowcase,
};

use dioxus::prelude::*;

use crate::components::code_block::CodeBlock;
use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger, Tabs};

#[component]
pub fn ShowcaseGallery() -> Element {
    let mut selected = use_signal(|| 4usize);
    let mut active = use_signal(|| false);
    let mut tab = use_signal(|| Some("preview".to_string()));
    let examples = showcase_items();
    let (title, preview, filename, source) = examples[selected()].clone();
    let source_url = format!(
        "https://github.com/wheregmis/dioxus-motion/blob/new_release/docs/src/old_showcase/components/{filename}"
    );
    rsx! {
        document::Title { "Examples · Dioxus Motion" }
        main { id: "main-content", class: "examples-shell",
            p { class: "eyebrow", "EXAMPLES" }
            h1 { "A little inspiration." }
            p { class: "lead", "Explore one animation at a time. Read the exact source, then make it your own." }
            div { class: "examples-layout",
                nav { class: "example-picker", aria_label: "Choose an example",
                    for (index, (name, _, _, _)) in examples.iter().enumerate() {
                        button { class: if selected() == index { "selected" } else { "" }, aria_pressed: selected() == index,
                            onclick: move |_| { selected.set(index); active.set(false); tab.set(Some("preview".to_string())); },
                            {*name}
                        }
                    }
                }
                div { class: "example-detail",
                    div { class: "example-heading", h2 { {title} } a { href: source_url, "Source ↗" } }
                    Tabs { value: tab, on_value_change: move |value: String| { tab.set(Some(value.clone())); if value == "code" { active.set(false); } }, horizontal: true,
                        TabList { class: "preview-tabs", aria_label: "Example view",
                            TabTrigger { value: "preview", index: 0usize, id: None, class: None, "Preview" }
                            TabTrigger { value: "code", index: 1usize, id: None, class: None, "Code" }
                        }
                        TabContent { value: "preview", index: 0usize, id: None, class: None,
                            div { class: "example-stage",
                                if active() { {preview} }
                                else { div { class: "example-placeholder", p { "Ready when you are." } button { class: "button primary", onclick: move |_| active.set(true), "Start demo →" } } }
                            }
                            if active() { div { class: "example-stop", button { class: "button secondary", onclick: move |_| active.set(false), "Stop demo" } span { "Starting again resets the example." } } }
                        }
                        TabContent { value: "code", index: 1usize, id: None, class: Some("example-source".to_string()),
                            CodeBlock { language: "Rust".to_string(), code: source }
                        }
                    }
                }
            }
        }
    }
}

fn showcase_items() -> Vec<(
    &'static str,
    Element,
    &'static str,
    dioxus_code::advanced::HighlightedSource,
)> {
    vec![
        (
            "Cube Animation",
            rsx!(SwingingCube {}),
            "cube_animation.rs",
            dioxus_code::code!("/src/old_showcase/components/cube_animation.rs"),
        ),
        (
            "Flower Animation",
            rsx!(AnimatedFlower {}),
            "animated_flower.rs",
            dioxus_code::code!("/src/old_showcase/components/animated_flower.rs"),
        ),
        (
            "Morphing Shape",
            rsx!(MorphingShape {
                shapes: vec!["square", "triangle"],
                duration: 3.0
            }),
            "morphing_shape.rs",
            dioxus_code::code!("/src/old_showcase/components/morphing_shape.rs"),
        ),
        (
            "Interactive Cube",
            rsx!(InteractiveCube {}),
            "interactive_cube.rs",
            dioxus_code::code!("/src/old_showcase/components/interactive_cube.rs"),
        ),
        (
            "Value Animation",
            rsx!(ValueAnimationShowcase {}),
            "value_animation.rs",
            dioxus_code::code!("/src/old_showcase/components/value_animation.rs"),
        ),
        (
            "Transform Animation",
            rsx!(TransformAnimationShowcase {}),
            "transform_animation.rs",
            dioxus_code::code!("/src/old_showcase/components/transform_animation.rs"),
        ),
        (
            "Animated Menu Bar",
            rsx!(
                section { class: "",
                    p { class: "text-gray-600", "Shows smooth transitions on hover" }
                    div { class: "space-y-2",
                        AnimatedMenuItem { label: "Home" }
                        AnimatedMenuItem { label: "About" }
                        AnimatedMenuItem { label: "Contact" }
                    }
                }
            ),
            "animated_menu_item.rs",
            dioxus_code::code!("/src/old_showcase/components/animated_menu_item.rs"),
        ),
        (
            "Rotating Button",
            rsx!(RotatingButton {}),
            "rotating_button.rs",
            dioxus_code::code!("/src/old_showcase/components/rotating_button.rs"),
        ),
        (
            "Progress Animation",
            rsx!(ProgressBar {
                title: "Loading..."
            }),
            "progress_bar.rs",
            dioxus_code::code!("/src/old_showcase/components/progress_bar.rs"),
        ),
        (
            "Bouncing Text",
            rsx!(BouncingText {
                text: "Dioxus Motion"
            }),
            "bouncing_text.rs",
            dioxus_code::code!("/src/old_showcase/components/bouncing_text.rs"),
        ),
        (
            "Path Animation",
            rsx!(PathAnimation {
                path: "M10 80 C 40 10, 65 10, 95 80 S 150 150, 180 80",
                duration: 5.0
            }),
            "path_animation.rs",
            dioxus_code::code!("/src/old_showcase/components/path_animation.rs"),
        ),
        (
            "Pulse Effect",
            rsx!(PulseEffect {
                color: "bg-blue-500",
                size: "w-16 h-16"
            }),
            "pulse_effect.rs",
            dioxus_code::code!("/src/old_showcase/components/pulse_effect.rs"),
        ),
        (
            "3D Card Flip",
            rsx!(Card3DFlip {}),
            "card_3d_flip.rs",
            dioxus_code::code!("/src/old_showcase/components/card_3d_flip.rs"),
        ),
        (
            "Typewriter Effect",
            rsx!(TypewriterEffect {
                text: "Hello, Dioxus Motion"
            }),
            "typewriter_effect.rs",
            dioxus_code::code!("/src/old_showcase/components/typewriter_effect.rs"),
        ),
        (
            "Counter Animation",
            rsx!(AnimatedCounter {}),
            "animated_counter.rs",
            dioxus_code::code!("/src/old_showcase/components/animated_counter.rs"),
        ),
    ]
}
