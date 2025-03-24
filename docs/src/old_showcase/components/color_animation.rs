use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::old_showcase::showcase_component::BG_COLOR;

#[component]
pub fn ColorAnimation() -> Element {
    let mut color = use_motion(Color::from_rgba(17, 24, 39, 255));

    let mut is_animating = use_signal(|| false);

    let animate_sequence = move |_| {
        *is_animating.write() = true;
        let sequence = AnimationSequence::new()
            .then(
                Color::from_rgba(31, 41, 55, 255), // Gray-800
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 80.0,
                    damping: 8.0,
                    mass: 0.8,
                    velocity: 5.0,
                })),
            )
            .then(
                Color::from_rgba(107, 114, 128, 255), // Gray-500
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 3.0,
                })),
            )
            .then(
                Color::from_rgba(255, 255, 255, 255), // White
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 3.0,
                })),
            )
            .on_complete(move || {
                *is_animating.write() = false;
            });

        color.animate_sequence(sequence);
    };

    use_effect(use_reactive((&color.get_value(),), |(data,)| {
        *BG_COLOR.write() = data;
    }));

    use_drop(move || {
        color.stop();
    });

    rsx! {
        div { class: "flex flex-col items-center gap-6 p-8",
            div {
                class: "relative group cursor-pointer",
                onclick: animate_sequence,
                // Glow effect
                div {
                    class: "absolute inset-0 border-teal-300 transition-all",
                    style: "background-color: rgba({color.get_value().to_rgba().0},
                                                {color.get_value().to_rgba().1}, 
                                                {color.get_value().to_rgba().2}, 0.5);",
                }
                // Color box with enhanced glow
                div {
                    class: "w-40 h-40 rounded-2xl
                           group-hover:scale-105",
                    style: "background-color: rgba({color.get_value().to_rgba().0},
                                                {color.get_value().to_rgba().1}, 
                                                {color.get_value().to_rgba().2}, 1);
                            ",
                }
            }
            // Status indicator
            if !*is_animating.read() {
                div { class: "text-sm font-mono", "Click to animate..." }
            }
        }
    }
}
