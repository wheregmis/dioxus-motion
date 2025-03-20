use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::utils::router::Route;

#[component]
pub fn NavBar() -> Element {
    let mut nav_bg = use_motion(Transform::new(0.0, -100.0, 1.0, 0.0));
    let mut nav_opacity = use_motion(0.0f32);

    // let mut is_dark_mode = use_signal(|| false);

    use_effect(move || {
        nav_bg.animate_to(
            Transform::new(0.0, 0.0, 1.0, 0.0),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 20.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );

        nav_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(300),
                easing: easer::functions::Cubic::ease_out,
            })),
        );
    });

    // let toggle_theme = move |_| {
    //     is_dark_mode.toggle();
    // };

    rsx! {
        div { class: "w-full h-full bg-background text-text-secondary",
            header {
                class: "fixed top-0 w-full z-50 h-16 backdrop-blur-md border-b border-surface-light/20",
                style: "
                    transform: translateY({nav_bg.get_value().y}px);
                    opacity: {nav_opacity.get_value()};
                ",
                div { class: "container mx-auto h-full px-4",
                    div { class: "flex items-center justify-between h-full",
                        // Left side - Logo and navigation
                        div { class: "flex items-center space-x-3",
                            div { class: "flex items-center gap-8 px-6 py-2 bg-surface/50 backdrop-blur-sm
                                       border border-surface-light/10 rounded-full shadow-lg shadow-background/5",
                                // Rocket emoji logo
                                span { class: "text-2xl", "🚀" }

                                // Logo/Home link
                                Link {
                                    to: Route::Home {},
                                    class: "text-lg font-semibold text-text-primary hover:text-primary transition-colors",
                                    "Dioxus Motion"
                                }

                                // Navigation links
                                nav { class: "hidden md:flex items-center space-x-6",
                                    NavLink { to: Route::DocsLanding {}, "Documentation" }
                                                                //  NavLink { to: Route::Blog {}, "Blog" }
                                }
                            }
                        }

                        // Right side - Theme toggle and GitHub
                        div { class: "flex items-center space-x-4",
                            // Theme toggle
                            // button {
                            //     class: "p-2 rounded-lg transition-colors duration-300
                            //            bg-surface-light/10 hover:bg-surface-light/20",
                            //     onclick: toggle_theme,
                            //     if *is_dark_mode.read() {
                            //         span { class: "text-xl", "☀️" }
                            //     } else {
                            //         span { class: "text-xl", "🌙" }
                            //     }
                            // }

                            // GitHub link
                            a {
                                class: "flex items-center px-4 py-2 rounded-lg
                                       bg-surface-light/10 hover:bg-surface-light/20
                                       text-text-secondary hover:text-text-primary
                                       transition-all duration-300",
                                href: "https://github.com/wheregmis/dioxus-motion",
                                target: "_blank",
                                rel: "noopener",
                                "GitHub"
                                span { class: "ml-2 px-2 py-1 text-xs rounded-full
                                           bg-surface-light/20 text-primary",
                                    "★ Star"
                                }
                            }

                            // Crates.io badge
                            a {
                                class: "flex items-center px-4 py-2 rounded-lg
                                       bg-surface-light/10 hover:bg-surface-light/20
                                       text-text-secondary hover:text-text-primary
                                       transition-all duration-300",
                                href: "https://crates.io/crates/dioxus-motion",
                                target: "_blank",
                                rel: "noopener",
                                "Crates.io"
                                span { class: "ml-2 px-2 py-1 text-xs rounded-full
                                           bg-surface-light/20 text-primary",
                                    "0.3.1"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "pt-16", Outlet::<Route> {} }
        }
    }
}

#[component]
fn NavLink(to: Route, children: Element) -> Element {
    let current_route = use_route::<Route>();
    let is_active = current_route == to;

    rsx! {
        Link {
            to,
            class: {
                let base_classes = "relative group text-sm text-text-secondary hover:text-text-primary transition-all duration-300";
                if is_active {
                    format!("{} text-primary", base_classes)
                } else {
                    base_classes.to_string()
                }
            },
            span { class: "relative z-10", {children} }
            // Animated underline
            div { class: "absolute -bottom-1 left-0 h-[2px] w-0 bg-primary
                       transition-all duration-300 group-hover:w-full" }
        }
    }
}
