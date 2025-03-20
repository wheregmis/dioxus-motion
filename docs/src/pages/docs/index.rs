use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::utils::router::Route;

#[component]
fn DocLayout(title: &'static str, description: &'static str, children: Element) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gradient-dark relative overflow-hidden w-full",

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
                        h1 {
                            class: "text-4xl font-bold text-text-primary mb-2",
                            {title}
                        }
                        p {
                            class: "text-lg text-text-secondary",
                            {description}
                        }
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
                                        label: "Getting Started"
                                    }
                                    SectionLink {
                                        to: Route::PageTransition {},
                                        icon: "🔄",
                                        label: "Page Transitions"
                                    }
                                    SectionLink {
                                        to: Route::Animations {},
                                        icon: "✨",
                                        label: "Animations"
                                    }
                                }
                            }
                        }

                        // Main content
                        div { class: "flex-[4] min-w-0",
                            {children}
                        }

                        // Right sidebar - Related links
                        div { class: "hidden lg:block flex-1",
                            div { class: "sticky top-24",
                                RelatedLinks {}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Docs() -> Element {
    rsx! {
        DocLayout {
            title: "Documentation",
            description: "Learn how to use Dioxus Motion to create beautiful animations in your applications.",
            AnimatedOutlet::<Route> {}
        }
    }
}

#[component]
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
                    format!("{} text-text-secondary hover:text-text-primary hover:bg-surface-light/10", base_classes)
                }
            },
            span { class: "text-lg", {icon} }
            span { {label} }
        }
    }
}

#[component]
fn RelatedLinks() -> Element {
    rsx! {
        div { class: "space-y-6",
            // Resources
            div { class: "space-y-4",
                h3 { class: "text-sm font-semibold text-text-primary uppercase tracking-wider", "Resources" }
                div { class: "space-y-2",
                    ResourceLink {
                        href: "https://github.com/wheregmis/dioxus-motion",
                        label: "GitHub Repository",
                        icon: "📦"
                    }
                    ResourceLink {
                        href: "https://crates.io/crates/dioxus-motion",
                        label: "Crates.io",
                        icon: "📦"
                    }
                    ResourceLink {
                        href: "https://docs.rs/dioxus-motion",
                        label: "API Documentation",
                        icon: "📚"
                    }
                }
            }

            // Community
            div { class: "space-y-4",
                h3 { class: "text-sm font-semibold text-text-primary uppercase tracking-wider", "Community" }
                div { class: "space-y-2",
                    ResourceLink {
                        href: "https://discord.gg/XgGxHRRc",
                        label: "Discord",
                        icon: "💬"
                    }
                    ResourceLink {
                        href: "https://github.com/wheregmis/dioxus-motion/issues",
                        label: "Report Issues",
                        icon: "🐛"
                    }
                }
            }
        }
    }
}

