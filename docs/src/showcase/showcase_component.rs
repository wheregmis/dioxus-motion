use crate::showcase::components::{
    AnimatedCounter, AnimatedFlower, AnimatedMenuItem, BouncingText, Card3DFlip, InteractiveCube,
    MorphingShape, PathAnimation, ProgressBar, PulseEffect, RotatingButton, SwingingCube,
    TransformAnimationShowcase, TypewriterEffect, ValueAnimationShowcase,
};

use dioxus::prelude::*;

use crate::components::code_block::CodeBlock;
use crate::utils::router::Route;
use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger, Tabs};

/// Picker labels and URL slugs, ordered to match `showcase_items` below.
/// Static so picker button handlers can capture slugs.
const PICKER: &[(&str, &str)] = &[
    ("Magnetic Card", "magnetic-card"),
    ("Interactive Cube", "interactive-cube"),
    ("Spring Nav", "spring-nav"),
    ("Comet Path", "comet-path"),
    ("Sonar", "sonar"),
    ("Swinging Cube", "swinging-cube"),
    ("Liquid Morph", "liquid-morph"),
    ("Holo Card Flip", "holo-card-flip"),
    ("Spring Gauge", "spring-gauge"),
    ("Flower", "flower"),
    ("Wave Text", "wave-text"),
    ("Terminal", "terminal"),
    ("Charge Bar", "charge-bar"),
    ("Launch Button", "launch-button"),
    ("Momentum Counter", "momentum-counter"),
];

