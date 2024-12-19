use dioxus::prelude::*;
use dioxus_motion::{use_motion, AnimationState, Duration, Motion, Spring};
use easer::functions::{Bounce, Easing};

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div { class: "container", style: "padding: 20px; font-family: Arial;",
            TimerComponent {}
            SpringBoxComponent {}
        }
    }
}

#[component]
fn TimerComponent() -> Element {
    let mut timer = use_motion(Motion::new(10.0).to(0.0).duration(Duration::from_secs(10)));
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
                    onclick: move |_| timer.resume(),
                    style: "{button_style} background: #2196F3;",
                    "Resume"
                }
                button {
                    onclick: move |_| timer.reset(),
                    style: "{button_style} background: #9C27B0;",
                    "Reset"
                }
            }
        }
    }
}

#[component]
fn SpringBoxComponent() -> Element {
    let mut spring_x = use_motion(Motion::new(0.0).to(300.0).spring(Spring {
        stiffness: 80.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
    }));

    let mut spring_scale = use_motion(Motion::new(1.0).to(1.5).spring(Spring {
        stiffness: 120.0,
        damping: 8.0,
        mass: 0.8,
        velocity: 0.0,
    }));

    rsx! {
        div {
            // Animated Box
            div { style: "position: relative; height: 200px; background: #f0f0f0; border-radius: 8px; overflow: hidden;",
                div { style: "position: absolute;
                           width: 100px; 
                           height: 100px; 
                           background: linear-gradient(45deg, #2196F3, #00BCD4); 
                           border-radius: 8px; 
                           top: 50%; 
                           transform: translateY(-50%) translateX({spring_x.value()}px) scale({spring_scale.value()})" }
            }

            // Spring Animation Controls
            div { style: "display: flex; gap: 10px; justify-content: center; margin-top: 20px;",
                button {
                    onclick: move |_| {
                        spring_x.start();
                        spring_scale.start();
                    },
                    style: "padding: 10px 20px; background: #2196F3; color: white; border: none; border-radius: 4px;",
                    "Animate Box"
                }
                button {
                    onclick: move |_| {
                        spring_x.reset();
                        spring_scale.reset();
                    },
                    style: "padding: 10px 20px; background: #9C27B0; color: white; border: none; border-radius: 4px;",
                    "Reset Box"
                }
            }
        }
    }
}
