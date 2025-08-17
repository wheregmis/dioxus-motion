use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn PulseEffect(color: &'static str, size: &'static str) -> Element {
    let scale = use_motion_store(1.0f32);
    let opacity = use_motion_store(0.8f32);

    use_effect(move || {
        // Main pulse animation
        animate_to(
            &scale,
            1.2,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 5.0,
                mass: 0.5,
                ..Default::default()
            }))
            .with_loop(LoopMode::Infinite),
        );

        // Fade animation
        animate_to(
            &opacity,
            0.2,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 80.0,
                damping: 10.0,
                mass: 0.5,
                ..Default::default()
            }))
            .with_loop(LoopMode::Infinite),
        );
    });

    rsx! {
        div { class: "relative flex items-center justify-center",
            // Main circle
            div {
                class: "{size} {color} rounded-full transition-all",
                style: "transform: scale({scale.current()}); opacity: {opacity.current()}",
            }
            // Background pulse rings
            div { class: "absolute inset-0",
                {
                    (0..3)
                        .map(|i| {
                            let delay = i as f32 * 0.2;
                            rsx! {
                                div {
                                    class: "{size} {color} rounded-full absolute inset-0 animate-ping",
                                    style: "animation-delay: {delay}s; opacity: 0.3",
                                }
                            }
                        })
                }
            }
        }
    }
}
