use dioxus::prelude::*;
use dioxus_motion::{
    animation::Tween,
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};
use easer::functions::Easing;

#[component]
pub fn ParticleSystem() -> Element {
    let particles = (0..20).map(|i| {
        let mut transform = use_transform_animation(
            Transform::default(),
            Transform {
                x: (i as f32 * 10.0).sin() * 100.0,
                y: (i as f32 * 10.0).cos() * 100.0,
                scale: 0.8,
                opacity: 0.6,
                ..Default::default()
            },
            AnimationMode::Tween(Tween {
                duration: Duration::from_secs(2),
                easing: easer::functions::Sine::ease_in_out,
            }),
        );

        rsx! {
            div {
                class: "flex items-center justify-center w-2 h-2 bg-gradient-to-br from-blue-400 to-purple-500 rounded-full",
                style: "{transform.style()}",
                onmounted: move |_| transform.loop_animation(),
            }
        }
    });

    use_drop(move || {
        // Stop all animations
        for i in 0..20 {
            let mut transform = use_transform_animation(
                Transform::default(),
                Transform {
                    x: (i as f32 * 10.0).sin() * 100.0,
                    y: (i as f32 * 10.0).cos() * 100.0,
                    scale: 0.8,
                    opacity: 0.6,
                    ..Default::default()
                },
                AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(2),
                    easing: easer::functions::Sine::ease_in_out,
                }),
            );
            transform.stop();
        }
    });

    rsx! {
        div { class: "relative w-full h-64 bg-gray-900 rounded-xl overflow-hidden",
            {particles}
        }
    }
}
