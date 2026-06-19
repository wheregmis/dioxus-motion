use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::components::code_block::CodeBlock;

fn initial_card_style() -> MotionStyle {
    motion_style! {
        opacity: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        scale: 1.0,
        rotate: 0.0,
        rotateX: 0.0,
        rotateY: 0.0,
        skewX: 0.0,
        perspective: 900.0,
        border_radius: 22.0,
        color: "#dbeafe",
        background_color: "#2563eb",
        border_color: "#60a5fa",
        box_shadow: "0px 18px 36px rgba(37, 99, 235, 0.28)",
        letter_spacing: "0.2px",
    }
}

fn next_random(seed: &mut u32) -> f32 {
    *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    ((*seed >> 8) as f32) / 16_777_215.0
}

fn random_between(seed: &mut u32, min: f32, max: f32) -> f32 {
    min + (max - min) * next_random(seed)
}

fn random_card_style(seed: &mut u32) -> MotionStyle {
    let hue = random_between(seed, 0.0, 360.0);
    let background = format!("hsl({hue:.0} 72% 42%)");
    let border = format!("hsl({hue:.0} 86% 72%)");
    let foreground = format!("hsl({hue:.0} 90% 94%)");
    let shadow = format!(
        "0px {:.0}px {:.0}px hsl({hue:.0} 72% 42% / 0.38)",
        random_between(seed, 18.0, 38.0),
        random_between(seed, 36.0, 76.0),
    );

    motion_style! {
        opacity: random_between(seed, 0.78, 1.0),
        x: random_between(seed, -58.0, 58.0),
        y: random_between(seed, -34.0, 100.0),
        z: random_between(seed, -20.0, 96.0),
        scale: random_between(seed, 0.82, 1.16),
        rotate: random_between(seed, -18.0, 18.0),
        rotateX: random_between(seed, -18.0, 18.0),
        rotateY: random_between(seed, -28.0, 28.0),
        skewX: random_between(seed, -7.0, 7.0),
        skewY: random_between(seed, -5.0, 5.0),
        perspective: random_between(seed, 650.0, 1200.0),
        border_radius: random_between(seed, 10.0, 40.0),
        color: foreground,
        background_color: background,
        border_color: border,
        box_shadow: shadow,
        letter_spacing: format!("{:.1}px", random_between(seed, 0.0, 1.4)),
    }
}

