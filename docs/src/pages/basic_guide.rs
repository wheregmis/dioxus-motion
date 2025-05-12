use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_motion::{AnimationTarget, TransitionConfig, TransitionType};

use crate::components::code_block::CodeBlock;
use crate::components::guide_navigation::GuideNavigation;

#[component]
pub fn BasicAnimationGuide() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Basic Animation Guide" }
                p { class: "text-text-secondary",
                    "Get started with Dioxus Motion animations through simple examples. This guide covers the fundamentals you need to create your first animations."
                }

                // Quick reference card
                div { class: "mt-6 p-4 bg-primary/5 rounded-lg border border-primary/10",
                    h3 { class: "text-lg font-medium text-primary mb-2", "Quick Reference" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        // Key concept 1
                        div { class: "p-3 bg-dark-200/50 rounded-lg",
                            p { class: "font-medium text-text-primary mb-1", "1. Use motion primitives" }
                            code { class: "text-sm text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm",
                                "motion::div"
                            }
                        }
                        // Key concept 2
                        div { class: "p-3 bg-dark-200/50 rounded-lg",
                            p { class: "font-medium text-text-primary mb-1", "2. Define animation properties" }
                            code { class: "text-sm text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm",
                                "initial: Some(AnimationTarget::new()...)"
                            }
                        }
                    }
                }
            }

            // Step 1: Basic Value Animation
            StepOne {}

            // Step 2: Animation Modes
            StepTwo {}

            // Navigation links
            GuideNavigation {}
        }
    }
}

#[component]
fn StepOne() -> Element {
    let mut show_animated = use_signal(|| false);

    let toggle_animation = move |_| {
        show_animated.set(!show_animated());
    };

    rsx! {
        section { class: "space-y-6",
            // Title and description
            div {
                h2 { class: "text-xl font-semibold mb-2", "Step 1: Your First Animation" }
                p { class: "text-text-secondary",
                    "Let's create a simple animation that moves a progress bar from 0% to 100% using motion primitives."
                }
            }

            // Two-column layout for desktop
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                // Left column: Code example
                div { class: "space-y-4",
                    h3 { class: "font-medium text-text-primary", "The Code" }
                    div { class: "bg-dark-200/50 p-3 rounded-lg",
                        CodeBlock {
                            code: r#"// 1. Import the prelude
use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_motion::{AnimationTarget, TransitionConfig, TransitionType};

// 2. Create a component with motion primitives
#[component]
fn ProgressBar() -> Element {
    let mut show_progress = use_signal(|| false);

    let toggle = move |_| {
        show_progress.set(!show_progress());
    };

    rsx! {
        // 3. Use motion::div with animation properties
        div { class: "relative h-16 bg-dark-200/30 rounded-lg overflow-hidden",
            motion::div {
                class: "absolute h-16 bg-primary/50 rounded-lg",
                style: if show_progress() { "width: 100%" } else { "width: 0%" },
                initial: Some(AnimationTarget::new().opacity(1.0)),
                animate: Some(AnimationTarget::new().opacity(1.0)),
                transition: Some(
                    TransitionConfig::default()
                        .type_(TransitionType::Tween)
                        .duration(1.0)
                ),
            }
        }

        button {
            onclick: toggle,
            if show_progress() { "Reset" } else { "Animate" }
        }
    }
}"#.to_string(),
                            language: "rust".to_string(),
                        }
                    }

                    // Key points
                    div { class: "mt-4 p-3 bg-primary/5 rounded-lg border border-primary/10",
                        h4 { class: "font-medium text-primary mb-2", "What's happening here?" }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1 text-sm",
                            li { "We use a motion::div component instead of a regular div" }
                            li { "We define initial and animate states declaratively" }
                            li { "The animation automatically runs when the animate property changes" }
                            li { "No need to manually manage animation state or update values" }
                        }
                    }
                }

                // Right column: Live preview
                div { class: "space-y-4",
                    h3 { class: "font-medium text-text-primary", "Live Preview" }
                    div { class: "p-4 bg-dark-200/30 rounded-lg space-y-4",
                        // Animation preview
                        div { class: "relative h-16 bg-dark-200/30 rounded-lg overflow-hidden",
                            motion::div {
                                class: "absolute h-16 bg-primary/50 rounded-lg",
                                style: if show_animated() { "width: 100%" } else { "width: 0%" },
                                initial: Some(AnimationTarget::new().opacity(1.0)),
                                animate: Some(AnimationTarget::new().opacity(1.0)),
                                transition: Some(
                                    TransitionConfig::default()
                                        .type_(TransitionType::Tween)
                                        .duration(1.0)
                                ),
                            }
                        }

                        // Value display
                        div { class: "text-sm text-text-secondary",
                            "Current width: ", if show_animated() { "100%" } else { "0%" }
                        }

                        // Controls
                        div { class: "flex gap-2 mt-2",
                            button {
                                class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                                onclick: toggle_animation,
                                if show_animated() { "Reset" } else { "Animate" }
                            }
                        }
                    }
                }
            }

            // Tips box
            div { class: "mt-4 p-4 bg-dark-200/50 rounded-lg border border-primary/10",
                h3 { class: "font-medium text-text-primary mb-2", "💡 Pro Tips" }
                ul { class: "list-disc list-inside text-text-secondary space-y-2",
                    li {
                        span { class: "font-medium", "Animation properties: " }
                        "Use initial, animate, while_hover, while_tap, and exit to define different animation states."
                    }
                    li {
                        span { class: "font-medium", "Transition options: " }
                        "Customize animations with ",
                        code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "TransitionConfig" },
                        " to control timing, easing, and physics."
                    }
                    li {
                        span { class: "font-medium", "Available primitives: " }
                        "The library provides motion versions of common HTML elements like div, button, span, etc."
                    }
                }
            }
        }
    }
}

