use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::components::code_block::CodeBlock;

#[component]
pub fn BasicAnimationGuide() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Animation Guide" }
                p { class: "text-text-secondary",
                    "Learn how to use Dioxus Motion through practical examples. Each step builds upon the previous one, introducing new concepts gradually."
                }
            }

            // Step 1: Basic Value Animation
            StepOne {}

            // Step 2: Animation Modes
            StepTwo {}
        }
    }
}

#[component]
fn StepOne() -> Element {
    let mut value = use_motion(0.0f32);

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
        section { class: "space-y-4",
            // Title and description
            div {
                h2 { class: "text-2xl font-semibold mb-2", "Step 1: Basic Value Animation" }
                p { class: "text-text-secondary",
                    "Let's start with the simplest possible animation - animating a single value from 0 to 100."
                }
            }

            // Code example
            div { class: "space-y-2",
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"// 1. Import the prelude
use dioxus_motion::prelude::*;

// 2. Create a motion value
let mut value = use_motion(0.0f32);

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

                // Live preview
                div { class: "space-y-3 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium", "Live Preview" }

                    // Animation preview
                    div { class: "relative h-16 bg-dark-200/30 rounded-lg overflow-hidden",
                        div {
                            class: "absolute h-16 bg-primary/50 rounded-lg",
                            style: "width: {value.get_value()}%"
                        }
                    }

                    // Value display
                    div { class: "text-sm text-text-secondary",
                        "Current value: {value.get_value():.1}"
                    }

                    // Controls
                    div { class: "flex gap-2",
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

            // Key points
            div { class: "space-y-2",
                h3 { class: "font-medium", "Key Points:" }
                ul { class: "list-disc list-inside text-text-secondary space-y-1",
                    li { "Use ", code { "use_motion()" }, " to create an animatable value" }
                    li { "Values can be any type that implements the ", code { "Animatable" }, " trait" }
                    li { "Use ", code { "animate_to()" }, " to start an animation" }
                    li { "Use ", code { "get_value()" }, " to read the current value" }
                }
            }
        }
    }
}

#[component]
fn StepTwo() -> Element {
    let mut tween_value = use_motion(0.0f32);
    let mut spring_value = use_motion(0.0f32);

    let animate_tween = move |_| {
        tween_value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            })),
        );
    };

    let animate_spring = move |_| {
        spring_value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    };

    let reset = move |_| {
        let tween_config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));
        let spring_config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));

        tween_value.animate_to(0.0, tween_config);
        spring_value.animate_to(0.0, spring_config);
    };

    rsx! {
        section { class: "space-y-6",
            // Title and description
            div {
                h2 { class: "text-2xl font-semibold mb-2", "Step 2: Animation Modes" }
                p { class: "text-text-secondary",
                    "There are two main types of animations: Tween and Spring. Each serves different purposes and creates different feelings of motion."
                }
            }

            // Examples
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                // Tween example
                div { class: "space-y-4",
                    h3 { class: "font-medium", "Tween Animation" }
                    p { class: "text-sm text-text-secondary",
                        "Time-based animations with precise control over duration and easing."
                    }

                    // Preview
                    div { class: "relative h-16 bg-dark-200/30 rounded-lg overflow-hidden",
                        div {
                            class: "absolute h-16 bg-primary/50 rounded-lg",
                            style: "width: {tween_value.get_value()}%"
                        }
                    }

                    button {
                        class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: animate_tween,
                        "Run Tween"
                    }
                }

                // Spring example
                div { class: "space-y-4",
                    h3 { class: "font-medium", "Spring Animation" }
                    p { class: "text-sm text-text-secondary",
                        "Physics-based animations that create natural, dynamic motion."
                    }

                    // Preview
                    div { class: "relative h-16 bg-dark-200/30 rounded-lg overflow-hidden",
                        div {
                            class: "absolute h-16 bg-primary/50 rounded-lg",
                            style: "width: {spring_value.get_value()}%"
                        }
                    }

                    button {
                        class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: animate_spring,
                        "Run Spring"
                    }
                }
            }

            // Reset button
            div { class: "flex justify-center",
                button {
                    class: "px-4 py-2 bg-dark-200 hover:bg-dark-300 rounded-lg text-text-secondary transition-colors",
                    onclick: reset,
                    "Reset Both"
                }
            }

            // Configuration examples
            div { class: "space-y-4",
                h3 { class: "font-medium", "Configuration" }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    // Tween config
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h4 { class: "font-medium mb-2", "Tween Configuration" }
                        CodeBlock {
                            code: r#"AnimationMode::Tween(Tween {
    duration: Duration::from_millis(1000),
    easing: easer::functions::Cubic::ease_in_out,
})"#.to_string(),
                            language: "rust".to_string(),
                        }
                    }

                    // Spring config
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h4 { class: "font-medium mb-2", "Spring Configuration" }
                        CodeBlock {
                            code: r#"AnimationMode::Spring(Spring {
    stiffness: 100.0,  // Spring force
    damping: 10.0,     // Bounce reduction
    mass: 1.0,         // Weight
    velocity: 0.0,     // Initial velocity
})"#.to_string(),
                            language: "rust".to_string(),
                        }
                    }
                }
            }
        }
    }
}
