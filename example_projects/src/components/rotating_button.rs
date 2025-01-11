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