#[component]
/// Renders the `motion_style!` guide with an interactive multi-property style animation.
pub fn MotionStyleGuide() -> Element {
    rsx! {
        div { class: "space-y-10",
            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold text-text-primary", "motion_style! Guide" }
                p { class: "text-text-secondary leading-relaxed",
                    "Use ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "motion_style!" },
                    " when an element has one visual state made from many CSS properties. Instead of coordinating separate ",
                    code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "use_motion()" },
                    " hooks for transform, opacity, size, color, and shadows, animate one typed ",
                    code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "MotionStyle" },
                    " value."
                }
            }

            MotionStyleShowcase {}

            section { class: "space-y-4",
                h3 { class: "text-xl font-semibold text-text-primary", "The Pattern" }
                div { class: "bg-dark-200/50 backdrop-blur-xs rounded-xl p-6 border border-primary/10",
                    CodeBlock {
                        code: MOTION_STYLE_EXAMPLE.to_string(),
                        language: "rust".to_string(),
                    }
                }
            }

            section { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10 space-y-2",
                    h3 { class: "font-medium text-primary", "What the macro buys you" }
                    ul { class: "list-disc list-inside text-sm text-text-secondary space-y-1",
                        li { "One animation handle owns the whole visual state." }
                        li { "Transform fields compose with CSS properties in one target." }
                        li { "Numbers, lengths, parsed colors, and compatible complex strings interpolate together." }
                    }
                }
                div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10 space-y-2",
                    h3 { class: "font-medium text-primary", "When to keep basic hooks" }
                    ul { class: "list-disc list-inside text-sm text-text-secondary space-y-1",
                        li { "A single scalar drives layout or business logic." }
                        li { "Different properties need independent timing or lifecycle control." }
                        li { "You are animating a custom domain type instead of CSS." }
                    }
                }
            }

            section { class: "space-y-4",
                h3 { class: "text-xl font-semibold text-text-primary", "Color formats" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10 space-y-2",
                        h4 { class: "font-medium text-primary", "Interpolated by MotionStyle" }
                        p { class: "text-sm text-text-secondary leading-relaxed",
                            "Hex, ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "rgb()" },
                            "/", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "rgba()" },
                            ", and ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "hsl()" },
                            "/", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "hsla()" },
                            " are parsed into RGBA channels and can be blended inside properties like ",
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "color" },
                            " and ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "box-shadow" },
                            "."
                        }
                    }
                    div { class: "p-4 bg-dark-200/50 rounded-lg border border-primary/10 space-y-2",
                        h4 { class: "font-medium text-primary", "Passed through as CSS" }
                        p { class: "text-sm text-text-secondary leading-relaxed",
                            "Browser-native color functions such as ",
                            code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "oklch()" },
                            ", ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "lab()" },
                            ", ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "lch()" },
                            ", and ", code { class: "text-primary/90 bg-primary/10 px-1 py-0.5 rounded-sm", "color()" },
                            " remain CSS strings. They can be applied, but Dioxus Motion does not numerically interpolate those color spaces yet."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MotionStyleShowcase() -> Element {
    let mut seed = use_signal(|| 0xC0FFEE_u32);
    let mut target_count = use_signal(|| 0_u32);
    let mut surface = use_motion(initial_card_style());

    let randomize = move |_| {
        let mut next_seed = seed();
        let target = random_card_style(&mut next_seed);
        seed.set(next_seed);
        target_count.set(target_count() + 1);
        surface.animate_to(
            target,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 120.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    };

    let pulse = move |_| {
        target_count.set(target_count() + 1);
        surface.animate_to(
            motion_style! {
                opacity: 1.0,
                x: 0.0,
                y: -28.0,
                z: 80.0,
                scale: 1.14,
                rotate: 10.0,
                rotateX: 12.0,
                rotateY: -20.0,
                skew: 0.0,
                perspective: 900.0,
                border_radius: 36.0,
                color: "#f0fdf4",
                background_color: "#16a34a",
                border_color: "#86efac",
                box_shadow: "0px 34px 68px rgba(22, 163, 74, 0.42)",
                letter_spacing: "1.2px",
            },
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(520),
                easing: easer::functions::Back::ease_out,
            })),
        );
    };

    let current_style = surface.get_value().to_css();
    rsx! {
        div { class: "min-h-[600px] p-6 bg-dark-200/30 rounded-lg border border-primary/10 overflow-hidden",
            div { class: "flex flex-wrap gap-2 mb-6",
                button {
                    class: "px-4 py-2 rounded-lg bg-primary/25 text-primary border border-primary/40 hover:bg-primary/35",
                    onclick: randomize,
                    "Randomize"
                }
                button {
                    class: "px-4 py-2 rounded-lg bg-secondary/20 text-secondary border border-secondary/30 hover:bg-secondary/30",
                    onclick: pulse,
                    "Pulse"
                }
            }

            div { class: "relative h-[400px] flex items-center justify-center [perspective:1200px]",
                div {
                    class: "relative border-2 p-5 flex flex-col justify-between overflow-hidden",
                    style: "{current_style}; transform-style: preserve-3d;",
                    div { class: "space-y-2",
                        div { class: "text-xs uppercase font-semibold opacity-80", "Color, shape, transform" }
                        div { class: "text-3xl font-bold leading-tight", "One motion handle" }
                    }
                    div { class: "grid grid-cols-3 gap-2 text-xs",
                        div { class: "rounded bg-white/15 px-2 py-1", "Transform" }
                        div { class: "rounded bg-white/15 px-2 py-1", "Color" }
                        div { class: "rounded bg-white/15 px-2 py-1", "Shape" }
                    }
                }
            }
        }
    }
}

const MOTION_STYLE_EXAMPLE: &str = r##"use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
fn Card() -> Element {
    let mut seed = use_signal(|| 0xC0FFEE_u32);
    let mut style = use_motion(motion_style! {
        opacity: 0.92,
        x: -18.0,
        y: 8.0,
        scale: 0.96,
        rotateY: -16.0,
        perspective: 900.0,
        border_radius: 22.0,
        color: "#dbeafe",
        background_color: "#2563eb",
        border_color: "#60a5fa",
        box_shadow: "0px 18px 36px rgba(37, 99, 235, 0.28)",
    });

    let randomize = move |_| {
        let mut next_seed = seed();
        let hue = next_number(&mut next_seed, 0.0, 360.0);
        let color = format!("hsl({hue:.0} 90% 94%)");
        let background = format!("hsl({hue:.0} 72% 42%)");
        let border = format!("hsl({hue:.0} 86% 72%)");

        style.animate_to(
            motion_style! {
                opacity: next_number(&mut next_seed, 0.78, 1.0),
                x: next_number(&mut next_seed, -58.0, 58.0),
                y: next_number(&mut next_seed, -34.0, 34.0),
                z: next_number(&mut next_seed, -20.0, 96.0),
                scale: next_number(&mut next_seed, 0.82, 1.16),
                rotate: next_number(&mut next_seed, -18.0, 18.0),
                rotateX: next_number(&mut next_seed, -18.0, 18.0),
                rotateY: next_number(&mut next_seed, -28.0, 28.0),
                perspective: next_number(&mut next_seed, 650.0, 1200.0),
                border_radius: next_number(&mut next_seed, 10.0, 40.0),
                color,
                background_color: background,
                border_color: border,
            },
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );

        seed.set(next_seed);
    };

    rsx! {
        button { onclick: randomize, "Randomize" }
        div { style: "{style.get_value().to_css()}", "Animated style" }
    }
}

fn next_number(seed: &mut u32, min: f32, max: f32) -> f32 {
    *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    min + (max - min) * (((*seed >> 8) as f32) / 16_777_215.0)
}"##;
