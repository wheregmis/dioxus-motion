use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::components::code_block::CodeBlock;
use crate::components::footer::Footer;
use crate::utils::router::Route;

#[component]
/// Renders a documentation layout with a header, navigation sidebar, main content, and related links.
///
/// This layout accepts a static title and description to be displayed in the header, and embeds
/// the provided children elements in the central content area. It also configures left and right sidebars:
/// the left one for section navigation and the right one for related links, all against a decorative gradient background.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// fn app(cx: Scope) -> Element {
///     DocLayout("Documentation", "Learn how to use Dioxus components", rsx! {
///         p { "Welcome to the docs!" }
///     })
/// }
///
/// // To render, pass `app` to the appropriate Dioxus launch method, e.g., `dioxus::desktop::launch(app);`.
/// ```
fn DocLayout(title: &'static str, description: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-dark relative overflow-hidden w-full",
            // Background elements
            div { class: "absolute inset-0 overflow-hidden",
                div { class: "absolute -top-1/2 -left-1/2 w-full h-full bg-primary/5 rounded-full blur-3xl" }
                div { class: "absolute -bottom-1/2 -right-1/2 w-full h-full bg-secondary/5 rounded-full blur-3xl" }
            }

            // Content
            div {
                class: "relative z-10 w-full flex flex-col min-h-screen", // Added flex and min-h-screen
                // Header
                div { class: "border-b border-surface-light/20 w-full",
                    div { class: "w-full px-4 sm:px-6 lg:px-8 py-6",
                        h1 { class: "text-4xl font-bold text-text-primary mb-2", {title} }
                        p { class: "text-lg text-text-secondary", {description} }
                    }
                }

                // Main content with three columns
                div {
                    class: "w-full px-4 sm:px-6 lg:px-8 py-8 grow", // Added grow
                    div { class: "flex gap-8 w-full",
                        // Left sidebar - Sections
                        div { class: "hidden lg:block flex-1",
                            div { class: "sticky top-24",
                                nav { class: "space-y-1",
                                    SectionLink {
                                        to: Route::DocsLanding {},
                                        icon: "📚",
                                        label: "Getting Started",
                                    }
                                    SectionLink {
                                        to: Route::PageTransition {},
                                        icon: "🔄",
                                        label: "Page Transitions",
                                    }
                                    SectionLink {
                                        to: Route::BasicAnimationGuide {},
                                        icon: "🎨",
                                        label: "Basic Animation Guide",
                                    }
                                    SectionLink {
                                        to: Route::IntermediateAnimationGuide {},
                                        icon: "🚀",
                                        label: "Intermediate Animation Guide",
                                    }
                                    SectionLink {
                                        to: Route::ComplexAnimationGuide {},
                                        icon: "✨",
                                        label: "Complex Animation Guide",
                                    }

                                    SectionLink {
                                        to: Route::MotionPrimitivesGuide {},
                                        icon: "🧩",
                                        label: "Motion Primitives Guide",
                                    }

                                }
                            }
                        }

                        // Main content
                        div { class: "flex-4 min-w-0",
                            AnimatedOutlet::<Route> {}
                        }

                        // Right sidebar - Related links
                        div { class: "hidden lg:block flex-1",
                            div { class: "sticky top-24", RelatedLinks {} }
                        }
                    }
                }

                // Footer
                Footer {}
            }
        }
    }
}

#[component]
/// Returns an Element representing the documentation page layout.
///
/// This function constructs the documentation page using the DocLayout component with a preset
/// title and description, and includes an AnimatedOutlet for rendering nested routes.
///
/// # Examples
///
/// ```rust
/// use crate::Docs;
///
/// // Create the documentation element
/// let docs_page = Docs();
///
/// // Use `docs_page` within a Dioxus application renderer.
/// ```
pub fn Docs() -> Element {
    rsx! {
        DocLayout {
            title: "Documentation",
            description: "Learn how to use Dioxus Motion to create beautiful animations in your applications.",
        }
    }
}

