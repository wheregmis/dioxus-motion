use crate::showcase::components::{
    AnimatedCounter, AnimatedFlower, AnimatedMenuItem, BouncingText, Card3DFlip, InteractiveCube,
    MorphingShape, PathAnimation, ProgressBar, PulseEffect, RotatingButton, SwingingCube,
    TransformAnimationShowcase, TypewriterEffect, ValueAnimationShowcase,
};

use dioxus::prelude::*;

use crate::components::code_block::CodeBlock;
use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger, Tabs};

#[component]
pub fn ShowcaseGallery() -> Element {
    let mut selected = use_signal(|| 0usize);
    let mut active = use_signal(|| false);
    let mut tab = use_signal(|| Some("preview".to_string()));
    let examples = showcase_items();
    let (title, blurb, preview, filename, source) = examples[selected()].clone();
    let source_url = format!(
        "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/showcase/components/{filename}"
    );
    rsx! {
        document::Title { "Examples · Dioxus Motion" }
        main { id: "main-content", class: "examples-shell",
            p { class: "eyebrow", "EXAMPLES" }
            h1 { "A little inspiration." }
            p { class: "lead", "Explore one animation at a time. Read the exact source, then make it your own." }
            div { class: "examples-layout",
                nav { class: "example-picker", aria_label: "Choose an example",
                    for (index, item) in examples.iter().enumerate() {
                        button { class: if selected() == index { "selected" } else { "" }, aria_pressed: selected() == index,
                            onclick: move |_| { selected.set(index); active.set(false); tab.set(Some("preview".to_string())); },
                            {item.0}
                        }
                    }
                }
                div { class: "example-detail",
                    div { class: "example-heading",
                        div { h2 { {title} } p { class: "example-blurb", {blurb} } }
                        a { href: source_url, "Source ↗" }
                    }
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
    &'static str,
    Element,
    &'static str,
    dioxus_code::advanced::HighlightedSource,
)> {
    vec![
        (
            "Cube Animation",
            "A custom Transform3D value springing between poses, projected into SVG.",
            rsx!(SwingingCube {}),
            "cube_animation.rs",
            dioxus_code::code!("/src/showcase/components/cube_animation.rs"),
        ),
        (
            "Flower Animation",
            "Coordinated springs grow the stem, petals, and center in stages.",
            rsx!(AnimatedFlower {}),
            "animated_flower.rs",
            dioxus_code::code!("/src/showcase/components/animated_flower.rs"),
        ),
        (
            "Morphing Shape",
            "A Transform spring rotates and scales between clip-path silhouettes.",
            rsx!(MorphingShape {
                shapes: vec!["square", "triangle"],
                duration: 3.0
            }),
            "morphing_shape.rs",
            dioxus_code::code!("/src/showcase/components/morphing_shape.rs"),
        ),
        (
            "Interactive Cube",
            "Click to chain a spin and bounce with AnimationSequence steps.",
            rsx!(InteractiveCube {}),
            "interactive_cube.rs",
            dioxus_code::code!("/src/showcase/components/interactive_cube.rs"),
        ),
        (
            "Value Animation",
            "A scalar tween with easing drives a readout and a progress bar.",
            rsx!(ValueAnimationShowcase {}),
            "value_animation.rs",
            dioxus_code::code!("/src/showcase/components/value_animation.rs"),
        ),
        (
            "Transform Animation",
            "Hover springs a typed Transform: translate, scale, and rotate together.",
            rsx!(TransformAnimationShowcase {}),
            "transform_animation.rs",
            dioxus_code::code!("/src/showcase/components/transform_animation.rs"),
        ),
        (
            "Animated Menu Bar",
            "Each item springs its offset, scale, and glow on hover.",
            rsx!(
                section {
                    p { class: "text-gray-600", "Shows smooth transitions on hover" }
                    div { class: "space-y-2",
                        AnimatedMenuItem { label: "Home" }
                        AnimatedMenuItem { label: "About" }
                        AnimatedMenuItem { label: "Contact" }
                    }
                }
            ),
            "animated_menu_item.rs",
            dioxus_code::code!("/src/showcase/components/animated_menu_item.rs"),
        ),
        (
            "Rotating Button",
            "A press spins the button a full turn with a single spring.",
            rsx!(RotatingButton {}),
            "rotating_button.rs",
            dioxus_code::code!("/src/showcase/components/rotating_button.rs"),
        ),
        (
            "Progress Animation",
            "A tween fills the bar while the same value drives the label.",
            rsx!(ProgressBar {
                title: "Loading..."
            }),
            "progress_bar.rs",
            dioxus_code::code!("/src/showcase/components/progress_bar.rs"),
        ),
        (
            "Bouncing Text",
            "Letters stagger their springs for a playful entrance.",
            rsx!(BouncingText {
                text: "Dioxus Motion"
            }),
            "bouncing_text.rs",
            dioxus_code::code!("/src/showcase/components/bouncing_text.rs"),
        ),
        (
            "Path Animation",
            "A dot follows an SVG path while a tween draws the stroke.",
            rsx!(PathAnimation {
                path: "M10 80 C 40 10, 65 10, 95 80 S 150 150, 180 80",
                duration: 5.0
            }),
            "path_animation.rs",
            dioxus_code::code!("/src/showcase/components/path_animation.rs"),
        ),
        (
            "Pulse Effect",
            "An alternating spring breathes scale and opacity on a loop.",
            rsx!(PulseEffect {
                color: "bg-blue-500",
                size: "w-16 h-16"
            }),
            "pulse_effect.rs",
            dioxus_code::code!("/src/showcase/components/pulse_effect.rs"),
        ),
        (
            "3D Card Flip",
            "Hover flips the card with a spring on a single rotation value.",
            rsx!(Card3DFlip {}),
            "card_3d_flip.rs",
            dioxus_code::code!("/src/showcase/components/card_3d_flip.rs"),
        ),
        (
            "Typewriter Effect",
            "Sequenced tweens type each character, then erase and repeat.",
            rsx!(TypewriterEffect {
                text: "Hello, Dioxus Motion"
            }),
            "typewriter_effect.rs",
            dioxus_code::code!("/src/showcase/components/typewriter_effect.rs"),
        ),
        (
            "Counter Animation",
            "A spring eases the number toward each new count you choose.",
            rsx!(AnimatedCounter {}),
            "animated_counter.rs",
            dioxus_code::code!("/src/showcase/components/animated_counter.rs"),
        ),
    ]
}
