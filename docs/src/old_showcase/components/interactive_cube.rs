use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const CONTAINER_SIZE: f32 = 100.0;
const PERSPECTIVE: f32 = 500.0;

#[component]
pub fn InteractiveCube() -> Element {
    let mut rotation_x = use_motion(0.0f32);
    let mut rotation_y = use_motion(0.0f32);
    let mut scale = use_motion(1.0f32);
    let mut glow = use_motion(0.0f32);

    let onclick = move |_e: Event<MouseData>| {
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
        let _y = (rect.y as f32 - CONTAINER_SIZE / 2.0) / CONTAINER_SIZE - 0.5;

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
