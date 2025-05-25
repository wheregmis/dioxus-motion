use dioxus::prelude::*;
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
    let mut infinite_value = use_motion(0.0f32);
    let mut times_value = use_motion(0.0f32);
    let mut alternate_value = use_motion(0.0f32);
    let mut alternate_times_value = use_motion(0.0f32);

    let start_infinite = move |_| {
        infinite_value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite),
        );
    };

    let start_times = move |_| {
        times_value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Times(3)),
        );
    };

    let start_alternate = move |_| {
        alternate_value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Alternate),
        );
    };

    let start_alternate_times = move |_| {
        alternate_times_value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::AlternateTimes(3)),
        );
    };

    // Simplify by using a single reset function
    let reset_all = move |_| {
        // Create a default tween config
        let config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));

        // Reset all values one by one
        infinite_value.animate_to(0.0, config.clone());
        times_value.animate_to(0.0, config.clone());
        alternate_value.animate_to(0.0, config.clone());
        alternate_times_value.animate_to(0.0, config);
    };

    rsx! {
        section { class: "space-y-4",
            div {
                h2 { class: "text-2xl font-semibold mb-2", "Step 1: Loop Modes" }
                p { class: "text-text-secondary",
                    "Animations can be configured to repeat in different ways using loop modes."
                }
            }

            div { class: "space-y-2",
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"// Infinite loop (0 -> 100 -> 0 -> 100...)
value.animate_to(
    100.0,
    config.with_loop(LoopMode::Infinite)
);

// Loop 3 times (0 -> 100) × 3
value.animate_to(
    100.0,
    config.with_loop(LoopMode::Times(3))
);

// Alternate infinitely (0 -> 100 -> 0 -> 100...)
value.animate_to(
    100.0,
    config.with_loop(LoopMode::Alternate)
);

// Alternate 3 times (0 -> 100 -> 0) × 3
value.animate_to(
    100.0,
    config.with_loop(LoopMode::AlternateTimes(3))
);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                div { class: "space-y-3 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium", "Live Preview" }

                    // Infinite loop preview
                    div { class: "space-y-2",
                        p { class: "text-sm text-text-secondary", "Infinite Loop:" }
                        div { class: "relative h-8 bg-dark-200/30 rounded-lg overflow-hidden",
                            div {
                                class: "absolute h-8 bg-primary/50 rounded-lg",
                                style: "width: {infinite_value.get_value()}%"
                            }
                        }
                    }

                    // Loop 3 times preview
                    div { class: "space-y-2",
                        p { class: "text-sm text-text-secondary", "Loop 3 Times:" }
                        div { class: "relative h-8 bg-dark-200/30 rounded-lg overflow-hidden",
                            div {
                                class: "absolute h-8 bg-primary/50 rounded-lg",
                                style: "width: {times_value.get_value()}%"
                            }
                        }
                    }

                    // Alternate preview
                    div { class: "space-y-2",
                        p { class: "text-sm text-text-secondary", "Alternate Infinite:" }
                        div { class: "relative h-8 bg-dark-200/30 rounded-lg overflow-hidden",
                            div {
                                class: "absolute h-8 bg-primary/50 rounded-lg",
                                style: "width: {alternate_value.get_value()}%"
                            }
                        }
                    }

                    // Alternate 3 times preview
                    div { class: "space-y-2",
                        p { class: "text-sm text-text-secondary", "Alternate 3 Times:" }
                        div { class: "relative h-8 bg-dark-200/30 rounded-lg overflow-hidden",
                            div {
                                class: "absolute h-8 bg-primary/50 rounded-lg",
                                style: "width: {alternate_times_value.get_value()}%"
                            }
                        }
                    }

                    // Controls
                    div { class: "flex flex-wrap gap-2 mt-4",
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: start_infinite,
                            "Start Infinite"
                        }
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: start_times,
                            "Loop 3 Times"
                        }
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: start_alternate,
                            "Start Alternate"
                        }
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: start_alternate_times,
                            "Alternate 3 Times"
                        }
                        button {
                            class: "px-4 py-2 bg-dark-200 hover:bg-dark-300 rounded-lg text-text-secondary transition-colors",
                            onclick: reset_all,
                            "Reset All"
                        }
                    }
                }
            }

            // Key points
            div { class: "space-y-2 mt-4",
                h3 { class: "font-medium", "Key Points:" }
                ul { class: "list-disc list-inside text-text-secondary space-y-1",
                    li { "Use ", code { "LoopMode::Infinite" }, " for endless repetition" }
                    li { "Use ", code { "LoopMode::Times(n)" }, " for a specific number of repetitions" }
                    li { "Use ", code { "LoopMode::Alternate" }, " for back-and-forth animation" }
                    li { "Use ", code { "LoopMode::AlternateTimes(n)" }, " for specific number of alternations" }
                }
            }
        }
    }
}

