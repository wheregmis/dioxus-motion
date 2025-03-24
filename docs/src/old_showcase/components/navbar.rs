use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut transform = use_motion(Transform::new(0.0, -100.0, 1.0, 0.0));

    use_effect(move || {
        // Animate transform with spring physics
        transform.animate_to(
            Transform::new(0.0, 0.0, 1.0, 0.0),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 20.0,
                mass: 1.0,
                velocity: 10.0,
            })),
        );
    });

    use_drop(move || {
        transform.stop();
    });

    rsx! {
        nav {
            class: "fixed top-0 left-0 right-0 backdrop-blur-md
                    shadow-lg shadow-black/5 dark:shadow-white/5
                    border-b border-gray-200/20 
                    transition-all duration-300 z-50
                    hover:shadow-xl",
            style: "transform: translateY({transform.get_value().y}px);",
            div { class: "max-w-6xl mx-auto px-4",
                div { class: "flex justify-between items-center h-28",
                    // Logo
                    div { class: "flex items-center space-x-4",
                        div { class: "text-3xl font-bold bg-gradient-to-r from-blue-500 to-purple-600 bg-clip-text text-transparent",
                            "Dioxus Motion"
                        }
                    }

                    // Social Links
                    div { class: "flex items-center space-x-4",
                        a {
                            class: " hover:text-blue-500 transition-colors",
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
