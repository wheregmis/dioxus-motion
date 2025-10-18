//! Store-based Sequence Animation Example
//!
//! This example demonstrates sequence animations using the unified store-based API
//! for chaining multiple animation steps with fine-grained reactivity.

use dioxus::prelude::*;
use dioxus_motion::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            style: "padding: 20px; font-family: Arial, sans-serif;",

            h1 { "Unified Store Sequence Animations" }

            p {
                "This demo shows sequence animations with the unified store API. "
                "Each sequence chains multiple animation steps together "
                "with different timing and easing for complex motion patterns."
            }

            // f32 sequence animation
            F32SequenceDemo {}

            hr { style: "margin: 30px 0;" }

            // Transform sequence animation
            TransformSequenceDemo {}

            hr { style: "margin: 30px 0;" }

            // Color sequence animation
            ColorSequenceDemo {}
        }
    }
}

#[component]
fn F32SequenceDemo() -> Element {
    let mut motion = use_motion_store(0.0f32);
    let current = motion.store().current();
    let is_running = motion.store().running();
    let current_step = motion.store().current_sequence_step();

    let start_animation = move |_| {
        // Create a sequence of animations with different configurations
        let sequence = AnimationSequence::new()
            .then(100.0, AnimationConfig::spring())
            .then(200.0, AnimationConfig::tween())
            .then(150.0, AnimationConfig::spring())
            .then(50.0, AnimationConfig::tween())
            .then(0.0, AnimationConfig::spring());

        motion.animate_sequence(sequence);
    };

    rsx! {
        div {
            style: "border: 2px solid #4CAF50; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "f32 Sequence Animation" }
            p { "Multi-step animation sequence with spring and tween transitions" }

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
                    margin: 20px 0;
                ",
                "📦"
            }

            div { style: "display: flex; gap: 10px; margin: 10px 0;",
                button {
                    onclick: start_animation,
                    disabled: is_running(),
                    style: if is_running() { "opacity: 0.5;" } else { "" },
                    "Start Sequence"
                }

                button {
                    onclick: move |_| {
                        motion.reset();
                    },
                    "Reset"
                }
            }

            p { style: "font-family: monospace; font-size: 12px;",
                "Position: {current():.1}px | Step: {current_step()} | Running: {is_running()}"
            }

            // Step indicator
            div { style: "display: flex; gap: 5px; margin: 10px 0;",
                for i in 0..5 {
                    {
                        let bg_color = if current_step() == i { "#4CAF50" } else { "#ddd" };
                        rsx! {
                            div {
                                style: "
                                    width: 30px; 
                                    height: 20px; 
                                    background: {bg_color}; 
                                    border-radius: 4px;
                                    display: flex;
                                    align-items: center;
                                    justify-content: center;
                                    font-size: 10px;
                                    color: white;
                                ",
                                "{i + 1}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TransformSequenceDemo() -> Element {
    let mut motion = use_motion_store(Transform::identity());
    let current = motion.store().current();
    let is_running = motion.store().running();
    let current_step = motion.store().current_sequence_step();

    let start_animation = move |_| {
        // Create a complex transform sequence
        let sequence = AnimationSequence::new()
            .then(
                Transform::new(80.0, 0.0, 1.0, 0.0),
                AnimationConfig::spring(),
            )
            .then(
                Transform::new(80.0, 60.0, 1.5, std::f32::consts::PI / 2.0),
                AnimationConfig::tween(),
            )
            .then(
                Transform::new(0.0, 60.0, 0.8, std::f32::consts::PI),
                AnimationConfig::spring(),
            )
            .then(
                Transform::new(0.0, 0.0, 1.2, std::f32::consts::PI * 1.5),
                AnimationConfig::tween(),
            )
            .then(Transform::identity(), AnimationConfig::spring());

        motion.animate_sequence(sequence);
    };

    rsx! {
        div {
            style: "border: 2px solid #2196F3; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "Transform Sequence Animation" }
            p { "Sequential 2D transform path - each step uses different animation physics" }

            div {
                style: "
                    width: 60px; 
                    height: 60px; 
                    background: linear-gradient(45deg, #2196F3, #1976D2);
                    border-radius: 50%;
                    transform: translateX({current().x}px) translateY({current().y}px) scale({current().scale}) rotate({current().rotation}rad);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    margin: 20px 0;
                ",
                "🎮"
            }

            div { style: "display: flex; gap: 10px; margin: 10px 0;",
                button {
                    onclick: start_animation,
                    disabled: is_running(),
                    style: if is_running() { "opacity: 0.5;" } else { "" },
                    "Start Transform Sequence"
                }

                button {
                    onclick: move |_| {
                        motion.reset();
                    },
                    "Reset"
                }
            }

            p { style: "font-family: monospace; font-size: 12px;",
                "X: {current().x:.1} | Y: {current().y:.1} | Scale: {current().scale:.2} | Rotation: {current().rotation:.2}rad"
            }

            // Step indicator with descriptions
            div { style: "margin: 10px 0;",
                for (i, desc) in ["Right", "Down+Rotate", "Left+Scale", "Up+Spin", "Home"].iter().enumerate() {
                    {
                        let is_current = current_step() == i as u8;
                        let bg_color = if is_current { "#2196F3" } else { "#ddd" };
                        let text_color = if is_current { "white" } else { "black" };
                        rsx! {
                            div {
                                style: "
                                    display: inline-block;
                                    margin: 2px;
                                    padding: 4px 8px;
                                    background: {bg_color}; 
                                    color: {text_color};
                                    border-radius: 4px;
                                    font-size: 10px;
                                ",
                                "{i + 1}: {desc}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ColorSequenceDemo() -> Element {
    let mut motion = use_motion_store(Color::new(1.0, 0.0, 0.0, 1.0));
    let current = motion.store().current();
    let is_running = motion.store().running();
    let current_step = motion.store().current_sequence_step();

    let start_animation = move |_| {
        // Create a color sequence through the spectrum
        let sequence = AnimationSequence::new()
            .then(
                Color::new(1.0, 0.5, 0.0, 1.0), // Orange
                AnimationConfig::spring(),
            )
            .then(
                Color::new(1.0, 1.0, 0.0, 1.0), // Yellow
                AnimationConfig::tween(),
            )
            .then(
                Color::new(0.0, 1.0, 0.0, 1.0), // Green
                AnimationConfig::spring(),
            )
            .then(
                Color::new(0.0, 0.0, 1.0, 1.0), // Blue
                AnimationConfig::tween(),
            )
            .then(
                Color::new(0.5, 0.0, 1.0, 1.0), // Purple
                AnimationConfig::spring(),
            )
            .then(
                Color::new(1.0, 0.0, 0.0, 1.0), // Back to Red
                AnimationConfig::tween(),
            );

        motion.animate_sequence(sequence);
    };

    rsx! {
        div {
            style: "border: 2px solid #FF9800; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "Color Sequence Animation" }
            p { "Sequential color transitions - each step to a new color" }

            div {
                style: "
                    width: 60px; 
                    height: 60px; 
                    background: rgba({(current().r * 255.0) as u8}, {(current().g * 255.0) as u8}, {(current().b * 255.0) as u8}, {current().a});
                    border-radius: 12px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    margin: 20px 0;
                    text-shadow: 1px 1px 2px rgba(0,0,0,0.7);
                ",
                "🎨"
            }

            div { style: "display: flex; gap: 10px; margin: 10px 0;",
                button {
                    onclick: start_animation,
                    disabled: is_running(),
                    style: if is_running() { "opacity: 0.5;" } else { "" },
                    "Start Color Sequence"
                }

                button {
                    onclick: move |_| {
                        motion.reset();
                    },
                    "Reset"
                }
            }

            p { style: "font-family: monospace; font-size: 12px;",
                "R: {current().r:.2} | G: {current().g:.2} | B: {current().b:.2} | Step: {current_step()}"
            }

            // Color step indicator
            div { style: "display: flex; gap: 3px; margin: 10px 0;",
                for (i, color) in [
                    "rgb(255,0,0)",     // Red
                    "rgb(255,128,0)",   // Orange
                    "rgb(255,255,0)",   // Yellow
                    "rgb(0,255,0)",     // Green
                    "rgb(0,0,255)",     // Blue
                    "rgb(128,0,255)",   // Purple
                ].iter().enumerate() {
                    {
                        let is_current = current_step() == i as u8;
                        let border = if is_current { "3px solid black" } else { "1px solid #ccc" };
                        rsx! {
                            div {
                                style: "
                                    width: 25px; 
                                    height: 25px; 
                                    background: {color}; 
                                    border: {border};
                                    border-radius: 50%;
                                ",
                            }
                        }
                    }
                }
            }
        }
    }
}
