use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::components::code_block::CodeBlock;

#[component]
pub fn IntermediateAnimationGuide() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Intermediate Animation Guide" }
                p { class: "text-text-secondary",
                    "Learn advanced animation techniques including loops, delays, and sequences. These features allow you to create more complex and engaging animations."
                }
            }

            // Step 1: Loop Modes
            StepOne {}

            // Step 2: Animation Delays
            StepTwo {}

            // Step 3: Animation Sequences
            StepThree {}
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

    let reset = move |_| {
        let config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));
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
                            onclick: reset,
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
    let mut value = use_motion(0.0f32);

    let start = move |_| {
        let sequence = AnimationSequence::new()
            .then(
                50.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                70.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                100.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 20.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );

        value.animate_sequence(sequence);
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
                h2 { class: "text-2xl font-semibold mb-2", "Step 3: Animation Sequences" }
                p { class: "text-text-secondary",
                    "Chain multiple animations together to create complex motion sequences."
                }
            }

            div { class: "space-y-2",
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"let sequence = AnimationSequence::new()
    .then(
        50.0,
        AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }))
    )
    .then(
        70.0,
        AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 200.0,
            damping: 15.0,
            mass: 1.0,
            velocity: 0.0,
        }))
    )
    .then(
        100.0,
        AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 300.0,
            damping: 20.0,
            mass: 1.0,
            velocity: 0.0,
        }))
    );

value.animate_sequence(sequence);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                div { class: "space-y-3 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium", "Live Preview" }
                    p { class: "text-sm text-text-secondary", "Sequence: 0% → 50% → 70% → 100%" }

                    div { class: "relative h-16 bg-dark-200/30 rounded-lg overflow-hidden",
                        div {
                            class: "absolute h-16 bg-primary/50 rounded-lg",
                            style: "width: {value.get_value()}%"
                        }
                    }

                    // Value display
                    div { class: "text-sm text-text-secondary",
                        "Current value: {value.get_value():.1}%"
                    }

                    div { class: "flex gap-2",
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: start,
                            "Start Sequence"
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
