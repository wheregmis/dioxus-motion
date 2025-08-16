//! Store-based Keyframe Animation Example
//!
//! This example demonstrates keyframe animations using the store-based API
//! for fine-grained reactivity and smooth complex motion paths.

use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_motion::KeyframeAnimation;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            style: "padding: 20px; font-family: Arial, sans-serif;",

            h1 { "Store-based Keyframe Animations" }

            p {
                "This demo shows keyframe animations with the store API. "
                "Each animation uses multiple keyframes with different easing functions "
                "and fine-grained reactivity."
            }

            // f32 keyframe animation
            F32KeyframeDemo {}

            hr { style: "margin: 30px 0;" }

            // Transform keyframe animation
            TransformKeyframeDemo {}

            hr { style: "margin: 30px 0;" }

            // Color keyframe animation
            ColorKeyframeDemo {}
        }
    }
}

#[component]
fn F32KeyframeDemo() -> Element {
    let (motion, mut animate_keyframes) = use_motion_store_with_keyframes(0.0f32);
    let current = motion.current();
    let is_running = motion.running();
    
    let start_animation = move |_| {
        // Create a complex keyframe animation with different easing
        let keyframes = KeyframeAnimation::new(Duration::from_millis(3000))
            .add_keyframe(0.0, 0.0, None).unwrap()                     // Start
            .add_keyframe(150.0, 0.25, Some(ease_out_quad)).unwrap()   // Fast start
            .add_keyframe(200.0, 0.5, Some(ease_in_out_back)).unwrap() // Overshoot
            .add_keyframe(50.0, 0.75, Some(ease_out_bounce)).unwrap()  // Bounce
            .add_keyframe(0.0, 1.0, Some(ease_in_cubic)).unwrap();     // Smooth return
        
        animate_keyframes(keyframes);
    };

    rsx! {
        div {
            style: "border: 2px solid #4CAF50; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "f32 Keyframe Animation" }
            p { "Multi-stage animation with different easing functions" }

            div {
                style: "
                    width: 60px; 
                    height: 60px; 
                    background: linear-gradient(45deg, #4CAF50, #45a049);
                    border-radius: 50%;
                    transform: translateX({current()}px);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    margin: 20px 0;
                ",
                "🚀"
            }

            div { style: "display: flex; gap: 10px; margin: 10px 0;",
                button {
                    onclick: start_animation,
                    disabled: is_running(),
                    style: if is_running() { "opacity: 0.5;" } else { "" },
                    "Start Keyframe Animation"
                }

                button {
                    onclick: move |_| {
                        motion.target().set(0.0);
                        motion.current().set(0.0);
                        motion.running().set(false);
                    },
                    "Reset"
                }
            }

            p { style: "font-family: monospace; font-size: 12px;",
                "Position: {current():.1}px | Running: {is_running()}"
            }
        }
    }
}

#[component]
fn TransformKeyframeDemo() -> Element {
    let (motion, mut animate_keyframes) = use_motion_store_with_keyframes(Transform::identity());
    let current = motion.current();
    let is_running = motion.running();

    let start_animation = move |_| {
        // Complex transform animation with rotation, scale, and position
        let keyframes = KeyframeAnimation::new(Duration::from_millis(4000))
            .add_keyframe(Transform::identity(), 0.0, None).unwrap()
            .add_keyframe(Transform::new(100.0, 0.0, 1.0, 0.0), 0.2, Some(ease_out_quad)).unwrap()
            .add_keyframe(Transform::new(100.0, 50.0, 1.5, std::f32::consts::PI / 2.0), 0.4, Some(ease_in_out_cubic)).unwrap()
            .add_keyframe(Transform::new(50.0, 50.0, 0.8, std::f32::consts::PI), 0.6, Some(ease_out_bounce)).unwrap()
            .add_keyframe(Transform::new(0.0, 25.0, 1.2, std::f32::consts::PI * 1.5), 0.8, Some(ease_in_out_back)).unwrap()
            .add_keyframe(Transform::identity(), 1.0, Some(ease_out_cubic)).unwrap();
        
        animate_keyframes(keyframes);
    };

    rsx! {
        div {
            style: "border: 2px solid #2196F3; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "Transform Keyframe Animation" }
            p { "Complex 2D transform path with rotation, scaling, and movement" }

            div {
                style: "
                    width: 60px; 
                    height: 60px; 
                    background: linear-gradient(45deg, #2196F3, #1976D2);
                    border-radius: 12px;
                    transform: translateX({current().x}px) translateY({current().y}px) scale({current().scale}) rotate({current().rotation}rad);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    margin: 20px 0;
                ",
                "🎯"
            }

            div { style: "display: flex; gap: 10px; margin: 10px 0;",
                button {
                    onclick: start_animation,
                    disabled: is_running(),
                    style: if is_running() { "opacity: 0.5;" } else { "" },
                    "Start Transform Path"
                }

                button {
                    onclick: move |_| {
                        motion.target().set(Transform::identity());
                        motion.current().set(Transform::identity());
                        motion.running().set(false);
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
fn ColorKeyframeDemo() -> Element {
        let (motion, mut animate_keyframes) = use_motion_store_with_keyframes(Color::new(1.0, 0.0, 0.0, 1.0));
    let current = motion.current();
    let is_running = motion.running();
    
    let start_animation = move |_| {
        // Rainbow color animation through keyframes
        let keyframes = KeyframeAnimation::new(Duration::from_millis(3500))
            .add_keyframe(Color::new(1.0, 0.0, 0.0, 1.0), 0.0, None).unwrap()    // Red
            .add_keyframe(Color::new(1.0, 0.5, 0.0, 1.0), 0.16, Some(ease_in_out_sine)).unwrap() // Orange
            .add_keyframe(Color::new(1.0, 1.0, 0.0, 1.0), 0.33, Some(ease_in_out_sine)).unwrap() // Yellow
            .add_keyframe(Color::new(0.0, 1.0, 0.0, 1.0), 0.5, Some(ease_in_out_sine)).unwrap()  // Green
            .add_keyframe(Color::new(0.0, 0.0, 1.0, 1.0), 0.66, Some(ease_in_out_sine)).unwrap() // Blue
            .add_keyframe(Color::new(0.5, 0.0, 1.0, 1.0), 0.83, Some(ease_in_out_sine)).unwrap() // Indigo
            .add_keyframe(Color::new(1.0, 0.0, 1.0, 1.0), 1.0, Some(ease_in_out_sine)).unwrap(); // Violet
        
        animate_keyframes(keyframes);
    };

    rsx! {
        div {
            style: "border: 2px solid #FF9800; border-radius: 8px; padding: 20px; margin: 20px 0;",

            h3 { "Color Keyframe Animation" }
            p { "Rainbow color transition through multiple keyframes" }

            div {
                style: "
                    width: 60px; 
                    height: 60px; 
                    background: rgba({(current().r * 255.0) as u8}, {(current().g * 255.0) as u8}, {(current().b * 255.0) as u8}, {current().a});
                    border-radius: 50%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    margin: 20px 0;
                    text-shadow: 1px 1px 2px rgba(0,0,0,0.7);
                ",
                "🌈"
            }

            div { style: "display: flex; gap: 10px; margin: 10px 0;",
                button {
                    onclick: start_animation,
                    disabled: is_running(),
                    style: if is_running() { "opacity: 0.5;" } else { "" },
                    "Start Rainbow Animation"
                }

                button {
                    onclick: move |_| {
                        motion.target().set(Color::new(1.0, 0.0, 0.0, 1.0));
                        motion.current().set(Color::new(1.0, 0.0, 0.0, 1.0));
                        motion.running().set(false);
                    },
                    "Reset"
                }
            }

            p { style: "font-family: monospace; font-size: 12px;",
                "R: {current().r:.2} | G: {current().g:.2} | B: {current().b:.2} | A: {current().a:.2}"
            }
        }
    }
}

// Easing functions for keyframe animations
fn ease_out_quad(t: f32, _b: f32, _c: f32, _d: f32) -> f32 {
    -t * (t - 2.0)
}

fn ease_in_out_back(t: f32, _b: f32, _c: f32, _d: f32) -> f32 {
    let s = 1.70158 * 1.525;
    let t = t * 2.0;
    if t < 1.0 {
        0.5 * (t * t * ((s + 1.0) * t - s))
    } else {
        let t = t - 2.0;
        0.5 * (t * t * ((s + 1.0) * t + s) + 2.0)
    }
}

fn ease_out_bounce(t: f32, _b: f32, _c: f32, _d: f32) -> f32 {
    if t < (1.0 / 2.75) {
        7.5625 * t * t
    } else if t < (2.0 / 2.75) {
        let t = t - (1.5 / 2.75);
        7.5625 * t * t + 0.75
    } else if t < (2.5 / 2.75) {
        let t = t - (2.25 / 2.75);
        7.5625 * t * t + 0.9375
    } else {
        let t = t - (2.625 / 2.75);
        7.5625 * t * t + 0.984375
    }
}

fn ease_in_cubic(t: f32, _b: f32, _c: f32, _d: f32) -> f32 {
    t * t * t
}

fn ease_out_cubic(t: f32, _b: f32, _c: f32, _d: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

fn ease_in_out_cubic(t: f32, _b: f32, _c: f32, _d: f32) -> f32 {
    let t = t * 2.0;
    if t < 1.0 {
        0.5 * t * t * t
    } else {
        let t = t - 2.0;
        0.5 * (t * t * t + 2.0)
    }
}

fn ease_in_out_sine(t: f32, _b: f32, _c: f32, _d: f32) -> f32 {
    -0.5 * ((std::f32::consts::PI * t).cos() - 1.0)
}
