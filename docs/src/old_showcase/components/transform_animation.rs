use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn TransformAnimationShowcase() -> Element {
    let mut transform = use_motion_store(Transform::identity());

    let animate_hover = move |_| {
        transform.animate_to(
            Transform::new(
                0.0,                                  // x
                -20.0,                                // y
                1.1,                                  // scale
                5.0 * (std::f32::consts::PI / 180.0), // rotation in radians
            ),
            AnimationConfig::custom_spring(180.0, 12.0, 1.0),
        );
    };

    let animate_reset = move |_| {
        transform.animate_to(
            Transform::identity(),
            AnimationConfig::custom_spring(200.0, 20.0, 1.0),
        );
    };

    let transform_style = use_memo(move || {
        format!(
            "transform: translate({}px, {}px) scale({}) rotate({}deg); transform-style: preserve-3d; will-change: transform;",
            transform.store().current()().x,
            transform.store().current()().y,
            transform.store().current()().scale,
            transform.store().current()().rotation * 180.0 / std::f32::consts::PI
        )
    });

    let glow_style = use_memo(move || {
        format!(
            "transform: translate({}px, {}px) scale(1.2); opacity: {};",
            transform.store().current()().x,
            transform.store().current()().y,
            if transform.store().current()().y < 0.0 {
                0.6
            } else {
                0.0
            }
        )
    });

    rsx! {
        div { class: "h-[400px] flex items-center justify-center p-4",
            div {
                class: "relative group cursor-pointer",
                onmouseenter: animate_hover,
                onmouseleave: animate_reset,
                // Main card - reduced from w-64/h-64 to w-48/h-48
                div {
                    class: "w-36 h-36 bg-linear-to-tr from-emerald-400 to-cyan-400 rounded-xl shadow-xl",
                    style: "{transform_style.read()}",
                    div { class: "h-full w-full flex flex-col items-center justify-center text-white",
                        span { class: "text-xl font-bold mb-1", "Hover Me!" }
                        span { class: "text-xs opacity-75", "Spring Animation" }
                    }
                }
                // Glow effect - scaled proportionally
                div {
                    class: "absolute inset-0 bg-linear-to-tr from-emerald-400/30 to-cyan-400/30
                            rounded-2xl blur-lg -z-10",
                    style: "{glow_style.read()}",
                }
            }
        }
    }
}
