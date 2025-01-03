use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut transform = use_animation(Transform {
        y: -100.0,
        opacity: 0.0,
        ..Default::default()
    });

    use_effect(move || {
        transform.animate_to(
            Transform {
                y: 0.0,
                opacity: 1.0,
                ..Default::default()
            },
            AnimationConfig {
                mode: AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 20.0,
                    mass: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            },
        );
    });

    rsx! {
        nav {
            class: "fixed top-0 left-0 right-0 bg-white/95 backdrop-blur-sm transition-shadow duration-300 z-50",
            style: "transform: translateY({transform.get_value().y}px); opacity: {transform.get_value().opacity}",
            div { class: "max-w-6xl mx-auto px-4",
                div { class: "flex justify-between items-center h-16",
                    // Logo
                    div { class: "flex items-center space-x-4",
                        div { class: "text-3xl font-bold bg-gradient-to-r from-blue-500 to-purple-600 bg-clip-text text-transparent",
                            "Dioxus Motion"
                        }
                    }

                    // Navigation Links
                    div { class: "flex items-center space-x-6",
                        a {
                            class: "text-gray-600 hover:text-blue-500 transition-colors duration-200",
                            href: "https://github.com/wheregmis/dioxus-motion",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "GitHub"
                        }
                        a {
                            class: "px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors duration-200",
                            href: "https://docs.rs/dioxus-motion",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Documentation"
                        }
                    }
                }
            }
        }
    }
}
