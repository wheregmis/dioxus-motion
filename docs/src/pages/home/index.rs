use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

use crate::components::animated_flower::AnimatedFlower;
use crate::components::cube_animation::SwingingCube;
use crate::{components::transformation_example::TransformAnimationShowcase, utils::router::Route};

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
        section {
            class: "h-screen bg-gradient-dark relative overflow-hidden flex flex-col",
            style: "opacity: {hero_opacity.get_value()}",

            // Content overlay
            div { class: "relative z-10",
                // Main content
                div { class: "container mx-auto px-4 pt-8 flex items-center justify-between",
                    // Left side - Simple animation
                    div { class: "w-1/4 hidden lg:block",
                        div { class: "flex flex-col items-center gap-4",
                            TransformAnimationShowcase {}
                            div { class: "text-center",
                                span { class: "inline-block text-lg font-medium bg-clip-text text-transparent
                                           bg-gradient-to-r from-text-secondary/70 to-text-secondary/40
                                           tracking-wide transform -rotate-12",
                                    "From Simple"
                                }
                                div { class: "mt-2 text-sm text-text-muted", "Basic Transitions" }
                            }
                        }
                    }

                    // Center content
                    div { class: "text-center lg:w-2/4",
                        h1 { class: "text-5xl font-bold mb-4",
                            span { class: "text-gradient-primary", "Dioxus Motion" }
                        }
                        p { class: "text-xl text-text-secondary mb-8",
                            "Simple and powerful animations for your Dioxus applications"
                        }

                        div { class: "my-8", AnimatedFlower {} }

                        // CTA buttons
                        div { class: "flex justify-center gap-4",
                            Link {
                                to: Route::DocsLanding {},
                                class: "px-6 py-2 bg-primary/90 backdrop-blur-sm text-dark-50 rounded-xl
                                       font-semibold transition-all duration-300 hover:scale-105 
                                       shadow-lg shadow-primary/20",
                                "Get Started →"
                            }
                            a {
                                href: "https://github.com/wheregmis/dioxus-motion",
                                target: "_blank",
                                class: "px-6 py-2 bg-dark-200/50 backdrop-blur-sm text-white/90 rounded-xl
                                       font-semibold transition-all duration-300 hover:scale-105 
                                       border border-white/10",
                                "Explore Examples"
                            }
                        }
                    }

                    // Right side - Advanced animation
                    div { class: "w-1/4 hidden lg:block",
                        div { class: "flex flex-col items-center gap-4",
                            SwingingCube {}
                            div { class: "text-center",
                                span { class: "inline-block text-lg font-medium bg-clip-text text-transparent
                                           bg-gradient-to-r from-text-secondary/70 to-text-secondary/40
                                           tracking-wide transform rotate-12",
                                    "To Advanced"
                                }
                                div { class: "mt-2 text-sm text-text-muted",
                                    "Complex Custom Transitions"
                                }
                            }
                        }
                    }
                }
            }

            // Features section
            section { class: "container mx-auto px-4 py-20 pb-4 relative z-10",
                h2 { class: "text-2xl font-bold text-center mb-6 text-gradient-primary",
                    "Features"
                }
                div { class: "grid grid-cols-1 md:grid-cols-4 gap-4 max-w-4xl mx-auto",
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
        }
    }
}

#[component]
fn FeatureCard(title: &'static str, description: &'static str, icon: &'static str) -> Element {
    rsx! {
        div { class: "p-4 rounded-lg bg-dark-200/50 backdrop-blur-sm
                    border border-primary/10 transition-all duration-300
                    hover:border-primary/20 hover:scale-105",
            div { class: "flex items-center gap-2 mb-2",
                span { class: "text-xl", {icon} }
                h3 { class: "text-lg font-medium text-text-primary", {title} }
            }
            p { class: "text-sm text-text-secondary", {description} }
        }
    }
}
