use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

#[component]
pub fn ProgressBar(title: &'static str) -> Element {
    let progress = use_motion(0.0f32);

    use_effect(move || {
        let mut progress = progress.clone();
        progress.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs(2),
                easing: easer::functions::Cubic::ease_in_out,
            })),
        );
    });

    let progress_val = progress.clone();
    let progress_style = format!("width: {}%", progress_val.get_value());
    rsx! {
        div { class: "w-full p-6  rounded-xl shadow-lg",
            // Title and percentage display
            div { class: "flex justify-between items-center mb-4",
                span { class: "text-lg font-semibold", "{title}" }
                span { class: "text-sm text-gray-500", "{progress_val.get_value():.0}%" }
            }

            // Progress bar container
            div { class: "relative w-full h-4 bg-gray-100 rounded-full overflow-hidden",
                // Progress fill
                div {
                    class: "absolute top-0 left-0 h-full bg-linear-to-r from-blue-500 to-purple-600 rounded-full transition-all duration-300 ease-out",
                    style: progress_style
                }
                // Shimmer effect
                div {
                    class: "absolute top-0 left-0 w-full h-full bg-linear-to-r from-transparent via-white/30 to-transparent animate-shimmer",
                    style: "background-size: 200% 100%",
                }
            }
        }
    }
}
