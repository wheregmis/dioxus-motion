use dioxus::logger::tracing::error;
use dioxus::prelude::MouseData;
use dioxus::prelude::*;
use dioxus_motion::animations::core::Animatable;
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
    let mut infinite_value = use_motion(0.0f32);
    let mut times_value = use_motion(0.0f32);
    let mut alternate_value = use_motion(0.0f32);
    let mut alternate_times_value = use_motion(0.0f32);

    let mut infinite_value_start = infinite_value.clone();
    let mut times_value_start = times_value.clone();
    let mut alternate_value_start = alternate_value.clone();
    let mut alternate_times_value_start = alternate_times_value.clone();
    let mut infinite_value_reset = infinite_value.clone();
    let mut times_value_reset = times_value.clone();
    let mut alternate_value_reset = alternate_value.clone();
    let mut alternate_times_value_reset = alternate_times_value.clone();

    let start_infinite = move |_| {
        infinite_value_start.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite),
        );
    };

    let start_times = move |_| {
        times_value_start.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Times(3)),
        );
    };

    let start_alternate = move |_| {
        alternate_value_start.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Alternate),
        );
    };

    let start_alternate_times = move |_| {
        alternate_times_value_start.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::AlternateTimes(3)),
        );
    };

    let reset_all = move |_| {
        infinite_value_reset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );
        times_value_reset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );
        alternate_value_reset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );
        alternate_times_value_reset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );
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
    let value = use_motion(0.0f32);
    let mut value_start = value.clone();
    let mut value_reset = value.clone();
    let value_val = value.clone();
    let start = move |_| {
        value_start.animate_to(
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
        value_reset.animate_to(
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
                            style: "width: {value_val.get_value()}%"
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
    let sequence_value = use_motion(0.0f32);
    let mut sequence_value_seq = sequence_value.clone();
    let mut sequence_value_reset = sequence_value.clone();
    let sequence_value_val = sequence_value.clone();
    let keyframe_value = use_motion(0.0f32);
    let mut keyframe_value_seq = keyframe_value.clone();
    let mut keyframe_value_reset = keyframe_value.clone();
    let keyframe_value_val = keyframe_value.clone();
    let start_sequence = move |_| {
        let sequence_vec = vec![0.0, 50.0, 100.0, 0.0];
        sequence_value_seq.interpolate_sequence(sequence_vec, 0.5);
    };
    let start_keyframes = move |_| {
        let keyframes_vec = vec![0.0, 25.0, 75.0, 100.0, 0.0];
        keyframe_value_seq.interpolate_sequence(keyframes_vec, 0.5);
    };
    let reset = move |_| {
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
        sequence_value_reset.animate_to(0.0, config.clone());
        keyframe_value_reset.animate_to(0.0, config);
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
                                style: "width: {sequence_value_val.get_value()}%"
                            }
                        }
                        // Add value display
                        div { class: "text-sm text-text-secondary mt-1",
                            "Current value: {sequence_value_val.get_value():.1}"
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
                                style: "width: {keyframe_value_val.get_value()}%"
                            }
                        }
                        // Add value display
                        div { class: "text-sm text-text-secondary mt-1",
                            "Current value: {keyframe_value_val.get_value():.1}"
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

// Helper for per-step config sequence animation
fn animate_sequence_with_config<T: Animatable + Clone + Send + Sync + 'static>(
    mut handle: EnhancedMotionHandle<T>,
    steps: Vec<(T, AnimationConfig)>,
) {
    let mut total_delay = std::time::Duration::ZERO;
    for (target, mut config) in steps {
        config = config.with_delay(total_delay);
        handle.animate_to(target, config.clone());
        // Use the config's duration if available, otherwise default to 500ms
        let step_duration = match &config.mode {
            AnimationMode::Tween(tween) => tween.duration,
            AnimationMode::Spring(_) => std::time::Duration::from_millis(500),
        };
        total_delay += step_duration;
    }
}

#[component]
fn StepFour() -> Element {
    let sequence_transform = use_motion(Transform::identity());
    let sequence_color = use_motion(Color::from_rgba(59, 130, 246, 255));
    let keyframe_transform = use_motion(Transform::identity());
    let keyframe_color = use_motion(Color::from_rgba(59, 130, 246, 255));
    let mut sequence_transform_seq = sequence_transform.clone();
    let mut sequence_color_seq = sequence_color.clone();
    let mut keyframe_transform_seq = keyframe_transform.clone();
    let mut keyframe_color_seq = keyframe_color.clone();
    let mut sequence_transform_reset = sequence_transform.clone();
    let mut sequence_color_reset = sequence_color.clone();
    let mut keyframe_transform_reset = keyframe_transform.clone();
    let mut keyframe_color_reset = keyframe_color.clone();
    let sequence_transform_val = sequence_transform.clone();
    let sequence_color_val = sequence_color.clone();
    let keyframe_transform_val = keyframe_transform.clone();
    let keyframe_color_val = keyframe_color.clone();
    let start_sequence = move |_: Event<MouseData>| {
        println!("[DEBUG] start_sequence called");
        let steps = vec![
            (
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            ),
            (
                Transform::new(50.0, 0.0, 1.2, 0.0),
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(400),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            ),
            (
                Transform::new(-50.0, 0.0, 0.8, 0.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 80.0,
                    damping: 8.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            ),
            (
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(300),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            ),
        ];
        animate_sequence_with_config(sequence_transform_seq.clone(), steps);
    };
    let start_color_sequence = move |_: Event<MouseData>| {
        println!("[DEBUG] start_color_sequence called");
        let steps = vec![
            (
                Color::from_rgba(59, 130, 246, 255),
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(300),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            ),
            (
                Color::from_rgba(16, 185, 129, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 90.0,
                    damping: 12.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            ),
            (
                Color::from_rgba(244, 63, 94, 255),
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(400),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            ),
            (
                Color::from_rgba(59, 130, 246, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            ),
        ];
        animate_sequence_with_config(sequence_color_seq.clone(), steps);
    };
    let start_keyframes = move |_: Event<MouseData>| {
        println!("[DEBUG] start_keyframes called");
        let steps = vec![
            (
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(200),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            ),
            (
                Transform::new(0.0, 50.0, 1.0, 0.5),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 120.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            ),
            (
                Transform::new(0.0, -50.0, 1.0, -0.5),
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(300),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            ),
            (
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            ),
        ];
        animate_sequence_with_config(keyframe_transform_seq.clone(), steps);
    };
    let start_color_keyframes = move |_: Event<MouseData>| {
        println!("[DEBUG] start_color_keyframes called");
        let steps = vec![
            (
                Color::from_rgba(59, 130, 246, 255),
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(200),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            ),
            (
                Color::from_rgba(251, 191, 36, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 80.0,
                    damping: 8.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            ),
            (
                Color::from_rgba(16, 185, 129, 255),
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(300),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            ),
            (
                Color::from_rgba(59, 130, 246, 255),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            ),
        ];
        animate_sequence_with_config(keyframe_color_seq.clone(), steps);
    };
    let reset = move |_: Event<MouseData>| {
        println!("[DEBUG] reset called");
        let config = AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 200.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
        }));
        let initial_color = Color::from_rgba(59, 130, 246, 255); // Blue
        sequence_transform_reset.animate_to(Transform::identity(), config.clone());
        sequence_color_reset.animate_to(initial_color, config.clone());
        let config2 = AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 200.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
        }));
        keyframe_transform_reset.animate_to(Transform::identity(), config2.clone());
        keyframe_color_reset.animate_to(initial_color, config2);
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
                                let (r, g, b, _) = sequence_color_val.get_value().to_rgba();
                                format!("background-color: rgb({r}, {g}, {b}); \
                                        transform: translate({}px, {}px) \
                                                  rotate({}deg) \
                                                  scale({})",
                                        sequence_transform_val.get_value().x,
                                        sequence_transform_val.get_value().y,
                                        sequence_transform_val.get_value().rotation,
                                        sequence_transform_val.get_value().scale)
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
                                let (r, g, b, _) = keyframe_color_val.get_value().to_rgba();
                                format!("background-color: rgb({r}, {g}, {b}); \
                                        transform: translate({}px, {}px) \
                                                  rotate({}deg) \
                                                  scale({})",
                                        keyframe_transform_val.get_value().x,
                                        keyframe_transform_val.get_value().y,
                                        keyframe_transform_val.get_value().rotation,
                                        keyframe_transform_val.get_value().scale)
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
