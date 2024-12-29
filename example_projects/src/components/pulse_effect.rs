use dioxus::prelude::*;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};
#[component]
pub fn PulseEffect(color: &'static str, size: &'static str) -> Element {
    let mut scale = use_value_animation(Motion::new(1.0).to(1.2).spring(Spring {
        stiffness: 100.0,
        damping: 5.0,
        mass: 0.5,
        velocity: 0.0,
    }));

    use_effect(move || {
        scale.loop_animation();
    });

    use_drop(move || {
        scale.stop_loop();
    });

    rsx! {
        div { class: "relative flex items-center justify-center",
            div {
                class: "{size} {color} rounded-full",
                style: "transform: scale({scale.value()})",
                div { class: "absolute inset-0 {color} rounded-full animate-ping opacity-75" }
            }
        }
    }
}
