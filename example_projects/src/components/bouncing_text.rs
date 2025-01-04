use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
fn BouncingLetter(letter: char, delay: f32) -> Element {
    let mut transform = use_animation(Transform::identity());

    use_effect(move || {
        transform.animate_to(
            Transform {
                y: -20.0,
                scale: 1.2,
                rotation: 5.0 * (std::f32::consts::PI / 180.0),
                x: 0.0,
            },
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 8.0,
                mass: 0.8,
                velocity: 10.0,
            }))
            .with_loop(LoopMode::Infinite),
        );
    });

    use_drop(move || {
        transform.stop();
    });

    rsx! {
        span {
            class: "text-4xl font-bold text-indigo-600 inline-block origin-bottom
                   transition-transform duration-300",
            style: "transform: translateY({transform.get_value().y}px)
                            scale({transform.get_value().scale})",
            "{letter}"
        }
    }
}

#[component]
pub fn BouncingText(text: String) -> Element {
    rsx! {
        div { class: "flex space-x-1",
            {
                text.chars()
                    .enumerate()
                    .map(|(i, char)| {
                        rsx! {
                            BouncingLetter { letter: char, delay: i as f32 * 0.1 }
                        }
                    })
            }
        }
    }
}
