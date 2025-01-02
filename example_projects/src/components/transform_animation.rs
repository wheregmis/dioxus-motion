use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn TransformAnimationShowcase() -> Element {
    let mut transform = use_animation(Transform::default());

    let mut animate_hover = move || {
        transform.animate_to(
            Transform {
                y: -20.0,
                scale: 1.1,
                rotate: 5.0,
                opacity: 0.5,
                ..Default::default()
            },
            AnimationMode::Spring(Spring {
                stiffness: 300.0,
                damping: 15.0,
                mass: 0.8,
                ..Default::default()
            }),
        );
    };

    let mut animate_reset = move || {
        transform.animate_to(
            Transform::default(),
            AnimationMode::Spring(Spring::default()),
        );
    };

    rsx! {
        div { class: "flex items-center justify-center p-8",
            div {
                class: "relative group cursor-pointer",
                onmouseenter: move |_| animate_hover(),
                onmouseleave: move |_| animate_reset(),
                // Main card
                div {
                    class: "w-64 h-64 bg-gradient-to-tr from-emerald-400 to-cyan-400 rounded-2xl shadow-xl transition-shadow",
                    style: "transform: translateY({transform.get_value().y}px)
                            scale({transform.get_value().scale}) 
                            rotate({transform.get_value().rotate}deg); 
                            opacity: {transform.get_value().opacity}",
                    // Content
                    div { class: "h-full w-full flex flex-col items-center justify-center text-white",
                        span { class: "text-2xl font-bold mb-2 transition-transform",
                            "Hover Me!"
                        }
                        span { class: "text-sm opacity-75", "Spring Animation" }
                    }
                }

                // Background glow effect
                div {
                    class: "absolute inset-0 bg-gradient-to-tr from-emerald-400/30 to-cyan-400/30
                           rounded-3xl blur-xl transition-opacity -z-10",
                    style: "transform: translateY({transform.get_value().y}px) scale(1.1);
                           opacity: {transform.get_value().opacity - 0.5}",
                }
            }
        }
    }
}
