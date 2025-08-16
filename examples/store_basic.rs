//! Basic example demonstrating the store-based Motion API
//!
//! This example shows how to use the new store-based API for fine-grained reactivity.
//! Multiple components can subscribe to different aspects of the animation state
//! without causing unnecessary re-renders.

use dioxus::prelude::*;
use dioxus_motion::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    // Store-based motion for fine-grained reactivity
    let motion = use_motion_store(0.0);

    rsx! {
        div {
            style: "padding: 20px; font-family: Arial, sans-serif;",

            h1 { "Store-based Motion API Demo" }

            p { "This demo shows fine-grained reactivity with the new store-based API." }

            // Animated element - only re-renders when current value changes
            AnimatedElement { motion }

            // Control panel - only re-renders when running state changes
            ControlPanel { motion }

            // Value display - only re-renders when current value changes
            ValueDisplay { motion }

            // Progress display - only re-renders when relevant state changes
            ProgressDisplay { motion }
        }
    }
}

#[component]
fn AnimatedElement(motion: Store<MotionStore<f32>>) -> Element {
    // This component only subscribes to the current animated value
    // It won't re-render when running state, elapsed time, or other fields change
    let value = motion.current();

    rsx! {
        div {
            style: "
                width: 100px; 
                height: 100px; 
                background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                margin: 20px 0;
                border-radius: 50%;
                transform: translateX({value}px);
            ",
            div {
                style: "
                    display: flex; 
                    align-items: center; 
                    justify-content: center; 
                    height: 100%; 
                    color: white; 
                    font-weight: bold;
                ",
                "🚀"
            }
        }
    }
}

#[component]
fn ControlPanel(motion: Store<MotionStore<f32>>) -> Element {
    // This component only subscribes to the running state
    // It won't re-render when the animated value changes every frame
    let is_running = motion.running();
    let current = motion.current();
    let target = motion.target();

    rsx! {
        div {
            style: "margin: 20px 0; padding: 20px; border: 1px solid #ddd; border-radius: 8px;",

            h3 { "Animation Controls" }

            div { style: "margin: 10px 0;",
                span {
                    style: if is_running() { "color: green;" } else { "color: red;" },
                    if is_running() { "Status: Running" } else { "Status: Stopped" }
                }
                if (current() - target()).abs() < 0.01 {
                    span { style: "margin-left: 10px; color: blue;", "✓ At Target" }
                }
            }

            div { style: "margin: 10px 0;",
                button {
                    onclick: move |_| {
                        motion.target().set(200.0);
                        motion.running().set(true);
                    },
                    disabled: is_running(),
                    style: "margin-right: 10px; padding: 8px 16px;",
                    "Move Right"
                }
                button {
                    onclick: move |_| {
                        motion.target().set(0.0);
                        motion.running().set(true);
                    },
                    disabled: is_running(),
                    style: "margin-right: 10px; padding: 8px 16px;",
                    "Move Left"
                }
                button {
                    onclick: move |_| motion.running().set(false),
                    disabled: !is_running(),
                    style: "margin-right: 10px; padding: 8px 16px; background: #ff6b6b; color: white;",
                    "Stop"
                }
                button {
                    onclick: move |_| {
                        let initial = motion.initial()();
                        motion.current().set(initial);
                        motion.target().set(initial);
                        motion.running().set(false);
                    },
                    style: "padding: 8px 16px;",
                    "Reset"
                }
            }
        }
    }
}

#[component]
fn ValueDisplay(motion: Store<MotionStore<f32>>) -> Element {
    // This component subscribes to multiple store fields but only the ones it needs
    let current = motion.current();
    let target = motion.target();
    let velocity = motion.velocity();

    rsx! {
        div {
            style: "margin: 20px 0; padding: 20px; background: #f5f5f5; border-radius: 8px;",

            h3 { "Animation Values" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px;",
                div {
                    strong { "Current: " }
                    span { "{current():.2}" }
                }
                div {
                    strong { "Target: " }
                    span { "{target():.2}" }
                }
                div {
                    strong { "Velocity: " }
                    span { "{velocity().abs():.2}" }
                }
            }
        }
    }
}

#[component]
fn ProgressDisplay(motion: Store<MotionStore<f32>>) -> Element {
    // This component shows internal animation state
    let elapsed = motion.elapsed();
    let current_loop = motion.current_loop();

    rsx! {
        div {
            style: "margin: 20px 0; padding: 20px; background: #e8f4f8; border-radius: 8px;",

            h3 { "Animation Progress" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 20px;",
                div {
                    strong { "Elapsed: " }
                    span { "{elapsed().as_secs_f32():.2}s" }
                }
                div {
                    strong { "Loop: " }
                    span { "{current_loop()}" }
                }
            }
        }
    }
}
