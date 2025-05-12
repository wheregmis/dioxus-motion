use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_motion::{AnimationTarget, TransitionConfig, TransitionType};
use easer::functions::Easing;

#[component]
pub fn ValueAnimationShowcase() -> Element {
    let mut animate_progress = use_signal(|| false);
    let mut percentage = use_motion(0.0f32);

    let toggle_animation = move |_| {
        animate_progress.toggle();

        // Use the motion hook directly for smoother animation
        if animate_progress() {
            percentage.animate_to(
                100.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(3), // 3 seconds for the animation
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            );
        } else {
            percentage.animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(1), // 1 second for reset
                    easing: easer::functions::Cubic::ease_out,
                })),
            );
        }
    };

    // Format the percentage for display
    let display_percentage = use_memo(move || format!("{}%", percentage.get_value().round()));

    // Format the conic gradient for the progress circle
    let circle_style = use_memo(move || {
        format!(
            "background: conic-gradient(from 0deg, #ffffff {}%, transparent 0)",
            percentage.get_value()
        )
    });

    rsx! {
        div { class: "h-[400px] flex items-center justify-center",
            div { class: "flex flex-col items-center justify-center p-6 bg-linear-to-br from-blue-500 to-purple-600 rounded-xl shadow-lg",
                // Counter with motion primitives
                div { class: "text-4xl font-bold text-white mb-3", "{display_percentage}" }

                // Progress circle with motion primitives
                div { class: "relative w-24 h-24",

                    // Use a wrapper div for the animation
                    div {
                        class: "absolute inset-0 rounded-full",
                        style: "{circle_style}",
                    }

                    div { class: "absolute inset-2 bg-blue-600 rounded-full" }
                }

                // Compact buttons
                div { class: "flex gap-2 mt-4",
                    motion::button {
                        class: "px-4 py-1.5 bg-white text-blue-600 rounded-full font-semibold text-sm flex items-center gap-2",
                        while_hover: Some(AnimationTarget::new().scale(1.05)),
                        while_tap: Some(AnimationTarget::new().scale(0.95)),
                        transition: Some(
                            TransitionConfig::default()
                                .type_(TransitionType::Spring)
                                .stiffness(300.0)
                                .damping(20.0),
                        ),
                        onclick: toggle_animation,
                        if animate_progress() {
                            "Reset"
                        } else {
                            "Start"
                        }
                    }
                }
            }
        }
    }
}
