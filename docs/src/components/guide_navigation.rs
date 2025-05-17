use dioxus::prelude::*;

use crate::utils::router::Route;

/// Represents a guide section with its route and title
struct GuideSection {
    route: Route,
    title: &'static str,
}

/// The ordered list of guide sections for navigation
fn guide_sections() -> Vec<GuideSection> {
    vec![
        GuideSection {
            route: Route::PageTransition {},
            title: "Page Transitions",
        },
        GuideSection {
            route: Route::BasicAnimationGuide {},
            title: "Basic Animation Guide",
        },
        GuideSection {
            route: Route::IntermediateAnimationGuide {},
            title: "Intermediate Animation Guide",
        },
        GuideSection {
            route: Route::ComplexAnimationGuide {},
            title: "Complex Animation Guide",
        },
    ]
}

#[component]
/// Renders a navigation component for moving between guide sections.
///
/// This component displays previous and next links based on the current route,
/// allowing users to navigate sequentially through the documentation.
/// It's designed to be responsive and mobile-friendly.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// fn app() -> Element {
///     rsx! {
///         // Your page content
///         GuideNavigation {}
///     }
/// }
/// ```
pub fn GuideNavigation() -> Element {
    let current_route = use_route::<Route>();
    let sections = guide_sections();

    // Find the current section index
    let current_index = sections
        .iter()
        .position(|section| section.route == current_route)
        .unwrap_or(0);

    // Determine previous and next sections
    let prev_section = if current_index > 0 {
        Some(&sections[current_index - 1])
    } else {
        None
    };

    let next_section = if current_index < sections.len() - 1 {
        Some(&sections[current_index + 1])
    } else {
        None
    };

    rsx! {
        // Navigation container
        nav { class: "mt-12 pt-6 border-t border-primary/10",
            div { class: "flex flex-col sm:flex-row justify-between items-center gap-4",

                // Previous link
                div { class: "w-full sm:w-auto",
                    if let Some(prev) = prev_section {
                        Link {
                            to: prev.route.clone(),
                            class: "flex items-center justify-center sm:justify-start gap-2 w-full px-4 py-3
                                   bg-dark-200/50 backdrop-blur-xs rounded-lg
                                   border border-primary/10 hover:border-primary/20
                                   text-text-secondary hover:text-text-primary
                                   transition-all duration-300 group",

                            // Arrow icon
                            span { class: "text-primary transform transition-transform group-hover:-translate-x-1",
                                "←"
                            }

                            // Label
                            div { class: "flex flex-col",
                                span { class: "text-xs text-text-muted", "Previous" }
                                span { class: "text-sm font-medium", {prev.title} }
                            }
                        }
                    } else {
                        // Empty div to maintain layout when there's no previous link
                        div { class: "hidden sm:block" }
                    }
                }

                // Next link
                div { class: "w-full sm:w-auto",
                    if let Some(next) = next_section {
                        Link {
                            to: next.route.clone(),
                            class: "flex items-center justify-center sm:justify-end gap-2 w-full px-4 py-3
                                   bg-dark-200/50 backdrop-blur-xs rounded-lg
                                   border border-primary/10 hover:border-primary/20
                                   text-text-secondary hover:text-text-primary
                                   transition-all duration-300 group",

                            // Label
                            div { class: "flex flex-col items-end",
                                span { class: "text-xs text-text-muted", "Next" }
                                span { class: "text-sm font-medium", {next.title} }
                            }

                            // Arrow icon
                            span { class: "text-primary transform transition-transform group-hover:translate-x-1",
                                "→"
                            }
                        }
                    } else {
                        // Empty div to maintain layout when there's no next link
                        div { class: "hidden sm:block" }
                    }
                }
            }
        }
    }
}
