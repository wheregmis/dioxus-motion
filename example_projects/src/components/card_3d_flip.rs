use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn Card3DFlip() -> Element {
    let mut transform = use_motion(Transform::identity());
    let mut is_flipped = use_signal(|| false);

    let animate_flip = move |_| {
        if *is_flipped.read() {
            transform.animate_to(
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 150.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 10.0,
                }))
                .with_loop(LoopMode::Infinite),
            );
        } else {
            transform.animate_to(
                Transform {
                    rotation: 180.0,
                    scale: 1.0,
                    x: 0.0,
                    y: 0.0,
                },
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 150.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 10.0,
                })),
            );
        }
        is_flipped.toggle();
    };

    use_drop(move || {
        transform.stop();
    });

    rsx! {
        div { class: "perspective-1000",
            div {
                class: "relative w-64 h-64 cursor-pointer transition-transform duration-700",
                style: "transform-style: preserve-3d;
                        transform: rotateY({transform.get_value().rotation}deg) 
                                 scale({transform.get_value().scale});",
                onclick: animate_flip,

                // Front
                div { class: "absolute w-full h-full bg-gradient-to-br from-cyan-400
                           to-blue-500 rounded-xl p-6 text-white backface-hidden",
                    div { class: "flex items-center justify-center h-full text-xl font-bold",
                        "Front Side"
                    }
                }

                // Back
                div {
                    class: "absolute w-full h-full bg-gradient-to-br from-purple-400
                           to-pink-500 rounded-xl p-6 text-white backface-hidden",
                    style: "transform: rotateY(180deg);",
                    div { class: "flex items-center justify-center h-full text-xl font-bold",
                        "Back Side"
                    }
                }
            }
        }
    }
}
