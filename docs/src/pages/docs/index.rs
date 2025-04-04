use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::components::code_block::CodeBlock;
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
            div { class: "relative z-10 w-full",
                // Header
                div { class: "border-b border-surface-light/20 w-full",
                    div { class: "w-full px-4 sm:px-6 lg:px-8 py-6",
                        h1 { class: "text-4xl font-bold text-text-primary mb-2", {title} }
                        p { class: "text-lg text-text-secondary", {description} }
                    }
                }

                // Main content with three columns
                div { class: "w-full px-4 sm:px-6 lg:px-8 py-8",
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
                                        to: Route::Animations {},
                                        icon: "✨",
                                        label: "Interactive Animation Guide",
                                    }
                                }
                            }
                        }

                        // Main content
                        div { class: "flex-[4] min-w-0",   // Animated outlet for nested routes
                            AnimatedOutlet::<Route> {} }

                        // Right sidebar - Related links
                        div { class: "hidden lg:block flex-1",
                            div { class: "sticky top-24", RelatedLinks {} }
                        }
                    }
                }
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
        div {
            class: "bg-yellow-500/10 border border-yellow-500/20 rounded-lg p-4 mb-6 mx-4",
            p {
                class: "text-yellow-500 text-sm",
                "⚠️ Documentation is currently under construction. Some sections may be incomplete or subject to change."
            }
        }
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
            // Installation Guide
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Installation" }
                p { class: "text-text-secondary",
                    "Get started with Dioxus Motion by adding it to your project's Cargo.toml."
                }

                // Basic Installation
                div { class: "space-y-4",
                    h3 { class: "text-lg font-semibold text-text-primary", "Basic Setup" }
                    div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
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
                }

                // With Page Transitions
                div { class: "space-y-4",
                    h3 { class: "text-lg font-semibold text-text-primary", "With Page Transitions" }
                    div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
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
                }

                // Platform Support
                div { class: "space-y-4",
                    h3 { class: "text-lg font-semibold text-text-primary", "Platform Support" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                            h4 { class: "font-semibold text-text-primary mb-2", "Web" }
                            p { class: "text-text-secondary text-sm",
                                "For web applications using WASM"
                            }
                        }
                        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                            h4 { class: "font-semibold text-text-primary mb-2", "Desktop" }
                            p { class: "text-text-secondary text-sm",
                                "For desktop and mobile applications"
                            }
                        }
                        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                            h4 { class: "font-semibold text-text-primary mb-2", "Default" }
                            p { class: "text-text-secondary text-sm",
                                "Web support (if no feature specified)"
                            }
                        }
                    }
                }
            }

            // Guide sections
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Guides" }
                div { class: "grid md:grid-cols-2 gap-6",
                    // Page Transitions Card
                    Link {
                        to: Route::PageTransition {},
                        class: "group relative overflow-hidden rounded-xl bg-dark-200/50 backdrop-blur-sm
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
                            p { class: "text-text-secondary leading-relaxed",
                                "Learn how to create smooth page transitions and routing animations in your Dioxus app."
                            }
                        }
                    }

                    // Animations Card
                    Link {
                        to: Route::Animations {},
                        class: "group relative overflow-hidden rounded-xl bg-dark-200/50 backdrop-blur-sm
                               border border-primary/10 transition-all duration-300 hover:border-primary/20
                               hover:shadow-lg hover:shadow-primary/10",
                        div { class: "p-6",
                            div { class: "flex items-center justify-between mb-4",
                                h3 { class: "text-xl font-semibold text-text-primary",
                                    "Interactive Animation Guide"
                                }
                                span { class: "text-primary transform transition-transform group-hover:translate-x-1",
                                    "→"
                                }
                            }
                            p { class: "text-text-secondary leading-relaxed",
                                "Learn animation basics with interactive examples, from simple tweens to custom types."
                            }
                        }
                    }
                }
            }
        }
    }
}
