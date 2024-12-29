use dioxus::prelude::*;
use dioxus_motion::platform::TimeProvider;
use dioxus_motion::Time;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};

#[component]
pub fn MorphingShape(shapes: Vec<&'static str>, duration: f32) -> Element {
    let mut current_shape = use_signal(|| 0);
    let shape_paths = [
        "polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)",
        "polygon(20% 0%, 80% 0%, 100% 100%, 0% 100%)",
    ];

    let mut transform = use_transform_animation(
        Transform::default(),
        Transform {
            rotate: 360.0,
            scale: 1.2,
            ..Default::default()
        },
        AnimationMode::Spring(Spring {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }),
    );

    use_effect(move || {
        let interval = Duration::from_secs_f32(duration);
        spawn(async move {
            loop {
                Time::delay(interval).await;
                let next = if *current_shape.read() + 1 >= shape_paths.len() {
                    0
                } else {
                    *current_shape.read() + 1
                };
                current_shape.set(next);
                transform.start();
            }
        });
    });

    use_drop(move || {
        transform.stop();
    });

    rsx! {
        div {
            class: "w-32 h-32 bg-gradient-to-r from-pink-500 to-orange-500 transition-all duration-500",
            style: "clip-path: {shape_paths[*current_shape.read()]}; {transform.style()}",
            onmounted: move |_| transform.start(),
        }
    }
}
