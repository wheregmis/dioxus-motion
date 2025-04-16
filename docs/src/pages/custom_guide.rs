use dioxus::prelude::*;
use dioxus_motion::{KeyframeAnimation, prelude::*};
use easer::functions::Easing;

use crate::components::code_block::CodeBlock;

// Custom struct for our animation
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

#[component]
pub fn CustomAnimationGuide() -> Element {
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
                        li { "Helper methods make the struct easier to use" }
                    }
                }
            }
        }
    }
}

#[component]
fn StepTwo() -> Element {
    let mut petal = use_motion(PetalTransform::zero());

    let animate = move |_| {
        petal.animate_to(
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
        petal.animate_to(
            PetalTransform::zero(),
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
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
                        code: r#"let mut petal = use_motion(PetalTransform::zero());

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
    let mut petal = use_motion(PetalTransform::zero());

    let animate_sequence = move |_| {
        let sequence = AnimationSequence::new()
            .then(
                PetalTransform::new(45.0, 1.2, 10.0, -10.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                PetalTransform::new(-45.0, 1.5, -10.0, 10.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 150.0,
                    damping: 12.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            )
            .then(
                PetalTransform::zero(),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 200.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );

        petal.animate_sequence(sequence);
    };

    let animate_keyframes = move |_| {
        let keyframes = KeyframeAnimation::new(Duration::from_secs(2))
            .add_keyframe(
                PetalTransform::zero(),
                0.0,
                Some(easer::functions::Cubic::ease_in),
            )
            .add_keyframe(
                PetalTransform::new(45.0, 1.2, 10.0, -10.0),
                0.3,
                Some(easer::functions::Elastic::ease_out),
            )
            .add_keyframe(
                PetalTransform::new(-45.0, 1.5, -10.0, 10.0),
                0.7,
                Some(easer::functions::Bounce::ease_out),
            )
            .add_keyframe(
                PetalTransform::zero(),
                1.0,
                Some(easer::functions::Back::ease_in_out),
            );

        petal.animate_keyframes(keyframes);
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
        PetalTransform::zero(),
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
    .add_keyframe(
        PetalTransform::zero(),
        0.0,
        Some(easer::functions::Cubic::ease_in),
    )
    .add_keyframe(
        PetalTransform::new(45.0, 1.2, 10.0, -10.0),
        0.3,
        Some(easer::functions::Elastic::ease_out),
    )
    .add_keyframe(
        PetalTransform::new(-45.0, 1.5, -10.0, 10.0),
        0.7,
        Some(easer::functions::Bounce::ease_out),
    )
    .add_keyframe(
        PetalTransform::zero(),
        1.0,
        Some(easer::functions::Back::ease_in_out),
    );

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
