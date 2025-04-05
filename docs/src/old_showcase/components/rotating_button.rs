use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

// A playful button that bounces on click
#[component]
pub fn RotatingButton() -> Element {
    let mut scale = use_motion(1.0f32);
    let mut rotation = use_motion(0.0f32);
    let mut glow = use_motion(0.0f32);

    let onclick = move |_| {
        // Optimized scale sequence with better physics and smoother transitions
        let scale_sequence = AnimationSequence::new()
            .then(
                1.15, // Reduced maximum scale for snappier feel
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 500.0, // Increased stiffness for faster response
                    damping: 15.0,    // Balanced damping for controlled bounce
                    mass: 0.8,        // Lighter mass for quicker movement
                    velocity: 8.0,    // Increased initial velocity
                })),
            )
            .then(
                0.9, // Subtle scale down
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 400.0,
                    damping: 12.0,
                    mass: 0.6,
                    velocity: -4.0, // Negative velocity for natural rebound
                })),
            )
            .then(
                1.0, // Return to original size
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 350.0,
                    damping: 20.0, // Higher damping for smooth finish
                    mass: 0.7,
                    velocity: 0.0,
                })),
            );

        // Optimized rotation with smoother easing
        let rotation_sequence = AnimationSequence::new().then(
            360.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(800),     // Faster rotation
                easing: easer::functions::Expo::ease_out, // Smoother deceleration
            })),
        );

        // Quick glow effect
        glow.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 450.0,
                damping: 15.0,
                mass: 0.5,
                velocity: 10.0,
            })),
        );

        scale.animate_sequence(scale_sequence);
        rotation.animate_sequence(rotation_sequence);
    };

    rsx! {
        button {
            class: "relative px-8 py-4 bg-gradient-to-r from-purple-500 to-pink-500
                   text-white rounded-xl font-bold text-lg overflow-hidden 
                   transition-all duration-300 hover:shadow-xl hover:shadow-purple-500/20",
            style: "transform: scale({scale.get_value()}) rotate({rotation.get_value()}deg)",
            onclick,
            // Enhanced glow effect
            div {
                class: "absolute inset-0 bg-white/30 blur-xl",
                style: "opacity: {glow.get_value()}",
            }
            "Click me!"
        }
    }
}
