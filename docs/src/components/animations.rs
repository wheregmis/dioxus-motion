use crate::components::code_block::CodeBlock;
use dioxus::prelude::*;
use dioxus_motion::{animations::utils::Animatable, prelude::*};
use easer::functions::Easing;

#[component]
/// Renders a two-column layout for presenting an animation example.
///
/// The left column shows a title, descriptive text, and a formatted Rust code snippet, while the right column displays a live demo provided through the child elements. This component is ideal for interactive documentation or tutorials where both the code and its effect are showcased.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn App(cx: Scope) -> Element {
///     AnimationStep(
///         "Fade In Animation".to_string(),
///         "This example demonstrates a fade in effect by animating opacity.".to_string(),
///         "animate_fade_in();".to_string(),
///         cx.render(rsx! {
///             div { "Live demo: fading element" }
///         })
///     )
/// }
/// ```
fn AnimationStep(title: String, description: String, code: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col md:flex-row gap-6 p-6 bg-dark-200/50 backdrop-blur-sm rounded-xl border border-primary/10",
            // Left side - Code
            div { class: "flex-1",
                h3 { class: "text-lg font-medium text-text-primary mb-2", {title} }
                p { class: "text-text-secondary mb-4", {description} }
                CodeBlock {
                    code: code,
                    language: "rust".to_string(),
                }
            }
            // Right side - Live Demo
            div { class: "flex-1 flex items-center justify-center min-h-[200px] bg-dark-300/50 rounded-lg",
                {children}
            }
        }
    }
}

#[component]
/// Animates an element's opacity based on a toggle state.
/// 
/// This component displays a button that toggles the visibility of a colored box. When the button is clicked,
/// the box's opacity animates between 0.0 (hidden) and 1.0 (visible) over 500 milliseconds using a cubic tween
/// easing function.
/// 
/// # Examples
/// 
/// ```rust
/// use dioxus::prelude::*;
/// 
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         BasicValueAnimation()
///     })
/// }
/// 
/// fn main() {
///     dioxus::desktop::launch(app);
/// }
/// ```
fn BasicValueAnimation() -> Element {
    let mut opacity = use_motion(0.0f32);
    let mut is_visible = use_signal(|| false);

    use_effect(move || {
        if *is_visible.read() {
            opacity.animate_to(
                1.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: std::time::Duration::from_millis(500),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            );
        } else {
            opacity.animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: std::time::Duration::from_millis(500),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            );
        }
    });

    rsx! {
        div { class: "space-y-4 w-full text-center",
            button {
                class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                onclick: move |_| is_visible.toggle(),
                if *is_visible.read() { "Hide" } else { "Show" }
            }
            div {
                class: "w-32 h-32 mx-auto bg-primary rounded-lg",
                style: "opacity: {opacity.get_value()}"
            }
        }
    }
}

