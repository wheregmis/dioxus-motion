use dioxus::prelude::*;
use dioxus_motion::prelude::*;

// An interactive menu item with smooth transitions
#[component]
pub fn AnimatedMenuItem(label: String) -> Element {
    let x_offset = use_motion_store(0.0f32);
    let scale = use_motion_store(1.0f32);
    let glow = use_motion_store(0.0f32);

    let onmouseenter = move |_| {
        animate_to(
            &x_offset,
            20.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        animate_to(
            &scale,
            1.1,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        animate_to(
            &glow,
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let onmouseleave = move |_| {
        animate_to(
            &x_offset,
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        animate_to(
            &scale,
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        animate_to(
            &glow,
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    rsx! {
        div {
            class: "relative p-4 cursor-pointer bg-linear-to-r from-gray-800 to-gray-900 text-white rounded-xl overflow-hidden group",
            style: "transform: translateX({x_offset.current()}px) scale({scale.current()})",
            onmouseenter,
            onmouseleave,
            // Glow effect
            div {
                class: "absolute inset-0 bg-linear-to-r from-blue-500/30 to-purple-500/30 transition-opacity duration-300",
                style: "opacity: {glow.current()}",
            }
            // Content
            div { class: "relative z-10 flex items-center gap-2",
                span { class: "text-lg font-medium", "{label}" }
                span { class: "text-blue-400 group-hover:translate-x-1 transition-transform duration-300",
                    "→"
                }
            }
        }
    }
}
