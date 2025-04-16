use crate::components::footer::Footer;
use crate::old_showcase::components::{
    AnimatedFlower, AnimatedMenuItem, BouncingText, Card3DFlip, InteractiveCube, MorphingShape,
    PathAnimation, ProgressBar, PulseEffect, RotatingButton, SwingingCube,
    TransformAnimationShowcase, TypewriterEffect, ValueAnimationShowcase,
};

use dioxus::prelude::*;

#[component]
pub fn ShowcaseGallery() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen relative bg-gradient-dark",
            div { class: "flex-grow mt-16",
                div { class: "container-lg mx-auto px-8 py-12 pt-20",
                    // div {
                    //     h2 { class: "text-xl font-display font-bold text-text-primary",
                    //         "Animated Counter"
                    //     }
                    //     AnimatedCounter {}
                    // }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        // Update each card with our theme
                        for (title , component , url) in showcase_items() {
                            ShowcaseCard { title, url, component }
                        }
                    }
                }
            }

            // Footer
            Footer {}
        }
    }
}

#[component]
fn ShowcaseCard(title: String, url: String, component: Element) -> Element {
    rsx! {
        div {
            class: "group relative flex flex-col items-start justify-between h-[400px]",
            class: "rounded-xl border border-surface-light/10 backdrop-blur-sm",
            class: "bg-surface/30 hover:bg-surface-light/10",
            class: "transition-all duration-500 ease-out",
            // Gradient glow effect on hover
            div {
                class: "absolute inset-0 rounded-xl opacity-0 group-hover:opacity-100",
                class: "bg-gradient-to-r from-primary/20 to-accent-purple/20",
                class: "transition-opacity duration-500 ease-out -z-10",
            }
            // Title section
            h3 {
                class: "text-lg font-display font-semibold text-text-primary mb-4 w-full p-4",
                class: "border-b border-surface-light/10",
                "{title}"
            }
            // Component showcase area
            div {
                class: "flex-grow w-full flex items-center justify-center p-4 overflow-hidden",
                class: "group-hover:scale-105 transition-transform duration-500 ease-out",
                style: "max-height: 280px;",
                {component}
            }
            // Button section with gradient border
            div { class: "w-full p-4 border-t border-surface-light/10",
                ViewCodeButton { url }
            }
        }
    }
}

#[component]
fn ViewCodeButton(url: String) -> Element {
    rsx! {
        a {
            class: "group relative inline-flex items-center gap-2 w-full",
            class: "px-4 py-2 rounded-lg font-medium",
            class: "bg-surface hover:bg-surface-light/20",
            class: "text-text-primary hover:text-primary-light",
            class: "transition-all duration-300 ease-out",
            href: "{url}",
            target: "_blank",
            // Button content
            span { class: "transition-transform duration-300 group-hover:translate-x-1",
                "View Example Code"
            }
            // Arrow icon with animation
            span {
                class: "text-xs transition-all duration-300",
                class: "transform group-hover:translate-x-1 group-hover:text-accent-purple",
                "→"
            }
        }
    }
}

// Helper function to organize showcase items
fn showcase_items() -> Vec<(&'static str, Element, &'static str)> {
    vec![
        (
            "Cube Animation",
            rsx!(SwingingCube {}),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/cube_animation.rs",
        ),
        (
            "Flower Animation",
            rsx!(AnimatedFlower {}),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/animated_flower.rs",
        ),
        (
            "Morphing Shape",
            rsx!(MorphingShape {
                shapes: vec!["square", "triangle"],
                duration: 3.0
            }),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/morphing_shape.rs",
        ),
        (
            "Interactive Cube",
            rsx!(InteractiveCube {}),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/interactive_cube.rs",
        ),
        (
            "Value Animation",
            rsx!(ValueAnimationShowcase {}),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/value_animation.rs",
        ),
        (
            "Transform Animation",
            rsx!(TransformAnimationShowcase {}),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/transform_animation.rs",
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
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/animated_menu_item.rs",
        ),
        (
            "Rotating Button",
            rsx!(RotatingButton {}),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/rotating_button.rs",
        ),
        (
            "Progress Animation",
            rsx!(ProgressBar {
                title: "Loading..."
            }),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/progress_bar.rs",
        ),
        (
            "Bouncing Text",
            rsx!(BouncingText {
                text: "Dioxus Motion"
            }),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/bouncing_text.rs",
        ),
        (
            "Path Animation",
            rsx!(PathAnimation {
                path: "M10 80 C 40 10, 65 10, 95 80 S 150 150, 180 80",
                duration: 5.0
            }),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/path_animation.rs",
        ),
        (
            "Pulse Effect",
            rsx!(PulseEffect {
                color: "bg-blue-500",
                size: "w-16 h-16"
            }),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/pulse_effect.rs",
        ),
        (
            "3D Card Flip",
            rsx!(Card3DFlip {}),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/card_3d_flip.rs",
        ),
        (
            "Typewriter Effect",
            rsx!(TypewriterEffect {
                text: "Hello, Dioxus Motion"
            }),
            "https://github.com/wheregmis/dioxus-motion/blob/main/docs/src/old_showcase/components/typewriter_effect.rs",
        ),
    ]
}