#[component]
fn StepTwo() -> Element {
    let mut value = use_motion(0.0f32);

    let start = move |_| {
        value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            }))
            .with_delay(Duration::from_millis(1000)),
        );
    };

    let reset = move |_| {
        value.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
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
                            style: "width: {value.get_value()}%"
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
    let mut sequence_value = use_motion(0.0f32);
    let mut keyframe_value = use_motion(0.0f32);

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
        // Create a default spring config
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));

        // Reset both values
        sequence_value.animate_to(0.0, config.clone());
        keyframe_value.animate_to(0.0, config);
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
let sequence = AnimationSequence::new()
    .then(100.0, spring_config.clone())
    .then(50.0, spring_config.clone())
    .then(0.0, spring_config);
value.animate_sequence(sequence);

// Keyframe animation
let keyframes = KeyframeAnimation::new(Duration::from_secs(2))
    .add_keyframe(0.0, 0.0, Some(easer::functions::Cubic::ease_in))
    .and_then(|kf| kf.add_keyframe(100.0, 0.3, Some(easer::functions::Elastic::ease_out)))
    .and_then(|kf| kf.add_keyframe(50.0, 0.7, Some(easer::functions::Bounce::ease_out)))
    .and_then(|kf| kf.add_keyframe(0.0, 1.0, Some(easer::functions::Back::ease_in_out)))
    .unwrap();
value.animate_keyframes(keyframes);"#.to_string(),
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
                                style: "width: {sequence_value.get_value()}%"
                            }
                        }
                        // Add value display
                        div { class: "text-sm text-text-secondary mt-1",
                            "Current value: {sequence_value.get_value():.1}"
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
                                style: "width: {keyframe_value.get_value()}%"
                            }
                        }
                        // Add value display
                        div { class: "text-sm text-text-secondary mt-1",
                            "Current value: {keyframe_value.get_value():.1}"
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
    let mut sequence_transform = use_motion(Transform::identity());
    let mut sequence_color = use_motion(Color::from_rgba(59, 130, 246, 255));
    let mut keyframe_transform = use_motion(Transform::identity());
    let mut keyframe_color = use_motion(Color::from_rgba(59, 130, 246, 255));

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
        let transform_keyframes = KeyframeAnimation::new(Duration::from_secs(2))
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
            .unwrap();

        let color_keyframes = KeyframeAnimation::new(Duration::from_secs(2))
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
            .unwrap();

        keyframe_transform.animate_keyframes(transform_keyframes);
        keyframe_color.animate_keyframes(color_keyframes);
    };

    // Simplify with a single reset function that handles one value at a time
    let reset = move |_| {
        // Create a spring config
        let config = AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 200.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
        }));

        // Initial color
        let initial_color = Color::from_rgba(59, 130, 246, 255); // Blue

        // Reset sequence transform
        sequence_transform.animate_to(Transform::identity(), config.clone());

        // Reset sequence color
        sequence_color.animate_to(initial_color, config.clone());

        // Create a new config for the next animations
        let config2 = AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 200.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
        }));

        // Reset keyframe transform
        keyframe_transform.animate_to(Transform::identity(), config2.clone());

        // Reset keyframe color
        keyframe_color.animate_to(initial_color, config2);
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
let mut transform = use_motion(Transform::identity());

// Animate to new position, scale, and rotation
transform.animate_to(
    Transform::new(100.0, 50.0, 1.2, 45.0), // x, y, scale, rotation(deg)
    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
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
let mut color = use_motion(Color::from_rgba(59, 130, 246, 255)); // Blue

// Animate to a new color
color.animate_to(
    Color::from_rgba(236, 72, 153, 255), // Pink
    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
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

// Color sequence
let color_sequence = AnimationSequence::new()
    .then(Color::from_rgba(236, 72, 153, 255), spring_config.clone())
    .then(Color::from_rgba(34, 197, 94, 255), spring_config.clone())
    .then(Color::from_rgba(59, 130, 246, 255), spring_config);"#.to_string(),
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
                                let (r, g, b, _) = sequence_color.get_value().to_rgba();
                                format!("background-color: rgb({r}, {g}, {b}); \
                                        transform: translate({}px, {}px) \
                                                  rotate({}deg) \
                                                  scale({})",
                                        sequence_transform.get_value().x,
                                        sequence_transform.get_value().y,
                                        sequence_transform.get_value().rotation,
                                        sequence_transform.get_value().scale)
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

// Color keyframes
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
    .unwrap();"#.to_string(),
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
                                let (r, g, b, _) = keyframe_color.get_value().to_rgba();
                                format!("background-color: rgb({r}, {g}, {b}); \
                                        transform: translate({}px, {}px) \
                                                  rotate({}deg) \
                                                  scale({})",
                                        keyframe_transform.get_value().x,
                                        keyframe_transform.get_value().y,
                                        keyframe_transform.get_value().rotation,
                                        keyframe_transform.get_value().scale)
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
}
