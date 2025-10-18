//! Generic Store Example
//!
//! This example demonstrates the generic store support for different Animatable types.
//! Shows how the same use_motion_store API works with f32, Transform, and Color.

use dioxus::prelude::*;
use dioxus_motion::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            style: "padding: 20px; font-family: Arial, sans-serif;",

            h1 { "Generic Store-based Motion Demo" }

            p {
                "This demo shows the unified use_motion_store() API working with different types. "
                "All types get the same fine-grained reactivity benefits."
            }

            // f32 animation
            F32Animation {}

            hr { style: "margin: 30px 0;" }

            // Transform animation
            TransformAnimation {}

            hr { style: "margin: 30px 0;" }

            // Color animation
            ColorAnimation {}
        }
    }
}

#[component]
fn F32Animation() -> Element {
    let motion = use_motion_store(0.0f32);
    let current = motion.store().current();

    rsx! {
        div {
            style: "border: 2px solid #4CAF50; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "f32 Animation" }
            p { "Basic f32 value animation with fine-grained reactivity" }

            div {
                style: "
                    width: 60px; 
                    height: 60px; 
                    background: linear-gradient(45deg, #4CAF50, #45a049);
                    border-radius: 8px;
                    transform: translateX({current()}px);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    margin: 10px 0;
                ",
                "F32"
            }

            div { style: "display: flex; gap: 10px; margin: 10px 0;",
                button {
                    onclick: move |_| {
                        motion.store().target().set(150.0);
                        motion.store().running().set(true);
                    },
                    "Animate to 150px"
                }

                button {
                    onclick: move |_| {
                        motion.store().target().set(0.0);
                        motion.store().running().set(true);
                    },
                    "Reset"
                }
            }

            p { style: "font-family: monospace; font-size: 12px;",
                "Current: {current():.1}px | Target: {motion.store().target()():.1}px | Running: {motion.store().running()()}"
            }
        }
    }
}

#[component]
fn TransformAnimation() -> Element {
    let motion = use_motion_store(Transform::identity());
    let current = motion.store().current();

    rsx! {
        div {
            style: "border: 2px solid #2196F3; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "Transform Animation" }
            p { "2D transform animation (position, scale, rotation) with fine-grained reactivity" }

            div {
                style: "
                    width: 60px; 
                    height: 60px; 
                    background: linear-gradient(45deg, #2196F3, #1976D2);
                    border-radius: 8px;
                    transform: translateX({current().x}px) translateY({current().y}px) scale({current().scale}) rotate({current().rotation}rad);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    margin: 10px 0;
                ",
                "2D"
            }

            div { style: "display: flex; flex-wrap: wrap; gap: 10px; margin: 10px 0;",
                button {
                    onclick: move |_| {
                        motion.store().target().set(Transform::new(100.0, 50.0, 1.0, 0.0));
                        motion.store().running().set(true);
                    },
                    "Move"
                }

                button {
                    onclick: move |_| {
                        motion.store().target().set(Transform::new(0.0, 0.0, 1.5, 0.0));
                        motion.store().running().set(true);
                    },
                    "Scale"
                }

                button {
                    onclick: move |_| {
                        motion.store().target().set(Transform::new(0.0, 0.0, 1.0, std::f32::consts::PI / 4.0));
                        motion.store().running().set(true);
                    },
                    "Rotate"
                }

                button {
                    onclick: move |_| {
                        motion.store().target().set(Transform::identity());
                        motion.store().running().set(true);
                    },
                    "Reset"
                }
            }

            p { style: "font-family: monospace; font-size: 12px;",
                "X: {current().x:.1} | Y: {current().y:.1} | Scale: {current().scale:.2} | Rotation: {current().rotation:.2}rad"
            }
        }
    }
}

#[component]
fn ColorAnimation() -> Element {
    let motion = use_motion_store(Color::new(1.0, 0.0, 0.0, 1.0)); // Start with red
    let current = motion.store().current();

    rsx! {
        div {
            style: "border: 2px solid #FF9800; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "Color Animation" }
            p { "RGBA color animation with fine-grained reactivity" }

            div {
                style: "
                    width: 60px; 
                    height: 60px; 
                    background: rgba({(current().r * 255.0) as u8}, {(current().g * 255.0) as u8}, {(current().b * 255.0) as u8}, {current().a});
                    border-radius: 8px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    margin: 10px 0;
                    text-shadow: 1px 1px 2px rgba(0,0,0,0.5);
                ",
                "RGB"
            }

            div { style: "display: flex; flex-wrap: wrap; gap: 10px; margin: 10px 0;",
                button {
                    onclick: move |_| {
                        motion.store().target().set(Color::new(1.0, 0.0, 0.0, 1.0)); // Red
                        motion.store().running().set(true);
                    },
                    "Red"
                }

                button {
                    onclick: move |_| {
                        motion.store().target().set(Color::new(0.0, 1.0, 0.0, 1.0)); // Green
                        motion.store().running().set(true);
                    },
                    "Green"
                }

                button {
                    onclick: move |_| {
                        motion.store().target().set(Color::new(0.0, 0.0, 1.0, 1.0)); // Blue
                        motion.store().running().set(true);
                    },
                    "Blue"
                }

                button {
                    onclick: move |_| {
                        motion.store().target().set(Color::new(1.0, 1.0, 0.0, 1.0)); // Yellow
                        motion.store().running().set(true);
                    },
                    "Yellow"
                }

                button {
                    onclick: move |_| {
                        motion.store().target().set(Color::new(1.0, 0.0, 1.0, 1.0)); // Magenta
                        motion.store().running().set(true);
                    },
                    "Magenta"
                }
            }

            p { style: "font-family: monospace; font-size: 12px;",
                "R: {current().r:.2} | G: {current().g:.2} | B: {current().b:.2} | A: {current().a:.2}"
            }
        }
    }
}
