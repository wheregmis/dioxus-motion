use dioxus::prelude::*;
use dioxus_motion::prelude::*;

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
