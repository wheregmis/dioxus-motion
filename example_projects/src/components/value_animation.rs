use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

#[component]
pub fn ValueAnimationShowcase() -> Element {
    let mut value = use_animation(0.0f32);
    let mut playing = use_signal(|| false);

    let mut start_animation = move || {
        if !playing() {
            playing.set(true);
            value.animate_to(
                100.0,
                AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(2),
                    easing: easer::functions::Sine::ease_out,
                }),
            );
        }
    };

    let mut reset_animation = move || {
        playing.set(false);
        value.animate_to(
            0.0,
            AnimationMode::Tween(Tween {
                duration: Duration::from_secs(2),
                easing: easer::functions::Sine::ease_out,
            }),
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
