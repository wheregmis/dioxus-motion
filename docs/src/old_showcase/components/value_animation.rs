use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

#[component]
pub fn ValueAnimationShowcase() -> Element {
    let mut value = use_motion(0.0f32);

    let start_animation = move |_| {
        value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs(10),
                easing: easer::functions::Sine::ease_in_out,
            })),
        );
    };

    let reset_animation = move |_| {
        value.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs(3),
                easing: easer::functions::Sine::ease_out,
            })),
        );
    };

    use_drop(move || {
        value.stop();
    });

    rsx! {
        div { class: "h-[400px] flex items-center justify-center",
            div { class: "flex flex-col items-center justify-center p-6 bg-linear-to-br from-blue-500 to-purple-600 rounded-xl shadow-lg",
                // Counter with smaller font
                div { class: "text-4xl font-bold text-white mb-3", "{value.get_value() as i32}%" }

                // Smaller progress circle
                div {
                    class: "relative w-24 h-24",
                    style: "background: conic-gradient(from 0deg, #ffffff {value.get_value()}%, transparent 0)",
                    div { class: "absolute inset-2 bg-blue-600 rounded-full" }
                }

                // Compact buttons
                div { class: "flex gap-2 mt-4",
                    button {
                        class: "px-4 py-1.5 bg-white text-blue-600 rounded-full font-semibold
                                hover:bg-opacity-90 transition-all text-sm flex items-center gap-2",
                        onclick: start_animation,
                        "Start"
                    }
                    button {
                        class: "px-4 py-1.5 bg-white text-blue-600 rounded-full font-semibold
                                hover:bg-opacity-90 transition-all text-sm flex items-center gap-2",
                        onclick: reset_animation,
                        "Reset"
                    }
                }
            }
        }
    }
}
