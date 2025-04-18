use dioxus::prelude::*;
use dioxus_motion::prelude::*;

// An animated counter that shows basic motion and sequences
#[component]
pub fn AnimatedCounter() -> Element {
    let mut value = use_motion(0.0f32);
    let mut scale = use_motion(1.0f32);
    let mut count = use_signal(|| 0);

    let onclick = move |_| {
        let sequence = AnimationSequence::new().then(
            ((*count)() + 1) as f32 * 100.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 180.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 10.0,
            })),
        );

        scale.animate_to(
            1.2,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        value.animate_sequence(sequence);
        count.set((*count)() + 1);
    };

    rsx! {
        div { class: "flex flex-col items-center gap-6 p-8 rounded-2xl backdrop-blur-xs",
            div {
                class: "relative text-5xl font-bold text-transparent bg-clip-text bg-linear-to-r from-blue-500 to-purple-500",
                style: "transform: translateY({value.get_value()}px) scale({scale.get_value()})",
                "Count: {count}"
            }
            button {
                class: "px-6 py-3 bg-linear-to-r from-blue-500 to-purple-500 text-white rounded-full font-semibold shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-300",
                onclick,
                "Increment"
            }
        }
    }
}
