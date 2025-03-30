use crate::components::code_block::CodeBlock;
use dioxus::prelude::*;
use dioxus_motion::{animations::utils::Animatable, prelude::*};
use easer::functions::Easing;

#[component]
/// Renders an animation step layout.
///
/// This component creates a styled container that presents an animation example. The left section displays a title, descriptive text, and a syntax-highlighted Rust code block, while the right section shows live demo content provided as children.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// let element = AnimationStep(
///     "Step Title".to_string(),
///     "A brief description of the animation.".to_string(),
///     "fn animated() { println!(\"Hello, world!\"); }".to_string(),
///     rsx! { div { "Live demo content" } },
/// );
/// ```
fn AnimationStep(title: String, description: String, code: String, children: Element) -> Element {
    rsx! {
        div {
            class: "flex flex-col lg:flex-row gap-6 p-6 bg-dark-200/50 backdrop-blur-sm rounded-xl border border-primary/10",
            // Left side - Code (85%)
            div {
                class: "lg:w-[85%] min-w-0", // Changed to explicit 85% width on larger screens
                h3 { class: "text-lg font-medium text-text-primary mb-2", {title} }
                p { class: "text-text-secondary mb-4", {description} }
                div {
                    class: "overflow-x-auto rounded-lg", // Added rounded corners to match the demo section
                    CodeBlock {
                        code: code,
                        language: "rust".to_string(),
                    }
                }
            }
            // Right side - Live Demo (15%)
            div {
                class: "lg:w-[15%] min-w-[200px] flex items-center justify-center min-h-[200px] bg-dark-300/50 rounded-lg",
                {children}
            }
        }
    }
}

