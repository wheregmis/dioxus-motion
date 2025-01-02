use std::vec;

use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use example_projects::components::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ShowcaseGallery {}

        for i in 0..3 {
            AnimationExamples {}
        }
    }
}

#[component]
fn AnimationExamples() -> Element {
    let mut value = use_animation(1.0f32);
    let mut position = use_animation(Transform::default());
    let mut color = use_animation(Color::default());

    rsx! {
        div { class: "min-h-screen bg-gray-50 p-8",
            div { class: "max-w-4xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                // Scale Animation Card
                div { class: "bg-white rounded-xl shadow-lg p-6 flex flex-col items-center",
                    h3 { class: "text-lg font-semibold mb-4", "Scale Animation" }
                    div {
                        class: "w-32 h-32 bg-blue-500 rounded-lg cursor-pointer transition-all",
                        style: "transform: scale({value.get_value()})",
                        onmounted: move |_| {
                            value
                                .animate_to(
                                    1.5,
                                    AnimationMode::Tween(Tween {
                                        duration: Duration::from_millis(100),
                                        easing: easer::functions::Sine::ease_in_out,
                                    }),
                                );
                        },
                    }
                    p { class: "mt-4 text-sm text-gray-600", "Hover to scale" }
                }

                // Position Animation Card
                div { class: "bg-red rounded-xl shadow-lg p-6 flex flex-col items-center",
                    h3 { class: "text-lg font-semibold mb-4", "Position Animation" }
                    div { class: "relative w-full h-32 bg-gray-100 rounded-lg",
                        div {
                            class: "absolute w-16 h-16 bg-purple-500 rounded-lg cursor-pointer",
                            style: "transform: translate({position.get_value().x}px, {position.get_value().y}px)",
                            onmounted: move |_| {
                                position
                                    .animate_to(
                                        Transform {
                                            x: 200.0,
                                            y: 50.0,
                                            ..Default::default()
                                        },
                                        AnimationMode::Spring(Spring {
                                            stiffness: 200.0,
                                            damping: 20.0,
                                            ..Default::default()
                                        }),
                                    );
                            },
                        }
                    }
                    p { class: "mt-4 text-sm text-gray-600", "Click to move" }
                }

                // Color Animation Card
                div { class: "bg-white rounded-xl shadow-lg p-6 flex flex-col items-center",
                    h3 { class: "text-lg font-semibold mb-4", "Color Animation" }
                    div {
                        class: "w-32 h-32 rounded-lg cursor-pointer transition-shadow hover:shadow-xl",
                        style: "background-color: rgb({color.get_value().r * 255.0}, {color.get_value().g * 255.0}, {color.get_value().b * 255.0})",
                        onmounted: move |_| {
                            color
                                .animate_to(
                                    Color { r: 1.0, g: 0.5, b: 0.0 },
                                    AnimationMode::Spring(Spring {
                                        stiffness: 150.0,
                                        damping: 15.0,
                                        ..Default::default()
                                    }),
                                );
                        },
                    }
                    p { class: "mt-4 text-sm text-gray-600", "Hover to change color" }
                }
            }
        }
    }
}

#[component]
pub fn ShowcaseGallery() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gradient-to-br from-gray-50 to-gray-100",
            Navbar {}

            // Showcase Grid
            div { class: "container mx-auto px-8 py-12 pt-20",
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    // Value Animation Card
                    div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-blue-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                        h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                            "Value Animation"
                        }
                        div { class: "flex-grow w-full flex items-center justify-center my-4",
                            ValueAnimationShowcase {}
                        }
                        ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/value_animation.rs" }
                    }

                    // Transform Animation Card
                    div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-purple-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                        h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                            "Transform Animation"
                        }
                        div { class: "flex-grow w-full flex items-center justify-center my-4",
                            TransformAnimationShowcase {}
                        }
                        ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/transform_animation.rs" }
                    }

                    // // Progress Bar Card
                    // div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                    //     h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                    //         "Progress Animation"
                    //     }
                    //     div { class: "flex-grow w-full flex items-center justify-center my-4",
                    //         ProgressBar { title: "Loading..." }
                    //     }
                    //     ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/progress_bar.rs" }
                    // }

                    // Morphing Shape Card
                    div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                        h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                            "Morphing Shape"
                        }
                        div { class: "flex-grow w-full flex items-center justify-center my-4",
                            MorphingShape {
                                shapes: vec!["square", "triangle"],
                                duration: 3.0,
                            }
                        }
                        ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/morphing_shape.rs" }
                    }

                    // Bouncing Text Card
                    div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                        h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                            "Bouncing Text"
                        }
                        div { class: "flex-grow w-full flex items-center justify-center my-4",
                            BouncingText { text: "Dioxus Motion" }
                        }
                        ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/bouncing_text.rs" }
                    }

                // // Path Animation Card
                // div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                //     h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                //         "Path Animation"
                //     }
                //     div { class: "flex-grow w-full flex items-center justify-center my-4",
                //         PathAnimation {
                //             path: "M10 80 C 40 10, 65 10, 95 80 S 150 150, 180 80",
                //             duration: 2.0,
                //         }
                //     }
                //     ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/path_animation.rs" }
                // }

                // // Pulse Effect Card
                // div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                //     h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                //         "Pulse Effect"
                //     }
                //     div { class: "flex-grow w-full flex items-center justify-center my-4",
                //         PulseEffect { color: "bg-blue-500", size: "w-16 h-16" }
                //     }
                //     ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/pulse_effect.rs" }
                // }

                // // Card 3D Flip Card
                // div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                //     h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                //         "3D Card Flip"
                //     }
                //     div { class: "flex-grow w-full flex items-center justify-center my-4",
                //         Card3DFlip {}
                //     }
                //     ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/card_3d_flip.rs" }
                // }

                // // Typewriter Effect Card
                // div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                //     h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                //         "Typewriter Effect"
                //     }
                //     div { class: "flex-grow w-full flex items-center justify-center my-4",
                //         TypewriterEffect { text: "Hello, World! Welcome to Dioxus Motion" }
                //     }
                //     ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/typewriter_effect.rs" }
                // }

                // // Particle System Card
                // div { class: "flex flex-col items-start justify-between h-full bg-white rounded-2xl shadow-lg shadow-green-500/5 p-6 hover:shadow-xl transition-shadow duration-300",
                //     h3 { class: "text-lg font-semibold text-gray-800 mb-4 w-full",
                //         "Particle System"
                //     }
                //     div { class: "flex-grow w-full flex items-center justify-center my-4",
                //         ParticleSystem {}
                //     }
                //     ViewCodeButton { url: "https://github.com/wheregmis/dioxus-motion/blob/main/example_projects/src/components/particle_system.rs" }
                // }
                }
            }

            // Footer with gradient border
            footer { class: "bg-white border-t border-gradient-to-r from-blue-500/20 to-purple-500/20 py-8 mt-auto",
                div { class: "max-w-6xl mx-auto px-4 text-center",
                    p { class: "text-sm text-gray-600", "© 2024 Sabin Regmi. No Rights Reserved." }
                    p { class: "text-xs text-gray-500 mt-2", "Built with ❤️ using Dioxus" }
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
