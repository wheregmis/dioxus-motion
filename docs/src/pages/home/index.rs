use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::utils::router::Route;

#[component]
pub fn Home() -> Element {
    let mut hero_opacity = use_motion(0.0f32);
    let mut demo_scale = use_motion(1.0f32);

    use_effect(move || {
        hero_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(600),
                easing: easer::functions::Cubic::ease_out,
            })),
        );

        demo_scale.animate_to(
            1.1,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            }))
            .with_loop(LoopMode::Infinite),
        );
    });

    rsx! {
        // Hero Section with animated background
        section {
            class: "min-h-screen bg-gradient-dark relative overflow-hidden",
            style: "opacity: {hero_opacity.get_value()}",

            // Content overlay
            div { class: "relative z-10",
                // Main content
                div { class: "container mx-auto px-4 py-24 text-center",
                    h1 { class: "text-6xl font-bold mb-6",
                        span { class: "text-gradient-primary", "Dioxus Motion" }
                    }
                    p { class: "text-xl text-text-secondary mb-12",
                        "Simple and powerful animations for your Dioxus applications"
                    }

                    // Demo animation
                    div {
                        class: "my-16",
                        style: "transform: scale({demo_scale.get_value()})",
                        div { class: "w-32 h-32 mx-auto bg-primary/80 backdrop-blur-sm rounded-2xl
                                   shadow-lg shadow-primary/20 border border-primary/30" }
                    }

                    // CTA buttons
                    div { class: "flex justify-center gap-6",
                        Link {
                            to: Route::DocsLanding {},
                            class: "px-6 py-3 bg-primary/90 backdrop-blur-sm text-dark-50 rounded-xl
                                   font-semibold transition-all duration-300 hover:scale-105 
                                   shadow-lg shadow-primary/20",
                            "Get Started →"
                        }
                        a {
                            href: "https://github.com/wheregmis/dioxus-motion",
                            target: "_blank",
                            class: "px-6 py-3 bg-dark-200/50 backdrop-blur-sm text-white/90 rounded-xl
                                   font-semibold transition-all duration-300 hover:scale-105 
                                   border border-white/10",
                            "GitHub ★"
                        }
                    }
                }
            }

            // Features section
            section { class: "container mx-auto px-4 py-24 relative z-10",
                h2 { class: "text-3xl font-bold text-center mb-12 text-gradient-primary",
                    "Features"
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 max-w-4xl mx-auto",
                    FeatureCard {
                        title: "Spring Physics",
                        description: "Natural animations with customizable spring parameters",
                        icon: "🌊",
                    }
                    FeatureCard {
                        title: "Easy to Use",
                        description: "Simple API with powerful configuration options",
                        icon: "🎯",
                    }
                    FeatureCard {
                        title: "Cross Platform",
                        description: "Works on Web, Desktop, and Mobile",
                        icon: "🌐",
                    }
                }
            }

            AnimationShowcase {}
        }
    }
}

#[component]
fn FeatureCard(title: &'static str, description: &'static str, icon: &'static str) -> Element {
    rsx! {
        div { class: "p-6 rounded-lg bg-dark-200/50 backdrop-blur-sm
                    border border-primary/10 transition-all duration-300
                    hover:border-primary/20 hover:scale-105",
            div { class: "flex items-center gap-3 mb-3",
                span { class: "text-2xl", {icon} }
                h3 { class: "text-xl font-medium text-text-primary", {title} }
            }
            p { class: "text-text-secondary", {description} }
        }
    }
}

#[component]
fn AnimationShowcase() -> Element {
    rsx! {
        section { class: "container mx-auto px-4 py-24 relative z-10",
            h2 { class: "text-3xl font-bold text-center mb-12 text-gradient-primary",
                "Quick Start Examples"
            }
            // Spring Animation Example
            ShowcaseCard {
                title: "Spring Animation",
                description: "Natural, physics-based animations",
                code: {"let mut scale = use_motion(1.0f32);

scale.animate_to(
    1.2,
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
    }))
);"},
                demo: rsx! {
                    SpringDemo {}
                },
            }

            // Tween Animation Example
            ShowcaseCard {
                title: "Tween Animation",
                description: "Time-based animations with easing functions",
                code: {"let mut opacity = use_motion(0.0f32);

opacity.animate_to(
    1.0,
    AnimationConfig::new(AnimationMode::Tween(Tween {
        duration: Duration::from_millis(800),
        easing: easer::functions::Cubic::ease_in_out,
    }))
);"},
                demo: rsx! {
                    TweenDemo {}
                },
            }

            // Transform Animation Example
            ShowcaseCard {
                title: "Transform Animation",
                description: "Animate position, scale, and rotation",
                code: "let mut transform = use_motion(Transform::identity());

transform.animate_to(
    Transform::new(100.0, 0.0, 1.2, 45.0),  // x, y, scale, rotation
    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
);",
                demo: rsx! {
                    TransformDemo {}
                },
            }
        }
    }
}

