use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

#[component]
pub fn ProgressBar(title: &'static str) -> Element {
    let mut progress = use_animation(0.0f32);

    use_effect(move || {
        progress.animate_to(
            100.0,
            AnimationMode::Tween(Tween {
                duration: Duration::from_secs(2),
                easing: easer::functions::Sine::ease_out,
            }),
        );
        progress.loop_animation();
    });

    use_drop(move || progress.stop_loop());

    rsx! {
        div { class: "w-full p-6 bg-white rounded-xl shadow-lg",
            // Title and percentage display
            div { class: "flex justify-between items-center mb-4",
                span { class: "text-lg font-semibold text-gray-700", "{title}" }
                span { class: "text-sm font-medium text-blue-600", "{progress.get_value() as i32}%" }
            }

            // Progress bar container
            div { class: "relative w-full h-4 bg-gray-100 rounded-full overflow-hidden",
                // Progress fill
                div {
                    class: "absolute top-0 left-0 h-full bg-gradient-to-r from-blue-500 to-purple-600
                           rounded-full transition-all duration-300 ease-out",
                    style: "width: {progress.get_value()}%",
                }
                // Shimmer effect
                div {
                    class: "absolute top-0 left-0 w-full h-full bg-gradient-to-r from-transparent
                           via-white/30 to-transparent animate-shimmer",
                    style: "background-size: 200% 100%",
                }
            }
        }
    }
}
