use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_motion::{AnimationTarget, TransitionConfig, TransitionType};

#[component]
pub fn TransformAnimationShowcase() -> Element {
    rsx! {
        div { class: "h-[400px] flex items-center justify-center p-4",
            div {
                class: "relative group cursor-pointer",
                // Main card with motion primitives
                motion::div {
                    class: "w-36 h-36 bg-linear-to-tr from-emerald-400 to-cyan-400 rounded-xl shadow-xl",
                    style: "transform-style: preserve-3d; will-change: transform;",

                    // Animation properties
                    initial: Some(AnimationTarget::new().x(0.0).y(0.0).scale(1.0).rotate(0.0)),
                    while_hover: Some(AnimationTarget::new().x(0.0).y(-20.0).scale(1.1).rotate(5.0)),
                    transition: Some(
                        TransitionConfig::default()
                            .type_(TransitionType::Spring)
                            .stiffness(180.0)
                            .damping(12.0)
                    ),

                    div { class: "h-full w-full flex flex-col items-center justify-center text-white",
                        span { class: "text-xl font-bold mb-1", "Hover Me!" }
                        span { class: "text-xs opacity-75", "Spring Animation" }
                    }
                }

                // Glow effect with motion primitives
                motion::div {
                    class: "absolute inset-0 bg-linear-to-tr from-emerald-400/30 to-cyan-400/30 rounded-2xl blur-lg -z-10",

                    // Animation properties
                    initial: Some(AnimationTarget::new().x(0.0).y(0.0).scale(1.2).opacity(0.0)),
                    while_hover: Some(AnimationTarget::new().x(0.0).y(-20.0).scale(1.2).opacity(0.6)),
                    transition: Some(
                        TransitionConfig::default()
                            .type_(TransitionType::Spring)
                            .stiffness(180.0)
                            .damping(12.0)
                    ),
                }
            }
        }
    }
}
