use crate::components::code_block::CodeBlock;
use crate::components::guide_navigation::GuideNavigation;
use dioxus::prelude::*;

#[component]
/// Renders a card component showcasing a transition effect.
///
/// This component displays the transition's name, description, and an example usage. It is ideal for demonstrating various transition effects in a Dioxus-based application.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn app(cx: Scope) -> Element {
///     render! {
///         TransitionCard(
///             "Fade",
///             "Smoothly fades an element in or out.",
///             "example: use for modal transitions"
///         )
///     }
/// }
/// ```
fn TransitionCard(name: &'static str, description: &'static str, example: &'static str) -> Element {
    rsx! {
        div { class: "p-6 rounded-xl bg-dark-200/50 backdrop-blur-sm
                    border border-primary/10 transition-all duration-300
                    hover:border-primary/20 hover:shadow-lg hover:shadow-primary/10",
            span { class: "block font-semibold text-text-primary mb-2", {name} }
            p { class: "text-sm text-text-secondary mb-2", {description} }
            p { class: "text-xs text-text-muted italic", {example} }
        }
    }
}

#[component]
/// Renders the main page component for demonstrating transitions in a Dioxus application.
///
/// This component organizes the content into three primary sections:
/// - **Quick Start**: Offers step-by-step instructions on enabling transitions, including adding the transitions feature in Cargo.toml, applying the MotionTransitions derive macro, and replacing Outlet with AnimatedOutlet.
/// - **Available Transitions**: Displays a grid of transition effects using individual TransitionCard components, with each card showing the transition's name, description, and a brief example.
/// - **Example with Nested Routes**: Provides a code sample illustrating how to configure nested routes with transitions.
///
/// # Examples
///
/// ```
/// // Create the PageTransition component to render the transitions guide.
/// let page = PageTransition();
/// // Typically, you would integrate this element within your Dioxus app's view.
/// ```
pub fn PageTransition() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Quick Start
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Quick Start" }
                div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
                    // Enable transitions feature
                    div { class: "mb-6",
                        h3 { class: "text-lg font-medium text-text-primary mb-2", "1. Enable Transitions Feature" }
                        p { class: "text-text-secondary mb-4",
                            "Add the transitions feature to your dioxus-motion dependency in Cargo.toml:"
                        }
                        CodeBlock {
                            code: r#"dioxus-motion = { git = "https://github.com/wheregmis/dioxus-motion.git", branch = "main", default-features = false, optional = true }

[features]
default = ["web"]
web = ["dioxus/web", "dioxus-motion/web", "dioxus-motion/transitions"]
desktop = [
    "dioxus/desktop",
    "dioxus-motion/desktop",
    "dioxus-motion/transitions",
]
mobile = ["dioxus/mobile", "dioxus-motion/desktop", "dioxus-motion/transitions"]"#.to_string(),
                            language: "toml".to_string(),
                        }
                    }

                    // Add MotionTransitions derive
                    div { class: "mb-6",
                        h3 { class: "text-lg font-medium text-text-primary mb-2", "2. Add MotionTransitions Derive" }
                        p { class: "text-text-secondary mb-4",
                            "Add the MotionTransitions derive macro to your Route enum:"
                        }
                        CodeBlock {
                            code: r#"#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        #[transition(Fade)]
        Home {},
        #[route("/slide-left")]
        #[transition(ZoomIn)]
        SlideLeft {},
        #[route("/slide-right")]
        SlideRight {},
        #[route("/slide-up")]
        SlideUp {},
        #[route("/slide-down")]
        SlideDown {},
        #[route("/fade")]
        Fade {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}"#.to_string(),
                            language: "rust".to_string(),
                        }
                    }

                    // Replace Outlet with AnimatedOutlet
                    div { class: "mb-6",
                        h3 { class: "text-lg font-medium text-text-primary mb-2", "3. Use AnimatedOutlet" }
                        p { class: "text-text-secondary mb-4",
                            "Replace Outlet with AnimatedOutlet in your layout component:"
                        }
                        CodeBlock {
                            code: r#"#[component]
fn NavBar() -> Element {
    rsx! {
        nav { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::SlideLeft {}, "Blog" }
        }
        AnimatedOutlet::<Route> {}
    }
}"#.to_string(),
                            language: "rust".to_string(),
                        }
                    }
                }
            }

            // Available Transitions
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Available Transitions" }
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                    TransitionCard {
                        name: "Fade",
                        description: "Smooth opacity transition",
                        example: "Perfect for subtle page changes"
                    }
                    TransitionCard {
                        name: "ZoomIn",
                        description: "Scale and fade combination",
                        example: "Great for modal dialogs or focus changes"
                    }
                    TransitionCard {
                        name: "SlideLeft",
                        description: "Horizontal slide animation",
                        example: "Ideal for forward navigation"
                    }
                    TransitionCard {
                        name: "SlideRight",
                        description: "Horizontal slide animation",
                        example: "Perfect for backward navigation"
                    }
                    TransitionCard {
                        name: "SlideUp",
                        description: "Vertical slide animation",
                        example: "Great for bottom sheets or modals"
                    }
                    TransitionCard {
                        name: "SlideDown",
                        description: "Vertical slide animation",
                        example: "Perfect for top sheets or dropdowns"
                    }
                }
            }

            // Example with Nested Routes
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Example with Nested Routes" }
                div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
                    CodeBlock {
                        code: r#"#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        #[transition(SlideDown)]
        Home {},

        #[nest("/blog")]
        #[layout(Blog)]
            #[route("/")]
            #[transition(SlideUp)]
            BlogList {},

            #[route("/:name")]
            #[transition(SlideRight)]
            BlogPost { name: String },

        #[end_layout]
        #[end_nest]

    #[end_layout]

    #[route("/:..route")]
    #[transition(Fade)]
    PageNotFound { route: Vec<String> },
}"#.to_string(),
                        language: "rust".to_string(),
                    }
                }
            }

            // Navigation links
            GuideNavigation {}
        }
    }
}
