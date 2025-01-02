use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
fn BouncingLetter(letter: char, delay: f32) -> Element {
    let mut y_pos = use_value_animation(Motion::new(0.0).to(-20.0).spring(Spring {
        stiffness: 200.0, // Reduced for smoother bounce
        damping: 8.0,     // Less damping for more bounce
        mass: 0.8,        // Lighter mass
        velocity: 0.0,
    }));

    let mut scale = use_value_animation(Motion::new(1.0).to(1.2).spring(Spring {
        stiffness: 200.0,
        damping: 8.0,
        mass: 0.8,
        velocity: 0.0,
    }));

    use_effect(move || {
        y_pos.loop_animation();
        scale.loop_animation();
    });

    use_drop(move || {
        y_pos.stop_loop();
        scale.stop_loop();
    });

    rsx! {
        span {
            class: "text-4xl font-bold text-indigo-600 inline-block",
            style: "transform: translateY({y_pos.value()}px) scale({scale.value()});
                   transition: transform 0.1s linear;",
            "{letter}"
        }
    }
}

#[component]
pub fn BouncingText(text: String, delay: f32) -> Element {
    rsx! {
        div { class: "flex space-x-1",
            {
                text.chars()
                    .enumerate()
                    .map(|(i, char)| {
                        rsx! {
                            BouncingLetter { letter: char, delay: i as f32 * delay }
                        }
                    })
            }
        }
    }
}
