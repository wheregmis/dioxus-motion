use dioxus::prelude::*;
use dioxus_motion::{prelude::*, AnimationSequence};
use easer::functions::Easing;
use std::time::Duration;

const CONTAINER_SIZE: f32 = 400.0;
const PERSPECTIVE: f32 = 800.0;

// An animated counter that shows basic motion and sequences
#[component]
fn AnimatedCounter() -> Element {
    let mut value = use_motion(0.0f32);
    let mut scale = use_motion(1.0f32);
    let mut count = use_signal(|| 0);

    let onclick = move |_| {
        let sequence = AnimationSequence::new().then(
            ((*count)() + 1) as f32 * 100.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 180.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 10.0,
            })),
        );

        scale.animate_to(
            1.2,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        value.animate_sequence(sequence);
        count.set((*count)() + 1);
    };

    rsx! {
        div { class: "flex flex-col items-center gap-6 p-8 bg-gradient-to-br from-blue-500/10 to-purple-500/10 rounded-2xl backdrop-blur-sm",
            div {
                class: "relative text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-500 to-purple-500",
                style: "transform: translateY({value.get_value()}px) scale({scale.get_value()})",
                "Count: {count}"
            }
            button {
                class: "px-6 py-3 bg-gradient-to-r from-blue-500 to-purple-500 text-white rounded-full font-semibold shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-300",
                onclick,
                "Increment"
            }
        }
    }
}

// An interactive menu item with smooth transitions
#[component]
fn AnimatedMenuItem(label: String) -> Element {
    let mut x_offset = use_motion(0.0f32);
    let mut scale = use_motion(1.0f32);
    let mut glow = use_motion(0.0f32);

    let onmouseenter = move |_| {
        x_offset.animate_to(
            20.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        scale.animate_to(
            1.1,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        glow.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let onmouseleave = move |_| {
        x_offset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        glow.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    rsx! {
        div {
            class: "relative p-4 cursor-pointer bg-gradient-to-r from-gray-800 to-gray-900 text-white rounded-xl overflow-hidden group",
            style: "transform: translateX({x_offset.get_value()}px) scale({scale.get_value()})",
            onmouseenter,
            onmouseleave,
            // Glow effect
            div {
                class: "absolute inset-0 bg-gradient-to-r from-blue-500/30 to-purple-500/30 transition-opacity duration-300",
                style: "opacity: {glow.get_value()}",
            }
            // Content
            div { class: "relative z-10 flex items-center gap-2",
                span { class: "text-lg font-medium", "{label}" }
                span { class: "text-blue-400 group-hover:translate-x-1 transition-transform duration-300",
                    "→"
                }
            }
        }
    }
}

// A playful button that bounces on click
#[component]
fn BouncyButton() -> Element {
    let mut scale = use_motion(1.0f32);
    let mut rotation = use_motion(0.0f32);
    let mut glow = use_motion(0.0f32);

    let onclick = move |_| {
        let scale_sequence = AnimationSequence::new()
            .then(
                1.2,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 400.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 5.0,
                })),
            )
            .then(
                0.8,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: -2.0,
                })),
            )
            .then(
                1.0,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );

        let rotation_sequence = AnimationSequence::new().then(
            360.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs(1),
                easing: easer::functions::Back::ease_in_out,
            })),
        );

        scale.animate_sequence(scale_sequence);
        rotation.animate_sequence(rotation_sequence);
        glow.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    rsx! {
        button {
            class: "relative px-8 py-4 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-xl font-bold text-lg overflow-hidden transition-all duration-300 hover:shadow-xl hover:shadow-purple-500/20",
            style: "transform: scale({scale.get_value()}) rotate({rotation.get_value()}deg)",
            onclick,
            // Glow effect
            div {
                class: "absolute inset-0 bg-white/30 blur-xl",
                style: "opacity: {glow.get_value()}",
            }
            "Click me!"
        }
    }
}

