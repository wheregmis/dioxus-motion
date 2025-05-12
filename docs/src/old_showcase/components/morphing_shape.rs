use dioxus::prelude::*;
use dioxus_motion::core::spring::Spring;
use dioxus_motion::prelude::transform::Transform;
use dioxus_motion::prelude::*;

#[derive(PartialEq, Copy, Clone)]
struct ShapeConfig {
    path: &'static str,
    rotation: f32,
    scale: f32,
    color_from: &'static str,
    color_to: &'static str,
}

#[component]
pub fn MorphingShape(shapes: Vec<&'static str>, duration: f32) -> Element {
    let mut current_shape = use_signal(|| 0);
    let mut transform = use_motion(Transform::identity());
    let mut scale_pulse = use_motion(1.0f32);

    let shape_configs = [
        ShapeConfig {
            path: "polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)", // Diamond
            rotation: 0.0,
            scale: 1.0,
            color_from: "blue-200", // Ice blue to aqua for diamond's crystalline look
            color_to: "cyan-200",
        },
        ShapeConfig {
            path: "polygon(25% 0%, 75% 0%, 100% 50%, 75% 100%, 25% 100%, 0% 50%)", // Hexagon
            rotation: 180.0,
            scale: 1.2,
            color_from: "amber-200", // Honey colors for hexagon (beehive inspired)
            color_to: "yellow-200",
        },
        ShapeConfig {
            path: "circle(50% at 50% 50%)", // Circle
            rotation: 360.0,
            scale: 0.9,
            color_from: "rose-200", // Soft pinks for smooth circular form
            color_to: "pink-200",
        },
        ShapeConfig {
            path: "polygon(0% 15%, 15% 15%, 15% 0%, 85% 0%, 85% 15%, 100% 15%, 100% 85%, 85% 85%, 85% 100%, 15% 100%, 15% 85%, 0% 85%)", // Cross
            rotation: 45.0,
            scale: 1.1,
            color_from: "emerald-200", // Nature-inspired greens for the cross
            color_to: "lime-200",
        },
        ShapeConfig {
            path: "polygon(50% 0%, 100% 38%, 82% 100%, 18% 100%, 0% 38%)", // Pentagon
            rotation: 270.0,
            scale: 1.15,
            color_from: "violet-200", // Royal purple tones for pentagon
            color_to: "purple-200",
        },
        ShapeConfig {
            path: "polygon(20% 0%, 80% 0%, 100% 20%, 100% 80%, 80% 100%, 20% 100%, 0% 80%, 0% 20%)", // Octagon
            rotation: 90.0,
            scale: 1.05,
            color_from: "orange-200", // Warm sunset colors for octagon
            color_to: "red-200",
        },
        ShapeConfig {
            path: "polygon(50% 0%, 61% 35%, 98% 35%, 68% 57%, 79% 91%, 50% 70%, 21% 91%, 32% 57%, 2% 35%, 39% 35%)", // Star
            rotation: 135.0,
            scale: 1.25,
            color_from: "sky-200", // Sky and sea colors for star
            color_to: "indigo-200",
        },
    ];

    use_effect(move || {
        // Main rotation and scale animation
        transform.animate_to(
            Transform {
                rotation: 360.0,
                scale: 1.2,
                x: 0.0,
                y: 0.0,
            },
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 35.0, // Reduced for more fluid motion
                damping: 5.0,    // Lower damping for organic movement
                mass: 0.6,       // Lighter mass for faster response
                velocity: 0.8,   // Increased initial velocity
            }))
            .with_loop(LoopMode::Infinite),
        );

        // Additional scale pulse animation
        scale_pulse.animate_to(
            1.15,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 25.0,
                damping: 3.0,
                mass: 0.5,
                velocity: 0.5,
            }))
            .with_loop(LoopMode::Infinite),
        );

        // Shape transition loop
        spawn(async move {
            loop {
                Time::delay(Duration::from_secs_f32(duration)).await;
                let next = (*current_shape.read() + 1) % shape_configs.len();
                current_shape.set(next);
            }
        });
    });

    let current_config = &shape_configs[*current_shape.read()];

    rsx! {
        div { class: "w-32 h-32 relative transition-all duration-300",
            div {
                class: "absolute inset-0 rounded-lg shadow-lg backdrop-blur-xs",
                class: "absolute inset-0 bg-linear-to-r from-pink-500 to-orange-500
                       hover:from-purple-500 hover:to-blue-500 rounded-lg",
                style: "clip-path: {current_config.path};
                       transform: rotate({transform.get_value().rotation}deg)
                                scale({transform.get_value().scale * scale_pulse.get_value()});
                       transition: clip-path 0.8s cubic-bezier(0.4, 0, 0.2, 1);
                       filter: brightness(1.2) contrast(1.1) saturate(1.2);",
                // Lighter inner glow effect
                div {
                    class: "absolute inset-0 bg-white/30 rounded-lg",
                    style: "mix-blend-mode: soft-light;",
                }
            }
        }
    }
}
