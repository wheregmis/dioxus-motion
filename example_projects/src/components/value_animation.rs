use dioxus::prelude::*;
use dioxus_motion::{prelude::*, AnimationConfig, LoopMode};
use easer::functions::Easing;

#[component]
pub fn ValueAnimationShowcase() -> Element {
    let mut value = use_animation(0.0f32);

    let mut start_animation = move || {
        value.animate_to(
            100.0,
            AnimationConfig {
                mode: AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(3),
                    easing: easer::functions::Linear::ease_in_out,
                }),
                loop_mode: Some(LoopMode::Infinite),
            },
        );
    };

    let mut reset_animation = move || {
        value.animate_to(
            0.0,
            AnimationConfig {
                mode: AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(10),
                    easing: easer::functions::Sine::ease_out,
                }),
                loop_mode: Some(LoopMode::None),
            },
        );
    };

    rsx! {
        div { class: "flex flex-col items-center justify-center p-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl shadow-lg",
            // Animated counter
            div { class: "text-6xl font-bold text-white mb-4", "{value.get_value() as i32}%" }

            // Progress circle
            div {
                class: "relative w-32 h-32",
                style: "background: conic-gradient(from 0deg, #ffffff {value.get_value()}%, transparent 0)",
                div { class: "absolute inset-2 bg-blue-600 rounded-full" }
            }

            // Controls
            div { class: "flex gap-4 mt-6",
                button {
                    class: "px-6 py-2 bg-white text-blue-600 rounded-full font-semibold hover:bg-opacity-90 transition-all",
                    onclick: move |_| start_animation(),
                    "Start"
                }
                button {
                    class: "px-6 py-2 bg-white/20 text-white rounded-full font-semibold hover:bg-opacity-30 transition-all",
                    onclick: move |_| reset_animation(),
                    "Reset"
                }
            }
        }
    }
}
