use dioxus::prelude::*;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};

#[component]
pub fn ProgressBar(title: &'static str) -> Element {
    let mut progress =
        use_value_animation(Motion::new(0.0).to(100.0).duration(Duration::from_secs(10)));

    use_effect(move || {
        progress.loop_animation();
    });

    rsx! {
        div { class: "w-full p-6 bg-white rounded-xl shadow-lg",
            // Title and percentage
            div { class: "flex justify-between items-center mb-4",
                span { class: "text-lg font-semibold text-gray-700", "{title}" }
                span { class: "text-sm font-medium text-blue-600", "{progress.value() as i32}%" }
            }

            // Progress bar
            div { class: "w-full h-4 bg-gray-200 rounded-full overflow-hidden",
                div {
                    class: "h-full w-full bg-gradient-to-r from-blue-500 to-purple-600 transition-all duration-300",
                    style: "width: {progress.value() as i32}%",
                }
            }
        }
    }
}
