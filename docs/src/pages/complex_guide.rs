use dioxus::prelude::*;
use dioxus_motion::{KeyframeAnimation, prelude::*};
use easer::functions::Easing;

use crate::components::code_block::CodeBlock;
use crate::components::guide_navigation::GuideNavigation;

// Custom struct for our animation
#[derive(Clone, Copy)]
struct PetalTransform {
    rotate: f32,
    scale: f32,
    translate_x: f32,
    translate_y: f32,
}

impl PetalTransform {
    fn new(rotate: f32, scale: f32, translate_x: f32, translate_y: f32) -> Self {
        Self {
            rotate,
            scale,
            translate_x,
            translate_y,
        }
    }
}

// Implement required traits for animation
impl std::ops::Mul<f32> for PetalTransform {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            rotate: self.rotate * rhs,
            scale: self.scale * rhs,
            translate_x: self.translate_x * rhs,
            translate_y: self.translate_y * rhs,
        }
    }
}

impl std::ops::Add for PetalTransform {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            rotate: self.rotate + rhs.rotate,
            scale: self.scale + rhs.scale,
            translate_x: self.translate_x + rhs.translate_x,
            translate_y: self.translate_y + rhs.translate_y,
        }
    }
}

impl Default for PetalTransform {
    fn default() -> Self {
        Self {
            rotate: 0.0,
            scale: 0.0, // Start invisible, consistent with animated_flower.rs
            translate_x: 0.0,
            translate_y: 0.0,
        }
    }
}

impl std::ops::Sub for PetalTransform {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            rotate: self.rotate - rhs.rotate,
            scale: self.scale - rhs.scale,
            translate_x: self.translate_x - rhs.translate_x,
            translate_y: self.translate_y - rhs.translate_y,
        }
    }
}

impl dioxus_motion::animations::core::Animatable for PetalTransform {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self::new(
            self.rotate + (other.rotate - self.rotate) * t,
            self.scale + (other.scale - self.scale) * t,
            self.translate_x + (other.translate_x - self.translate_x) * t,
            self.translate_y + (other.translate_y - self.translate_y) * t,
        )
    }

    fn magnitude(&self) -> f32 {
        (self.rotate * self.rotate
            + self.scale * self.scale
            + self.translate_x * self.translate_x
            + self.translate_y * self.translate_y)
            .sqrt()
    }

    // Uses default epsilon of 0.01 from the trait
}

#[component]
pub fn ComplexAnimationGuide() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Custom Animation Guide" }
                p { class: "text-text-secondary",
                    "Learn how to create and animate custom structs for complex animations. We'll build a flower petal animation step by step."
                }
            }

            // Step 1: Define Custom Struct
            StepOne {}

            // Step 2: Basic Custom Animation
            StepTwo {}

            // Step 3: Advanced Custom Animation
            StepThree {}

            // Navigation links
            GuideNavigation {}
        }
    }
}

#[component]
fn StepOne() -> Element {
    rsx! {
        section { class: "space-y-4",
            div {
                h2 { class: "text-2xl font-semibold mb-2", "Step 1: Define Custom Struct" }
                p { class: "text-text-secondary",
                    "First, we need to define our custom struct and implement necessary traits for animation."
                }
            }

            div { class: "space-y-2",
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"// Define the custom struct
#[derive(Clone, Copy)]
struct PetalTransform {
    rotate: f32,
    scale: f32,
    translate_x: f32,
    translate_y: f32,
}

impl PetalTransform {
    fn zero() -> Self {
        Self {
            rotate: 0.0,
            scale: 0.0,
            translate_x: 0.0,
            translate_y: 0.0,
        }
    }

    fn new(rotate: f32, scale: f32, translate_x: f32, translate_y: f32) -> Self {
        Self { rotate, scale, translate_x, translate_y }
    }
}

// Implement required traits for animation
impl std::ops::Mul<f32> for PetalTransform {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            rotate: self.rotate * rhs,
            scale: self.scale * rhs,
            translate_x: self.translate_x * rhs,
            translate_y: self.translate_y * rhs,
        }
    }
}

impl std::ops::Add for PetalTransform {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            rotate: self.rotate + rhs.rotate,
            scale: self.scale + rhs.scale,
            translate_x: self.translate_x + rhs.translate_x,
            translate_y: self.translate_y + rhs.translate_y,
        }
    }
}

