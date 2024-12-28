use dioxus::prelude::*;

use dioxus_motion::{
    animation::Tween,
    motion::Motion,
    platform::TimeProvider,
    prelude::*,
    spring::Spring,
    use_transform_motion::{use_transform_animation, Transform},
    use_value_animation, Time,
};
use easer::functions::Easing;
use example_projects::components::{Navbar, TransformAnimationShowcase, ValueAnimationShowcase};

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        ShowcaseGallery {}
    }
}
#[component]
pub fn ShowcaseGallery() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Navbar {}

            // Main content with padding-top to account for fixed navbar
            div { class: "min-h-screen bg-gray-100 pt-20 p-8",
                h1 { class: "text-4xl font-bold text-center text-gray-800 mb-12", "Animation Gallery" }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 max-w-6xl mx-auto",
                    ValueAnimationShowcase {}
                    TransformAnimationShowcase {}
                    div { class: "group perspective-1000",
                        ProgressBar { title: "Okay Loading ..." }
                    }
                    div { class: "flex items-center justify-center",
                        MorphingShape { shapes: vec!["square", "triangle"], duration: 3.0 }
                    }

                // div { class: "flex items-center justify-center",
                //     BouncingText { text: "Dioxus", delay: 0.1 }
                // }
                }
            }

            // Footer
            footer { class: "bg-gray-800 text-white py-8 mt-auto",
                div { class: "max-w-6xl mx-auto px-4 text-center",
                    p { class: "text-sm opacity-75", "© 2024 Sabin Regmi. No Rights Reserved." }
                    p { class: "text-xs mt-2 opacity-50", "Built with ❤️ using Dioxus" }
                }
            }
        }
    }
}

#[component]
fn ProgressBar(title: &'static str) -> Element {
    let mut progress =
        use_value_animation(Motion::new(0.0).to(100.0).duration(Duration::from_secs(10)));

    use_effect(move || {
        progress.loop_animation();
    });

    rsx! {
        div { class: "w-full p-6 bg-white rounded-xl shadow-lg",
            // Title and percentage
            div { class: "flex justify-between items-center mb-4",
                span { class: "text-lg font-semibold text-gray-700", "{title}" }
                span { class: "text-sm font-medium text-blue-600", "{progress.value() as i32}%" }
            }

            // Progress bar
            div { class: "w-full h-4 bg-gray-200 rounded-full overflow-hidden",
                div {
                    class: "h-full w-full bg-gradient-to-r from-blue-500 to-purple-600 transition-all duration-300",
                    style: "width: {progress.value() as i32}%",
                }
            }
        }
    }
}

// // #[component]
// // fn BouncingLetter(letter: char, delay: f32) -> Element {
// //     let mut y_pos = use_value_animation(Motion::new(0.0).to(-20.0).spring(Spring {
// //         stiffness: 400.0,
// //         damping: 10.0,
// //         mass: 1.0,
// //         velocity: 0.0,
// //     }));

// //     let mut scale = use_value_animation(Motion::new(1.0).to(1.2).spring(Spring {
// //         stiffness: 400.0,
// //         damping: 10.0,
// //         mass: 1.0,
// //         velocity: 0.0,
// //     }));

// //     rsx! {
// //         span {
// //             class: "text-4xl font-bold text-indigo-600",
// //             style: "transform: translateY({y_pos.value()}px) scale({scale.value()});
// //                    animation-delay: {delay}s;",
// //             onmounted: move |_| {
// //                 y_pos.start();
// //                 scale.start();
// //             },
// //             "{letter}"
// //         }
// //     }
// // }

// #[component]
// fn BouncingText(text: &'static str, delay: f32) -> Element {
//     rsx! {
//         div { class: "flex space-x-1",
//             {
//                 text.chars()
//                     .enumerate()
//                     .map(|(i, char)| {
//                         rsx! {
//                             BouncingLetter { letter: char, delay: i as f32 * delay }
//                         }
//                     })
//             }
//         }
//     }
// }

#[component]
fn MorphingShape(shapes: Vec<&'static str>, duration: f32) -> Element {
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

    rsx! {
        div {
            class: "w-32 h-32 bg-gradient-to-r from-pink-500 to-orange-500 transition-all duration-500",
            style: "clip-path: {shape_paths[*current_shape.read()]}; {transform.style()}",
            onmounted: move |_| transform.start(),
        }
    }
}
