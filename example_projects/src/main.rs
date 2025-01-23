use std::vec;

use dioxus::prelude::*;

use dioxus_motion::colors::Color;
use example_projects::components::{
    AnimatedCounter, AnimatedFlower, AnimatedMenuItem, BouncingText, Card3DFlip, ColorAnimation,
    InteractiveCube, MorphingShape, Navbar, PathAnimation, ProgressBar, PulseEffect,
    RotatingButton, SwingingCube, TransformAnimationShowcase, TypewriterEffect,
    ValueAnimationShowcase,
};
use example_projects::BG_COLOR;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ShowcaseGallery {}
    }
}

fn is_dark_background(color: Color) -> bool {
    let (r, g, b, _) = color.to_rgba();
    // Calculate relative luminance using sRGB formula
    let luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
    luminance < 0.5
}

#[component]
pub fn ShowcaseGallery() -> Element {
    rsx! {
        div {
            class: if is_dark_background(*BG_COLOR.read()) { "text-white" } else { "text-black" },
            class: "flex flex-col min-h-screen relative",
            style: "background-color: rgba({BG_COLOR.read().to_rgba().0},
                {BG_COLOR.read().to_rgba().1}, 
                {BG_COLOR.read().to_rgba().2}, 1);",
            Navbar {}
            // Rest of the content
            div { class: "flex-grow mt-16",
                div { class: "container mx-auto px-8 py-12 pt-20",
                    div {
                        h2 { class: "text-xl font-bold", "Animated Counter" }
                        AnimatedCounter {}
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        // Update each card with fixed height and overflow control
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Cube Animation" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                SwingingCube {}
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/cube_animation.rs" }
                        }

                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Flower Animation" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                AnimatedFlower {}
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/animated_flower.rs" }
                        }

                        // Morphing Shape Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Morphing Shape" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                MorphingShape {
                                    shapes: vec!["square", "triangle"],
                                    duration: 3.0,
                                }
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/morphing_shape.rs" }
                        }

                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Interactive Cube" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                InteractiveCube {}
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/interactive_cube.rs" }
                        }

                        // Value Animation Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-blue-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Value Animation" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                ValueAnimationShowcase {}
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/value_animation.rs" }
                        }

                        // Transform Animation Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-purple-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Transform Animation" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                TransformAnimationShowcase {}
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/transform_animation.rs" }
                        }

                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Animated Menu Bar" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                section { class: "",
                                    p { class: "text-gray-600", "Shows smooth transitions on hover" }
                                    div { class: "space-y-2",
                                        AnimatedMenuItem { label: "Home" }
                                        AnimatedMenuItem { label: "About" }
                                        AnimatedMenuItem { label: "Contact" }
                                    }
                                }
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/animated_menu_item.rs" }
                        }

                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Rotating Button" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                RotatingButton {}
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/rotating_button.rs" }
                        }

                        // Progress Bar Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Progress Animation" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                ProgressBar { title: "Loading..." }
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/progress_bar.rs" }
                        }

                        // Bouncing Text Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Bouncing Text" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                BouncingText { text: "Dioxus Motion" }
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/bouncing_text.rs" }
                        }

                        // Path Animation Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Path Animation" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                PathAnimation {
                                    path: "M10 80 C 40 10, 65 10, 95 80 S 150 150, 180 80",
                                    duration: 5.0,
                                }
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/path_animation.rs" }
                        }

                        // Pulse Effect Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Pulse Effect" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                PulseEffect { color: "bg-blue-500", size: "w-16 h-16" }
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/pulse_effect.rs" }
                        }

                        // Card 3D Flip Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "3D Card Flip" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                Card3DFlip {}
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/card_3d_flip.rs" }
                        }

                        // Typewriter Effect Card
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Typewriter Effect" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                TypewriterEffect { text: "Hello, Dioxus Motion" }
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/typewriter_effect.rs" }
                        }

                        // Color Animation
                        div { class: "flex flex-col items-start justify-between h-[400px] rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                            h3 { class: "text-lg font-semibold  mb-4 w-full", "Color Animation" }
                            div {
                                class: "flex-grow w-full flex items-center justify-center my-4 overflow-hidden",
                                style: "max-height: 280px;",
                                ColorAnimation {}
                            }
                            ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/color_animation.rs" }
                        }
                    }
                }
            }

            // Footer with gradient border
            footer { class: " border-t border-gradient-to-r from-blue-500/20 to-purple-500/20 py-8 mt-auto",
                div { class: "max-w-6xl mx-auto px-4 text-center",
                    p { class: "text-sm ", "© 2024 Sabin Regmi. No Rights Reserved." }
                    p { class: "text-xs  mt-2", "Built with ❤️ using Dioxus" }
                }
            }
        }
    }
}

#[component]
fn ViewCodeButton(url: String) -> Element {
    rsx! {
        a {
            class: "inline-flex items-center px-4 py-2 mt-4 text-sm font-medium text-blue-600 bg-blue-50 rounded-lg hover:bg-blue-100 transition-colors",
            href: "{url}",
            target: "_blank",
            span { class: "mr-2", "Example Code" }
            span { class: "text-xs", "→" }
        }
    }
}
