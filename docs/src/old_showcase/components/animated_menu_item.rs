use dioxus::prelude::*;
use dioxus_motion::prelude::*;

// An interactive menu item with smooth transitions
#[component]
pub fn AnimatedMenuItem(label: String) -> Element {
    let x_offset = use_motion(0.0f32);
    let scale = use_motion(1.0f32);
    let glow = use_motion(0.0f32);

    let x_offset_mouse = x_offset.clone();
    let scale_mouse = scale.clone();
    let glow_mouse = glow.clone();
    let onmouseenter = move |_| {
        x_offset_mouse.animate_to(
            20.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        scale_mouse.animate_to(
            1.1,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        glow_mouse.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let x_offset_leave = x_offset.clone();
    let scale_leave = scale.clone();
    let glow_leave = glow.clone();
    let onmouseleave = move |_| {
        x_offset_leave.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        scale_leave.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        glow_leave.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let x_offset_val = x_offset.clone();
    let scale_val = scale.clone();
    let glow_val = glow.clone();
    rsx! {
        div {
            class: "relative p-4 cursor-pointer bg-linear-to-r from-gray-800 to-gray-900 text-white rounded-xl overflow-hidden group",
            style: format!("transform: translateX({}px) scale({})", x_offset_val.get_value(), scale_val.get_value()),
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            div {
                class: "absolute inset-0 bg-gradient-to-r from-purple-500/10 to-blue-500/10 rounded-xl pointer-events-none",
                style: format!("opacity: {}", glow_val.get_value()),
            }
            span { class: "relative z-10", "{label}" }
        }
    }
}