impl dioxus_motion::animations::core::Animatable for PetalTransform {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Self::new(
            self.rotate + (other.rotate - self.rotate) * t,
            self.scale + (other.scale - self.scale) * t,
            self.translate_x + (other.translate_x - self.translate_x) * t,
            self.translate_y + (other.translate_y - self.translate_y) * t,
        )
    }

    fn magnitude(&self) -> f32 {
        (self.rotate * self.rotate
            + self.scale * self.scale
            + self.translate_x * self.translate_x
            + self.translate_y * self.translate_y)
            .sqrt()
    }

    // Uses default epsilon of 0.01 from the trait
}

impl Default for PetalTransform {
    fn default() -> Self {
        Self {
            rotate: 0.0,
            scale: 0.0, // Start invisible, consistent with animated_flower.rs
            translate_x: 0.0,
            translate_y: 0.0,
        }
    }
}

impl std::ops::Sub for PetalTransform {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            rotate: self.rotate - rhs.rotate,
            scale: self.scale - rhs.scale,
            translate_x: self.translate_x - rhs.translate_x,
            translate_y: self.translate_y - rhs.translate_y,
        }
    }
}
}"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                div { class: "mt-4 p-4 bg-dark-200/30 rounded-lg",
                    h3 { class: "font-medium mb-2", "Key Points:" }
                    ul { class: "list-disc list-inside text-text-secondary space-y-1",
                        li { "Custom struct must implement Clone and Copy" }
                        li { "Implement std::ops::Mul<f32> for interpolation" }
                        li { "Implement std::ops::Add for combining values" }
                        li { "Implement Animatable trait for animation support" }
                        li { "Helper methods make the struct easier to use" }
                    }
                }
            }
        }
    }
}

