use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn SpringBoxComponent() -> Element {
    let mut spring_x = use_value_animation(Motion::new(0.0).to(300.0).spring(Spring {
        stiffness: 80.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
    }));

    let mut spring_scale = use_value_animation(Motion::new(1.0).to(1.5).spring(Spring {
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
                button {
                    onclick: move |_| {
                        spring_x.reverse();
                        spring_scale.reverse();
                    },
                    style: "padding: 10px 20px; background: #9C27B0; color: white; border: none; border-radius: 4px;",
                    "Reverse Box"
                }
            }
        }
    }
}