/// Example gallery: picker, live preview, and highlighted source for each demo.
/// The selected demo lives in the URL (`?demo=slug`) so previews are deep-linkable
/// and browser Back/Forward moves between demos.
#[component]
pub fn ShowcaseGallery(demo: Option<String>) -> Element {
    let mut tab = use_signal(|| Some("preview".to_string()));
    let nav = use_navigator();
    let selected = demo
        .as_deref()
        .and_then(|slug| PICKER.iter().position(|(_, s)| *s == slug))
        .unwrap_or(0);
    let examples = showcase_items();
    let (title, blurb, preview, filename, source) = examples[selected].clone();
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
                    for (index, (title, slug)) in PICKER.iter().enumerate() {
                        button { class: if selected == index { "selected" } else { "" }, aria_pressed: selected == index,
                            onclick: move |_| {
                                nav.push(Route::ShowcaseGallery { demo: Some(slug.to_string()) });
                                tab.set(Some("preview".to_string()));
                            },
                            {*title}
                        }
                    }
                }
                div { class: "example-detail",
                    div { class: "example-heading",
                        div { h2 { {title} } p { class: "example-blurb", {blurb} } }
                        a { href: source_url, "Source ↗" }
                    }
                    Tabs { value: tab, on_value_change: move |value: String| tab.set(Some(value)), horizontal: true,
                        TabList { class: "preview-tabs", aria_label: "Example view",
                            TabTrigger { value: "preview", index: 0usize, id: None, class: None, "Preview" }
                            TabTrigger { value: "code", index: 1usize, id: None, class: None, "Code" }
                        }
                        TabContent { value: "preview", index: 0usize, id: None, class: None,
                            // Demos mount on selection and unmount on the Code tab,
                            // so animations only run while the preview is visible
                            div { class: "example-stage",
                                if tab() == Some("preview".to_string()) { {preview} }
                            }
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

/// Every gallery entry: display title, one-line description, preview element,
/// source filename, and its compile-time highlighted code. Order must match PICKER.
fn showcase_items() -> Vec<(
    &'static str,
    &'static str,
    Element,
    &'static str,
    dioxus_code::advanced::HighlightedSource,
)> {
    vec![
        (
            "Magnetic Card",
            "Three springs retarget every frame — the card chases your cursor and settles without snapping.",
            rsx!(TransformAnimationShowcase {}),
            "transform_animation.rs",
            dioxus_code::code!("/src/showcase/components/transform_animation.rs"),
        ),
        (
            "Interactive Cube",
            "Hover tilts a glass 3D cube; click chains a spin, squash, and wobble through sequence steps.",
            rsx!(InteractiveCube {}),
            "interactive_cube.rs",
            dioxus_code::code!("/src/showcase/components/interactive_cube.rs"),
        ),
        (
            "Spring Nav",
            "A pill indicator springs between nav items — the shared-element feel with two scalar springs.",
            rsx!(AnimatedMenuItem {}),
            "animated_menu_item.rs",
            dioxus_code::code!("/src/showcase/components/animated_menu_item.rs"),
        ),
        (
            "Comet Path",
            "A glowing comet rides an SVG path on offset-path while a tween draws its trailing wake.",
            rsx!(PathAnimation {
                path: "M10 160 C 70 20, 140 30, 170 105 S 270 190, 310 55",
                duration: 4.5
            }),
            "path_animation.rs",
            dioxus_code::code!("/src/showcase/components/path_animation.rs"),
        ),
        (
            "Sonar",
            "A rotating conic sweep over three staggered rings, each expanding and fading on its own tween.",
            rsx!(PulseEffect {}),
            "pulse_effect.rs",
            dioxus_code::code!("/src/showcase/components/pulse_effect.rs"),
        ),
        (
            "Swinging Cube",
            "A custom Transform3D value springs between poses and projects into SVG — define Animatable yourself.",
            rsx!(SwingingCube {}),
            "cube_animation.rs",
            dioxus_code::code!("/src/showcase/components/cube_animation.rs"),
        ),
        (
            "Liquid Morph",
            "clip-path crossfades between silhouettes while a slow infinite spin keeps the blob drifting.",
            rsx!(MorphingShape {}),
            "morphing_shape.rs",
            dioxus_code::code!("/src/showcase/components/morphing_shape.rs"),
        ),
        (
            "Holo Card Flip",
            "Click springs a 3D flip — the foil shine sweeps across at the same angle for depth.",
            rsx!(Card3DFlip {}),
            "card_3d_flip.rs",
            dioxus_code::code!("/src/showcase/components/card_3d_flip.rs"),
        ),
        (
            "Spring Gauge",
            "Preset buttons spring the needle to each mark — retarget mid-swing and it still lands clean.",
            rsx!(ValueAnimationShowcase {}),
            "value_animation.rs",
            dioxus_code::code!("/src/showcase/components/value_animation.rs"),
        ),
        (
            "Flower",
            "Staged springs grow the stem, leaves, and petals in sequence, then sway on an alternate loop.",
            rsx!(AnimatedFlower {}),
            "animated_flower.rs",
            dioxus_code::code!("/src/showcase/components/animated_flower.rs"),
        ),
        (
            "Wave Text",
            "Each letter is its own component with a staggered sine loop — scale, rise, and shadow in sync.",
            rsx!(BouncingText { text: "MOTION" }),
            "bouncing_text.rs",
            dioxus_code::code!("/src/showcase/components/bouncing_text.rs"),
        ),
        (
            "Terminal",
            "A linear tween types each character while a second alternating tween blinks the block cursor.",
            rsx!(TypewriterEffect {
                text: "cargo add dioxus-motion"
            }),
            "typewriter_effect.rs",
            dioxus_code::code!("/src/showcase/components/typewriter_effect.rs"),
        ),
        (
            "Charge Bar",
            "A looping tween fills the bar while the glowing tip and tick marks ride the same value.",
            rsx!(ProgressBar {}),
            "progress_bar.rs",
            dioxus_code::code!("/src/showcase/components/progress_bar.rs"),
        ),
        (
            "Launch Button",
            "Three keyframe tracks play on one press: squash-and-stretch, a full spin, and a ripple ring.",
            rsx!(RotatingButton {}),
            "rotating_button.rs",
            dioxus_code::code!("/src/showcase/components/rotating_button.rs"),
        ),
        (
            "Momentum Counter",
            "The number springs to each new total and pops on impact, with a meter filling to the next level.",
            rsx!(AnimatedCounter {}),
            "animated_counter.rs",
            dioxus_code::code!("/src/showcase/components/animated_counter.rs"),
        ),
    ]
}