#[component]
fn StepTwo() -> Element {
    let mut show_tween = use_signal(|| false);
    let mut show_spring = use_signal(|| false);

    let toggle_tween = move |_| {
        show_tween.set(!show_tween());
    };

    let toggle_spring = move |_| {
        show_spring.set(!show_spring());
    };

    let reset_both = move |_| {
        show_tween.set(false);
        show_spring.set(false);
    };

    rsx! {
        section { class: "space-y-6",
            // Title and description
            div {
                h2 { class: "text-xl font-semibold mb-2", "Step 2: Tween vs Spring Animations" }
                p { class: "text-text-secondary",
                    "Compare the two main animation types: Tween (time-based) and Spring (physics-based)."
                }
            }

            // Comparison card
            div { class: "p-4 bg-dark-200/50 rounded-lg mb-6",
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    // Tween description
                    div { class: "p-3 bg-dark-200/80 rounded-lg",
                        h3 { class: "font-medium text-primary mb-2", "Tween Animation" }
                        ul { class: "text-sm text-text-secondary space-y-1 list-disc list-inside",
                            li { "Time-based with fixed duration" }
                            li { "Precise control over timing" }
                            li { "Uses easing functions" }
                            li { "Best for UI transitions" }
                        }
                    }
                    // Spring description
                    div { class: "p-3 bg-dark-200/80 rounded-lg",
                        h3 { class: "font-medium text-primary mb-2", "Spring Animation" }
                        ul { class: "text-sm text-text-secondary space-y-1 list-disc list-inside",
                            li { "Physics-based, no fixed duration" }
                            li { "Natural, organic motion" }
                            li { "Configurable stiffness & damping" }
                            li { "Best for interactive elements" }
                        }
                    }
                }
            }

            // Interactive examples
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                // Tween example
                div { class: "space-y-3 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium", "Tween Example" }

                    // Code snippet
                    div { class: "bg-dark-200/50 p-2 rounded-lg text-xs mb-3",
                        code { class: "text-primary/90",
                            "motion::div {{\n  // Other properties...\n  transition: Some(\n    TransitionConfig::default()\n      .type_(TransitionType::Tween)\n      .duration(1.0)\n      .ease(\"cubic-bezier(0.65, 0, 0.35, 1)\")\n  )\n}}"
                        }
                    }

                    // Preview
                    div { class: "relative h-12 bg-dark-200/30 rounded-lg overflow-hidden",
                        motion::div {
                            class: "absolute h-12 bg-primary/50 rounded-lg",
                            style: if show_tween() { "width: 100%" } else { "width: 0%" },
                            transition: Some(
                                TransitionConfig::default()
                                    .type_(TransitionType::Tween)
                                    .duration(1.0)
                            ),
                        }
                    }

                    button {
                        class: "w-full mt-2 px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: toggle_tween,
                        if show_tween() { "Reset Tween" } else { "Run Tween" }
                    }
                }

                // Spring example
                div { class: "space-y-3 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium", "Spring Example" }

                    // Code snippet
                    div { class: "bg-dark-200/50 p-2 rounded-lg text-xs mb-3",
                        code { class: "text-primary/90",
                            "motion::div {{\n  // Other properties...\n  transition: Some(\n    TransitionConfig::default()\n      .type_(TransitionType::Spring)\n      .stiffness(100.0)  // Spring force\n      .damping(10.0)     // Bounce reduction\n  )\n}}"
                        }
                    }

                    // Preview
                    div { class: "relative h-12 bg-dark-200/30 rounded-lg overflow-hidden",
                        motion::div {
                            class: "absolute h-12 bg-primary/50 rounded-lg",
                            style: if show_spring() { "width: 100%" } else { "width: 0%" },
                            transition: Some(
                                TransitionConfig::default()
                                    .type_(TransitionType::Spring)
                                    .stiffness(100.0)
                                    .damping(10.0)
                            ),
                        }
                    }

                    button {
                        class: "w-full mt-2 px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: toggle_spring,
                        if show_spring() { "Reset Spring" } else { "Run Spring" }
                    }
                }
            }

            // Reset button
            div { class: "flex justify-center mt-4",
                button {
                    class: "px-6 py-2 bg-dark-200 hover:bg-dark-300 rounded-lg text-text-secondary transition-colors",
                    onclick: reset_both,
                    "Reset Both"
                }
            }

            // When to use which
            div { class: "mt-6 p-4 bg-primary/5 rounded-lg border border-primary/10",
                h3 { class: "font-medium text-primary mb-2", "When to use which?" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 text-sm",
                    div {
                        p { class: "font-medium text-text-primary", "Use Tween for:" }
                        ul { class: "list-disc list-inside text-text-secondary mt-1 space-y-1",
                            li { "Page transitions" }
                            li { "Fade effects" }
                            li { "Precise timing requirements" }
                        }
                    }
                    div {
                        p { class: "font-medium text-text-primary", "Use Spring for:" }
                        ul { class: "list-disc list-inside text-text-secondary mt-1 space-y-1",
                            li { "Draggable elements" }
                            li { "Interactive UI" }
                            li { "Natural-feeling animations" }
                        }
                    }
                }

                // Motion primitives vs use_motion hook
                div { class: "mt-4 border-t border-primary/10 pt-4",
                    h3 { class: "font-medium text-primary mb-2", "Motion Primitives vs use_motion Hook" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 text-sm",
                        div {
                            p { class: "font-medium text-text-primary", "Use Motion Primitives for:" }
                            ul { class: "list-disc list-inside text-text-secondary mt-1 space-y-1",
                                li { "Most UI animations" }
                                li { "Declarative, simple animations" }
                                li { "Hover and tap interactions" }
                                li { "When you want less boilerplate" }
                            }
                        }
                        div {
                            p { class: "font-medium text-text-primary", "Use use_motion Hook for:" }
                            ul { class: "list-disc list-inside text-text-secondary mt-1 space-y-1",
                                li { "Complex animation sequences" }
                                li { "Custom animation logic" }
                                li { "When you need direct control" }
                                li { "Advanced use cases" }
                            }
                        }
                    }
                }
            }
        }
    }
}
