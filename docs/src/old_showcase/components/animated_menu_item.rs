use dioxus::prelude::*;
use dioxus_motion::prelude::*;

// An interactive menu item with smooth transitions
#[component]
pub fn AnimatedMenuItem(label: String) -> Element {
    let mut x_offset = use_motion(0.0f32);
    let mut scale = use_motion(1.0f32);
    let mut glow = use_motion(0.0f32);

    let onmouseenter = move |_| {
        x_offset.animate_to(
            20.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        scale.animate_to(
            1.1,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        glow.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let onmouseleave = move |_| {
        x_offset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        glow.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    rsx! {
        div {
            class: "relative p-4 cursor-pointer bg-gradient-to-r from-gray-800 to-gray-900 text-white rounded-xl overflow-hidden group",
            style: "transform: translateX({x_offset.get_value()}px) scale({scale.get_value()})",
            onmouseenter,
            onmouseleave,
            // Glow effect
            div {
                class: "absolute inset-0 bg-gradient-to-r from-blue-500/30 to-purple-500/30 transition-opacity duration-300",
                style: "opacity: {glow.get_value()}",
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
