use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

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
                            p { class: "font-medium text-text-primary mb-1", "1. Create a motion value" }
                            code { class: "text-sm text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm",
                                "let mut value = use_motion_store(0.0f32);"
                            }
                        }
                        // Key concept 2
                        div { class: "p-3 bg-dark-200/50 rounded-lg",
                            p { class: "font-medium text-text-primary mb-1", "2. Animate the value" }
                            code { class: "text-sm text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm",
                                "value.animate_to(100.0, config);"
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
    let mut value = use_motion_store(0.0f32);

    let animate = move |_| {
        value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Linear::ease_in_out,
            })),
        );
    };

    let reset = move |_| {
        value.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );
    };

    rsx! {
        section { class: "space-y-6",
            // Title and description
            div {
                h2 { class: "text-xl font-semibold mb-2", "Step 1: Your First Animation" }
                p { class: "text-text-secondary",
                    "Let's create a simple animation that moves a progress bar from 0% to 100%."
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
use dioxus_motion::prelude::*;

// 2. Create a motion value
let mut value = use_motion_store(0.0f32);

// 3. Animate the value
value.animate_to(
    100.0,
    AnimationConfig::new(AnimationMode::Tween(Tween {
        duration: Duration::from_millis(1000),
        easing: easer::functions::Linear::ease_in_out,
    })),
);"#.to_string(),
                            language: "rust".to_string(),
                        }
                    }

                    // Key points
                    div { class: "mt-4 p-3 bg-primary/5 rounded-lg border border-primary/10",
                        h4 { class: "font-medium text-primary mb-2", "What's happening here?" }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1 text-sm",
                            li { "We create a motion value starting at 0" }
                            li { "We animate it to 100 over 1 second" }
                            li { "We use a linear easing function for smooth motion" }
                        }
                    }
                }

                // Right column: Live preview
                div { class: "space-y-4",
                    h3 { class: "font-medium text-text-primary", "Live Preview" }
                    div { class: "p-4 bg-dark-200/30 rounded-lg space-y-4",
                        // Animation preview
                        div { class: "relative h-16 bg-dark-200/30 rounded-lg overflow-hidden",
                            div {
                                class: "absolute h-16 bg-primary/50 rounded-lg",
                                style: "width: {value.store().current()()}%"
                            }
                        }

                        // Value display
                        div { class: "text-sm text-text-secondary",
                            "Current value: {value.store().current()():.1}"
                        }

                        // Controls
                        div { class: "flex gap-2 mt-2",
                            button {
                                class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                                onclick: animate,
                                "Animate"
                            }
                            button {
                                class: "px-4 py-2 bg-dark-200 hover:bg-dark-300 rounded-lg text-text-secondary transition-colors",
                                onclick: reset,
                                "Reset"
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
                        span { class: "font-medium", "Different value types: " }
                        "You can animate f32, f64, Transform, Color, and custom types."
                    }
                    li {
                        span { class: "font-medium", "Reading values: " }
                        "Use ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "value.store().current()" }, " to access the current value at any time."
                    }
                }
            }
        }
    }
}

#[component]
fn StepTwo() -> Element {
    let mut tween_value = use_motion_store(0.0f32);
    let mut spring_value = use_motion_store(0.0f32);

    let animate_tween = move |_| {
        tween_value.animate_to(
            100.0,
            AnimationConfig::custom_tween(
                Duration::from_millis(1000),
                easer::functions::Cubic::ease_in_out,
            ),
        );
    };

    let animate_spring = move |_| {
        spring_value.animate_to(100.0, AnimationConfig::custom_spring(100.0, 10.0, 1.0));
    };

    // Instead of using separate reset functions, let's simplify
    let reset_both = move |_| {
        // Reset tween value
        tween_value.animate_to(0.0, AnimationConfig::tween());

        // Reset spring value
        spring_value.animate_to(0.0, AnimationConfig::spring());
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
                            "AnimationMode::Tween(Tween {{\n  duration: Duration::from_millis(1000),\n  easing: easer::functions::Cubic::ease_in_out\n}})"
                        }
                    }

                    // Preview
                    div { class: "relative h-12 bg-dark-200/30 rounded-lg overflow-hidden",
                        div {
                            class: "absolute h-12 bg-primary/50 rounded-lg",
                            style: "width: {tween_value.store().current()()}%"
                        }
                    }

                    button {
                        class: "w-full mt-2 px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: animate_tween,
                        "Run Tween"
                    }
                }

                // Spring example
                div { class: "space-y-3 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium", "Spring Example" }

                    // Code snippet
                    div { class: "bg-dark-200/50 p-2 rounded-lg text-xs mb-3",
                        code { class: "text-primary/90",
                            "AnimationMode::Spring(Spring {{\n  stiffness: 100.0,  // Spring force\n  damping: 10.0,     // Bounce reduction\n  mass: 1.0          // Weight\n}})"
                        }
                    }

                    // Preview
                    div { class: "relative h-12 bg-dark-200/30 rounded-lg overflow-hidden",
                        div {
                            class: "absolute h-12 bg-primary/50 rounded-lg",
                            style: "width: {spring_value.store().current()()}%"
                        }
                    }

                    button {
                        class: "w-full mt-2 px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: animate_spring,
                        "Run Spring"
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
            }
        }
    }
}
