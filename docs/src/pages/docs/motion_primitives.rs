use crate::components::code_block::CodeBlock;
use crate::components::guide_navigation::GuideNavigation;
use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn MotionPrimitivesGuide() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Motion Primitives Guide" }
                p { class: "text-text-secondary",
                    "Learn how to use the motion primitives in the dioxus-motion library to create fluid, interactive animations in your Dioxus applications."
                }

                // Quick reference card
                div { class: "mt-6 p-4 bg-primary/5 rounded-lg border border-primary/10",
                    h3 { class: "text-lg font-medium text-primary mb-2", "Quick Reference" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        // Key concept 1
                        div { class: "p-3 bg-dark-200/50 rounded-lg",
                            p { class: "font-medium text-text-primary mb-1", "1. Import the prelude" }
                            code { class: "text-sm text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm",
                                "use dioxus_motion::prelude::*;"
                            }
                        }
                        // Key concept 2
                        div { class: "p-3 bg-dark-200/50 rounded-lg",
                            p { class: "font-medium text-text-primary mb-1", "2. Use motion components" }
                            code { class: "text-sm text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm",
                                "motion::div {{ ... }}"
                            }
                        }
                    }
                }
            }

            // Basic Motion Components
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Basic Motion Components" }

                p { class: "text-text-secondary",
                    "The ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "motion::" },
                    " namespace provides motion-enabled versions of standard HTML elements. These components accept animation properties in addition to the standard properties."
                }

                h3 { class: "text-lg font-semibold mt-6 mb-2", "Example: Basic Animation" }

                // Code example
                div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10",
                    CodeBlock {
                        code: r#"#[component]
fn FadeInHeading() -> Element {
    rsx! {
        motion::h1 {
            class: "text-2xl font-bold",
            initial: Some(AnimationTarget::new().opacity(0.0).y(-20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
            ),
            "Hello, Motion!"
        }
    }
}"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                // Explanation
                div { class: "mt-4 p-3 bg-primary/5 rounded-lg text-sm text-text-secondary",
                    p { class: "mb-2",
                        "This example creates a heading that fades in and slides up when it appears on the screen."
                    }
                    ul { class: "list-disc list-inside space-y-1",
                        li {
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "initial" },
                            " defines the starting state (invisible and offset upward)"
                        }
                        li {
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "animate" },
                            " defines the final state (fully visible at the correct position)"
                        }
                        li {
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "transition" },
                            " configures how the animation should be performed"
                        }
                    }
                }
            }

            // Core Animation Properties
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Core Animation Properties" }

                // Properties grid
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    // initial property
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h3 { class: "text-lg font-semibold mb-2 text-primary", "initial" }

                        p { class: "text-text-secondary mb-3",
                            "The ",
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "initial" },
                            " property defines the starting state of the animation."
                        }

                        div { class: "bg-dark-300/50 p-3 rounded-lg",
                            CodeBlock {
                                code: "initial: Some(AnimationTarget::new().opacity(0.0).y(-20.0))".to_string(),
                                language: "rust".to_string(),
                            }
                        }
                    }

                    // animate property
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h3 { class: "text-lg font-semibold mb-2 text-primary", "animate" }

                        p { class: "text-text-secondary mb-3",
                            "The ",
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "animate" },
                            " property defines the end state of the animation."
                        }

                        div { class: "bg-dark-300/50 p-3 rounded-lg",
                            CodeBlock {
                                code: "animate: Some(AnimationTarget::new().opacity(1.0).y(0.0))".to_string(),
                                language: "rust".to_string(),
                            }
                        }
                    }

                    // transition property
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h3 { class: "text-lg font-semibold mb-2 text-primary", "transition" }

                        p { class: "text-text-secondary mb-3",
                            "The ",
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "transition" },
                            " property defines how the animation should be performed."
                        }

                        div { class: "bg-dark-300/50 p-3 rounded-lg",
                            CodeBlock {
                                code: r#"transition: Some(
    TransitionConfig::new(TransitionType::Spring)
        .stiffness(100.0)
        .damping(15.0)
        .delay(0.2)
)"#.to_string(),
                                language: "rust".to_string(),
                            }
                        }
                    }
                }

                // Additional explanation
                div { class: "mt-6 p-4 bg-primary/5 rounded-lg border border-primary/10",
                    h3 { class: "font-medium text-primary mb-2", "How It Works" }
                    p { class: "text-text-secondary",
                        "When a motion component mounts, it automatically animates from the ",
                        code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "initial" },
                        " state to the ",
                        code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "animate" },
                        " state using the configuration specified in ",
                        code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "transition" },
                        ". This declarative approach makes it easy to create complex animations with minimal code."
                    }
                }
            }

            // Transition Types
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Transition Types" }

                p { class: "text-text-secondary",
                    "Dioxus Motion supports two main types of animations: Spring and Tween. Each has its own characteristics and use cases."
                }

                // Comparison grid
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 mt-4",
                    // Spring Animation
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h3 { class: "text-lg font-semibold mb-3 text-primary", "Spring Animation" }

                        p { class: "text-text-secondary mb-3",
                            "Spring animations provide a natural, physics-based motion. They're great for creating realistic, fluid animations."
                        }

                        div { class: "bg-dark-300/50 p-3 rounded-lg mb-3",
                            CodeBlock {
                                code: r#"TransitionConfig::new(TransitionType::Spring)
    .stiffness(100.0)  // Controls the "tightness" of the spring
    .damping(15.0)     // Controls how quickly the spring comes to rest"#.to_string(),
                                language: "rust".to_string(),
                            }
                        }

                        // Spring properties
                        div { class: "space-y-2 text-sm",
                            div { class: "flex",
                                span { class: "font-medium text-primary/90 w-24", "stiffness:" }
                                span { class: "text-text-secondary", "Higher values create tighter, faster springs" }
                            }
                            div { class: "flex",
                                span { class: "font-medium text-primary/90 w-24", "damping:" }
                                span { class: "text-text-secondary", "Higher values reduce oscillation/bounce" }
                            }
                            div { class: "flex",
                                span { class: "font-medium text-primary/90 w-24", "mass:" }
                                span { class: "text-text-secondary", "Higher values create slower animations" }
                            }
                        }
                    }

                    // Tween Animation
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h3 { class: "text-lg font-semibold mb-3 text-primary", "Tween Animation" }

                        p { class: "text-text-secondary mb-3",
                            "Tween animations provide a more traditional, keyframe-based animation. They're useful for precise, controlled animations."
                        }

                        div { class: "bg-dark-300/50 p-3 rounded-lg mb-3",
                            CodeBlock {
                                code: r#"TransitionConfig::new(TransitionType::Tween)
    .duration(0.5)     // Duration in seconds
    .ease(EasingFunction::EaseInOut)"#.to_string(),
                                language: "rust".to_string(),
                            }
                        }

                        // Tween properties
                        div { class: "space-y-2 text-sm",
                            div { class: "flex",
                                span { class: "font-medium text-primary/90 w-24", "duration:" }
                                span { class: "text-text-secondary", "Animation length in seconds" }
                            }
                            div { class: "flex",
                                span { class: "font-medium text-primary/90 w-24", "ease:" }
                                span { class: "text-text-secondary", "Controls acceleration/deceleration pattern" }
                            }
                            div { class: "flex",
                                span { class: "font-medium text-primary/90 w-24", "delay:" }
                                span { class: "text-text-secondary", "Time to wait before starting animation" }
                            }
                        }
                    }
                }

                // When to use which
                div { class: "mt-6 p-4 bg-primary/5 rounded-lg border border-primary/10",
                    h3 { class: "font-medium text-primary mb-2", "When to use which?" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 text-sm",
                        div {
                            p { class: "font-medium text-text-primary", "Use Spring for:" }
                            ul { class: "list-disc list-inside text-text-secondary mt-1 space-y-1",
                                li { "Interactive elements (buttons, cards)" }
                                li { "Draggable components" }
                                li { "Natural-feeling animations" }
                                li { "When you want physics-based behavior" }
                            }
                        }
                        div {
                            p { class: "font-medium text-text-primary", "Use Tween for:" }
                            ul { class: "list-disc list-inside text-text-secondary mt-1 space-y-1",
                                li { "Precise timing requirements" }
                                li { "Sequential animations" }
                                li { "Fade effects" }
                                li { "When you need exact control over duration" }
                            }
                        }
                    }
                }
            }

            // Interactive Animations
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Interactive Animations" }

                p { class: "text-text-secondary",
                    "Dioxus Motion provides properties for creating interactive animations that respond to user input like hovering and clicking."
                }

                // Interactive properties grid
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6 mt-4",
                    // while_hover
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h3 { class: "text-lg font-semibold mb-3 text-primary", "while_hover" }

                        p { class: "text-text-secondary mb-3",
                            "The ",
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "while_hover" },
                            " property defines the state of the element when it's being hovered over."
                        }

                        div { class: "bg-dark-300/50 p-3 rounded-lg",
                            CodeBlock {
                                code: r#"motion::button {
    class: "px-4 py-2 bg-blue-500 text-white rounded",
    while_hover: Some(AnimationTarget::new().scale(1.05)),
    transition: Some(
        TransitionConfig::new(TransitionType::Spring)
            .stiffness(300.0)
            .damping(20.0)
    ),
    "Hover Me"
}"#.to_string(),
                                language: "rust".to_string(),
                            }
                        }

                        // Example use cases
                        div { class: "mt-3 text-sm text-text-secondary",
                            p { class: "font-medium", "Common hover effects:" }
                            ul { class: "list-disc list-inside mt-1 space-y-1",
                                li { "Scale up buttons and cards" }
                                li { "Change colors smoothly" }
                                li { "Reveal additional information" }
                            }
                        }
                    }

                    // while_tap
                    div { class: "bg-dark-200/50 p-4 rounded-lg",
                        h3 { class: "text-lg font-semibold mb-3 text-primary", "while_tap" }

                        p { class: "text-text-secondary mb-3",
                            "The ",
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "while_tap" },
                            " property defines the state of the element when it's being pressed."
                        }

                        div { class: "bg-dark-300/50 p-3 rounded-lg",
                            CodeBlock {
                                code: r#"motion::button {
    class: "px-4 py-2 bg-blue-500 text-white rounded",
    while_tap: Some(AnimationTarget::new().scale(0.95)),
    transition: Some(
        TransitionConfig::new(TransitionType::Spring)
            .stiffness(300.0)
            .damping(20.0)
    ),
    "Press Me"
}"#.to_string(),
                                language: "rust".to_string(),
                            }
                        }

                        // Example use cases
                        div { class: "mt-3 text-sm text-text-secondary",
                            p { class: "font-medium", "Common tap effects:" }
                            ul { class: "list-disc list-inside mt-1 space-y-1",
                                li { "Scale down buttons when pressed" }
                                li { "Create a 'push' effect" }
                                li { "Provide visual feedback for interactions" }
                            }
                        }
                    }
                }

                // Combined example
                div { class: "mt-6 p-4 bg-primary/5 rounded-lg border border-primary/10",
                    h3 { class: "font-medium text-primary mb-2", "Combining Interactive Properties" }
                    p { class: "text-text-secondary mb-3",
                        "You can combine ",
                        code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "while_hover" },
                        " and ",
                        code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "while_tap" },
                        " to create rich interactive experiences:"
                    }

                    div { class: "bg-dark-300/50 p-3 rounded-lg",
                        CodeBlock {
                            code: r#"motion::button {
    class: "px-6 py-3 bg-blue-600 text-white font-medium rounded-md shadow-sm",
    // Grow and change shadow on hover
    while_hover: Some(AnimationTarget::new().scale(1.05).shadow("0 10px 15px -3px rgba(0, 0, 0, 0.1)")),
    // Shrink when pressed
    while_tap: Some(AnimationTarget::new().scale(0.95)),
    // Spring transition for natural feel
    transition: Some(
        TransitionConfig::new(TransitionType::Spring)
            .stiffness(300.0)
            .damping(20.0)
    ),

    "Interactive Button"
}"#.to_string(),
                            language: "rust".to_string(),
                        }
                    }
                }
            }

            // Conclusion
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Conclusion" }

                p { class: "text-text-secondary",
                    "The dioxus-motion library provides a powerful set of primitives for creating fluid, interactive animations in your Dioxus applications. By combining these primitives with Dioxus's reactive programming model, you can create rich, engaging user experiences with minimal code."
                }

                div { class: "mt-4 p-4 bg-primary/5 rounded-lg border border-primary/10",
                    h3 { class: "font-medium text-primary mb-2", "Next Steps" }
                    ul { class: "list-disc list-inside text-text-secondary space-y-2",
                        li {
                            "Explore the ",
                            a {
                                class: "text-primary hover:underline",
                                href: "https://github.com/wheregmis/dioxus-motion/tree/main/example_project",
                                "example project"
                            },
                            " for more complex animation examples."
                        }
                        li {
                            "Check out the ",
                            a {
                                class: "text-primary hover:underline",
                                href: "https://docs.rs/dioxus-motion",
                                "API documentation"
                            },
                            " for detailed information on all available properties and methods."
                        }
                        li {
                            "Join the ",
                            a {
                                class: "text-primary hover:underline",
                                href: "https://discord.gg/XgGxHRRc",
                                "Discord community"
                            },
                            " to get help and share your creations."
                        }
                    }
                }
            }

            // Navigation
            GuideNavigation {}
        }
    }
}
