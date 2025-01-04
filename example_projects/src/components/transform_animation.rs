use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn TransformAnimationShowcase() -> Element {
    let mut transform = use_motion(Transform::identity());

    let animate_hover = move |_| {
        transform.animate_to(
            Transform::new(
                0.0,                                  // x
                -20.0,                                // y
                1.1,                                  // scale
                5.0 * (std::f32::consts::PI / 180.0), // rotation in radians
            ),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 180.0, // Softer spring
                damping: 12.0,    // Less damping for bounce
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    let animate_reset = move |_| {
        transform.animate_to(
            Transform::identity(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 20.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    use_drop(move || {
        transform.stop();
    });

    rsx! {
        div { class: "flex items-center justify-center p-8",
            div {
                class: "relative group cursor-pointer",
                onmouseenter: animate_hover,
                onmouseleave: animate_reset,

                // Main card
                div {
                    class: "w-64 h-64 bg-gradient-to-tr from-emerald-400 to-cyan-400 rounded-2xl shadow-xl",
                    style: "transform: translate({transform.get_value().x}px, {transform.get_value().y}px)
                                scale({transform.get_value().scale}) 
                                rotate({transform.get_value().rotation * 180.0 / std::f32::consts::PI}deg);",

                    div { class: "h-full w-full flex flex-col items-center justify-center text-white",
                        span { class: "text-2xl font-bold mb-2", "Hover Me!" }
                        span { class: "text-sm opacity-75", "Spring Animation" }
                    }
                }

                // Glow effect
                div {
                    class: "absolute inset-0 bg-gradient-to-tr from-emerald-400/30 to-cyan-400/30
                            rounded-3xl blur-xl -z-10",
                    style: "transform: translate({transform.get_value().x}px, {transform.get_value().y}px)
                            scale(1.2);
                            opacity: {{if transform.get_value().y < 0.0 { 0.6 } else { 0.0 }}};",
                }
            }
        }
    }
}
