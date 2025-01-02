use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut transform = use_transform_animation(
        Transform {
            y: 0.0,
            opacity: 0.0,
            ..Default::default()
        },
        Transform {
            y: 0.0,
            opacity: 1.0,
            scale: 1.0,
            ..Default::default()
        },
        AnimationMode::Spring(Spring {
            stiffness: 200.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
        }),
    );

    use_drop(move || {
        transform.stop();
    });

    rsx! {
        nav { class: "fixed top-0 w-full bg-white shadow-lg z-50",
            div { class: "max-w-6xl mx-auto px-4",
                div {
                    class: "flex justify-between items-center h-16",
                    onmounted: move |_| transform.start(),
                    style: "{transform.style()}",

                    // Logo and Title
                    div { class: "flex items-center space-x-4",
                        div { class: "text-3xl font-bold bg-gradient-to-r from-blue-500 to-purple-600 bg-clip-text text-transparent",
                            "Dioxus Motion"
                        }
                    }

                    // Social Links
                    div { class: "flex items-center space-x-4",
                        a {
                            class: "text-gray-600 hover:text-blue-500 transition-colors",
                            href: "https://github.com/wheregmis/dioxus-motion",
                            target: "_blank",
                            "GitHub"
                        }
                    }
                }
            }
        }
    }
}
