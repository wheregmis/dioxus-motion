use dioxus::prelude::*;

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
        footer { class: "relative z-10 border-t border-primary/10 mt-auto py-8",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-col items-center justify-center space-y-4 text-center",
                    // Made with love
                    p { class: "text-text-secondary",
                        "Made with "
                        span { class: "text-red-500 animate-pulse", "♥" }
                        " using "
                        a {
                            href: "https://dioxuslabs.com",
                            target: "_blank",
                            class: "text-primary hover:text-primary/80 transition-colors",
                            "Dioxus"
                        }
                    }
                    // Copyright
                    p { class: "text-text-muted text-sm",
                        "© 2025 Dioxus Motion. All rights reserved."
                    }
                    // Links
                    div { class: "flex items-center space-x-4 text-sm text-text-secondary",
                        a {
                            href: "https://github.com/wheregmis/dioxus-motion",
                            target: "_blank",
                            class: "hover:text-text-primary transition-colors",
                            "GitHub"
                        }
                        span { "·" }
                        a {
                            href: "https://crates.io/crates/dioxus-motion",
                            target: "_blank",
                            class: "hover:text-text-primary transition-colors",
                            "Crates.io"
                        }
                        span { "·" }
                        a {
                            href: "https://docs.rs/dioxus-motion",
                            target: "_blank",
                            class: "hover:text-text-primary transition-colors",
                            "Documentation"
                        }
                    }
                }
            }
        }
    }
}
