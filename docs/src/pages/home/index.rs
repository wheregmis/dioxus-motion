use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::components::animated_flower::AnimatedFlower;
use crate::components::cube_animation::SwingingCube;
use crate::{components::transformation_example::TransformAnimationShowcase, utils::router::Route};

#[component]
/// Renders the main landing page of the application.
///
/// This component initializes animated states for opacity, scale, and vertical positioning of key elements.
/// On mount, it triggers staggered spring and tween animations that animate the hero section, titles, and feature overlays,
/// creating a dynamic and engaging home page layout.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// // Adjust the import path below according to your project setup.
/// use your_crate::Home;
///
/// fn main() {
///     dioxus::web::launch(Home);
/// }
/// ```
pub fn Home() -> Element {
    let hero_opacity = use_motion(1.0f32); // Changed from 0.0 to 1.0
    let mut demo_scale = use_motion(1.0f32);
    // Remove these animations since we don't want them
    // let mut title_y = use_motion(-20.0f32);
    // let mut subtitle_y = use_motion(20.0f32);

    use_effect(move || {
        {
            // Remove title and subtitle animations
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
                        h1 { class: "text-4xl md:text-5xl lg:text-6xl font-bold mb-4",
                            // Remove the transform style
                            span { class: "text-gradient-primary", "Dioxus Motion" }
                        }
                        p { class: "text-lg md:text-xl text-text-secondary mb-8",
                            // Remove the transform style
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
/// Renders an animated feature card with a specified icon, title, and description.
///
/// This component displays a card that animates on hover by scaling up and shifting slightly upward,
/// then reverting to its original state when the mouse leaves. The animations are achieved using spring
/// dynamics to ensure smooth transitions.
///
/// # Arguments
///
/// * `title` - The title text displayed on the card.
/// * `description` - A brief description of the feature.
/// * `icon` - A static string representing the feature's icon (e.g., an emoji).
///
/// # Returns
///
/// A Dioxus `Element` representing the rendered feature card.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         FeatureCard("Efficiency", "Boosts performance significantly.", "⚡")
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
