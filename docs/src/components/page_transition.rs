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
        div { class: "p-6 rounded-xl bg-dark-200/50 backdrop-blur-xs
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
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Page Transitions" }
                p { class: "text-text-secondary leading-relaxed",
                    "Page transitions are a powerful way to enhance the user experience in your Dioxus application. "
                    "They provide visual continuity between route changes, making your app feel more polished and "
                    "professional. Dioxus Motion makes implementing these transitions simple and declarative."
                }

                div { class: "mt-6 p-4 bg-primary/5 rounded-lg border border-primary/10",
                    h3 { class: "text-lg font-medium text-primary mb-2", "Why Use Page Transitions?" }
                    ul { class: "list-disc list-inside space-y-2 text-text-secondary",
                        li {
                            span { class: "font-medium", "Improved User Experience: " }
                            "Smooth transitions between pages reduce the jarring effect of instant content changes."
                        }
                        li {
                            span { class: "font-medium", "Visual Hierarchy: " }
                            "Different transition types can convey navigation direction and relationship between pages."
                        }
                        li {
                            span { class: "font-medium", "Professional Polish: " }
                            "Well-implemented transitions add a level of refinement that users associate with high-quality applications."
                        }
                    }
                }
            }

            // How It Works
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "How It Works" }
                p { class: "text-text-secondary",
                    "Dioxus Motion's page transitions work by intercepting route changes and applying animations during the transition. "
                    "The library handles all the complexity of managing the animation timing and ensuring both the entering and "
                    "exiting pages are properly animated."
                }

                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mt-4",
                    // Step 1
                    div { class: "p-4 bg-dark-200/50 rounded-lg",
                        div { class: "w-8 h-8 flex items-center justify-center bg-primary/20 text-primary rounded-full mb-2", "1" }
                        h4 { class: "font-medium text-text-primary mb-1", "Define Transitions" }
                        p { class: "text-sm text-text-secondary",
                            "Add transition attributes to your route definitions to specify which animation to use for each route."
                        }
                    }
                    // Step 2
                    div { class: "p-4 bg-dark-200/50 rounded-lg",
                        div { class: "w-8 h-8 flex items-center justify-center bg-primary/20 text-primary rounded-full mb-2", "2" }
                        h4 { class: "font-medium text-text-primary mb-1", "Use AnimatedOutlet" }
                        p { class: "text-sm text-text-secondary",
                            "Replace the standard Outlet component with AnimatedOutlet in your layout to enable the transitions."
                        }
                    }
                    // Step 3
                    div { class: "p-4 bg-dark-200/50 rounded-lg",
                        div { class: "w-8 h-8 flex items-center justify-center bg-primary/20 text-primary rounded-full mb-2", "3" }
                        h4 { class: "font-medium text-text-primary mb-1", "Navigate Normally" }
                        p { class: "text-sm text-text-secondary",
                            "Use standard Dioxus navigation. The transitions will automatically apply when routes change."
                        }
                    }
                }
            }

            // Quick Start
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Implementation Steps" }
                div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10",
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

            // Configurable Animation Context
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Configurable Animation Context" }
                p { class: "text-text-secondary",
                    "Dioxus Motion allows you to customize the spring physics used in transitions through Dioxus's context system. "
                    "This provides fine-grained control over animation behavior without modifying individual transition definitions."
                }

                div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10 mb-6",
                    h3 { class: "text-lg font-medium text-text-primary mb-4", "Basic Context Setup" }
                    p { class: "text-text-secondary mb-4",
                        "Provide custom spring configuration to all page transitions using a context provider:"
                    }
                    CodeBlock {
                        code: r#"use dioxus::prelude::*;
use dioxus_motion::prelude::*;

fn main() {
    dioxus::launch(|| {
        rsx! {
            head {
                link { rel: "stylesheet", href: MAIN_CSS }
            }
            App {}
        }
    });
}

#[component]
fn App() -> Element {
    // Define custom spring configuration
    let spring = use_signal(|| Spring {
        stiffness: 200.0,  // Higher = faster, snappier
        damping: 30.0,     // Higher = less bounce
        mass: 0.8,         // Lower = more responsive
        velocity: 0.0,     // Initial velocity
    });

    // Provide spring context to all child components
    use_context_provider(|| spring);

    rsx! {
        Router::<Route> {}
    }
}"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    // Spring Parameters
                    div { class: "space-y-4",
                        h3 { class: "text-lg font-medium text-text-primary mb-3", "Spring Parameters" }
                        div { class: "space-y-3",
                            div { class: "p-3 bg-dark-100/30 rounded-lg",
                                h4 { class: "font-medium text-primary text-sm", "Stiffness (default: 160.0)" }
                                p { class: "text-xs text-text-secondary mt-1",
                                    "Controls how quickly the animation reaches its target. Higher values = faster, more aggressive motion."
                                }
                            }
                            div { class: "p-3 bg-dark-100/30 rounded-lg",
                                h4 { class: "font-medium text-primary text-sm", "Damping (default: 25.0)" }
                                p { class: "text-xs text-text-secondary mt-1",
                                    "Controls oscillation and bounce. Higher values = smoother, less bouncy motion."
                                }
                            }
                            div { class: "p-3 bg-dark-100/30 rounded-lg",
                                h4 { class: "font-medium text-primary text-sm", "Mass (default: 1.0)" }
                                p { class: "text-xs text-text-secondary mt-1",
                                    "Controls inertia. Higher values = more sluggish, weighty motion."
                                }
                            }
                            div { class: "p-3 bg-dark-100/30 rounded-lg",
                                h4 { class: "font-medium text-primary text-sm", "Velocity (default: 0.0)" }
                                p { class: "text-xs text-text-secondary mt-1",
                                    "Initial velocity. Can create pre-existing motion effects."
                                }
                            }
                        }
                    }

                    // Preset Configurations
                    div { class: "space-y-4",
                        h3 { class: "text-lg font-medium text-text-primary mb-3", "Common Configurations" }
                        div { class: "space-y-3",
                            div { class: "p-3 bg-dark-100/30 rounded-lg",
                                h4 { class: "font-medium text-green-400 text-sm", "Bouncy (Fun & Playful)" }
                                div { class: "text-xs text-text-secondary mt-1 font-mono",
                                    "stiffness: 180.0, damping: 15.0"
                                }
                            }
                            div { class: "p-3 bg-dark-100/30 rounded-lg",
                                h4 { class: "font-medium text-blue-400 text-sm", "Smooth (Professional)" }
                                div { class: "text-xs text-text-secondary mt-1 font-mono",
                                    "stiffness: 140.0, damping: 35.0"
                                }
                            }
                            div { class: "p-3 bg-dark-100/30 rounded-lg",
                                h4 { class: "font-medium text-orange-400 text-sm", "Fast (Snappy)" }
                                div { class: "text-xs text-text-secondary mt-1 font-mono",
                                    "stiffness: 220.0, damping: 20.0, mass: 0.8"
                                }
                            }
                            div { class: "p-3 bg-dark-100/30 rounded-lg",
                                h4 { class: "font-medium text-purple-400 text-sm", "Gentle (Subtle)" }
                                div { class: "text-xs text-text-secondary mt-1 font-mono",
                                    "stiffness: 120.0, damping: 30.0, mass: 1.3"
                                }
                            }
                        }
                    }
                }

                div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10 mt-6",
                    h3 { class: "text-lg font-medium text-text-primary mb-4", "Multiple Animation Contexts" }
                    p { class: "text-text-secondary mb-4",
                        "You can provide different spring configurations for different parts of your application:"
                    }
                    CodeBlock {
                        code: r#"#[component]
fn AdminSection() -> Element {
    // Faster, more aggressive animations for admin interfaces
    let admin_spring = use_signal(|| Spring {
        stiffness: 220.0,
        damping: 20.0,
        mass: 0.8,
        velocity: 0.0,
    });

    use_context_provider(|| admin_spring);

    rsx! {
        AnimatedOutlet::<AdminRoute> {}
    }
}

#[component]
fn UserSection() -> Element {
    // Gentler animations for user-facing content
    let user_spring = use_signal(|| Spring {
        stiffness: 140.0,
        damping: 35.0,
        mass: 1.0,
        velocity: 0.0,
    });

    use_context_provider(|| user_spring);

    rsx! {
        AnimatedOutlet::<UserRoute> {}
    }
}"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                div { class: "mt-6 p-4 bg-blue-900/20 rounded-lg border border-blue-500/20",
                    div { class: "flex items-start space-x-3",
                        div { class: "text-blue-400 text-lg", "💡" }
                        div {
                            h4 { class: "font-medium text-blue-300 mb-1", "Pro Tip: Context Fallback" }
                            p { class: "text-sm text-blue-200/80",
                                "If no spring context is provided, Dioxus Motion automatically falls back to sensible defaults. "
                                "This means transitions work out-of-the-box while still allowing customization when needed."
                            }
                        }
                    }
                }
            }

            // Best Practices
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Best Practices" }
                p { class: "text-text-secondary mb-4",
                    "Follow these guidelines to create effective and performant page transitions in your Dioxus application."
                }

                div { class: "space-y-4",
                    // Practice 1
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10",
                        h3 { class: "text-lg font-medium text-primary mb-2", "Be Consistent" }
                        p { class: "text-text-secondary",
                            "Use consistent transition patterns throughout your application. For example, use SlideLeft for "
                            "forward navigation and SlideRight for backward navigation to create a cohesive experience."
                        }
                    }

                    // Practice 2
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10",
                        h3 { class: "text-lg font-medium text-primary mb-2", "Keep Transitions Short" }
                        p { class: "text-text-secondary",
                            "Aim for transitions between 200-300ms. Longer transitions can make your app feel sluggish, "
                            "while shorter ones might be too abrupt. The default durations are optimized for most cases."
                        }
                    }

                    // Practice 3
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10",
                        h3 { class: "text-lg font-medium text-primary mb-2", "Consider Mobile Performance" }
                        p { class: "text-text-secondary",
                            "On mobile devices, prefer simpler transitions like Fade or SlideLeft/Right. Complex animations "
                            "with multiple properties might cause performance issues on lower-end devices."
                        }
                    }

                    // Practice 4
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10",
                        h3 { class: "text-lg font-medium text-primary mb-2", "Use Semantic Transitions" }
                        p { class: "text-text-secondary",
                            "Choose transitions that match the mental model of your navigation. For hierarchical navigation, "
                            "consider using ZoomIn for drilling down and ZoomOut for going back up a level."
                        }
                    }
                }
            }

            // Example with Nested Routes
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Example with Nested Routes" }
                div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10",
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

            // Troubleshooting
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Troubleshooting" }
                p { class: "text-text-secondary mb-4",
                    "If you encounter issues with your page transitions, here are some common problems and their solutions."
                }

                div { class: "space-y-4",
                    // Issue 1
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10",
                        h3 { class: "text-lg font-medium text-primary mb-2", "Transitions Not Working" }
                        p { class: "text-text-secondary mb-2",
                            "If your transitions aren't working at all, check the following:"
                        }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1 text-sm",
                            li { "Verify that you've enabled the ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "transitions" }, " feature in your Cargo.toml" }
                            li { "Ensure you've added the ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "MotionTransitions" }, " derive to your Route enum" }
                            li { "Confirm you're using ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "AnimatedOutlet" }, " instead of the standard Outlet" }
                            li { "Check that you've specified transition attributes for your routes" }
                        }
                    }

                    // Issue 2
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10",
                        h3 { class: "text-lg font-medium text-primary mb-2", "Flickering or Jumpy Transitions" }
                        p { class: "text-text-secondary mb-2",
                            "If your transitions appear to flicker or jump:"
                        }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1 text-sm",
                            li { "Ensure your page components have consistent dimensions to avoid layout shifts" }
                            li { "Try using simpler transitions like Fade instead of complex ones" }
                            li { "Check for any CSS that might be interfering with the animations" }
                        }
                    }

                    // Issue 3
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10",
                        h3 { class: "text-lg font-medium text-primary mb-2", "Performance Issues" }
                        p { class: "text-text-secondary mb-2",
                            "If transitions are causing performance problems:"
                        }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1 text-sm",
                            li { "Reduce the complexity of your page components" }
                            li { "Use simpler transitions for mobile devices" }
                            li { "Ensure you're not animating too many properties simultaneously" }
                            li { "Consider using hardware acceleration with CSS transforms where possible" }
                        }
                    }
                }
            }

            // Dynamic Transition Resolver Example
            section {
                h2 { class: "text-2xl font-semibold text-text-primary", "Dynamic Transition Direction" }
                p { class: "text-text-secondary mb-4",
                    "You can dynamically choose the transition direction based on navigation context. For example, in a card navigation UI, you may want to slide left or right depending on which card the user is moving to."
                }
                div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10 mb-6",
                    h3 { class: "text-lg font-medium text-text-primary mb-4", "How to Provide a Dynamic Resolver" }
                    p { class: "text-text-secondary mb-4",
                        "Provide a resolver function via context that receives the previous and next route, and returns the appropriate transition variant."
                    }
                    CodeBlock {
                        code: r#"use dioxus_motion::transitions::page_transitions::TransitionVariantResolver;

let resolver: TransitionVariantResolver<Route> = std::rc::Rc::new(|from, to| {
    // Assuming Route::Card { idx } for cards
    match (from, to) {
        (Route::Card { idx: from_idx }, Route::Card { idx: to_idx }) => {
            if to_idx > from_idx {
                TransitionVariant::SlideLeft
            } else if to_idx < from_idx {
                TransitionVariant::SlideRight
            } else {
                TransitionVariant::Fade
            }
        }
        _ => to.get_transition(),
    }
});
use_context_provider(|| resolver);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }
                p { class: "text-text-secondary mt-2",
                    "This enables advanced navigation flows, such as card stacks, wizards, or any scenario where the transition direction depends on user action."
                }
            }

            // Navigation links
            GuideNavigation {}
        }
    }
}
