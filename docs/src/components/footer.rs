use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
/// Renders a consistent footer component used across all pages.
///
/// This footer includes:
/// - A "Made with love" message
/// - Copyright information
/// - Social and documentation links
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// fn app() -> Element {
///     rsx! {
///         // Your page content
///         Footer {}
///     }
/// }
/// ```
pub fn Footer() -> Element {
    rsx! {
        motion::footer {
            class: "relative z-10 border-t border-primary/10 mt-auto py-8",
            initial: Some(AnimationTarget::new().opacity(0.0).y(30.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(TransitionConfig::default().type_(TransitionType::Spring).stiffness(80.0).damping(18.0)),
            div { class: "container mx-auto px-4",
                div { class: "flex flex-col items-center justify-center space-y-4 text-center",
                    // Made with love
                    p { class: "text-text-secondary",
                        "Made with "
                        motion::span { class: "text-red-500 animate-pulse", "♥" }
                        " using "
                        motion::a {
                            href: "https://dioxuslabs.com",
                            target: "_blank",
                            class: "text-primary hover:text-primary/80 transition-colors",
                            while_hover: Some(AnimationTarget::new().scale(1.08)),
                            while_tap: Some(AnimationTarget::new().scale(0.96)),
                            transition: Some(TransitionConfig::default().type_(TransitionType::Spring).stiffness(200.0).damping(18.0)),
                            "Dioxus"
                        }
                    }
                    // Copyright
                    p { class: "text-text-muted text-sm",
                        "© 2025 Dioxus Motion. All rights reserved."
                    }
                    // Links
                    div { class: "flex items-center space-x-4 text-sm text-text-secondary",
                        motion::a {
                            href: "https://github.com/wheregmis/dioxus-motion",
                            target: "_blank",
                            class: "hover:text-text-primary transition-colors",
                            while_hover: Some(AnimationTarget::new().scale(1.08)),
                            while_tap: Some(AnimationTarget::new().scale(0.96)),
                            transition: Some(TransitionConfig::default().type_(TransitionType::Spring).stiffness(200.0).damping(18.0)),
                            "GitHub"
                        }
                        motion::span { "·" }
                        motion::a {
                            href: "https://crates.io/crates/dioxus-motion",
                            target: "_blank",
                            class: "hover:text-text-primary transition-colors",
                            while_hover: Some(AnimationTarget::new().scale(1.08)),
                            while_tap: Some(AnimationTarget::new().scale(0.96)),
                            transition: Some(TransitionConfig::default().type_(TransitionType::Spring).stiffness(200.0).damping(18.0)),
                            "Crates.io"
                        }
                        motion::span { "·" }
                        motion::a {
                            href: "https://docs.rs/dioxus-motion",
                            target: "_blank",
                            class: "hover:text-text-primary transition-colors",
                            while_hover: Some(AnimationTarget::new().scale(1.08)),
                            while_tap: Some(AnimationTarget::new().scale(0.96)),
                            transition: Some(TransitionConfig::default().type_(TransitionType::Spring).stiffness(200.0).damping(18.0)),
                            "Documentation"
                        }
                    }
                }
            }
        }
    }
}
