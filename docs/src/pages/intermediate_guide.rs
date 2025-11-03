use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use dioxus_motion::keyframes::KeyframeError;
use dioxus_motion::{KeyframeAnimation, prelude::*};
use easer::functions::Easing;

use crate::components::code_block::CodeBlock;
use crate::components::guide_navigation::GuideNavigation;

#[component]
pub fn IntermediateAnimationGuide() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Intermediate Animation Guide" }
                p { class: "text-text-secondary",
                    "Take your animations to the next level with loops, delays, and sequences. This guide shows you how to create more engaging and complex animations."
                }

                // Quick reference card
                div { class: "mt-6 p-4 bg-primary/5 rounded-lg border border-primary/10",
                    h3 { class: "text-lg font-medium text-primary mb-2", "What You'll Learn" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        // Feature 1
                        div { class: "p-3 bg-dark-200/50 rounded-lg",
                            p { class: "font-medium text-text-primary mb-1", "Loop Animations" }
                            p { class: "text-sm text-text-secondary",
                                "Create repeating animations with different loop modes"
                            }
                        }
                        // Feature 2
                        div { class: "p-3 bg-dark-200/50 rounded-lg",
                            p { class: "font-medium text-text-primary mb-1", "Delayed Animations" }
                            p { class: "text-sm text-text-secondary",
                                "Add timing delays for staggered effects"
                            }
                        }
                        // Feature 3
                        div { class: "p-3 bg-dark-200/50 rounded-lg",
                            p { class: "font-medium text-text-primary mb-1", "Sequences & Keyframes" }
                            p { class: "text-sm text-text-secondary",
                                "Chain multiple animations together"
                            }
                        }
                    }
                }
            }

            // Step 1: Loop Modes
            StepOne {}

            // Step 2: Animation Delays
            StepTwo {}

            // Step 3: Animation Approaches
            StepThree {}

            // Step 4: Advanced Animations
            StepFour {}

            // Navigation links
            GuideNavigation {}
        }
    }
}