#[component]
/// Renders a component that animates an element's opacity based on a visibility toggle.
///
/// The component displays a button to toggle the visibility state. When toggled,
/// it smoothly animates the opacity of a square element between 0.0 and 1.0 over
/// 500 milliseconds using a tween animation with cubic easing.
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
///         BasicValueAnimation()
///     })
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
/// Animates an element's transform properties with a spring dynamic effect.
///
/// This component toggles the transformation state of a div element—adjusting its translation,
/// scaling, and rotation—between an initial and a target configuration based on user interaction.
/// When the button is clicked, the element smoothly transitions between positions using spring physics.
///
/// # Examples
///
/// ```
/// // In a Dioxus application, include the TransformAnimation component:
/// let element = TransformAnimation();
/// // Render `element` within your application view. Clicking the button animates the transform.
/// ```
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
    /// Returns a new `ColorValue` with all components (r, g, and b) set to zero.
    ///
    /// Creates a zero-initialized color, which can be used as a default or starting point in color operations.
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

    /// Returns a small epsilon value (0.001) used as a tolerance threshold in floating-point calculations.
    ///
    /// This constant is useful for comparing floating point numbers and mitigating precision issues.
    ///
    /// # Examples
    ///
    /// ```
    /// let tol = epsilon();
    /// // Use epsilon for approximate comparisons between floating point numbers
    /// assert!((tol - 0.001).abs() < 1e-6);
    /// ```
    fn epsilon() -> f32 {
        0.001
    }

    /// Computes the Euclidean magnitude of the color by calculating the square root of the sum of the squares of its red, green, and blue components.
    ///
    /// # Examples
    ///
    /// ```
    /// let color = ColorValue { r: 3.0, g: 4.0, b: 0.0 };
    /// assert_eq!(color.magnitude(), 5.0);
    /// ```
    fn magnitude(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b).sqrt()
    }

    /// Scales the color by multiplying each channel by the given factor and clamping the result to the [0.0, 1.0] range.
    ///
    /// This method multiplies the red, green, and blue components by the specified factor, ensuring that each channel remains within valid bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let color = ColorValue { r: 0.3, g: 0.5, b: 0.7 };
    /// let scaled_color = color.scale(2.0);
    /// // Each component is scaled and clamped to a maximum of 1.0.
    /// assert!(scaled_color.r <= 1.0 && scaled_color.g <= 1.0 && scaled_color.b <= 1.0);
    /// ```
    fn scale(&self, factor: f32) -> Self {
        ColorValue {
            r: (self.r * factor).clamp(0.0, 1.0),
            g: (self.g * factor).clamp(0.0, 1.0),
            b: (self.b * factor).clamp(0.0, 1.0),
        }
    }

    /// Adds two `ColorValue` instances component-wise, clamping each resulting channel to the range [0.0, 1.0].
    ///
    /// Each channel (red, green, and blue) is summed individually and then clamped to ensure the resulting value
    /// remains within the valid range. This operation is useful for blending colors without exceeding the maximum intensity.
    ///
    /// # Examples
    ///
    /// ```
    /// let color1 = ColorValue { r: 0.5, g: 0.4, b: 0.3 };
    /// let color2 = ColorValue { r: 0.7, g: 0.8, b: 0.9 };
    /// let result = color1.add(&color2);
    /// assert_eq!(result.r, 1.0);
    /// assert_eq!(result.g, 1.0);
    /// assert_eq!(result.b, 1.0);
    /// ```
    fn add(&self, other: &Self) -> Self {
        ColorValue {
            r: (self.r + other.r).clamp(0.0, 1.0),
            g: (self.g + other.g).clamp(0.0, 1.0),
            b: (self.b + other.b).clamp(0.0, 1.0),
        }
    }

    /// Subtracts the corresponding color components of another `ColorValue` from this one, clamping each result between 0.0 and 1.0.
    ///
    /// This method performs component-wise subtraction on the red, green, and blue channels, ensuring that the result does not fall below 0.0 or exceed 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// let color1 = ColorValue { r: 0.8, g: 0.7, b: 0.6 };
    /// let color2 = ColorValue { r: 0.3, g: 0.4, b: 0.5 };
    /// let result = color1.sub(&color2);
    ///
    /// // Expected results after subtraction and clamping:
    /// // red: 0.8 - 0.3 = 0.5, green: 0.7 - 0.4 = 0.3, blue: 0.6 - 0.5 = 0.1
    /// assert_eq!(result.r, 0.5);
    /// assert_eq!(result.g, 0.3);
    /// assert_eq!(result.b, 0.1);
    /// ```
    fn sub(&self, other: &Self) -> Self {
        ColorValue {
            r: (self.r - other.r).clamp(0.0, 1.0),
            g: (self.g - other.g).clamp(0.0, 1.0),
            b: (self.b - other.b).clamp(0.0, 1.0),
        }
    }

    /// Linearly interpolates between the current color and a target color.
    ///
    /// Produces a new `ColorValue` by blending `self` and `target` based on the
    /// interpolation factor `t`. When `t` is 0.0, the result is `self`; when `t` is 1.0,
    /// the result is `target`.
    ///
    /// # Examples
    ///
    /// ```
    /// let start = ColorValue { r: 0.0, g: 0.0, b: 0.0 };
    /// let end = ColorValue { r: 1.0, g: 1.0, b: 1.0 };
    /// let mid = start.interpolate(&end, 0.5);
    ///
    /// // The mid point should have each component approximately equal to 0.5.
    /// assert!((mid.r - 0.5).abs() < f32::EPSILON);
    /// assert!((mid.g - 0.5).abs() < f32::EPSILON);
    /// assert!((mid.b - 0.5).abs() < f32::EPSILON);
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
/// Renders an interactive color animation component that transitions a div's background color between cool and warm states.
///
/// The component uses spring-based animations to smoothly switch between a predefined cool color (default) and a warm color when the button is clicked. The button label dynamically reflects the current color mode.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn App(cx: Scope) -> Element {
///     cx.render(rsx! {
///         CustomColorAnimation()
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
/// Displays a count with a coordinated sequence animation.
///
/// On clicking the "Increment" button, the component increases an internal counter and animates its display by sequentially applying a vertical translation (with an offset proportional to the new count) and a scaling effect using spring physics. This demonstrates how to chain multiple animations together for a dynamic visual update.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
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
/// Renders an interactive guide to animations using Dioxus Motion.
///
/// This function returns a Dioxus Element that assembles various animation examples—including basic tween animations, spring animations, transform animations, custom animations, and sequence animations—along with best practice recommendations for performance and user experience.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn main() {
///     dioxus::desktop::launch(app);
/// }
///
/// fn app(cx: Scope) -> Element {
///     Animations()
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

            // Advanced Features Animation Step
            AnimationStep {
                title: "6. Advanced Animation Features".to_string(),
                description: "Explore additional features like loops, delays, and completion callbacks for more control over your animations.".to_string(),
                code: r#"// Loop animations infinitely or a specific number of times
value.animate_to(
    1.0,
    AnimationConfig::new(AnimationMode::Tween(Tween::default()))
        .with_loop(LoopMode::Infinite)  // Loop forever
);

value.animate_to(
    1.0,
    AnimationConfig::new(AnimationMode::Tween(Tween::default()))
        .with_loop(LoopMode::Times(3))  // Loop 3 times
);

// Add delays before starting animations
value.animate_to(
    1.0,
    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
        .with_delay(Duration::from_secs(1))  // Wait 1 second before starting
);

// Execute callbacks when animations complete
value.animate_to(
    1.0,
    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
        .with_on_complete(|| println!("Animation complete!"))
);"#.to_string(),
                AdvancedFeaturesAnimation {}  // You'll need to implement this component
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

#[component]
fn AdvancedFeaturesAnimation() -> Element {
    let mut infinite_value = use_motion(0.0f32);
    let mut delayed_value = use_motion(0.0f32);
    let mut callback_value = use_motion(0.0f32);
    // Infinite loop animation
    let start_infinite = move |_| {
        infinite_value.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite),
        );
    };

    // Delayed animation
    let start_delayed = move |_| {
        delayed_value.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                .with_delay(Duration::from_secs(1)),
        );
    };

    // Animation with completion callback
    let start_callback = move |_| {
        callback_value.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Times(3))
            .with_on_complete(|| println!("Animation completed after 3 loops!")),
        );
    };

    // Reset all animations
    let reset_all = move |_| {
        infinite_value.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(500),
                easing: easer::functions::Cubic::ease_out,
            })),
        );
        delayed_value.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(500),
                easing: easer::functions::Cubic::ease_out,
            })),
        );
        callback_value.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(500),
                easing: easer::functions::Cubic::ease_out,
            })),
        );
    };

    rsx! {
        div {
            class: "space-y-8 flex flex-col items-center w-full", // Added centering classes
            // Infinite loop animation
            div {
                class: "space-y-2 flex flex-col items-center", // Center this section
                h3 { class: "text-lg font-semibold text-text-primary", "Infinite Loop" }
                div { class: "flex gap-4 items-center justify-center", // Center buttons and animation
                    button {
                        class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: start_infinite,
                        "Start Infinite"
                    }
                    div {
                        class: "w-16 h-16 bg-primary rounded-lg transition-transform",
                        style: "opacity: {infinite_value.get_value()}"
                    }
                }
            }

            // Delayed animation
            div {
                class: "space-y-2 flex flex-col items-center", // Center this section
                h3 { class: "text-lg font-semibold text-text-primary", "Delayed Animation (1s)" }
                div { class: "flex gap-4 items-center justify-center", // Center buttons and animation
                    button {
                        class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: start_delayed,
                        "Start Delayed"
                    }
                    div {
                        class: "w-16 h-16 bg-primary rounded-lg transition-transform",
                        style: "opacity: {delayed_value.get_value()}"
                    }
                }
            }

            // Callback animation
            div {
                class: "space-y-2 flex flex-col items-center", // Center this section
                h3 { class: "text-lg font-semibold text-text-primary", "Loop with Callback" }
                div { class: "flex gap-4 items-center justify-center", // Center buttons and animation
                    button {
                        class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                        onclick: start_callback,
                        "Start (3 loops)"
                    }
                    div {
                        class: "w-16 h-16 bg-primary rounded-lg transition-transform",
                        style: "opacity: {callback_value.get_value()}"
                    }
                }
            }

            // Reset button
            div {
                class: "pt-4 w-full flex justify-center", // Center the reset button
                button {
                    class: "px-4 py-2 bg-primary/20 hover:bg-primary/30 rounded-lg text-primary transition-colors",
                    onclick: reset_all,
                    "Reset All"
                }
            }
        }
    }
}
