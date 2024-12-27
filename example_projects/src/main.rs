use dioxus::prelude::*;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};
use easer::functions::{Bounce, Easing};
use example_projects::components::{SpringBoxComponent, TimerComponent};

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div { style: "position: relative; width: 100%; height: 100vh; background: #f0f0f0;",
            TestElementBinding {}
        }
    }
}

#[component]
fn TestElementBinding() -> Element {
    let mut transform = use_transform_animation(
        Transform::default(),
        Transform {
            x: 200.0,
            y: 100.0,
            scale: 1.5,
            rotate: 180.0,
            opacity: 0.5,
        },
        AnimationMode::Spring(Spring {
            stiffness: 100.0,
            damping: 40.0,
            mass: 1.0,
            velocity: 0.0,
        }),
    );

    rsx! {
        div { class: "container", style: "padding: 20px;",

            // Animation controls
            div { style: "margin-bottom: 20px;",
                button { onclick: move |_| transform.start(), "Animate" }
                button { onclick: move |_| transform.reset(), "Reset" }
                button { onclick: move |_| transform.reverse(), "Reverse" }
            }

            // Animated element
            div {
                onmounted: move |_| transform.start(),
                style: "{transform.style()} width: 10px; height: 10px; background: #2196F3; border-radius: 8px;",
                "Animated Box"
            }
        }
    }
}

// fn app() -> Element {
//     rsx! {
//         div { class: "container", style: "padding: 20px; font-family: Arial;",
//             // Title
//             h1 { style: "text-align: center; color: #333; margin-bottom: 40px;",
//                 "Dioxus Motion Examples"
//             }

//             // Tween Animation Section
//             div {
//                 h2 { style: "text-align: center; color: #2196F3; margin-bottom: 20px;",
//                     "Tween Animation - Timer Example"
//                 }
//                 TimerComponent {}
//             }

//             // Spacing between sections
//             div { style: "margin: 40px 0;" }

//             // Spring Animation Section
//             div {
//                 h2 { style: "text-align: center; color: #4CAF50; margin-bottom: 20px;",
//                     "Spring Animation - Interactive Box"
//                 }
//                 SpringBoxComponent {}
//             }
//         }
//     }
// }
