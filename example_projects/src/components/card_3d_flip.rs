use dioxus::prelude::*;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};

#[component]
pub fn Card3DFlip() -> Element {
    let mut transform = use_transform_animation(
        Transform::default(),
        Transform {
            rotate: 180.0,
            scale: 1.1,
            ..Default::default()
        },
        AnimationMode::Spring(Spring::default()),
    );

    rsx! {
        div { class: "group perspective-1000",
            div {
                class: "relative w-64 h-64 cursor-pointer",
                style: "transform-style: preserve-3d; {transform.style()}",
                onmouseenter: move |_| transform.start(),
                onmouseleave: move |_| transform.reset(),

                // Front
                div { class: "absolute w-full h-full bg-gradient-to-br from-cyan-400 to-blue-500 rounded-xl p-6 text-white backface-hidden",
                    "Front Side"
                }
                // Back
                div {
                    class: "absolute w-full h-full bg-gradient-to-br from-purple-400 to-pink-500 rounded-xl p-6 text-white backface-hidden",
                    style: "transform: rotateY(180deg);",
                    "Back Side"
                }
            }
        }
    }
}
