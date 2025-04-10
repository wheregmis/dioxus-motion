use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::components::code_block::CodeBlock;
use crate::components::footer::Footer;
use crate::old_showcase::components::{AnimatedFlower, SwingingCube, TransformAnimationShowcase};
use crate::utils::router::Route;

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
    // Combine related animations into fewer motion values
    let mut hero_opacity = use_motion(0.0f32);
    let mut demo_scale = use_motion(0.95f32);
    let mut cta_opacity = use_motion(0.0f32);
    let mut features_opacity = use_motion(0.0f32);
    let mut bg_rotate = use_motion(0.0f32);

    use_effect(move || {
        // Use more performant spring settings
        let spring = Spring {
            stiffness: 100.0, // Reduced from original values
            damping: 20.0,    // Increased for less oscillation
            mass: 1.0,        // Reduced mass
            velocity: 0.0,
        };

        // Optimize animations with better performance settings
        hero_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: std::time::Duration::from_millis(400), // Reduced duration
                easing: easer::functions::Cubic::ease_out,
            })),
        );

        demo_scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(spring))
                .with_delay(std::time::Duration::from_millis(100)), // Reduced delay
        );

        cta_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: std::time::Duration::from_millis(400),
                easing: easer::functions::Cubic::ease_out,
            }))
            .with_delay(std::time::Duration::from_millis(300)),
        );

        features_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: std::time::Duration::from_millis(400),
                easing: easer::functions::Cubic::ease_out,
            }))
            .with_delay(std::time::Duration::from_millis(400)),
        );

        // Optimize background rotation
        bg_rotate.animate_to(
            360.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: std::time::Duration::from_millis(30000), // Reduced from 60s to 30s
                easing: easer::functions::Linear::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite),
        );
    });

    rsx! {
        section {
            class: "min-h-screen bg-gradient-dark relative overflow-hidden flex flex-col",


            // Optimize background animation
            div {
                class: "absolute inset-0 overflow-hidden will-change-transform", // Added will-change-transform
                div {
                    class: "absolute top-0 left-0 w-[200%] h-[200%] -translate-x-1/4 -translate-y-1/4",

                    div { class: "absolute top-1/4 left-1/4 w-1/2 h-1/2 bg-primary/5 rounded-full blur-3xl" }
                    div { class: "absolute bottom-1/4 right-1/4 w-1/2 h-1/2 bg-secondary/5 rounded-full blur-3xl" }
                    div { class: "absolute top-1/3 right-1/3 w-1/3 h-1/3 bg-primary/3 rounded-full blur-3xl" }
                }
            }

            // Content overlay
            div { class: "relative z-10 flex-1",
                // Main content
                div { class: "container mx-auto px-4 pt-12 md:pt-16",
                    // Hero section with animations in a row
                    div {
                        class: "flex flex-col lg:flex-row items-center justify-between gap-8 mb-16",


                        // Left side - Simple animation
                        div { class: "w-full lg:w-1/3",
                            div { class: "flex flex-col items-center gap-4",
                                div { class: "relative p-4 bg-dark-200/30 backdrop-blur-sm rounded-xl border border-primary/10",
                                    TransformAnimationShowcase {}
                                }
                                div { class: "text-center",
                                    span { class: "inline-block text-lg font-medium bg-clip-text text-transparent
                                               bg-gradient-to-r from-text-secondary/70 to-text-secondary/40
                                               tracking-wide transform -rotate-6",
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
                                div { class: "relative p-4 bg-dark-200/30 backdrop-blur-sm rounded-xl border border-primary/10",
                                    AnimatedFlower {}
                                }
                                div { class: "text-center",
                                    span { class: "inline-block text-lg font-medium bg-clip-text text-transparent
                                               bg-gradient-to-r from-primary/70 to-primary/40
                                               tracking-wide",
                                        "Through Intermediate"
                                    }
                                    div { class: "mt-2 text-sm text-text-muted", "Complex Animations" }
                                }
                            }
                        }

                        // Right side - Advanced animation
                        div { class: "w-full lg:w-1/3",
                            div { class: "flex flex-col items-center gap-4",
                                div { class: "relative p-4 bg-dark-200/30 backdrop-blur-sm rounded-xl border border-primary/10",
                                    SwingingCube {}
                                }
                                div { class: "text-center",
                                    span { class: "inline-block text-lg font-medium bg-clip-text text-transparent
                                               bg-gradient-to-r from-text-secondary/70 to-text-secondary/40
                                               tracking-wide transform rotate-6",
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
                        // Title with animation
                        div {
                            h1 { class: "text-4xl md:text-5xl lg:text-6xl font-bold mb-4",
                                span { class: "text-gradient-primary", "Dioxus Motion" }
                            }
                            p { class: "text-lg md:text-xl text-text-secondary mb-4",
                                "Simple, powerful animations for your "
                                span { class: "text-primary font-medium", "Dioxus" }
                                " applications"
                            }
                            p { class: "text-base text-text-secondary/80 max-w-2xl mx-auto mb-8",
                                "Create fluid, physics-based animations with minimal code. "
                                "From simple transitions to complex interactive effects, "
                                "Dioxus Motion makes animation easy."
                            }
                        }

                        // CTA buttons with animation
                        div {
                            class: "flex flex-col sm:flex-row justify-center gap-4",
                            style: "opacity: {cta_opacity.get_value()}",
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
            section {
                class: "container mx-auto px-4 py-20 pb-4 relative z-10",
                style: "opacity: {features_opacity.get_value()}",
                h2 { class: "text-3xl font-bold text-center mb-6 text-gradient-primary",
                    "Features"
                }
                p { class: "text-center text-text-secondary max-w-2xl mx-auto mb-12",
                    "Dioxus Motion provides a comprehensive set of tools to create beautiful, "
                    "performant animations with minimal effort."
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 max-w-5xl mx-auto",
                    FeatureCard {
                        title: "Spring Physics",
                        description: "Natural, fluid animations with customizable spring parameters for realistic motion",
                        icon: "🌊",
                    }
                    FeatureCard {
                        title: "Easy to Use",
                        description: "Intuitive API that integrates seamlessly with Dioxus components and hooks",
                        icon: "🎯",
                    }
                    FeatureCard {
                        title: "Cross Platform",
                        description: "Works consistently across Web, Desktop, and Mobile platforms",
                        icon: "🌐",
                    }
                    FeatureCard {
                        title: "Page Transitions",
                        description: "Smooth, declarative animations for route changes with minimal configuration",
                        icon: "🔄",
                    }
                }
            }

            // Why Dioxus Motion section
            section {
                class: "container mx-auto px-4 py-16 relative z-10",
                style: "opacity: {features_opacity.get_value()}",
                h2 { class: "text-3xl font-bold text-center mb-6 text-gradient-primary",
                    "Why Dioxus Motion?"
                }

                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 max-w-5xl mx-auto mt-12",
                    // Left side - Code example
                    div { class: "bg-dark-200/50 backdrop-blur-sm rounded-xl overflow-hidden border border-primary/10",
                        // Code header with title and run button
                        div { class: "p-6 pb-2",
                            div { class: "flex justify-between items-center mb-4",
                                h3 { class: "text-xl font-medium text-text-primary",
                                    "Simple, Declarative API"
                                }


                            }
                        }
                        // Code block
                        div { class: "px-6 pb-6",
                            CodeBlock {
                                code: r#"// Create an animatable value
let mut scale = use_motion(1.0f32);

// Animate on hover
let hover = move |_| {
    scale.animate_to(
        1.2,  // Target value
        AnimationConfig::new(
            AnimationMode::Spring(Spring {
                stiffness: 180.0,
                damping: 12.0,
                mass: 1.0,
                velocity: 0.0,
            })
        )
    );
};

// Use in your component
rsx! {
    button {
        style: "transform: scale({{scale.get_value()}})",
        onmouseenter: hover,
        "Hover me!"
    }
}
"#.to_string(),
                                language: "rust".to_string(),
                            }
                        }
                    }

                    // Right side - Benefits
                    div { class: "flex flex-col justify-center",
                        div { class: "space-y-6",
                            div { class: "flex items-start gap-4",
                                div { class: "w-10 h-10 flex items-center justify-center bg-primary/20 text-primary rounded-full flex-shrink-0",
                                    "💡"
                                }
                                div {
                                    h4 { class: "text-lg font-medium text-text-primary mb-1",
                                        "Intuitive Mental Model"
                                    }
                                    p { class: "text-text-secondary",
                                        "Based on real-world physics concepts that make animations feel natural and predictable."
                                    }
                                }
                            }

                            div { class: "flex items-start gap-4",
                                div { class: "w-10 h-10 flex items-center justify-center bg-primary/20 text-primary rounded-full flex-shrink-0",
                                    "💪"
                                }
                                div {
                                    h4 { class: "text-lg font-medium text-text-primary mb-1",
                                        "Powerful Customization"
                                    }
                                    p { class: "text-text-secondary",
                                        "Fine-tune every aspect of your animations while maintaining a clean, readable codebase."
                                    }
                                }
                            }

                            div { class: "flex items-start gap-4",
                                div { class: "w-10 h-10 flex items-center justify-center bg-primary/20 text-primary rounded-full flex-shrink-0",
                                    "🚀"
                                }
                                div {
                                    h4 { class: "text-lg font-medium text-text-primary mb-1",
                                        "Performance Focused"
                                    }
                                    p { class: "text-text-secondary",
                                        "Optimized for smooth 60fps animations even on complex UIs and lower-end devices."
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Footer
            Footer {}
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
    rsx! {
        div {
            class: "p-6 rounded-xl bg-dark-200/50 backdrop-blur-sm
                    border border-primary/10 transition-all duration-300
                    hover:border-primary/20 hover:shadow-lg hover:shadow-primary/5
                    h-full flex flex-col will-change-transform", // Added will-change-transform

            // Icon with animation
            div { class: "flex items-center gap-3 mb-4",
                div {
                    class: "w-12 h-12 flex items-center justify-center bg-primary/10 rounded-lg text-primary will-change-transform",
                    span { class: "text-2xl", {icon} }
                }
                h3 { class: "text-xl font-medium text-text-primary", {title} }
            }

            // Description
            p { class: "text-text-secondary leading-relaxed mt-2", {description} }
        }
    }
}
