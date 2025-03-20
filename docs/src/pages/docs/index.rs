use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::utils::router::Route;

#[component]
pub fn Docs() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-dark dark:from-dark-50 dark:to-dark-100",
            div { class: "max-w-4xl mx-auto px-6 py-12",
                h1 { class: "text-4xl font-bold bg-clip-text text-transparent bg-gradient-primary mb-4",
                    "Documentation"
                }
                AnimatedOutlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn DocsLanding() -> Element {
    rsx! {
        div { class: "space-y-12",
            // Introduction section
            section { class: "prose max-w-none",
                h2 { class: "text-3xl font-bold mb-6 text-text-primary dark:text-text-primary",
                    "Getting Started with Dioxus Motion"
                }
                p { class: "text-text-secondary dark:text-text-secondary mb-8",
                    "Dioxus Motion provides powerful animation capabilities for your Dioxus applications. Choose a guide below to get started:"
                }
            }

            // Guide sections
            div { class: "grid md:grid-cols-2 gap-6",
                // Page Transitions Card
                Link {
                    to: Route::PageTransition {},
                    class: "group card hover:shadow-lg",
                    div {
                        class: "flex items-center justify-between mb-4",
                        h3 { class: "text-xl font-semibold text-text-primary dark:text-text-primary", "Page Transitions" }
                        span { class: "text-primary group-hover:translate-x-1 transition-transform", "→" }
                    }
                    p { class: "text-text-secondary dark:text-text-secondary",
                        "Learn how to create smooth page transitions and routing animations in your Dioxus app."
                    }
                }

                // Animations Card
                Link {
                    to: Route::Animations {},
                    class: "group card hover:shadow-lg",
                    div {
                        class: "flex items-center justify-between mb-4",
                        h3 { class: "text-xl font-semibold text-text-primary dark:text-text-primary", "Animations" }
                        span { class: "text-primary group-hover:translate-x-1 transition-transform", "→" }
                    }
                    p { class: "text-text-secondary dark:text-text-secondary",
                        "Master component animations using springs, tweens, and transforms."
                    }
                }
            }
        }
    }
}

#[component]
pub fn PageTransition() -> Element {
    rsx! {
        div { class: "space-y-8",
            h1 { class: "text-4xl font-bold text-text-primary dark:text-text-primary mb-4", "Page Transitions" }

            // Introduction
            section { class: "prose max-w-none",
                p { class: "text-lg text-text-secondary dark:text-text-secondary",
                    "Page transitions in Dioxus Motion allow you to create smooth animations between route changes."
                }
            }

            // Setup Guide
            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold text-text-primary dark:text-text-primary", "Setup" }
                div { class: "bg-dark-50 dark:bg-dark-100 rounded-lg p-6",
                    pre { class: "language-rust",
                        code {
                            {"#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[route(\"/\")]
    #[transition(SlideDown)]
    Home {},

    #[route(\"/about\")]
    #[transition(SlideLeft)]
    About {},
}"}
                        }
                    }
                }
            }

            // Available Transitions
            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold text-text-primary dark:text-text-primary", "Available Transitions" }
                ul { class: "grid grid-cols-2 gap-4",
                    li { class: "card",
                        span { class: "font-semibold", "SlideDown" }
                    }
                    li { class: "card",
                        span { class: "font-semibold", "SlideUp" }
                    }
                    li { class: "card",
                        span { class: "font-semibold", "SlideLeft" }
                    }
                    li { class: "card",
                        span { class: "font-semibold", "SlideRight" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Animations() -> Element {
    rsx! {
        div { class: "space-y-8",
            h1 { class: "text-4xl font-bold text-text-primary dark:text-text-primary mb-4", "Animations" }

            // Introduction
            section { class: "prose max-w-none",
                p { class: "text-lg text-text-secondary dark:text-text-secondary",
                    "Learn how to create fluid animations using Dioxus Motion's powerful animation system."
                }
            }

            // Spring Animations
            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold text-text-primary dark:text-text-primary", "Spring Animations" }
                div { class: "bg-dark-50 dark:bg-dark-100 rounded-lg p-6",
                    pre { class: "language-rust code-block",
                        code {
                            // ... code content ...
                        }
                    }
                }
            }

            // Tween Animations
            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold text-text-primary dark:text-text-primary", "Tween Animations" }
                div { class: "bg-dark-50 dark:bg-dark-100 rounded-lg p-6",
                    pre { class: "language-rust code-block",
                        code {
                            // ... code content ...
                        }
                    }
                }
            }

            // Transform Animations
            section { class: "space-y-4",
                h2 { class: "text-2xl font-semibold text-text-primary dark:text-text-primary", "Transform Animations" }
                div { class: "bg-dark-50 dark:bg-dark-100 rounded-lg p-6",
                    pre { class: "language-rust code-block",
                        code {
                            // ... code content ...
                        }
                    }
                }
            }
        }
    }
}