#[component]
/// Renders a sidebar link component with an icon and label, applying active styling when needed.
///
/// This function creates a navigational link element that compares the provided destination route
/// with the current route. It uses this comparison to conditionally adjust its styling for active
/// and inactive states, ensuring that the active link is highlighted.
///
/// # Examples
///
/// ```
/// # use dioxus::prelude::*;
/// # use crate::Route;
///
/// fn Example(cx: Scope) -> Element {
///     SectionLink(Route::Home, "🏠", "Home")
/// }
///
/// // In an application, the returned element would be included in a sidebar navigation menu.
/// ```
fn SectionLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    let current_route = use_route::<Route>();
    let is_active = current_route == to;

    rsx! {
        Link {
            to,
            class: {
                let base_classes = "flex items-center gap-3 px-4 py-2 rounded-lg text-sm
                                                                         transition-all duration-300";
                if is_active {
                    format!("{} bg-primary/10 text-primary", base_classes)
                } else {
                    format!(
                        "{} text-text-secondary hover:text-text-primary hover:bg-surface-light/10",
                        base_classes,
                    )
                }
            },
            span { class: "text-lg", {icon} }
            span { {label} }
        }
    }
}

#[component]
/// Renders a collection of related links divided into "Resources" and "Community" sections.
///
/// The "Resources" section provides links for the GitHub repository, Crates.io, and API documentation, while the "Community" section contains links for joining Discord and reporting issues.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn main() {
///     // Create the related links element and include it in your layout.
///     let element = RelatedLinks();
///     // Render the element as part of your component's tree.
/// }
/// ```
fn RelatedLinks() -> Element {
    rsx! {
        div { class: "space-y-6",
            // Resources
            div { class: "space-y-4",
                h3 { class: "text-sm font-semibold text-text-primary uppercase tracking-wider",
                    "Resources"
                }
                div { class: "space-y-2",
                    ResourceLink {
                        href: "https://github.com/wheregmis/dioxus-motion",
                        label: "GitHub Repository",
                        icon: "📦",
                    }
                    ResourceLink {
                        href: "https://crates.io/crates/dioxus-motion",
                        label: "Crates.io",
                        icon: "📦",
                    }
                    ResourceLink {
                        href: "https://docs.rs/dioxus-motion",
                        label: "API Documentation",
                        icon: "📚",
                    }
                }
            }

            // Community
            div { class: "space-y-4",
                h3 { class: "text-sm font-semibold text-text-primary uppercase tracking-wider",
                    "Community"
                }
                div { class: "space-y-2",
                    ResourceLink {
                        href: "https://discord.gg/XgGxHRRc",
                        label: "Discord",
                        icon: "💬",
                    }
                    ResourceLink {
                        href: "https://github.com/wheregmis/dioxus-motion/issues",
                        label: "Report Issues",
                        icon: "🐛",
                    }
                }
            }
        }
    }
}

#[component]
/// Creates a styled external resource link element displaying an icon and label.
///
/// This function returns an Element containing an `<a>` tag configured to open the provided URL
/// in a new tab with appropriate security attributes. It embeds the icon and label within separate
/// `<span>` elements and applies predefined classes for consistent styling and hover effects.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// let link = ResourceLink("https://example.com", "Example Site", "🔗");
/// // Render `link` in your Dioxus component as needed.
/// ```
fn ResourceLink(href: &'static str, label: &'static str, icon: &'static str) -> Element {
    rsx! {
        a {
            href,
            target: "_blank",
            rel: "noopener",
            class: "flex items-center gap-3 px-4 py-2 rounded-lg text-sm
                   text-text-secondary hover:text-text-primary hover:bg-surface-light/10
                   transition-all duration-300",
            span { class: "text-lg", {icon} }
            span { {label} }
        }
    }
}

