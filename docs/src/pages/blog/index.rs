use crate::components::code_block::CodeBlock;
use crate::utils::router::Route;
use dioxus::prelude::*;
use dioxus_code::{CodeOptions, Language, code_str};

/// Long-form article on why the library exists, with highlighted API samples.
#[component]
pub fn Blog() -> Element {
    rsx! {
        document::Title { "Blog · Dioxus Motion" }
        document::Meta {
            name: "description",
            content: "Why dioxus-motion exists: typed, interruptible animation for Rust user interfaces.",
        }
        main { id: "main-content", class: "blog-shell",
            article { class: "blog-article",
                header { class: "blog-intro",
                    p { class: "eyebrow", "BLOG / 2024-03-15" }
                    h1 { "Building dioxus-motion: a physics-based animation library for Rust." }
                    p { class: "lead",
                        "JavaScript has Framer Motion. Rust deserved a native answer: springs and tweens that are typed, interruptible, and honest about errors."
                    }
                    p { class: "blog-note",
                        "Code samples use the current development API. The published 0.3.6 API differs; see the "
                        a { href: "https://github.com/wheregmis/dioxus-motion/blob/main/CHANGELOG.md", "release notes" }
                        " for migration details."
                    }
                }
                section {
                    h2 { "The gap in the ecosystem" }
                    p {
                        "Dioxus made building Rust user interfaces feel natural, but animation stayed manual: timers, string interpolation, and hope. The JavaScript ecosystem long ago solved this with libraries like Framer Motion — declarative targets, spring physics, and graceful interruption. Rust had no equivalent that felt idiomatic."
                    }
                    p {
                        "The goal became a library with four properties: Rust-first design that leans on the type system, zero-cost animation state, one API across web and native renderers, and a surface that a Rust developer would guess correctly on the first try."
                    }
                }
                section {
                    h2 { "Two motion models, one handle" }
                    p {
                        "Every animation is a spring or a tween. A spring integrates displacement and velocity until it settles, so a retargeted animation keeps its momentum. A tween interpolates over a fixed duration with an easing function. Both run through the same "
                        code { "MotionHandle<T>" }
                        " handle returned by "
                        code { "use_motion" }
                        "."
                    }
                    CodeBlock {
                        language: "Rust".to_string(),
                        code: code_str!(
                            r#"let mut x = use_motion(0.0_f32)?;

// A spring keeps momentum when retargeted mid-flight.
x.animate_to(160.0, AnimationConfig::spring(Spring::default()))?;

// A tween runs on a fixed clock.
x.animate_to(0.0, AnimationConfig::tween(Duration::from_millis(300)))?;"#,
                            CodeOptions::builder().with_language(Language::Rust)
                        ),
                    }
                    p {
                        "Both calls return "
                        code { "Result" }
                        ": invalid values or configuration surface as typed "
                        code { "AnimationError" }
                        "s instead of silently doing nothing."
                    }
                }
                section {
                    h2 { "Typed values, not style strings" }
                    p {
                        "The "
                        code { "Animatable" }
                        " trait defines what a value needs to move: ordinary "
                        code { "Add" }
                        ", "
                        code { "Sub" }
                        ", and "
                        code { "Mul<f32>" }
                        " arithmetic, an "
                        code { "interpolate" }
                        " for tweens, a "
                        code { "magnitude" }
                        " for spring convergence, and "
                        code { "is_finite" }
                        " so bad data never reaches the frame loop."
                    }
                    CodeBlock {
                        language: "Rust".to_string(),
                        code: code_str!(
                            r#"#[derive(Clone, PartialEq, Default)]
struct Point { x: f32, y: f32 }

// Animatable needs Add, Sub, and Mul<f32> arithmetic.
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, scale: f32) -> Self {
        Self { x: self.x * scale, y: self.y * scale }
    }
}

impl Animatable for Point {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        Self {
            x: self.x.interpolate(&target.x, t),
            y: self.y.interpolate(&target.y, t),
        }
    }

    fn magnitude(&self) -> f32 {
        self.x.hypot(self.y)
    }

    fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}"#,
                            CodeOptions::builder().with_language(Language::Rust)
                        ),
                    }
                    p {
                        "Scalars, transforms, colors, and "
                        code { "MotionStyle" }
                        " ship with the crate; your own domain types implement the same contract. A custom type guide walks through every requirement — "
                        Link { to: Route::ComplexAnimationGuide {}, "animate a type you own →" }
                    }
                }
                section {
                    h2 { "What was hard" }
                    ul {
                        li {
                            strong { "Frame timing. " }
                            "Browsers disagree about requestAnimationFrame details, so the library keeps a platform-agnostic clock; between animations the frame loop drops to a slow 100 ms idle poll instead of holding frame-rate timers."
                        }
                        li {
                            strong { "Render-cycle isolation. " }
                            "Animation state lives outside the component render path; components subscribe to values and only re-render when an observed value actually changes."
                        }
                        li {
                            strong { "Type-safe springs. " }
                            "A spring needs compatible arithmetic on both sides of a transition. Animating width from px to % is a type error the library reports as "
                            code { "IncompatibleSpringValues" }
                            " before playback changes — a tween is the right tool there."
                        }
                    }
                }
                section {
                    h2 { "Where it went next" }
                    p {
                        "The same foundation now coordinates enter and exit animations with "
                        code { "AnimatePresence" }
                        ", applies motion at route outlets with "
                        code { "AnimatedOutlet" }
                        ", and animates whole CSS states through "
                        code { "motion_style!" }
                        ". The examples gallery runs them all in the browser."
                    }
                    div { class: "hero-actions",
                        Link { class: "button primary", to: Route::DocsLanding {}, "Start the guides →" }
                        Link { class: "button secondary", to: Route::ShowcaseGallery {}, "Browse examples" }
                    }
                }
            }
        }
    }
}
