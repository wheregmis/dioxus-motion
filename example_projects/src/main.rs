use dioxus::prelude::*;
use dioxus_motion::{
    animation::Tween,
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};
use easer::functions::{Bounce, Easing};
use example_projects::components::{
    motion_div::DragConstraints, MotionDiv, SpringBoxComponent, TimerComponent,
};

fn main() {
    launch(app);
}

#[component]
fn BasicTransformExample() -> Element {
    let mut transform = use_transform_animation(
        Transform::default(),
        Transform {
            x: 0.0,
            y: 0.0,
            scale: 2.2,
            rotate: 360.0,
            opacity: 1.0,
        },
        AnimationMode::Tween(Tween {
            duration: dioxus_motion::Duration::from_secs(3),
            easing: easer::functions::Linear::ease_in,
        }),
    );

    rsx! {
        div { style: "width: 100%;
                   height: 100vh; 
                   display: flex; 
                   justify-content: center; 
                   align-items: center; 
                   background: #f0f0f0;",

            div {
                style: "width: 200px;
                       height: 200px;
                       background: #2196F3;
                       border-radius: 8px;
                       display: flex;
                       justify-content: center;
                       align-items: center;
                       color: white;
                       font-size: 20px;
                       transform: translate({transform.x()}px, {transform.y()}px)
                                scale({transform.scale()})
                                rotate({transform.rotate()}deg);
                       opacity: {transform.opacity()};",
                onmounted: move |_| transform.start(),
                "Basic Animation"
            }
        }
    }
}

fn app() -> Element {
    rsx! {
        div { class: "container", style: "padding: 20px; font-family: Arial;",
            // Title
            h1 { style: "text-align: center; color: #333; margin-bottom: 40px;",
                "Dioxus Motion Examples"
            }

            // Tween Animation Section
            div {
                h2 { style: "text-align: center; color: #2196F3; margin-bottom: 20px;",
                    "Tween Animation - Timer Example"
                }
                TimerComponent {}
            }

            // Spacing between sections
            div { style: "margin: 40px 0;" }

            // Spring Animation Section
            div {
                h2 { style: "text-align: center; color: #4CAF50; margin-bottom: 20px;",
                    "Spring Animation - Interactive Box"
                }
                SpringBoxComponent {}
            }

            // Spacing between sections
            div { style: "margin: 40px 0;" }

            // Transform Animation Section
            div {
                h2 { style: "text-align: center; color: #9C27B0; margin-bottom: 20px;",
                    "Transform Animation - Basic Example"
                }
                BasicTransformExample {}
            }
        }
    }
}
