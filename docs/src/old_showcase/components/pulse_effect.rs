use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn PulseEffect(color: &'static str, size: &'static str) -> Element {
    let scale = use_motion(1.0f32);
    let opacity = use_motion(0.8f32);

    use_effect(move || {
        let mut scale = scale.clone();
        let mut opacity = opacity.clone();
        // Main pulse animation
        scale.animate_to(
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
        opacity.animate_to(
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

    let scale_val = scale.clone();
    let opacity_val = opacity.clone();
    rsx! {
        div { class: "relative flex items-center justify-center",
            // Main circle
            div {
                class: "{size} {color} rounded-full transition-all",
                style: format!("transform: scale({}); opacity: {}", scale_val.get_value(), opacity_val.get_value()),
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
                                    style: format!("animation-delay: {}s; opacity: 0.3", delay),
                                }
                            }
                        })
                }
            }
        }
    }
}
