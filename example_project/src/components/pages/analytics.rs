use dioxus::prelude::*;
use dioxus_motion::{prelude::*, AnimationTarget, TransitionConfig, TransitionType};

#[component]
pub fn AnalyticsPage() -> Element {
    rsx! {
        div { class: "p-6 max-w-4xl mx-auto",
            // Demonstrate new motion components
            motion::h2 {
                class: "text-2xl font-bold text-gray-800 dark:text-white mb-4",
                initial: Some(AnimationTarget::new().opacity(0.0).y(-20.0)),
                animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
                transition: Some(TransitionConfig::new(TransitionType::Spring).stiffness(100.0).damping(15.0)),
                "Analytics Overview"
            }

            motion::p {
                class: "text-gray-600 dark:text-gray-300 mb-6",
                initial: Some(AnimationTarget::new().opacity(0.0).x(-10.0)),
                animate: Some(AnimationTarget::new().opacity(1.0).x(0.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(0.1),
                ),
                "Track your performance metrics and user engagement statistics in real-time."
            }

            // Animated image
            motion::div {
                class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-4 mb-6",
                initial: Some(AnimationTarget::new().opacity(0.0).scale(0.8)),
                animate: Some(AnimationTarget::new().opacity(1.0).scale(1.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(0.2),
                ),

                img {
                    class: "w-full h-auto rounded",
                    src: "https://via.placeholder.com/800x400?text=Analytics+Dashboard",
                    alt: "Analytics Dashboard",
                    loading: "lazy",
                }
            }

            // Key metrics section
            motion::div {
                class: "bg-white dark:bg-gray-800 rounded-lg shadow-md p-6 mb-6",
                initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
                animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(0.3),
                ),

                h3 { class: "text-lg font-semibold text-gray-800 dark:text-white mb-4",
                    "Key Metrics"
                }

                // Metrics grid
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    // Metric card 1
                    motion::div {
                        class: "bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg border border-blue-100 dark:border-blue-800",
                        initial: Some(AnimationTarget::new().opacity(0.0).scale(0.9)),
                        animate: Some(AnimationTarget::new().opacity(1.0).scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.4),
                        ),

                        div { class: "text-sm text-blue-600 dark:text-blue-300 font-medium",
                            "User Engagement"
                        }
                        div { class: "text-2xl font-bold text-gray-800 dark:text-white mt-1",
                            "78%"
                        }
                        div { class: "text-xs text-green-600 dark:text-green-400 mt-1",
                            "+12% from last month"
                        }
                    }

                    // Metric card 2
                    motion::div {
                        class: "bg-purple-50 dark:bg-purple-900/20 p-4 rounded-lg border border-purple-100 dark:border-purple-800",
                        initial: Some(AnimationTarget::new().opacity(0.0).scale(0.9)),
                        animate: Some(AnimationTarget::new().opacity(1.0).scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.5),
                        ),

                        div { class: "text-sm text-purple-600 dark:text-purple-300 font-medium",
                            "Conversion Rate"
                        }
                        div { class: "text-2xl font-bold text-gray-800 dark:text-white mt-1",
                            "12.5%"
                        }
                        div { class: "text-xs text-green-600 dark:text-green-400 mt-1",
                            "+2.3% from last month"
                        }
                    }

                    // Metric card 3
                    motion::div {
                        class: "bg-amber-50 dark:bg-amber-900/20 p-4 rounded-lg border border-amber-100 dark:border-amber-800",
                        initial: Some(AnimationTarget::new().opacity(0.0).scale(0.9)),
                        animate: Some(AnimationTarget::new().opacity(1.0).scale(1.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.6),
                        ),

                        div { class: "text-sm text-amber-600 dark:text-amber-300 font-medium",
                            "Avg. Session Duration"
                        }
                        div { class: "text-2xl font-bold text-gray-800 dark:text-white mt-1",
                            "3m 42s"
                        }
                        div { class: "text-xs text-red-600 dark:text-red-400 mt-1",
                            "-18s from last month"
                        }
                    }
                }
            }

            // Action button
            div { class: "flex justify-center mt-8",
                motion::a {
                    class: "inline-flex items-center px-6 py-3 bg-blue-600 text-white font-medium rounded-md shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors",
                    href: "/analytics/detailed-report",
                    role: "button",
                    aria_label: "View detailed analytics report",
                    initial: Some(AnimationTarget::new().opacity(0.0).y(10.0)),
                    animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
                    transition: Some(
                        TransitionConfig::new(TransitionType::Spring)
                            .stiffness(100.0)
                            .damping(15.0)
                            .delay(0.7),
                    ),
                    while_hover: Some(AnimationTarget::new().scale(1.05)),
                    while_tap: Some(AnimationTarget::new().scale(0.95)),

                    "View Detailed Analytics Report"
                }
            }
        }
    }
}
