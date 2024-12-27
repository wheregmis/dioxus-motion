use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn TimerComponent() -> Element {
    let mut timer =
        use_value_animation(Motion::new(10.0).to(0.0).duration(Duration::from_secs(10)));
    let button_style =
        "padding: 10px 20px; color: white; border: none; border-radius: 4px; margin: 0 5px;";

    // Get state color based on animation state
    let state_color = match timer.state() {
        AnimationState::Idle => "#9E9E9E",
        AnimationState::Running => "#4CAF50",
        AnimationState::Completed => "#2196F3",
    };

    rsx! {
        div {
            // Timer Display
            div { style: "text-align: center; font-size: 48px; margin-bottom: 20px;",
                "Timer: {timer.value().ceil() as i32}"
            }

            // Animation State Display
            div { style: "text-align: center; margin-bottom: 20px; font-size: 18px; color: {state_color}; font-weight: bold;",
                match timer.state() {
                    AnimationState::Idle => "Idle",
                    AnimationState::Running => "Running",
                    AnimationState::Completed => "Completed",
                }
            }

            // Timer Controls
            div { style: "display: flex; gap: 10px; justify-content: center; margin-bottom: 40px;",
                button {
                    onclick: move |_| timer.start(),
                    style: "{button_style} background: #4CAF50;",
                    "Start"
                }
                button {
                    onclick: move |_| timer.stop(),
                    style: "{button_style} background: #f44336;",
                    "Stop"
                }
                button {
                    onclick: move |_| timer.reset(),
                    style: "{button_style} background: #9C27B0;",
                    "Reset"
                }

                button {
                    onclick: move |_| timer.reverse(),
                    style: "{button_style} background: #FF9800;",
                    "Reverse"
                }

                button {
                    onclick: move |_| timer.loop_animation(),
                    style: "{button_style} background: #2196F3;",
                    "Loop"
                }

            }
        }
    }
}
