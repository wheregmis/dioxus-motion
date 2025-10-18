//! Store Migration Example
//!
//! This example demonstrates how to migrate from signal-based to store-based motion
//! for better performance through fine-grained reactivity.

use dioxus::prelude::*;
use dioxus_motion::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            style: "padding: 20px; font-family: Arial, sans-serif;",

            h1 { "Store-based Motion Migration Demo" }

            p {
                "This demo shows the new store-based API for fine-grained reactivity. "
                "Each component only re-renders when its specific subscribed data changes."
            }

            // Main animation demo
            AnimationDemo {}

            hr { style: "margin: 30px 0;" }

            // Performance comparison
            PerformanceComparison {}
        }
    }
}

#[component]
fn AnimationDemo() -> Element {
    // NEW: Use store-based motion for fine-grained reactivity
    let motion = use_motion_store(0.0);

    rsx! {
        div {
            style: "border: 2px solid #4CAF50; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "Store-based Motion (Recommended)" }
            p { "Uses use_motion_store_f32() for fine-grained reactivity and better performance." }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin: 20px 0;",

                // Animated element - only re-renders when position changes
                AnimatedElement { motion: motion.store().clone() }

                // Control panel - only re-renders when running state or target changes
                ControlPanel { motion: motion.store().clone() }
            }

            // Debug info - only re-renders when debug data changes
            DebugInfo { motion: motion.store().clone() }
        }
    }
}

#[component]
fn AnimatedElement(motion: Store<MotionStore<f32>>) -> Element {
    let current = motion.current(); // Fine-grained subscription to position only

    rsx! {
        div {
            h4 { "Animated Element" }
            p { style: "font-size: 12px; color: #666;", "Only re-renders when position changes" }

            div {
                style: "
                    width: 80px; 
                    height: 80px; 
                    background: linear-gradient(45deg, #4CAF50, #45a049);
                    border-radius: 8px;
                    transform: translateX({current()}px);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                ",
                "🎯"
            }

            p { style: "margin-top: 10px; font-family: monospace;",
                "Position: {current():.1}px"
            }
        }
    }
}

#[component]
fn ControlPanel(motion: Store<MotionStore<f32>>) -> Element {
    let is_running = motion.running(); // Subscribe only to running state
    let target = motion.target(); // Subscribe only to target value

    rsx! {
        div {
            h4 { "Control Panel" }
            p { style: "font-size: 12px; color: #666;", "Only re-renders when controls change" }

            div { style: "display: flex; flex-direction: column; gap: 10px;",
                button {
                    disabled: is_running(),
                    onclick: move |_| {
                        // Direct store manipulation
                        motion.target().set(200.0);
                        motion.running().set(true);
                    },
                    "Animate to 200px"
                }

                button {
                    disabled: is_running(),
                    onclick: move |_| {
                        // Direct store manipulation
                        motion.target().set(0.0);
                        motion.running().set(true);
                    },
                    "Animate to 0px"
                }

                button {
                    onclick: move |_| {
                        // Direct store manipulation
                        motion.running().set(false);
                        motion.velocity().set(0.0);
                    },
                    "Stop Animation"
                }

                // Direct manipulation
                button {
                    onclick: move |_| {
                        motion.current().set(100.0);  // Jump directly to value
                        motion.running().set(false);   // Stop any animation
                    },
                    "Jump to 100px"
                }
            }

            p { style: "margin-top: 10px; font-family: monospace;",
                "Target: {target():.1}px"
            }
            p { style: "font-family: monospace;",
                {
                    let status = if is_running() { "Running" } else { "Stopped" };
                    format!("Status: {}", status)
                }
            }
        }
    }
}

#[component]
fn DebugInfo(motion: Store<MotionStore<f32>>) -> Element {
    let velocity = motion.velocity(); // Subscribe only to velocity
    let elapsed = motion.elapsed(); // Subscribe only to elapsed time
    let current_loop = motion.current_loop(); // Subscribe only to loop count

    rsx! {
        div {
            style: "margin-top: 20px; padding: 15px; background: #f5f5f5; border-radius: 4px;",

            h4 { "Debug Information" }
            p { style: "font-size: 12px; color: #666;", "Only re-renders when debug data changes" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 15px; font-family: monospace; font-size: 14px;",
                div {
                    strong { "Velocity:" }
                    br {}
                    "{velocity():.2} px/s"
                }
                div {
                    strong { "Elapsed:" }
                    br {}
                    "{elapsed().as_millis()}ms"
                }
                div {
                    strong { "Loop Count:" }
                    br {}
                    "{current_loop()}"
                }
            }
        }
    }
}

#[component]
fn PerformanceComparison() -> Element {
    rsx! {
        div {
            style: "border: 2px solid #2196F3; border-radius: 8px; padding: 20px;",

            h3 { "Performance Benefits" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 20px;",

                div {
                    h4 { style: "color: #f44336;", "❌ Old Signal-based Approach" }
                    pre { style: "background: #ffebee; padding: 10px; border-radius: 4px; font-size: 12px;",
                        "let motion = use_motion(0.0f32);\n\n"
                        "// ALL components re-render when\n"
                        "// ANY motion property changes:\n"
                        "// - Position updates\n"
                        "// - Velocity changes  \n"
                        "// - Running state changes\n"
                        "// - Target changes\n"
                        "// - Internal state updates"
                    }
                }

                div {
                    h4 { style: "color: #4CAF50;", "✅ New Store-based Approach" }
                    pre { style: "background: #e8f5e8; padding: 10px; border-radius: 4px; font-size: 12px;",
                        "let motion = use_motion_store_f32(0.0);\n\n"
                        "// Components subscribe only to\n"
                        "// what they actually need:\n"
                        "let current = motion.current();\n"
                        "let running = motion.running();\n"
                        "let velocity = motion.velocity();\n\n"
                        "// Fine-grained reactivity!"
                    }
                }
            }

            div { style: "margin-top: 15px; padding: 15px; background: #e3f2fd; border-radius: 4px;",
                h4 { "🚀 Performance Impact" }
                ul {
                    li { "Components only re-render when their subscribed data changes" }
                    li { "Eliminates unnecessary re-renders in complex UIs" }
                    li { "Better performance for animation-heavy applications" }
                    li { "Easier debugging with targeted subscriptions" }
                    li { "Direct manipulation capabilities for advanced use cases" }
                }
            }
        }
    }
}