#[component]
/// Generates the landing page for the documentation.
///
/// This component returns a Dioxus Element that renders a comprehensive landing page
/// for the documentation. The page includes sections for installation instructions—with code
/// examples for basic setup and setups with page transitions—as well as an overview of platform support.
/// Additionally, it features guide cards that link to deeper topics like page transitions and interactive animations.
///
/// # Examples
///
/// ```
/// use your_crate::DocsLanding;
///
/// // Create the documentation landing page element.
/// let landing_page = DocsLanding();
///
/// // Render `landing_page` using your Dioxus app as required.
/// ```
pub fn DocsLanding() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Getting Started with Dioxus Motion" }
                p { class: "text-text-secondary leading-relaxed",
                    "Dioxus Motion is a powerful animation library for Dioxus that makes it easy to create smooth, "
                    "interactive animations and transitions in your Rust web applications. Whether you're building "
                    "a simple UI with basic animations or a complex application with intricate motion effects, "
                    "Dioxus Motion provides the tools you need."
                }

                // Why Dioxus Motion?
                div { class: "mt-8 p-6 bg-primary/5 rounded-xl border border-primary/10",
                    h3 { class: "text-xl font-semibold text-primary mb-4", "Why Dioxus Motion?" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                        // Feature 1
                        div { class: "space-y-2",
                            div { class: "text-primary text-2xl", "🚀" }
                            h4 { class: "font-medium text-text-primary", "Performance Focused" }
                            p { class: "text-sm text-text-secondary",
                                "Built with performance in mind, using efficient animation algorithms that run smoothly even on complex UIs."
                            }
                        }
                        // Feature 2
                        div { class: "space-y-2",
                            div { class: "text-primary text-2xl", "🧩" }
                            h4 { class: "font-medium text-text-primary", "Composable API" }
                            p { class: "text-sm text-text-secondary",
                                "Create complex animations by composing simple primitives. Chain, sequence, and combine animations with ease."
                            }
                        }
                        // Feature 3
                        div { class: "space-y-2",
                            div { class: "text-primary text-2xl", "🔄" }
                            h4 { class: "font-medium text-text-primary", "Seamless Transitions" }
                            p { class: "text-sm text-text-secondary",
                                "Add beautiful page transitions to your Dioxus app with minimal code changes using the transitions feature."
                            }
                        }
                    }
                }
            }

            // Quick Example
            section { class: "space-y-6 mt-12",
                h2 { class: "text-2xl font-semibold text-text-primary", "Quick Example" }
                p { class: "text-text-secondary",
                    "Here's a simple example of how to use Dioxus Motion to animate a value:"
                }

                div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10",
                    CodeBlock {
                        code: r#"use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
fn AnimatedButton() -> Element {

    rsx! {
        motion::button {
            class: "px-4 py-2 bg-blue-500 text-white rounded-sm",
            while_hover: Some(AnimationTarget::new().scale(1.2)),
            while_tap: Some(AnimationTarget::new().scale(0.95)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
            ),
            "Hover me!"
        }
    }
}"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                p { class: "text-text-secondary mt-4",
                    "This example creates a button that scales up when hovered and scales down when pressed, "
                    "using motion primitives with spring animation for a natural, bouncy effect. "
                    "Motion primitives provide a declarative way to define animations without manually managing state."
                }
            }

            // Installation Guide
            section { class: "space-y-6 mt-12",
                h2 { class: "text-2xl font-semibold text-text-primary", "Installation" }
                p { class: "text-text-secondary",
                    "Add Dioxus Motion to your project by updating your Cargo.toml file with the appropriate dependencies and features."
                }

                // Basic Installation
                div { class: "space-y-4",
                    h3 { class: "text-lg font-semibold text-text-primary", "Basic Setup" }
                    p { class: "text-text-secondary mb-3",
                        "For basic animations without page transitions, use this configuration. This setup is perfect for "
                        "when you want to animate UI elements but don't need route transitions."
                    }
                    div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10",
                        CodeBlock {
                            code: r#"[dependencies]
dioxus-motion = { version = "0.3.0", optional = true, default-features = false }

[features]
default = ["web"]
web = ["dioxus/web", "dioxus-motion/web"]
desktop = ["dioxus/desktop", "dioxus-motion/desktop"]
mobile = ["dioxus/mobile", "dioxus-motion/desktop"]"#.to_string(),
                            language: "toml".to_string(),
                        }
                    }
                    div { class: "mt-3 p-3 bg-dark-200/80 rounded-lg text-sm text-text-secondary",
                        span { class: "text-primary font-medium", "Tip: " }
                        "Making the dependency optional with feature flags allows you to conditionally compile animations "
                        "for different platforms, which can be useful for cross-platform applications."
                    }
                }

                // With Page Transitions
                div { class: "space-y-4",
                    h3 { class: "text-lg font-semibold text-text-primary", "With Page Transitions" }
                    p { class: "text-text-secondary mb-3",
                        "To enable page transitions, add the ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "transitions" }, " feature to your configuration. "
                        "This allows you to create smooth animations between route changes in your application."
                    }
                    div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10",
                        CodeBlock {
                            code: r#"[dependencies]
dioxus-motion = { version = "0.3.0", optional = true, default-features = false }

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
                    div { class: "mt-3 p-3 bg-dark-200/80 rounded-lg text-sm text-text-secondary",
                        span { class: "text-primary font-medium", "Note: " }
                        "After enabling the transitions feature, you'll need to add the ",
                        code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "MotionTransitions" }, " derive macro to your Route enum "
                        "and replace the standard ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "Outlet" }, " with ",
                        code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "AnimatedOutlet" }, ". See the Page Transitions guide for details."
                    }
                }

                // Platform Support
                div { class: "space-y-4",
                    h3 { class: "text-lg font-semibold text-text-primary", "Platform Support" }
                    p { class: "text-text-secondary mb-3",
                        "Dioxus Motion works across all platforms supported by Dioxus. Here's a breakdown of the platform-specific features:"
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-xs border border-primary/10",
                            h4 { class: "font-semibold text-text-primary mb-2", "Web" }
                            p { class: "text-text-secondary text-sm",
                                "Optimized for web applications using WebAssembly. Leverages requestAnimationFrame for smooth animations."
                            }
                            div { class: "mt-2 text-xs text-primary/80 font-medium", "Feature: web" }
                        }
                        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-xs border border-primary/10",
                            h4 { class: "font-semibold text-text-primary mb-2", "Desktop" }
                            p { class: "text-text-secondary text-sm",
                                "Works with desktop applications built with Dioxus Desktop. Uses the same animation engine as the web version."
                            }
                            div { class: "mt-2 text-xs text-primary/80 font-medium", "Feature: desktop" }
                        }
                        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-xs border border-primary/10",
                            h4 { class: "font-semibold text-text-primary mb-2", "Mobile" }
                            p { class: "text-text-secondary text-sm",
                                "Support for mobile applications through Dioxus Mobile. Optimized for touch interactions and mobile performance."
                            }
                            div { class: "mt-2 text-xs text-primary/80 font-medium", "Feature: mobile" }
                        }
                    }
                    div { class: "mt-4 p-3 bg-primary/5 rounded-lg text-sm text-text-secondary",
                        span { class: "text-primary font-medium", "Default Behavior: " }
                        "If no specific feature is enabled, Dioxus Motion defaults to web support. For cross-platform applications, "
                        "it's recommended to explicitly specify the features you need for each target platform."
                    }
                }
            }

            // Guide sections
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Documentation Guides" }
                p { class: "text-text-secondary mb-4",
                    "Explore our comprehensive guides to master Dioxus Motion. Each guide builds on the previous one, "
                    "taking you from basic concepts to advanced techniques."
                }

                div { class: "grid md:grid-cols-2 gap-6",
                    // Page Transitions Card
                    Link {
                        to: Route::PageTransition {},
                        class: "group relative overflow-hidden rounded-xl bg-dark-200/50 backdrop-blur-xs
                               border border-primary/10 transition-all duration-300 hover:border-primary/20
                               hover:shadow-lg hover:shadow-primary/10",
                        div { class: "p-6",
                            div { class: "flex items-center justify-between mb-4",
                                h3 { class: "text-xl font-semibold text-text-primary",
                                    "Page Transitions"
                                }
                                span { class: "text-primary transform transition-transform group-hover:translate-x-1",
                                    "→"
                                }
                            }
                            p { class: "text-text-secondary leading-relaxed mb-3",
                                "Learn how to create smooth page transitions and routing animations in your Dioxus app."
                            }
                            div { class: "flex items-center text-xs text-primary/80",
                                span { class: "mr-2 px-2 py-0.5 bg-primary/10 rounded-sm", "Beginner-Friendly" }
                                span { "5 min read" }
                            }
                        }
                    }

                    // Basic Animation Guide Card
                    Link {
                        to: Route::BasicAnimationGuide {},
                        class: "group relative overflow-hidden rounded-xl bg-dark-200/50 backdrop-blur-xs
                               border border-primary/10 transition-all duration-300 hover:border-primary/20
                               hover:shadow-lg hover:shadow-primary/10",
                        div { class: "p-6",
                            div { class: "flex items-center justify-between mb-4",
                                h3 { class: "text-xl font-semibold text-text-primary",
                                    "Basic Animation Guide"
                                }
                                span { class: "text-primary transform transition-transform group-hover:translate-x-1",
                                    "→"
                                }
                            }
                            p { class: "text-text-secondary leading-relaxed mb-3",
                                "Start your animation journey with the fundamentals. Learn how to create simple animations with tweens and springs."
                            }
                            div { class: "flex items-center text-xs text-primary/80",
                                span { class: "mr-2 px-2 py-0.5 bg-primary/10 rounded-sm", "Beginner" }
                                span { "10 min read" }
                            }
                        }
                    }
                }

                // Motion Primitives Guide Card
                div { class: "grid md:grid-cols-2 gap-6 mt-6",
                    // Motion Primitives Guide Card
                    Link {
                        to: Route::MotionPrimitivesGuide {},
                        class: "group relative overflow-hidden rounded-xl bg-dark-200/50 backdrop-blur-xs
                               border border-primary/10 transition-all duration-300 hover:border-primary/20
                               hover:shadow-lg hover:shadow-primary/10",
                        div { class: "p-6",
                            div { class: "flex items-center justify-between mb-4",
                                h3 { class: "text-xl font-semibold text-text-primary",
                                    "Motion Primitives Guide"
                                }
                                span { class: "text-primary transform transition-transform group-hover:translate-x-1",
                                    "→"
                                }
                            }
                            p { class: "text-text-secondary leading-relaxed mb-3",
                                "Learn how to use motion primitives to create fluid, interactive animations with minimal code."
                            }
                            div { class: "flex items-center text-xs mb-2",
                                span { class: "mr-2 px-2 py-0.5 bg-amber-500/10 text-amber-600 dark:text-amber-400 rounded-sm", "Experimental" }
                                span { class: "mr-2 px-2 py-0.5 bg-primary/10 text-primary/80 rounded-sm", "Beginner-Friendly" }
                                span { class: "text-primary/80", "8 min read" }
                            }
                        }
                    }
                }

                // Advanced guides
                div { class: "grid md:grid-cols-2 gap-6 mt-6",
                    // Intermediate Guide Card
                    Link {
                        to: Route::IntermediateAnimationGuide {},
                        class: "group relative overflow-hidden rounded-xl bg-dark-200/50 backdrop-blur-xs
                               border border-primary/10 transition-all duration-300 hover:border-primary/20
                               hover:shadow-lg hover:shadow-primary/10",
                        div { class: "p-6",
                            div { class: "flex items-center justify-between mb-4",
                                h3 { class: "text-xl font-semibold text-text-primary",
                                    "Intermediate Animation Guide"
                                }
                                span { class: "text-primary transform transition-transform group-hover:translate-x-1",
                                    "→"
                                }
                            }
                            p { class: "text-text-secondary leading-relaxed mb-3",
                                "Take your animations to the next level with loops, delays, and sequences. Create more complex and engaging animations."
                            }
                            div { class: "flex items-center text-xs text-primary/80",
                                span { class: "mr-2 px-2 py-0.5 bg-primary/10 rounded-sm", "Intermediate" }
                                span { "15 min read" }
                            }
                        }
                    }

                    // Complex Guide Card
                    Link {
                        to: Route::ComplexAnimationGuide {},
                        class: "group relative overflow-hidden rounded-xl bg-dark-200/50 backdrop-blur-xs
                               border border-primary/10 transition-all duration-300 hover:border-primary/20
                               hover:shadow-lg hover:shadow-primary/10",
                        div { class: "p-6",
                            div { class: "flex items-center justify-between mb-4",
                                h3 { class: "text-xl font-semibold text-text-primary",
                                    "Custom Animation Guide"
                                }
                                span { class: "text-primary transform transition-transform group-hover:translate-x-1",
                                    "→"
                                }
                            }
                            p { class: "text-text-secondary leading-relaxed mb-3",
                                "Master advanced techniques by creating custom animatable types. Perfect for complex UI elements and creative animations."
                            }
                            div { class: "flex items-center text-xs text-primary/80",
                                span { class: "mr-2 px-2 py-0.5 bg-primary/10 rounded-sm", "Advanced" }
                                span { "20 min read" }
                            }
                        }
                    }
                }
            }
        }
    }
}