#[component]
fn StepOne() -> Element {
    let mut infinite_value = use_motion_store(0.0f32);
    let mut times_value = use_motion_store(0.0f32);
    let mut alternate_value = use_motion_store(0.0f32);
    let mut alternate_times_value = use_motion_store(0.0f32);

    let start_infinite = move |_| {
        infinite_value.animate_to(
            100.0,
            AnimationConfig::custom_tween(
                Duration::from_millis(1000),
                easer::functions::Cubic::ease_in_out,
            )
            .with_loop(LoopMode::Infinite),
        );
    };

    let start_times = move |_| {
        times_value.animate_to(
            100.0,
            AnimationConfig::custom_tween(
                Duration::from_millis(1000),
                easer::functions::Cubic::ease_in_out,
            )
            .with_loop(LoopMode::Times(3)),
        );
    };

    let start_alternate = move |_| {
        alternate_value.animate_to(
            100.0,
            AnimationConfig::custom_tween(
                Duration::from_millis(1000),
                easer::functions::Cubic::ease_in_out,
            )
            .with_loop(LoopMode::Alternate),
        );
    };

    let start_alternate_times = move |_| {
        alternate_times_value.animate_to(
            100.0,
            AnimationConfig::custom_tween(
                Duration::from_millis(1000),
                easer::functions::Cubic::ease_in_out,
            )
            .with_loop(LoopMode::AlternateTimes(3)),
        );
    };

    let stop_all = move |_| {
        infinite_value.store().running().set(false);
        times_value.store().running().set(false);
        alternate_value.store().running().set(false);
        alternate_times_value.store().running().set(false);
    };

    let reset_all = move |_| {
        infinite_value.store().current().set(0.0);
        times_value.store().current().set(0.0);
        alternate_value.store().current().set(0.0);
        alternate_times_value.store().current().set(0.0);
    };

    rsx! {
        section { class: "space-y-6",
            // Title and description
            div {
                h2 { class: "text-xl font-semibold mb-2", "Step 1: Loop Modes" }
                p { class: "text-text-secondary",
                    "Learn how to create repeating animations with different loop behaviors."
                }
            }

            // Loop modes explanation
            div { class: "p-4 bg-dark-200/50 rounded-lg mb-6",
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    // Left column: Loop types
                    div { class: "space-y-3",
                        div { class: "p-3 bg-dark-200/80 rounded-lg",
                            h3 { class: "font-medium text-primary mb-2", "Infinite" }
                            p { class: "text-sm text-text-secondary", "Runs forever until stopped" }
                        }
                        div { class: "p-3 bg-dark-200/80 rounded-lg",
                            h3 { class: "font-medium text-primary mb-2", "Times(n)" }
                            p { class: "text-sm text-text-secondary", "Runs exactly n times then stops" }
                        }
                        div { class: "p-3 bg-dark-200/80 rounded-lg",
                            h3 { class: "font-medium text-primary mb-2", "Alternate" }
                            p { class: "text-sm text-text-secondary", "Bounces back and forth forever" }
                        }
                        div { class: "p-3 bg-dark-200/80 rounded-lg",
                            h3 { class: "font-medium text-primary mb-2", "AlternateTimes(n)" }
                            p { class: "text-sm text-text-secondary", "Bounces back and forth n times" }
                        }
                    }
                    // Right column: Code example
                    div { class: "p-3 bg-dark-200/80 rounded-lg",
                        h3 { class: "font-medium text-primary mb-2", "Code Example" }
                        div { class: "bg-dark-200/50 p-2 rounded-lg text-xs",
                            code { class: "text-primary/90",
                                "AnimationConfig::new(AnimationMode::Tween(tween))\n  .with_loop(LoopMode::Infinite)"
                            }
                        }
                    }
                }
            }

            // Interactive examples
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                // Infinite loop
                div { class: "p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium mb-3", "Infinite Loop" }
                    div { class: "relative h-12 bg-dark-200/30 rounded-lg overflow-hidden mb-3",
                        div {
                            class: "absolute h-12 bg-primary/50 rounded-lg",
                            style: "width: {infinite_value.store().current()}%"
                        }
                    }
                    button {
                        class: "w-full px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: start_infinite,
                        "Start Infinite"
                    }
                }

                // Times loop
                div { class: "p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium mb-3", "Times Loop (3x)" }
                    div { class: "relative h-12 bg-dark-200/30 rounded-lg overflow-hidden mb-3",
                        div {
                            class: "absolute h-12 bg-primary/50 rounded-lg",
                            style: "width: {times_value.store().current()}%"
                        }
                    }
                    button {
                        class: "w-full px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: start_times,
                        "Start Times"
                    }
                }

                // Alternate loop
                div { class: "p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium mb-3", "Alternate Loop" }
                    div { class: "relative h-12 bg-dark-200/30 rounded-lg overflow-hidden mb-3",
                        div {
                            class: "absolute h-12 bg-primary/50 rounded-lg",
                            style: "width: {alternate_value.store().current()}%"
                        }
                    }
                    button {
                        class: "w-full px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: start_alternate,
                        "Start Alternate"
                    }
                }

                // Alternate times loop
                div { class: "p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium mb-3", "Alternate Times (3x)" }
                    div { class: "relative h-12 bg-dark-200/30 rounded-lg overflow-hidden mb-3",
                        div {
                            class: "absolute h-12 bg-primary/50 rounded-lg",
                            style: "width: {alternate_times_value.store().current()}%"
                        }
                    }
                    button {
                        class: "w-full px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: start_alternate_times,
                        "Start Alternate Times"
                    }
                }
            }

            // Control buttons
            div { class: "flex justify-center gap-4 mt-6",
                button {
                    class: "px-6 py-2 bg-red-500/20 hover:bg-red-500/30 rounded-lg text-red-500 transition-colors",
                    onclick: stop_all,
                    "Stop All"
                }
                button {
                    class: "px-6 py-2 bg-dark-200 hover:bg-dark-300 rounded-lg text-text-secondary transition-colors",
                    onclick: reset_all,
                    "Reset All"
                }
            }
        }
    }
}