#[component]
pub fn InteractiveCube() -> Element {
    let mut rotation_x = use_motion(0.0f32);
    let mut rotation_y = use_motion(0.0f32);
    let mut scale = use_motion(1.0f32);
    let mut glow = use_motion(0.0f32);

    let onclick = move |e: Event<MouseData>| {
        let spin_sequence = AnimationSequence::new().then(
            rotation_y.get_value() + 360.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 8.0,
                mass: 1.0,
                velocity: 20.0,
            })),
        );

        let bounce_sequence = AnimationSequence::new()
            .then(
                1.2,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 300.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 5.0,
                })),
            )
            .then(
                1.0,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );
        scale.animate_sequence(bounce_sequence);

        rotation_y.animate_sequence(spin_sequence);

        glow.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let onmousemove = move |e: Event<MouseData>| {
        let rect = e.data().client_coordinates();
        // Now using fixed dimensions for precise calculations
        let x = (rect.x as f32 - CONTAINER_SIZE / 2.0) / CONTAINER_SIZE - 0.5;
        let y = (rect.y as f32 - CONTAINER_SIZE / 2.0) / CONTAINER_SIZE - 0.5;

        rotation_x.animate_to(
            x * 45.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 15.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    };

    rsx! {
        div {
            class: "relative",
            style: "width: {CONTAINER_SIZE}px; height: {CONTAINER_SIZE}px; perspective: {PERSPECTIVE}px",
            // Glow background
            div {
                class: "absolute inset-0 bg-gradient-to-r from-blue-500/20 to-purple-500/20 blur-2xl -z-10 transition-opacity duration-300",
                style: "opacity: {glow.get_value()}",
            }

            div {
                onclick,
                onmousemove,
                class: "relative w-full h-full items-center justify-center transform-style-3d transition-transform duration-100 group-hover:shadow-2xl",
                style: "transform: rotateX({rotation_x.get_value()}deg) rotateY({rotation_y.get_value()}deg) scale({scale.get_value()})",
                // Cube faces with enhanced gradients and transitions
                div { class: "absolute w-full h-full flex items-center justify-center text-xl font-bold text-white bg-gradient-to-br from-blue-500 to-purple-500 shadow-lg transform translate-z-[100px] opacity-90 hover:opacity-100 transition-all duration-300",
                    "Front"
                }
                div { class: "absolute w-full h-full flex items-center justify-center text-xl font-bold text-white bg-gradient-to-br from-purple-500 to-pink-500 shadow-lg transform -translate-z-[100px] rotate-y-180 opacity-90 hover:opacity-100 transition-all duration-300",
                    "Back"
                }
                div { class: "absolute w-full h-full flex items-center justify-center text-xl font-bold text-white bg-gradient-to-br from-pink-500 to-green-500 shadow-lg transform translate-x-[100px] rotate-y-90 opacity-90 hover:opacity-100 transition-all duration-300",
                    "Right"
                }
                div { class: "absolute w-full h-full flex items-center justify-center text-xl font-bold text-white bg-gradient-to-br from-green-500 to-blue-500 shadow-lg transform -translate-x-[100px] -rotate-y-90 opacity-90 hover:opacity-100 transition-all duration-300",
                    "Left"
                }
                div { class: "absolute w-full h-full flex items-center justify-center text-xl font-bold text-white bg-gradient-to-br from-yellow-400 to-pink-500 shadow-lg transform translate-y-[-100px] rotate-x-90 opacity-90 hover:opacity-100 transition-all duration-300",
                    "Top"
                }
                div { class: "absolute w-full h-full flex items-center justify-center text-xl font-bold text-white bg-gradient-to-br from-red-500 to-purple-500 shadow-lg transform translate-y-[100px] -rotate-x-90 opacity-90 hover:opacity-100 transition-all duration-300",
                    "Bottom"
                }
            }
        }
    }
}

#[component]
pub fn BatchAnimationDemo() -> Element {
    let mut circles = use_signal(|| {
        (0..10)
            .map(|i| {
                (
                    use_motion(0.0f32), // x position
                    use_motion(0.0f32), // scale
                    i as f32,           // delay multiplier
                )
            })
            .collect::<Vec<_>>()
    });

    let animate_all = move |_| {
        for (i, (x_pos, scale, delay)) in circles.write().iter_mut().enumerate() {
            x_pos.animate_to(
                200.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                }))
                .with_delay(Duration::from_millis((*delay * 100.0) as u64)),
            );

            scale.animate_to(
                2.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 120.0,
                    damping: 8.0,
                    mass: 1.0,
                    velocity: 0.0,
                }))
                .with_delay(Duration::from_millis((*delay * 100.0) as u64)),
            );
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4 p-8",
            button {
                class: "px-4 py-2 bg-blue-500 text-white rounded",
                onclick: animate_all,
                "Animate All"
            }
            div { class: "relative h-64 bg-gray-100 rounded-lg",
                {
                    circles
                        .read()
                        .iter()
                        .enumerate()
                        .map(|(i, (x_pos, scale, _))| {
                            rsx! {
                                div {
                                    key: "{i}",
                                    class: "absolute top-1/2 left-0 w-8 h-8 bg-blue-500 rounded-full",
                                    style: "transform: translateX({x_pos.get_value()}px) translateY(-50%) scale({scale.get_value()})",
                                }
                            }
                        })
                }
            }
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
            // Bouncy button section
            section { class: "space-y-4",
                h2 { class: "text-xl font-bold", "Interactive Button" }
                p { class: "text-gray-600", "Combines spring physics with rotation" }
                BouncyButton {}
            }

            // Interactive cube section
            section { class: "space-y-4",
                h2 { class: "text-xl font-bold", "Interactive Cube" }
                p { class: "text-gray
                -600", "Click to rotate the cube" }
                InteractiveCube {}
            }
        }
    }
}