#[component]
/// Renders a component that animates an element's transform properties using a spring-based animation.
///
/// This function returns a Dioxus Element containing a button and an animated box. When the button is clicked,
/// the component toggles its state, causing the element to transition between its default transform (no translation,
/// unit scale, no rotation) and an animated state (translated, scaled, and rotated). The animation uses a spring
/// configuration for a smooth, natural motion.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// fn App(cx: Scope) -> Element {
///     cx.render(rsx! {
///         TransformAnimation()
///     })
/// }
/// ```
///
/// // In this example, clicking the button toggles the transform animation of the box.
fn TransformAnimation() -> Element {
    let mut transform = use_motion(Transform::new(0.0, 0.0, 1.0, 0.0));
    let mut is_animated = use_signal(|| false);

    use_effect(move || {
        if *is_animated.read() {
            transform.animate_to(
                Transform::new(100.0, 50.0, 1.2, 45.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );
        } else {
            transform.animate_to(
                Transform::new(0.0, 0.0, 1.0, 0.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );
        }
    });

    let transform_style = use_memo(move || {
        format!(
            "transform: translate({}px, {}px) scale({}) rotate({}deg);",
            transform.get_value().x,
            transform.get_value().y,
            transform.get_value().scale,
            transform.get_value().rotation * 180.0 / std::f32::consts::PI
        )
    });

    rsx! {
        div { class: "space-y-4 w-full text-center",
            button {
                class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                onclick: move |_| is_animated.toggle(),
                if *is_animated.read() { "Reset" } else { "Animate" }
            }
            div {
                class: "w-32 h-32 mx-auto bg-primary rounded-lg",
                style: "{transform_style}"
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ColorValue {
    r: f32,
    g: f32,
    b: f32,
}

impl Animatable for ColorValue {
    /// Returns a `ColorValue` with all components set to zero.
    ///
    /// This method provides the zero-initialized state for a `ColorValue`, which is useful as a starting point for color animations.
    ///
    /// # Examples
    ///
    /// ```
    /// let color = ColorValue::zero();
    /// assert_eq!(color.r, 0.0);
    /// assert_eq!(color.g, 0.0);
    /// assert_eq!(color.b, 0.0);
    /// ```
    fn zero() -> Self {
        ColorValue {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Returns a small epsilon value used for floating-point comparisons.
    ///
    /// This constant serves as a threshold to account for numerical imprecision in calculations,
    /// particularly useful when determining if two floating-point numbers are effectively equal.
    ///
    /// # Examples
    ///
    /// ```
    /// let tol = epsilon();
    /// assert!((tol - 0.001).abs() < f32::EPSILON);
    /// ```
    fn epsilon() -> f32 {
        0.001
    }

    /// Computes the Euclidean magnitude of the color using its RGB components.
    ///
    /// The magnitude is calculated as the square root of the sum of the squares of the red, green, and blue values.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming ColorValue is defined in the current module or imported appropriately.
    /// let color = ColorValue { r: 3.0, g: 4.0, b: 0.0 };
    /// assert_eq!(color.magnitude(), 5.0);
    /// ```
    fn magnitude(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b).sqrt()
    }

    /// Returns a new `ColorValue` with each channel scaled by the given factor.
    ///
    /// Multiplies the red, green, and blue components by the provided `factor` and clamps each
    /// resulting value to the range [0.0, 1.0] to ensure the color remains valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let color = ColorValue { r: 0.3, g: 0.6, b: 0.9 };
    /// let scaled = color.scale(1.5);
    /// assert_eq!(scaled.r, 0.45);
    /// assert_eq!(scaled.g, 0.9);
    /// // 0.9 * 1.5 = 1.35, which is clamped to 1.0.
    /// assert_eq!(scaled.b, 1.0);
    /// ```
    fn scale(&self, factor: f32) -> Self {
        ColorValue {
            r: (self.r * factor).clamp(0.0, 1.0),
            g: (self.g * factor).clamp(0.0, 1.0),
            b: (self.b * factor).clamp(0.0, 1.0),
        }
    }

    /// Returns a new `ColorValue` that is the component-wise sum of two colors, with each channel clamped to the range [0.0, 1.0].
    ///
    /// Each color component (red, green, and blue) of `self` and `other` is added together, and the result is clamped to ensure it remains within valid bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let color1 = ColorValue { r: 0.4, g: 0.5, b: 0.6 };
    /// let color2 = ColorValue { r: 0.7, g: 0.8, b: 0.3 };
    /// let result = color1.add(&color2);
    /// assert!(result.r >= 0.0 && result.r <= 1.0);
    /// assert!(result.g >= 0.0 && result.g <= 1.0);
    /// assert!(result.b >= 0.0 && result.b <= 1.0);
    /// ```
    fn add(&self, other: &Self) -> Self {
        ColorValue {
            r: (self.r + other.r).clamp(0.0, 1.0),
            g: (self.g + other.g).clamp(0.0, 1.0),
            b: (self.b + other.b).clamp(0.0, 1.0),
        }
    }

    /// Subtracts the corresponding components of another `ColorValue` from this one, clamping each resulting value between 0.0 and 1.0.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let a = ColorValue { r: 0.7, g: 0.6, b: 0.5 };
    /// let b = ColorValue { r: 0.2, g: 0.4, b: 0.3 };
    /// let result = a.sub(&b);
    /// 
    /// assert_eq!(result.r, 0.5);
    /// assert_eq!(result.g, 0.2);
    /// assert_eq!(result.b, 0.2);
    /// ```
    fn sub(&self, other: &Self) -> Self {
        ColorValue {
            r: (self.r - other.r).clamp(0.0, 1.0),
            g: (self.g - other.g).clamp(0.0, 1.0),
            b: (self.b - other.b).clamp(0.0, 1.0),
        }
    }

    /// Returns a new `ColorValue` that is a linear interpolation between `self` and `target`.
    ///
    /// The interpolation is performed on each color channel (red, green, and blue) using the formula:
    /// `result = self + (target - self) * t`. The interpolation factor `t` should typically be in the range
    /// [0.0, 1.0], where 0.0 returns the original color and 1.0 returns the target color.
    ///
    /// # Examples
    ///
    /// ```
    /// let color1 = ColorValue { r: 0.0, g: 0.0, b: 0.0 };
    /// let color2 = ColorValue { r: 1.0, g: 1.0, b: 1.0 };
    /// let mid_color = color1.interpolate(&color2, 0.5);
    /// assert!((mid_color.r - 0.5).abs() < f32::EPSILON);
    /// assert!((mid_color.g - 0.5).abs() < f32::EPSILON);
    /// assert!((mid_color.b - 0.5).abs() < f32::EPSILON);
    /// ```
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        ColorValue {
            r: self.r + (target.r - self.r) * t,
            g: self.g + (target.g - self.g) * t,
            b: self.b + (target.b - self.b) * t,
        }
    }
}

#[component]
/// Renders an interactive component that animates a color transition between warm and cool color schemes.
///
/// The component displays a button that toggles the animation between two predefined color values using a spring animation.
/// When the button is clicked, the background color of a square div smoothly transitions between a cool blue-inspired color
/// and a warm reddish hue.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn main() {
///     dioxus::web::launch(app);
/// }
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         CustomColorAnimation {}
///     })
/// }
/// ```
fn CustomColorAnimation() -> Element {
    let mut color = use_motion(ColorValue {
        r: 0.2,
        g: 0.5,
        b: 0.8,
    });
    let mut is_warm = use_signal(|| false);

    use_effect(move || {
        if *is_warm.read() {
            color.animate_to(
                ColorValue {
                    r: 0.8,
                    g: 0.3,
                    b: 0.2,
                },
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );
        } else {
            color.animate_to(
                ColorValue {
                    r: 0.2,
                    g: 0.5,
                    b: 0.8,
                },
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );
        }
    });

    let color_style = use_memo(move || {
        format!(
            "background-color: rgb({}%, {}%, {}%)",
            color.get_value().r * 100.0,
            color.get_value().g * 100.0,
            color.get_value().b * 100.0,
        )
    });

    rsx! {
        div { class: "space-y-4 w-full text-center",
            button {
                class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                onclick: move |_| is_warm.toggle(),
                if *is_warm.read() { "Cool Colors" } else { "Warm Colors" }
            }
            div {
                class: "w-32 h-32 mx-auto rounded-lg transition-all duration-300",
                style: "{color_style}"
            }
        }
    }
}

#[component]
/// Generates a UI component demonstrating a sequence animation.
/// 
/// The component displays a counter that increments on each button press. When clicked, it triggers a chained animation:
/// a spring-based vertical translation proportional to the updated counter and a brief scale animation that enlarges the element.
/// These animations combine to provide dynamic visual feedback for sequential interactions.
/// 
/// # Examples
/// 
/// ```rust
/// use dioxus::prelude::*;
///
/// fn main() {
///     dioxus::desktop::launch(app);
/// }
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         SequenceAnimation()
///     })
/// }
/// ```
fn SequenceAnimation() -> Element {
    let mut value = use_motion(0.0f32);
    let mut scale = use_motion(1.0f32);
    let mut count = use_signal(|| 0);

    let onclick = move |_| {
        let sequence = AnimationSequence::new().then(
            ((*count)() + 1) as f32 * 100.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 180.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 10.0,
            })),
        );

        scale.animate_to(
            1.2,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        value.animate_sequence(sequence);
        count.set((*count)() + 1);
    };

    rsx! {
        div { class: "space-y-4 w-full text-center",
            div {
                class: "text-4xl font-bold text-primary",
                style: "transform: translateY({value.get_value()}px) scale({scale.get_value()})",
                "Count: {count}"
            }
            button {
                class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                onclick: onclick,
                "Increment"
            }
        }
    }
}

#[component]
/// Returns a Dioxus element representing the interactive animation guide.
///
/// This component composes a comprehensive demo of various animation techniques available
/// in Dioxus Motion. It displays interactive examples for basic tween animations, spring animations,
/// transform animations, custom animations, and animation sequences, along with sections outlining best practices for animation performance and user experience.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn main() {
///     dioxus::desktop::launch(App);
/// }
///
/// #[component]
/// fn App(cx: Scope) -> Element {
///     cx.render(rsx! {
///         Animations()
///     })
/// }
/// ```
pub fn Animations() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Interactive Animation Guide" }
                p { class: "text-text-secondary",
                    "Learn how to create beautiful animations in Dioxus Motion through interactive examples.
                     Start with basic animations and progress to more complex ones."
                }

                // Animation Types Overview
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 mt-6",
                    div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                        h3 { class: "font-semibold text-text-primary mb-2", "Core Animation Types" }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1",
                            li {
                                span { class: "text-primary font-semibold", "Tween: " }
                                "Time-based animations with precise duration and easing"
                            }
                            li {
                                span { class: "text-primary font-semibold", "Spring: " }
                                "Physics-based animations that feel natural and responsive"
                            }
                        }
                    }
                    div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                        h3 { class: "font-semibold text-text-primary mb-2", "Advanced Features" }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1",
                            li {
                                span { class: "text-primary font-semibold", "Transform: " }
                                "Built-in type for position, scale, and rotation"
                            }
                            li {
                                span { class: "text-primary font-semibold", "Custom Types: " }
                                "Create your own animatable types"
                            }
                            li {
                                span { class: "text-primary font-semibold", "Sequences: " }
                                "Chain multiple animations for complex, coordinated motion"
                            }
                        }
                    }
                }
            }

            // Basic Value Animation with Tween
            AnimationStep {
                title: "1. Basic Tween Animation".to_string(),
                description: "Time-based animations with precise control over duration and easing. Perfect for fade effects and smooth transitions.".to_string(),
                code: r#"// Initialize the motion value
let mut opacity = use_motion(0.0f32);

// Option 1: Trigger on mount
use_effect(move || {
    opacity.animate_to(
        1.0,
        AnimationConfig::new(AnimationMode::Tween(Tween {
            duration: Duration::from_millis(500),
            easing: easer::functions::Cubic::ease_in_out,
        })),
    );
});

// Option 2: Trigger on state change
let mut is_visible = use_signal(|| false);
use_effect(move || {
    if *is_visible.read() {
        opacity.animate_to(1.0, /* config */);
    } else {
        opacity.animate_to(0.0, /* config */);
    }
});

// Option 3: Trigger on event
rsx! {
    button {
        onclick: move |_| {
            opacity.animate_to(1.0, /* config */);
        },
        "Animate"
    }
}"#.to_string(),
                BasicValueAnimation {}
            }

            // Spring Animation
            AnimationStep {
                title: "2. Spring Animation".to_string(),
                description: "Physics-based animations that create natural motion. Great for interactive elements that need organic movement.".to_string(),
                code: r#"
                
#[component]
fn TransformAnimation() -> Element {
    let mut transform = use_motion(Transform::new(0.0, 0.0, 1.0, 0.0));
    let mut is_animated = use_signal(|| false);

    use_effect(move || {
        if *is_animated.read() {
            transform.animate_to(
                Transform::new(100.0, 50.0, 1.2, 45.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );
        } else {
            transform.animate_to(
                Transform::new(0.0, 0.0, 1.0, 0.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );
        }
    });

    let transform_style = use_memo(move || {
        format!(
            "transform: translate({}px, {}px) scale({}) rotate({}deg);",
            transform.get_value().x,
            transform.get_value().y,
            transform.get_value().scale,
            transform.get_value().rotation * 180.0 / std::f32::consts::PI
        )
    });

    rsx! {
        div { class: "space-y-4 w-full text-center",
            button {
                class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                onclick: move |_| is_animated.toggle(),
                if *is_animated.read() { "Reset" } else { "Animate" }
            }
            div {
                class: "w-32 h-32 mx-auto bg-primary rounded-lg",
                style: "{transform_style}"
            }
        }
    }
}
                "#.to_string(),
                TransformAnimation {}
            }

            // Transform Animation
            AnimationStep {
                title: "3. Transform Animation".to_string(),
                description: "Built-in Transform type for animating position, scale, and rotation. Uses the same animation modes as basic values.".to_string(),
                code: r#"// Transform combines multiple properties:
// - x, y: Position
// - scale: Size
// - rotation: Angle in radians
let mut transform = use_motion(Transform::new(0.0, 0.0, 1.0, 0.0));

// Animate with spring for natural motion
transform.animate_to(
    Transform::new(100.0, 50.0, 1.2, 45.0),
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
    })),
);

