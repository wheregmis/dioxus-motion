use dioxus::prelude::*;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};

#[component]
pub fn TransformAnimationShowcase() -> Element {
    let mut transform = use_transform_animation(
        Transform::default(),
        Transform {
            x: 0.0,
            y: -20.0,
            scale: 1.1,
            rotate: 5.0,
            opacity: 1.0,
        },
        AnimationMode::Spring(Spring {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }),
    );

    rsx! {
        div { class: "flex items-center justify-center p-8",
            div {
                class: "group cursor-pointer",
                onmouseenter: move |_| transform.start(),
                onmouseleave: move |_| transform.reset(),
                div {
                    class: "w-64 h-64 bg-gradient-to-tr from-emerald-400 to-cyan-400 rounded-2xl shadow-xl transition-all",
                    style: "{transform.style()}",
                    div { class: "h-full w-full flex flex-col items-center justify-center text-white",
                        span { class: "text-2xl font-bold mb-2", "Hover Me!" }
                        span { class: "text-sm opacity-75", "Spring Animation" }
                    }
                }
            }
        }
    }
}