#[component]
fn ShowcaseCard(
    title: &'static str,
    description: &'static str,
    code: &'static str,
    demo: Element,
) -> Element {
    let mut is_playing = use_signal(|| false);

    rsx! {
        div { class: "mb-16",
            div { class: "mb-8 text-center",
                h3 { class: "text-2xl font-semibold mb-2 text-text-primary", {title} }
                p { class: "text-text-secondary", {description} }
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                // Code example with play button
                div { class: "relative",
                    div {
                        class: "bg-dark-300/50 backdrop-blur-sm rounded-lg p-6 font-mono text-sm overflow-x-auto",
                        pre { {code} }
                    }
                    button {
                        class: "absolute bottom-4 right-4 px-4 py-2 bg-primary/90 text-dark-50
                               rounded-lg font-semibold transition-all duration-300 hover:scale-105 
                               flex items-center gap-2 text-sm",
                        onclick: move |_| is_playing.set(true),
                        disabled: *is_playing.read(),
                        if *is_playing.read() {
                           span { class: "animate-spin", "↻" }
                                "Playing..."
                        } else {
                            span { "▶" }
                            "Play"
                        }
                    }
                }
                // Live demo
                div {
                    class: "bg-dark-200/50 backdrop-blur-sm rounded-lg p-6 flex items-center justify-center",
                    DemoWrapper {
                        is_playing: *is_playing.read(),
                        on_complete: move || is_playing.set(false),
                        demo: demo
                    }
                }
            }
        }
    }
}

#[component]
fn DemoWrapper(is_playing: bool, on_complete: EventHandler<()>, demo: Element) -> Element {
    use_effect(move || {
        if !is_playing {
            // Simulate animation duration
            on_complete.call(());
        }
    });

    demo
}

#[component]
fn SpringDemo() -> Element {
    let mut scale = use_motion(1.0f32);
    let is_playing = use_signal(|| false);

    use_effect(move || {
        if *is_playing.read() {
            scale.animate_to(
                1.2,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );
        } else {
            scale.animate_to(
                1.0,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );
        }
    });

    rsx! {
        div {
            class: "w-24 h-24 bg-primary rounded-lg",
            style: "transform: scale({scale.get_value()})",
        }
    }
}

#[component]
fn TweenDemo() -> Element {
    let mut opacity = use_motion(0.0f32);
    let is_playing = use_signal(|| false);

    use_effect(move || {
        if *is_playing.read() {
            opacity.animate_to(
                1.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(800),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            );
        } else {
            opacity.animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(800),
                    easing: easer::functions::Cubic::ease_in_out,
                })),
            );
        }
    });

    rsx! {
        div {
            class: "w-24 h-24 bg-secondary rounded-lg",
            style: "opacity: {opacity.get_value()}",
        }
    }
}

#[component]
fn TransformDemo() -> Element {
    let mut transform = use_motion(Transform::identity());
    let is_playing = use_signal(|| false);

    use_effect(move || {
        if *is_playing.read() {
            transform.animate_to(
                Transform::new(
                    50.0,  // x
                    -20.0, // y
                    1.2,   // scale
                    45.0,  // rotation (degrees)
                ),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );
        } else {
            transform.animate_to(
                Transform::identity(),
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );
        }
    });

    rsx! {
        div {
            class: "w-24 h-24 bg-accent rounded-lg",
            style: "transform:
                translate({transform.get_value().x}px, {transform.get_value().y}px)
                scale({transform.get_value().scale})
                rotate({transform.get_value().rotation}deg)",
        }
    }
}
