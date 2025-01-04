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
    let mut transform = use_motion(Transform::identity());

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
        transform.animate_to(
            Transform {
                rotation: 360.0,
                scale: 1.2,
                x: 0.0,
                y: 0.0,
            },
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 50.0, // Reduced for smoother motion
                damping: 8.0,    // Less damping for more organic motion
                mass: 0.8,       // Lighter mass for faster response
                velocity: 0.5,   // Initial velocity for continuous motion
            }))
            .with_loop(LoopMode::Infinite), // Make animation continuous
        );

        // Start shape transition loop
        spawn(async move {
            loop {
                Time::delay(Duration::from_secs_f32(duration)).await;
                let next = (*current_shape.read() + 1) % shape_configs.len();
                current_shape.set(next);
            }
        });
    });

    rsx! {
        div { class: "w-32 h-32 relative transition-all duration-300",
            div {
                class: "absolute inset-0 bg-gradient-to-r from-pink-500 to-orange-500
                       hover:from-purple-500 hover:to-blue-500 rounded-lg",
                style: "clip-path: {shape_configs[*current_shape.read()].path};
                       transform: rotate({transform.get_value().rotation}deg) 
                                scale({transform.get_value().scale});
                       transition: clip-path 0.5s ease-in-out;",
            }
        }
    }
}
