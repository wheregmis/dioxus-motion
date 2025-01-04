use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn ColorAnimation() -> Element {
    let mut color = use_motion(Color::from_rgba(59, 130, 246, 255)); // Initial blue
    let mut current_index = use_signal(|| 0);

    let colors = [
        Color::from_rgba(59, 130, 246, 255), // Blue
        Color::from_rgba(168, 85, 247, 255), // Purple
        Color::from_rgba(234, 88, 12, 255),  // Orange
        Color::from_rgba(22, 163, 74, 255),  // Green
    ];

    let animate_next = move |_| {
        if !color.is_running() {
            let next_index = (*current_index.read() + 1) % colors.len();
            current_index.set(next_index);

            color.animate_to(
                colors[next_index],
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 80.0,
                    damping: 8.0,
                    mass: 0.8,
                    ..Default::default()
                })),
            );
        }
    };

    use_drop(move || {
        color.stop();
    });

    rsx! {
        div { class: "flex flex-col items-center gap-6 p-8",
            div { class: "relative group cursor-pointer", onclick: animate_next,
                // Glow effect
                div {
                    class: "absolute inset-0 blur-xl opacity-50 group-hover:opacity-75 transition-all",
                    style: "background-color: rgba({color.get_value().to_rgba().0},
                                                {color.get_value().to_rgba().1}, 
                                                {color.get_value().to_rgba().2}, 0.5);",
                }
                // Color box
                div {
                    class: "w-40 h-40 rounded-2xl shadow-lg transform transition-all
                           group-hover:scale-105 group-hover:shadow-2xl",
                    style: "background-color: rgba({color.get_value().to_rgba().0},
                                                {color.get_value().to_rgba().1}, 
                                                {color.get_value().to_rgba().2}, 1);",
                }
            }
            // Status indicator
            div { class: "text-sm font-mono",
                if color.is_running() {
                    "Animating..."
                } else {
                    "Click to animate"
                }
            }
        }
    }
}