#[component]
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
pub fn DocsLanding() -> Element {
    rsx! {
        div { class: "space-y-12",

            // Guide sections
            div { class: "grid md:grid-cols-2 gap-6",
                // Page Transitions Card
                Link {
                    to: Route::PageTransition {},
                    class: "group relative overflow-hidden rounded-xl bg-dark-200/50 backdrop-blur-sm
                           border border-primary/10 transition-all duration-300 hover:border-primary/20
                           hover:shadow-lg hover:shadow-primary/10",
                    div {
                        class: "p-6",
                        div {
                            class: "flex items-center justify-between mb-4",
                            h3 { class: "text-xl font-semibold text-text-primary", "Page Transitions" }
                            span { class: "text-primary transform transition-transform group-hover:translate-x-1", "→" }
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
                    div {
                        class: "p-6",
                        div {
                            class: "flex items-center justify-between mb-4",
                            h3 { class: "text-xl font-semibold text-text-primary", "Animations" }
                            span { class: "text-primary transform transition-transform group-hover:translate-x-1", "→" }
                        }
                        p { class: "text-text-secondary leading-relaxed",
                            "Master component animations using springs, tweens, and transforms."
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PageTransition() -> Element {
    rsx! {
        div { class: "space-y-12",
        // Quick Start
        section { class: "space-y-6",
            h2 { class: "text-2xl font-semibold text-text-primary", "Quick Start" }
            div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
                pre { class: "language-rust overflow-x-auto",
                    code {
                        {"use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
pub enum Route {
#[layout(NavBar)]
#[route(\"/\")]
#[transition(SlideDown)]
Home {},

#[route(\"/about\")]
#[transition(SlideLeft)]
About {},
}"}
                    }
                }
            }
        }

        // Available Transitions
        section { class: "space-y-6",
            h2 { class: "text-2xl font-semibold text-text-primary", "Available Transitions" }
            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                TransitionCard {
                    name: "SlideDown",
                    description: "Slides content down from the top",
                    example: "Perfect for main navigation or landing pages"
                }
                TransitionCard {
                    name: "SlideUp",
                    description: "Slides content up from the bottom",
                    example: "Great for modal dialogs or bottom sheets"
                }
                TransitionCard {
                    name: "SlideLeft",
                    description: "Slides content in from the left",
                    example: "Ideal for forward navigation"
                }
                TransitionCard {
                    name: "SlideRight",
                    description: "Slides content in from the right",
                    example: "Perfect for backward navigation"
                }
            }
        }

        // Custom Transitions
        section { class: "space-y-6",
            h2 { class: "text-2xl font-semibold text-text-primary", "Custom Transitions" }
            div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
                pre { class: "language-rust overflow-x-auto",
                    code {
                        {"#[derive(Clone, Debug, PartialEq)]
pub struct CustomTransition;

impl Transition for CustomTransition {
fn enter(&self, element: &Element) -> AnimationConfig {
AnimationConfig::new(AnimationMode::Spring(Spring {
    stiffness: 100.0,
    damping: 15.0,
    mass: 1.0,
    velocity: 0.0,
}))
}

fn exit(&self, element: &Element) -> AnimationConfig {
AnimationConfig::new(AnimationMode::Tween(Tween {
    duration: Duration::from_millis(300),
    easing: easer::functions::Cubic::ease_in_out,
}))
}
}"}
                    }
                }
            }
        }
    }

    }
}

#[component]
pub fn Animations() -> Element {
    rsx! {
        div { class: "space-y-12",
                // Spring Animations
                section { class: "space-y-6",
                    h2 { class: "text-2xl font-semibold text-text-primary", "Spring Animations" }
                    p { class: "text-text-secondary",
                        "Spring animations provide natural, physics-based motion that feels organic and responsive."
                    }
                    div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
                        pre { class: "language-rust overflow-x-auto",
                            code {
                                {"let mut scale = use_motion(1.0f32);

// Basic spring animation
scale.animate_to(
    1.2,
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 100.0,  // Controls the spring's strength
        damping: 10.0,     // Controls how quickly the spring settles
        mass: 1.0,         // Controls the weight of the spring
        velocity: 0.0,     // Initial velocity
    })),
);

// Spring with loop
scale.animate_to(
    1.2,
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
    }))
    .with_loop(LoopMode::Infinite),
);"}
                            }
                        }
                    }
                }

                // Tween Animations
                section { class: "space-y-6",
                    h2 { class: "text-2xl font-semibold text-text-primary", "Tween Animations" }
                    p { class: "text-text-secondary",
                        "Tween animations provide precise control over timing and easing functions for predictable animations."
                    }
                    div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
                        pre { class: "language-rust overflow-x-auto",
                            code {
                                {"let mut opacity = use_motion(0.0f32);

// Basic tween animation
opacity.animate_to(
    1.0,
    AnimationConfig::new(AnimationMode::Tween(Tween {
        duration: Duration::from_millis(300),
        easing: easer::functions::Cubic::ease_out,
    })),
);

// Tween with custom easing
opacity.animate_to(
    1.0,
    AnimationConfig::new(AnimationMode::Tween(Tween {
        duration: Duration::from_millis(500),
        easing: easer::functions::Elastic::ease_out,
    })),
);"}
                            }
                        }
                    }
                }

                // Transform Animations
                section { class: "space-y-6",
                    h2 { class: "text-2xl font-semibold text-text-primary", "Transform Animations" }
                    p { class: "text-text-secondary",
                        "Transform animations allow you to animate multiple properties simultaneously with precise control."
                    }
                    div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl p-6 border border-primary/10",
                        pre { class: "language-rust overflow-x-auto",
                            code {
                                {"let mut transform = use_motion(Transform::new(0.0, 0.0, 1.0, 0.0));

// Basic transform animation
transform.animate_to(
    Transform::new(0.0, -20.0, 1.0, 0.0),
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
    })),
);

// Complex transform with multiple properties
transform.animate_to(
    Transform::new(10.0, -20.0, 1.2, 45.0),
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 200.0,
        damping: 15.0,
        mass: 0.8,
        velocity: 0.0,
    })),
);"}
                            }
                        }
                    }
                }

                // Best Practices
                section { class: "space-y-6",
                    h2 { class: "text-2xl font-semibold text-text-primary", "Best Practices" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                            h3 { class: "font-semibold text-text-primary mb-2", "Performance" }
                            ul { class: "list-disc list-inside text-text-secondary space-y-1",
                                li { "Use spring animations for natural motion" }
                                li { "Keep animations under 300ms for snappy feedback" }
                                li { "Avoid animating too many elements simultaneously" }
                                li { "Use transform instead of position for better performance" }
                            }
                        }
                        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                            h3 { class: "font-semibold text-text-primary mb-2", "UX Guidelines" }
                            ul { class: "list-disc list-inside text-text-secondary space-y-1",
                                li { "Maintain consistent animation durations" }
                                li { "Use easing functions that match your app's personality" }
                                li { "Provide visual feedback for user interactions" }
                                li { "Consider reduced motion preferences" }
                            }
                        }
                    }
                }
            }
    }
}
