use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::components::animated_flower::AnimatedFlower;
use crate::components::cube_animation::SwingingCube;
use crate::{components::transformation_example::TransformAnimationShowcase, utils::router::Route};

#[component]
/// Renders the home page with animated elements for a dynamic user experience.
/// 
/// This component initializes several motion states for animations such as fading the hero section,
/// translating the title and subtitle, and continuously scaling a demo element. When the component mounts,
/// these animations are triggered using spring and tween configurations. The layout includes a hero
/// section with animated backgrounds, a content area with a title, subtitle, call-to-action buttons,
/// a features grid showcasing various functionality, and a footer with external links.
/// 
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// fn main() {
///     dioxus_web::launch(app);
/// }
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         Home {}
///     })
/// }
/// ```
pub fn Home() -> Element {
    let mut hero_opacity = use_motion(0.0f32);
    let mut demo_scale = use_motion(1.0f32);
    let mut title_y = use_motion(-20.0f32);
    let mut subtitle_y = use_motion(20.0f32);

    use_effect(move || {
        // Stagger the animations
        {
            title_y.animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );

            subtitle_y.animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 15.0,
                    mass: 1.0,
                    velocity: 0.0,
                })),
            );

            hero_opacity.animate_to(
                1.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(800),
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
        }
    });

    rsx! {
        section {
            class: "min-h-screen bg-gradient-dark relative overflow-hidden flex flex-col",
            style: "opacity: {hero_opacity.get_value()}",

            // Animated background elements
            div { class: "absolute inset-0 overflow-hidden",
                div { class: "absolute -top-1/2 -left-1/2 w-full h-full bg-primary/5 rounded-full blur-3xl" }
                div { class: "absolute -bottom-1/2 -right-1/2 w-full h-full bg-secondary/5 rounded-full blur-3xl" }
            }

            // Content overlay
            div { class: "relative z-10 flex-1",
                // Main content
                div { class: "container mx-auto px-4 pt-8",
                    // Hero section with animations in a row
                    div { class: "flex flex-col lg:flex-row items-center justify-between gap-8 mb-12",
                        // Left side - Simple animation
                        div { class: "w-full lg:w-1/3",
                            div { class: "flex flex-col items-center gap-4",
                                TransformAnimationShowcase {}
                                div { class: "text-center",
                                    span { class: "inline-block text-lg font-medium bg-clip-text text-transparent
                                               bg-gradient-to-r from-text-secondary/70 to-text-secondary/40
                                               tracking-wide transform -rotate-12",
                                        "From Simple"
                                    }
                                    div { class: "mt-2 text-sm text-text-muted",
                                        "Basic Transformations"
                                    }
                                }
                            }
                        }

                        // Center content - Flower
                        div { class: "w-full lg:w-1/3",
                            div { class: "flex flex-col items-center gap-4",
                                AnimatedFlower {}
                                div { class: "text-center",
                                    span { class: "inline-block text-lg font-medium bg-clip-text text-transparent
                                               bg-gradient-to-r from-text-secondary/70 to-text-secondary/40
                                               tracking-wide",
                                        ""
                                    }
                                    div { class: "mt-2 text-sm text-text-muted", "Complex Animations" }
                                }
                            }
                        }

                        // Right side - Advanced animation
                        div { class: "w-full lg:w-1/3",
                            div { class: "flex flex-col items-center gap-4",
                                SwingingCube {}
                                div { class: "text-center",
                                    span { class: "inline-block text-lg font-medium bg-clip-text text-transparent
                                               bg-gradient-to-r from-text-secondary/70 to-text-secondary/40
                                               tracking-wide transform rotate-12",
                                        "To Advanced"
                                    }
                                    div { class: "mt-2 text-sm text-text-muted",
                                        "Custom Transformations"
                                    }
                                }
                            }
                        }
                    }

                    // Title and CTA section
                    div { class: "text-center max-w-4xl mx-auto",
                        h1 {
                            class: "text-4xl md:text-5xl lg:text-6xl font-bold mb-4",
                            style: "transform: translateY({title_y.get_value()}px)",
                            span { class: "text-gradient-primary", "Dioxus Motion" }
                        }
                        p {
                            class: "text-lg md:text-xl text-text-secondary mb-8",
                            style: "transform: translateY({subtitle_y.get_value()}px)",
                            "Simple and powerful animations for your Dioxus applications"
                        }

                        // CTA buttons
                        div { class: "flex flex-col sm:flex-row justify-center gap-4",
                            Link {
                                to: Route::DocsLanding {},
                                class: "px-8 py-3 bg-primary/90 backdrop-blur-sm text-dark-50 rounded-xl
                                       font-semibold transition-all duration-300 hover:scale-105 
                                       shadow-lg shadow-primary/20 hover:shadow-primary/30",
                                "Get Started →"
                            }
                            a {
                                href: "https://github.com/wheregmis/dioxus-motion",
                                target: "_blank",
                                class: "px-8 py-3 bg-dark-200/50 backdrop-blur-sm text-white/90 rounded-xl
                                       font-semibold transition-all duration-300 hover:scale-105 
                                       border border-white/10 hover:border-white/20",
                                "Explore Examples"
                            }
                        }
                    }
                }
            }

            // Features section
            section { class: "container mx-auto px-4 py-20 pb-4 relative z-10",
                h2 { class: "text-3xl font-bold text-center mb-12 text-gradient-primary",
                    "Features"
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 max-w-5xl mx-auto",
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
                    FeatureCard {
                        title: "Page Transitions",
                        description: "Smooth animations for route changes",
                        icon: "🔄",
                    }
                }
            }

            // Footer
            footer { class: "relative z-10 border-t border-primary/10 mt-auto py-8",
                div { class: "container mx-auto px-4",
                    div { class: "flex flex-col items-center justify-center space-y-4 text-center",
                        // Made with love
                        p { class: "text-text-secondary",
                            "Made with "
                            span { class: "text-red-500 animate-pulse", "♥" }
                            " using "
                            a {
                                href: "https://dioxuslabs.com",
                                target: "_blank",
                                class: "text-primary hover:text-primary/80 transition-colors",
                                "Dioxus"
                            }
                        }
                        // Copyright
                        p { class: "text-text-muted text-sm",
                            "© 2025 Dioxus Motion. All rights reserved."
                        }
                        // Links
                        div { class: "flex items-center space-x-4 text-sm text-text-secondary",
                            a {
                                href: "https://github.com/wheregmis/dioxus-motion",
                                target: "_blank",
                                class: "hover:text-text-primary transition-colors",
                                "GitHub"
                            }
                            span { "·" }
                            a {
                                href: "https://crates.io/crates/dioxus-motion",
                                target: "_blank",
                                class: "hover:text-text-primary transition-colors",
                                "Crates.io"
                            }
                            span { "·" }
                            a {
                                href: "https://docs.rs/dioxus-motion",
                                target: "_blank",
                                class: "hover:text-text-primary transition-colors",
                                "Documentation"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
/// Creates a feature card component that displays an icon, title, and description with hover animations.
/// 
/// When hovered, the card scales up and shifts upward, providing a subtle interactive effect. This component
/// is designed for use in Dioxus-based interfaces to showcase key features alongside descriptive information.
/// 
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         FeatureCard("Feature Title", "This is a description.", "🔥")
///     })
/// }
/// ```
fn FeatureCard(title: &'static str, description: &'static str, icon: &'static str) -> Element {
    let mut card_scale = use_motion(1.0f32);
    let mut card_y = use_motion(0.0f32);

    rsx! {
        div {
            class: "p-6 rounded-xl bg-dark-200/50 backdrop-blur-sm
                    border border-primary/10 transition-all duration-300
                    hover:border-primary/20",
            style: "transform: translateY({card_y.get_value()}px) scale({card_scale.get_value()})",
            onmouseenter: move |_| {
                card_scale
                    .animate_to(
                        1.05,
                        AnimationConfig::new(
                            AnimationMode::Spring(Spring {
                                stiffness: 300.0,
                                damping: 20.0,
                                mass: 1.0,
                                velocity: 0.0,
                            }),
                        ),
                    );
                card_y
                    .animate_to(
                        -5.0,
                        AnimationConfig::new(
                            AnimationMode::Spring(Spring {
                                stiffness: 300.0,
                                damping: 20.0,
                                mass: 1.0,
                                velocity: 0.0,
                            }),
                        ),
                    );
            },
            onmouseleave: move |_| {
                card_scale
                    .animate_to(
                        1.0,
                        AnimationConfig::new(
                            AnimationMode::Spring(Spring {
                                stiffness: 300.0,
                                damping: 20.0,
                                mass: 1.0,
                                velocity: 0.0,
                            }),
                        ),
                    );
                card_y
                    .animate_to(
                        0.0,
                        AnimationConfig::new(
                            AnimationMode::Spring(Spring {
                                stiffness: 300.0,
                                damping: 20.0,
                                mass: 1.0,
                                velocity: 0.0,
                            }),
                        ),
                    );
            },
            div { class: "flex items-center gap-3 mb-4",
                span { class: "text-2xl", {icon} }
                h3 { class: "text-xl font-medium text-text-primary", {title} }
            }
            p { class: "text-text-secondary leading-relaxed", {description} }
        }
    }
}
