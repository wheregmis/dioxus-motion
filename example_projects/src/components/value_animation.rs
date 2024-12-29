use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn ValueAnimationShowcase() -> Element {
    let mut counter =
        use_value_animation(Motion::new(0.0).to(100.0).duration(Duration::from_secs(2)));

    use_drop(move || {
        counter.stop();
    });

    rsx! {
        div { class: "flex flex-col items-center justify-center p-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl shadow-lg",
            // Animated counter
            div { class: "text-6xl font-bold text-white mb-4", "{counter.value() as i32}%" }

            // Progress circle
            div {
                class: "relative w-32 h-32",
                style: "background: conic-gradient(from 0deg, #ffffff {counter.value()}%, transparent 0)",
                div { class: "absolute inset-2 bg-blue-600 rounded-full" }
            }

            button {
                class: "mt-6 px-6 py-2 bg-white text-blue-600 rounded-full font-semibold hover:bg-opacity-90 transition-all",
                onclick: move |_| counter.start(),
                "Animate"
            }
        }
    }
}