#[component]
fn StepTwo() -> Element {
    let mut value = use_motion_store(0.0f32);

    let start = move |_| {
        value.animate_to(
            100.0,
            AnimationConfig::custom_spring(100.0, 10.0, 1.0)
                .with_delay(Duration::from_millis(1000)),
        );
    };

    let reset = move |_| {
        value.animate_to(0.0, AnimationConfig::spring());
    };

    rsx! {
        section { class: "space-y-4",
            div {
                h2 { class: "text-2xl font-semibold mb-2", "Step 2: Animation Delays" }
                p { class: "text-text-secondary",
                    "Add delays to your animations to create staggered effects or wait for specific events."
                }
            }

            div { class: "space-y-2",
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"value.animate_to(
    100.0,
    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
        .with_delay(Duration::from_millis(1000))  // 1 second delay
);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                div { class: "space-y-3 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium", "Live Preview" }
                    p { class: "text-sm text-text-secondary", "Animation starts after 1 second delay:" }

                    div { class: "relative h-16 bg-dark-200/30 rounded-lg overflow-hidden",
                        div {
                            class: "absolute h-16 bg-primary/50 rounded-lg",
                            style: "width: {value.store().current()}%"
                        }
                    }

                    div { class: "flex gap-2",
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: start,
                            "Start Delayed Animation"
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
    }
}

#[component]
fn StepThree() -> Element {
    let mut sequence_value = use_motion_store(0.0f32);
    let mut keyframe_value = use_motion_store(0.0f32);

    let start_sequence = move |_| {
        let sequence = AnimationSequence::new()
            .then(
                100.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                50.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                0.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 20.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );

        sequence_value.animate_sequence(sequence);
    };

    let start_keyframes = move |_| {
        let keyframes = KeyframeAnimation::new(Duration::from_secs(2))
            .add_keyframe(0.0, 0.0, Some(easer::functions::Cubic::ease_in))
            .and_then(|kf| kf.add_keyframe(100.0, 0.3, Some(easer::functions::Elastic::ease_out)))
            .and_then(|kf| kf.add_keyframe(50.0, 0.7, Some(easer::functions::Bounce::ease_out)))
            .and_then(|kf| kf.add_keyframe(0.0, 1.0, Some(easer::functions::Back::ease_in_out)))
            .unwrap();

        keyframe_value.animate_keyframes(keyframes);
    };

    // Simplify with a single reset function
    let reset = move |_| {
        // Reset both values
        sequence_value.reset();
        keyframe_value.reset();
    };

    rsx! {
        section { class: "space-y-4",
            div {
                h2 { class: "text-2xl font-semibold mb-2", "Step 3: Basic Sequences and Keyframes" }
                p { class: "text-text-secondary",
                    "Learn how to create simple sequences and keyframe animations with numeric values."
                }
            }

            div { class: "space-y-2",
                // Code example
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"// Sequence animation
let mut motion = use_motion_store(0.0f32);
let sequence = AnimationSequence::new()
    .then(100.0, spring_config.clone())
    .then(50.0, spring_config.clone())
    .then(0.0, spring_config);
motion.animate_sequence(sequence);

// Keyframe animation
let mut motion = use_motion_store(0.0f32);
let keyframes = KeyframeAnimation::new(Duration::from_secs(2))
    .add_keyframe(0.0, 0.0, Some(easer::functions::Cubic::ease_in))
    .and_then(|kf| kf.add_keyframe(100.0, 0.3, Some(easer::functions::Elastic::ease_out)))
    .and_then(|kf| kf.add_keyframe(50.0, 0.7, Some(easer::functions::Bounce::ease_out)))
    .and_then(|kf| kf.add_keyframe(0.0, 1.0, Some(easer::functions::Back::ease_in_out)))
    .unwrap();
motion.animate_keyframes(keyframes);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                // Live preview
                div { class: "space-y-3 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium", "Live Preview" }

                    // Sequence preview
                    div { class: "space-y-2",
                        p { class: "text-sm text-text-secondary", "Sequence Animation:" }
                        div { class: "relative h-8 bg-dark-200/30 rounded-lg overflow-hidden",
                            div {
                                class: "absolute h-8 bg-primary/50 rounded-lg",
                                style: "width: {sequence_value.store().current()}%"
                            }
                        }
                        // Add value display
                        div { class: "text-sm text-text-secondary mt-1",
                            "Current value: {sequence_value.store().current():.1}"
                        }
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: start_sequence,
                            "Start Sequence"
                        }
                    }

                    // Keyframe preview
                    div { class: "space-y-2 mt-4",
                        p { class: "text-sm text-text-secondary", "Keyframe Animation:" }
                        div { class: "relative h-8 bg-dark-200/30 rounded-lg overflow-hidden",
                            div {
                                class: "absolute h-8 bg-primary/50 rounded-lg",
                                style: "width: {keyframe_value.store().current()}%"
                            }
                        }
                        // Add value display
                        div { class: "text-sm text-text-secondary mt-1",
                            "Current value: {keyframe_value.store().current():.1}"
                        }
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: start_keyframes,
                            "Start Keyframes"
                        }
                    }

                    // Reset button at the bottom
                    div { class: "mt-4",
                        button {
                            class: "px-4 py-2 bg-dark-200 hover:bg-dark-300 rounded-lg text-text-secondary transition-colors",
                            onclick: reset,
                            "Reset All"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StepFour() -> Element {
    let mut sequence_transform = use_motion_store(Transform::identity());
    let mut sequence_color = use_motion_store(Color::from_rgba(59, 130, 246, 255));
    let mut keyframe_transform = use_motion_store(Transform::identity());
    let mut keyframe_color = use_motion_store(Color::from_rgba(59, 130, 246, 255));

    let start_sequence = move |_| {
        let transform_sequence = AnimationSequence::new()
            .then(
                Transform::new(100.0, 0.0, 1.2, 45.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                Transform::new(100.0, 100.0, 0.8, 180.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 20.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );

        let color_sequence = AnimationSequence::new()
            .then(
                Color::from_rgba(236, 72, 153, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .then(
                Color::from_rgba(34, 197, 94, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .then(
                Color::from_rgba(59, 130, 246, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );

        sequence_transform.animate_sequence(transform_sequence);
        sequence_color.animate_sequence(color_sequence);
    };

    let start_keyframes = move |_| {
        match create_transform_keyframes() {
            Ok(transform_keyframes) => keyframe_transform.animate_keyframes(transform_keyframes),
            Err(e) => error!("Failed to create transform keyframes: {e}"),
        }
        match create_color_keyframes() {
            Ok(color_keyframes) => keyframe_color.animate_keyframes(color_keyframes),
            Err(e) => error!("Failed to create color keyframes: {e}"),
        }
    };

    // Simplify with a single reset function that handles one value at a time
    let reset = move |_| {
        // Initial color
        let initial_color = Color::from_rgba(59, 130, 246, 255); // Blue

        // Reset all values directly
        sequence_transform
            .store()
            .current()
            .set(Transform::identity());
        sequence_color.store().current().set(initial_color);
        keyframe_transform
            .store()
            .current()
            .set(Transform::identity());
        keyframe_color.store().current().set(initial_color);
    };

    rsx! {
    section { class: "space-y-4",
        div {
            h2 { class: "text-2xl font-semibold mb-2", "Step 4: Transform and Color Animations" }
            p { class: "text-text-secondary",
                "Learn how to animate more complex types like Transform for position/scale/rotation and Color for smooth color transitions."
            }
        }

        // Introduction to Transform and Color
        div { class: "space-y-4 mb-6",
            // Transform introduction
            div { class: "p-4 bg-dark-200/30 rounded-lg",
                h3 { class: "font-medium mb-2", "Transform Animation" }
                p { class: "text-text-secondary mb-3",
                    "Transform combines position (x, y), scale, and rotation into a single animatable value."
                }
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"// Create a transform motion value
let mut transform = use_motion_store(Transform::identity());

// Animate to new position, scale, and rotation
transform.animate_to(
    Transform::new(100.0, 50.0, 1.2, 45.0), // x, y, scale, rotation(deg)
    AnimationConfig::spring()
);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }
            }

            // Color introduction
            div { class: "p-4 bg-dark-200/30 rounded-lg",
                h3 { class: "font-medium mb-2", "Color Animation" }
                p { class: "text-text-secondary mb-3",
                    "Smoothly interpolate between colors in RGB space."
                }
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"// Create a color motion value (RGBA format)
let mut color = use_motion_store(Color::from_rgba(59, 130, 246, 255)); // Blue

// Animate to a new color
color.animate_to(
    Color::from_rgba(236, 72, 153, 255), // Pink
    AnimationConfig::spring()
);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }
            }

            // Key points about Transform and Color
            div { class: "space-y-2 mt-4",
                h3 { class: "font-medium", "Key Points:" }
                ul { class: "list-disc list-inside text-text-secondary space-y-1",
                    li { "Transform combines multiple properties into a single animation" }
                    li { "Rotation is in radians internally, but typically specified in degrees for convenience" }
                    li { "Colors are interpolated smoothly in RGB space" }
                    li { "Both types work with all animation modes (Spring, Tween) and sequences" }
                }
            }
        }

        // Sequence Animations
        div { class: "space-y-4 p-4 bg-dark-200/30 rounded-lg",
            h3 { class: "font-medium", "Sequence Animations" }

            // Code example
            div { class: "bg-dark-200/50 p-3 rounded-lg mb-4",
                CodeBlock {
                    code: r#"// Transform sequence
let mut transform = use_motion_store(Transform::identity());
let transform_sequence = AnimationSequence::new()
    .then(
        Transform::new(100.0, 0.0, 1.2, 45.0),
        spring_config.clone(),
    )
    .then(
        Transform::new(100.0, 100.0, 0.8, 180.0),
        spring_config.clone(),
    )
    .then(
        Transform::identity(),
        spring_config,
    );
transform.animate_sequence(transform_sequence);

// Color sequence
let mut color = use_motion_store(Color::from_rgba(59, 130, 246, 255));
let color_sequence = AnimationSequence::new()
    .then(Color::from_rgba(236, 72, 153, 255), spring_config.clone())
    .then(Color::from_rgba(34, 197, 94, 255), spring_config.clone())
    .then(Color::from_rgba(59, 130, 246, 255), spring_config);
color.animate_sequence(color_sequence);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }
            }

            // Preview
            div { class: "space-y-4",
                // Transform preview
                div { class: "h-32 flex items-center justify-center bg-dark-200/30 rounded-lg",
                    div {
                        class: "w-16 h-16 rounded-lg",
                        style: {
                            let (r, g, b, _) = sequence_color.store().current()().to_rgba();
                            format!("background-color: rgb({r}, {g}, {b}); \
                                    transform: translate({}px, {}px) \
                                              rotate({}deg) \
                                              scale({})",
                                    sequence_transform.store().current()().x,
                                    sequence_transform.store().current()().y,
                                    sequence_transform.store().current()().rotation,
                                    sequence_transform.store().current()().scale)
                        }
                    }
                }

                button {
                    class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                    onclick: start_sequence,
                    "Start Sequence"
                }
            }
        }

        // Keyframe Animations
        div { class: "space-y-4 p-4 bg-dark-200/30 rounded-lg",
            h3 { class: "font-medium", "Keyframe Animations" }

            // Code example
            div { class: "bg-dark-200/50 p-3 rounded-lg mb-4",
                CodeBlock {
                    code: r#"// Transform keyframes
let (transform, animate_transform) = use_motion_store_with_keyframes(Transform::identity());
let transform_keyframes = KeyframeAnimation::new(Duration::from_secs(2))
    .add_keyframe(
        Transform::identity(),
        0.0,
        Some(easer::functions::Cubic::ease_in),
    )
    .and_then(|kf| kf.add_keyframe(
        Transform::new(100.0, 50.0, 1.2, 180.0),
        0.3,
        Some(easer::functions::Elastic::ease_out),
    ))
    .and_then(|kf| kf.add_keyframe(
        Transform::new(50.0, 100.0, 0.8, 90.0),
        0.7,
        Some(easer::functions::Bounce::ease_out),
    ))
    .and_then(|kf| kf.add_keyframe(
        Transform::identity(),
        1.0,
        Some(easer::functions::Back::ease_in_out),
    ))
    .unwrap();
animate_transform(transform_keyframes);

// Color keyframes
let (color, animate_color) = use_motion_store_with_keyframes(Color::from_rgba(59, 130, 246, 255));
let color_keyframes = KeyframeAnimation::new(Duration::from_secs(2))
    .add_keyframe(
        Color::from_rgba(59, 130, 246, 255),
        0.0,
        Some(easer::functions::Cubic::ease_in),
    )
    .and_then(|kf| kf.add_keyframe(
        Color::from_rgba(236, 72, 153, 255),
        0.5,
        Some(easer::functions::Cubic::ease_out),
    ))
    .and_then(|kf| kf.add_keyframe(
        Color::from_rgba(59, 130, 246, 255),
        1.0,
        Some(easer::functions::Cubic::ease_in_out),
    ))
    .unwrap();
animate_color(color_keyframes);"#.to_string(),
                    language: "rust".to_string(),
                }
            }

            // Preview
            div { class: "space-y-4",
                // Transform preview
                div { class: "h-32 flex items-center justify-center bg-dark-200/30 rounded-lg",
                    div {
                        class: "w-16 h-16 rounded-lg",
                        style: {
                            let (r, g, b, _) = keyframe_color.store().current()().to_rgba();
                            format!("background-color: rgb({r}, {g}, {b}); \
                                    transform: translate({}px, {}px) \
                                              rotate({}deg) \
                                              scale({})",
                                    keyframe_transform.store().current()().x,
                                    keyframe_transform.store().current()().y,
                                    keyframe_transform.store().current()().rotation,
                                    keyframe_transform.store().current()().scale)
                        }
                    }
                }

                button {
                    class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                    onclick: start_keyframes,
                    "Start Keyframes"
                }
            }
        }

        // Reset button
        div { class: "mt-6",
            button {
                class: "px-4 py-2 bg-dark-200 hover:bg-dark-300 rounded-lg text-text-secondary transition-colors",
                onclick: reset,
                "Reset All"
            }
        }
    }
}

fn create_transform_keyframes() -> Result<KeyframeAnimation<Transform>, KeyframeError> {
    KeyframeAnimation::new(Duration::from_secs(2))
        .add_keyframe(
            Transform::identity(),
            0.0,
            Some(easer::functions::Cubic::ease_in),
        )
        .and_then(|kf| {
            kf.add_keyframe(
                Transform::new(100.0, 50.0, 1.2, 180.0),
                0.3,
                Some(easer::functions::Elastic::ease_out),
            )
        })
        .and_then(|kf| {
            kf.add_keyframe(
                Transform::new(50.0, 100.0, 0.8, 90.0),
                0.7,
                Some(easer::functions::Bounce::ease_out),
            )
        })
        .and_then(|kf| {
            kf.add_keyframe(
                Transform::identity(),
                1.0,
                Some(easer::functions::Back::ease_in_out),
            )
        })
}

fn create_color_keyframes() -> Result<KeyframeAnimation<Color>, KeyframeError> {
    KeyframeAnimation::new(Duration::from_secs(2))
        .add_keyframe(
            Color::from_rgba(59, 130, 246, 255),
            0.0,
            Some(easer::functions::Cubic::ease_in),
        )
        .and_then(|kf| {
            kf.add_keyframe(
                Color::from_rgba(236, 72, 153, 255),
                0.5,
                Some(easer::functions::Cubic::ease_out),
            )
        })
        .and_then(|kf| {
            kf.add_keyframe(
                Color::from_rgba(59, 130, 246, 255),
                1.0,
                Some(easer::functions::Cubic::ease_in_out),
            )
        })
}
