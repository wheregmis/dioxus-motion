use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn ColorAnimation() -> Element {
    let mut color = use_motion(Color::from_rgba(0, 0, 0, 0)); // Initial blue color

    let animate_hover = move |_| {
        color.animate_to(
            Color::from_rgba(255, 255, 255, 255), // Purple color
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    let animate_reset = move |_| {
        color.animate_to(
            Color::from_rgba(0, 0, 0, 0), // Back to blue
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    rsx! {
        div { class: "flex flex-col items-center gap-4",
            // Color display
            div {
                class: "w-32 h-32 rounded-xl shadow-lg cursor-pointer transition-shadow hover:shadow-xl",
                onmouseenter: animate_hover,
                onmouseleave: animate_reset,
                style: "background-color: rgba({color.get_value().to_rgba().0}, {color.get_value().to_rgba().1}, {color.get_value().to_rgba().2}, {color.get_value().to_rgba().3});",
            }
            // Color values
            div { class: "text-sm font-mono",
                "RGB({color.get_value().to_rgba().0}, {color.get_value().to_rgba().1}, {color.get_value().to_rgba().2})"
            }
        }
    }
}