#[component]
fn StepTwo() -> Element {
    let petal = use_motion(PetalTransform::default());
    let mut petal_animate = petal.clone();
    let mut petal_reset = petal.clone();
    let petal_val = petal.clone();
    let animate = move |_| {
        petal_animate.animate_to(
            PetalTransform::new(45.0, 1.2, 10.0, -10.0),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    };
    let reset = move |_| {
        petal_reset.animate_to(
            PetalTransform::default(),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    };

    rsx! {
        section { class: "space-y-4",
            div {
                h2 { class: "text-2xl font-semibold mb-2", "Step 2: Basic Custom Animation" }
                p { class: "text-text-secondary",
                    "Now let's animate our custom struct with a simple spring animation."
                }
            }

            div { class: "space-y-4",
                // Code example
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    CodeBlock {
                        code: r#"let mut petal = use_motion(PetalTransform::default());

// Animate to new values
petal.animate_to(
    PetalTransform::new(45.0, 1.2, 10.0, -10.0),
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
    })),
);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                // Live demo
                div { class: "p-4 bg-dark-200/30 rounded-lg space-y-4",
                    h3 { class: "font-medium", "Live Demo" }

                    // Visual representation
                    div { class: "relative h-32 w-32 mx-auto",
                        div {
                            class: "absolute w-16 h-16 bg-primary/50 rounded-lg",
                            style: "
                                transform: translate({petal.get_value().translate_x}px, {petal.get_value().translate_y}px)
                                rotate({petal.get_value().rotate}deg)
                                scale({petal.get_value().scale})
                            "
                        }
                    }

                    // Current values
                    div { class: "text-sm text-text-secondary space-y-1",
                        p { "Rotation: {petal.get_value().rotate:.1}°" }
                        p { "Scale: {petal.get_value().scale:.2}" }
                        p { "X: {petal.get_value().translate_x:.1}px" }
                        p { "Y: {petal.get_value().translate_y:.1}px" }
                    }

                    // Controls
                    div { class: "flex gap-2",
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: animate,
                            "Animate"
                        }
                        button {
                            class: "px-4 py-2 bg-dark-200 hover:bg-dark-300 rounded-lg text-text-secondary transition-colors",
                            onclick: reset,
                            "Reset"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StepThree() -> Element {
    let petal = use_motion(PetalTransform::default());
    let mut petal_seq = petal.clone();
    let mut petal_kf = petal.clone();
    let petal_val = petal.clone();
    let animate_sequence = move |_| {
        let sequence_vec = vec![
            PetalTransform::new(45.0, 1.2, 10.0, -10.0),
            PetalTransform::new(-45.0, 1.5, -10.0, 10.0),
            PetalTransform::default(),
        ];
        petal_seq.interpolate_sequence(sequence_vec, 0.5);
    };
    let animate_keyframes = move |_| {
        let keyframes_vec = vec![
            PetalTransform::default(),
            PetalTransform::new(45.0, 1.2, 10.0, -10.0),
            PetalTransform::new(-45.0, 1.5, -10.0, 10.0),
            PetalTransform::default(),
        ];
        petal_kf.interpolate_sequence(keyframes_vec, 0.5);
    };

    rsx! {
        section { class: "space-y-4",
            div {
                h2 { class: "text-2xl font-semibold mb-2", "Step 3: Advanced Custom Animation" }
                p { class: "text-text-secondary",
                    "Finally, let's explore sequences and keyframes with our custom struct."
                }
            }

            div { class: "space-y-4",
                // Sequence example
                div { class: "bg-dark-200/50 p-3 rounded-lg",
                    h3 { class: "font-medium mb-2", "Sequence Animation" }
                    CodeBlock {
                        code: r#"let sequence = AnimationSequence::new()
    .then(
        PetalTransform::new(45.0, 1.2, 10.0, -10.0),
        spring_config.clone(),
    )
    .then(
        PetalTransform::new(-45.0, 1.5, -10.0, 10.0),
        spring_config.clone(),
    )
    .then(
        PetalTransform::default(),
        spring_config,
    );

petal.animate_sequence(sequence);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                // Keyframe example
                div { class: "bg-dark-200/50 p-3 rounded-lg mt-4",
                    h3 { class: "font-medium mb-2", "Keyframe Animation" }
                    CodeBlock {
                        code: r#"let keyframes = KeyframeAnimation::new(Duration::from_secs(2))
    .add_keyframe(...)
    .and_then(|kf| kf.add_keyframe(...))
    .and_then(|kf| kf.add_keyframe(...))
    .and_then(|kf| kf.add_keyframe(...))
    .unwrap();
petal.animate_keyframes(keyframes);"#.to_string(),
                        language: "rust".to_string(),
                    }
                }

                // Live demo
                div { class: "p-4 bg-dark-200/30 rounded-lg space-y-4",
                    h3 { class: "font-medium", "Live Demo" }

                    // Visual representation
                    div { class: "relative h-32 w-32 mx-auto",
                        div {
                            class: "absolute w-16 h-16 bg-primary/50 rounded-lg",
                            style: "
                                transform: translate({petal.get_value().translate_x}px, {petal.get_value().translate_y}px)
                                rotate({petal.get_value().rotate}deg)
                                scale({petal.get_value().scale})
                            "
                        }
                    }

                    // Current values
                    div { class: "text-sm text-text-secondary space-y-1",
                        p { "Rotation: {petal.get_value().rotate:.1}°" }
                        p { "Scale: {petal.get_value().scale:.2}" }
                        p { "X: {petal.get_value().translate_x:.1}px" }
                        p { "Y: {petal.get_value().translate_y:.1}px" }
                    }

                    // Controls
                    div { class: "flex gap-2",
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: animate_sequence,
                            "Sequence"
                        }
                        button {
                            class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                            onclick: animate_keyframes,
                            "Keyframes"
                        }
                    }
                }
            }
        }
    }
}
