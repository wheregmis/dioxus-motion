use dioxus::prelude::*;
use dioxus_motion::{
    enhanced_motion::{
        use_enhanced_motion, AnimationPath, AnimationSequence, EnhancedAnimationManager,
        TransitionConfig,
    },
    prelude::*,
};
use easer::functions::Easing;
use std::time::Duration;

// An animated counter that shows basic motion and sequences
#[component]
fn AnimatedCounter() -> Element {
    let mut value = use_enhanced_motion(0.0f32);
    let mut count = use_signal(|| 0);

    let onclick = move |_| {
        let sequence = AnimationSequence::new()
            .then(
                ((*count)() + 1) as f32 * 100.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 180.0,
                    damping: 12.0,
                    mass: 1.0,
                    velocity: 10.0,
                })),
            )
            .then(
                ((*count)() + 1) as f32 * 50.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(500),
                    easing: easer::functions::Bounce::ease_out,
                })),
            )
            .on_complete(move || println!("Animation completed"));

        value.animate_sequence(sequence);
        count.set((*count)() + 1);
    };

    rsx! {
        div { class: "flex flex-col items-center gap-4",
            div {
                class: "text-4xl font-bold",
                style: "transform: translateY({value.get_value()}px)",
                "Count: {count}"
            }
            button {
                class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                onclick,
                "Increment"
            }
        }
    }
}

// An interactive menu item with smooth transitions
#[component]
fn AnimatedMenuItem(label: String) -> Element {
    let mut x_offset = use_enhanced_motion(0.0f32);
    let mut scale = use_enhanced_motion(1.0f32);
    let mut opacity = use_enhanced_motion(0.7f32);

    let onmouseenter = move |_| {
        let transition_config = TransitionConfig {
            duration: Duration::from_millis(300),
            easing: std::sync::Arc::new(|t| easer::functions::Cubic::ease_out(t, 0.0, 1.0, 1.0)),
            blend_function: std::sync::Arc::new(|a, b| a + (b - a)),
        };

        x_offset.transition_to(20.0, transition_config.clone());
        scale.transition_to(1.1, transition_config.clone());
        opacity.transition_to(1.0, transition_config);
    };

    let onmouseleave = move |_| {
        let transition_config = TransitionConfig {
            duration: Duration::from_millis(300),
            easing: std::sync::Arc::new(|t| easer::functions::Cubic::ease_in(t, 0.0, 1.0, 1.0)),
            blend_function: std::sync::Arc::new(|a, b| a + (b - a)),
        };

        x_offset.transition_to(0.0, transition_config.clone());
        scale.transition_to(1.0, transition_config.clone());
        opacity.transition_to(0.7, transition_config);
    };

    rsx! {
        div {
            class: "p-4 cursor-pointer bg-gray-800 text-white rounded-lg",
            style: "transform: translateX({x_offset.get_value()}px) scale({scale.get_value()}); opacity: {opacity.get_value()}",
            onmouseenter,
            onmouseleave,
            "{label}"
        }
    }
}
#[component]
fn LoadingSpinner() -> Element {
    let mut position = use_enhanced_motion(Transform::identity());
    let mut is_mounted = use_signal(|| false);

    let start_animation = move |_| {
        if !*is_mounted.read() {
            println!("Starting spinner animation"); // Debug
            let path = AnimationPath {
                points: vec![
                    Transform::new(0.0, 0.0, 1.0, 0.0),
                    Transform::new(20.0, 0.0, 1.0, 0.0),
                    Transform::new(20.0, 20.0, 1.0, 0.0),
                    Transform::new(0.0, 20.0, 1.0, 0.0),
                    Transform::new(0.0, 0.0, 1.0, 0.0),
                ],
                tension: 0.8,
                closed: true,
            };

            position.follow_path(
                path,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(1),
                    easing: easer::functions::Linear::ease_in_out,
                }))
                .with_loop(LoopMode::Infinite),
            );
            is_mounted.set(true);
        }
    };

    rsx! {
        style { {SPINNER_CSS} }
        div { class: "spinner-container", onmounted: start_animation,
            div {
                class: "spinner-dot",
                style: "transform: translate({position.get_value().x}px, {position.get_value().y}px)",
            }
        }
    }
}

const SPINNER_CSS: &str = r#"
.spinner-container {
    position: relative;
    width: 100px;
    height: 100px;
    border: 1px solid #eee;
    margin: 20px;
}

.spinner-dot {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 12px;
    height: 12px;
    margin: -6px 0 0 -6px;
    background: #3b82f6;
    border-radius: 50%;
}
"#;

// A playful button that bounces on click
#[component]
fn BouncyButton() -> Element {
    let mut scale = use_enhanced_motion(1.0f32);
    let mut rotation = use_enhanced_motion(0.0f32);

    let onclick = move |_| {
        let spring_config = AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 200.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }));

        scale.animate_to(1.2, spring_config.clone());

        rotation.animate_relative(
            360.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs(1),
                easing: easer::functions::Back::ease_in_out,
            })),
        );
    };

    rsx! {
        button {
            class: "px-6 py-3 bg-purple-500 text-white rounded-lg shadow-lg",
            style: "transform: scale({scale.get_value()}) rotate({rotation.get_value()}deg)",
            onclick,
            "Click me!"
        }
    }
}

// Main component showcasing all animations
#[component]
pub fn AnimationShowcase() -> Element {
    rsx! {
        div { class: "p-8 space-y-12 bg-gray-100 min-h-screen",
            h1 { class: "text-3xl font-bold mb-8", "Animation Showcase" }
            // Counter section
            section { class: "space-y-4",
                h2 { class: "text-xl font-bold", "Animated Counter" }
                p { class: "text-gray-600",
                    "Demonstrates animation sequences with springs and tweens"
                }
                AnimatedCounter {}
            }
            // Menu items section
            section { class: "space-y-4",
                h2 { class: "text-xl font-bold", "Interactive Menu" }
                p { class: "text-gray-600", "Shows smooth transitions on hover" }
                div { class: "space-y-2",
                    AnimatedMenuItem { label: "Home" }
                    AnimatedMenuItem { label: "About" }
                    AnimatedMenuItem { label: "Contact" }
                }
            }
            // Loading spinner section
            section { class: "space-y-4",
                h2 { class: "text-xl font-bold", "Loading Animation" }
                p { class: "text-gray-600", "Uses path following with infinite loop" }
                LoadingSpinner {}
            }
            // Bouncy button section
            section { class: "space-y-4",
                h2 { class: "text-xl font-bold", "Interactive Button" }
                p { class: "text-gray-600", "Combines spring physics with rotation" }
                BouncyButton {}
            }
        }
    }
}