// Or use tween for precise timing
transform.animate_to(
    Transform::new(0.0, 0.0, 1.0, 0.0),
    AnimationConfig::new(AnimationMode::Tween(Tween {
        duration: Duration::from_millis(300),
        easing: easer::functions::Cubic::ease_out,
    })),
);"#.to_string(),
                TransformAnimation {}
            }

            // Custom Animation
            AnimationStep {
                title: "4. Custom Animation Type".to_string(),
                description: "Create your own animatable types by implementing the Animatable trait. This example shows color interpolation.".to_string(),
                code: r#"#[derive(Debug, Copy, Clone)]
struct ColorValue {
    r: f32, g: f32, b: f32,
}

// Implement Animatable to enable animation
impl Animatable for ColorValue {
    fn zero() -> Self { ColorValue { r: 0.0, g: 0.0, b: 0.0 } }
    fn epsilon() -> f32 { 0.001 }
    fn magnitude(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b).sqrt()
    }
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        ColorValue {
            r: self.r + (target.r - self.r) * t,
            g: self.g + (target.g - self.g) * t,
            b: self.b + (target.b - self.b) * t,
        }
    }
}

// Use it like any other motion value
let mut color = use_motion(ColorValue { r: 0.2, g: 0.5, b: 0.8 });
color.animate_to(
    ColorValue { r: 0.8, g: 0.3, b: 0.2 },
    AnimationConfig::new(AnimationMode::Spring(Spring::default())),
);"#.to_string(),
                CustomColorAnimation {}
            }

            // Sequence Animation
            AnimationStep {
                title: "5. Animation Sequences".to_string(),
                description: "Chain multiple animations together to create complex, coordinated motion. Perfect for multi-step animations and interactive counters.".to_string(),
                code: r#"// Initialize multiple motion values
let mut value = use_motion(0.0f32);
let mut scale = use_motion(1.0f32);
let mut count = use_signal(|| 0);

// Create and trigger a sequence on button click
let onclick = move |_| {
    // Create a new sequence that animates based on count
    let sequence = AnimationSequence::new()
        .then(
            (count + 1) as f32 * 100.0,  // Dynamic target value
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 180.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 10.0,
            }))
        );
    
    // Animate scale independently
    scale.animate_to(
        1.2,
        AnimationConfig::new(AnimationMode::Spring(Spring::default()))
    );
    
    // Start the sequence animation
    value.animate_sequence(sequence);
    count += 1;
}

// Use the animated values in your component
rsx! {
    div {
        style: "transform: translateY({value.get_value()}px) scale({scale.get_value()})",
        "Count: {count}"
    }
    button {
        onclick: onclick,
        "Increment"
    }
}"#.to_string(),
                SequenceAnimation {}
            }

            // Best Practices
            section { class: "space-y-6",
                h2 { class: "text-2xl font-semibold text-text-primary", "Best Practices" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                        h3 { class: "font-semibold text-text-primary mb-2", "Performance" }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1",
                            li { "Use spring animations for natural motion" }
                            li { "Keep tween durations under 300ms for snappy feedback" }
                            li { "Avoid animating too many elements simultaneously" }
                            li { "Use transform instead of position for better performance" }
                        }
                    }
                    div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm border border-primary/10",
                        h3 { class: "font-semibold text-text-primary mb-2", "UX Guidelines" }
                        ul { class: "list-disc list-inside text-text-secondary space-y-1",
                            li { "Maintain consistent animation durations" }
                            li { "Use easing functions that match your app's personality" }
                            li { "Provide visual feedback for user interactions" }
                            li { "Consider reduced motion preferences" }
                        }
                    }
                }
            }
        }
    }
}
