use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_motion::{AnimationTarget, TransitionConfig, TransitionType};

// An animated counter that shows basic motion and sequences
#[component]
pub fn AnimatedCounter() -> Element {
    let mut count = use_signal(|| 0);
    let mut animate_count = use_signal(|| false);

    let increment = move |_| {
        count.set(count() + 1);

        // Toggle animation state
        animate_count.toggle();
    };

    rsx! {
        div { class: "flex flex-col items-center gap-6 p-8 rounded-2xl backdrop-blur-xs",
            // Animated counter with motion primitives
            motion::div {
                class: "relative text-5xl font-bold text-transparent bg-clip-text bg-linear-to-r from-blue-500 to-purple-500",

                // Animation properties
                animate: Some(
                    if animate_count() {
                        AnimationTarget::new().y(count() as f32 * 10.0).scale(1.2)
                    } else {
                        AnimationTarget::new().y(0.0).scale(1.0)
                    }
                ),
                transition: Some(
                    TransitionConfig::default()
                        .type_(TransitionType::Spring)
                        .stiffness(180.0)
                        .damping(12.0)
                ),

                "Count: {count}"
            }

            // Animated button with motion primitives
            motion::button {
                class: "px-6 py-3 bg-linear-to-r from-blue-500 to-purple-500 text-white rounded-full font-semibold shadow-lg",

                // Animation properties
                while_hover: Some(AnimationTarget::new().y(-2.0)),
                while_tap: Some(AnimationTarget::new().scale(0.95)),
                transition: Some(
                    TransitionConfig::default()
                        .type_(TransitionType::Spring)
                        .stiffness(300.0)
                        .damping(20.0)
                ),

                onclick: increment,
                "Increment"
            }
        }
    }
}
