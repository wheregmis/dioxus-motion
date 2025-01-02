use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[derive(PartialEq, Copy, Clone)]
struct ShapeConfig {
    path: &'static str,
    rotation: f32,
    scale: f32,
}

#[component]
pub fn MorphingShape(shapes: Vec<&'static str>, duration: f32) -> Element {
    let mut current_shape = use_signal(|| 0);
    let mut transform = use_animation(Transform::default());

    let shape_configs = [
        ShapeConfig {
            path: "polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)", // Diamond
            rotation: 0.0,
            scale: 1.0,
        },
        ShapeConfig {
            path: "polygon(20% 0%, 80% 0%, 100% 100%, 0% 100%)", // Trapezoid
            rotation: 180.0,
            scale: 1.2,
        },
        ShapeConfig {
            path: "circle(50% at 50% 50%)", // Circle
            rotation: 360.0,
            scale: 0.8,
        },
    ];

    use_effect(move || {
        spawn(async move {
            loop {
                Time::delay(Duration::from_secs_f32(duration)).await;

                let next = if *current_shape.read() + 1 >= shape_configs.len() {
                    0
                } else {
                    *current_shape.read() + 1
                };

                current_shape.set(next);
                transform.animate_to(
                    Transform {
                        rotate: shape_configs[next].rotation,
                        scale: shape_configs[next].scale,
                        ..Default::default()
                    },
                    AnimationMode::Spring(Spring {
                        stiffness: 100.0,
                        damping: 10.0,
                        mass: 1.0,
                        ..Default::default()
                    }),
                );
            }
        });
    });

    rsx! {
        div {
            class: "w-32 h-32 bg-gradient-to-r from-pink-500 to-orange-500 transition-[clip-path] duration-500 ease-in-out hover:from-purple-500 hover:to-blue-500",
            style: "clip-path: {shape_configs[*current_shape.read()].path};
                   transform: rotate({transform.get_value().rotate}deg) scale({transform.get_value().scale})",
        }
    }
}
