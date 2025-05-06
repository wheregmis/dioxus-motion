use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn ReportsPage() -> Element {
    rsx! {
        motion::section {
            class: "p-6 max-w-4xl mx-auto",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(TransitionConfig::new(TransitionType::Spring).stiffness(100.0).damping(15.0)),

            // Global attributes for accessibility
            role: "region",
            aria_label: "Reports section",

            motion::h2 {
                class: "text-2xl font-bold text-gray-800 dark:text-white mb-4",
                initial: Some(AnimationTarget::new().opacity(0.0).x(-20.0)),
                animate: Some(AnimationTarget::new().opacity(1.0).x(0.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(0.1),
                ),
                "Available Reports"
            }

            motion::p {
                class: "text-gray-600 dark:text-gray-300 mb-6",
                initial: Some(AnimationTarget::new().opacity(0.0).y(10.0)),
                animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
                transition: Some(
                    TransitionConfig::new(TransitionType::Spring)
                        .stiffness(100.0)
                        .damping(15.0)
                        .delay(0.2),
                ),
                "Access and download detailed reports for your business analytics."
            }

            // Report cards grid
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                // Report card 1
                motion::article {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden border border-gray-200 dark:border-gray-700",
                    initial: Some(AnimationTarget::new().opacity(0.0).scale(0.9)),
                    animate: Some(AnimationTarget::new().opacity(1.0).scale(1.0)),
                    transition: Some(
                        TransitionConfig::new(TransitionType::Spring)
                            .stiffness(100.0)
                            .damping(15.0)
                            .delay(0.3),
                    ),
                    while_hover: Some(AnimationTarget::new().scale(1.03).y(-5.0)),

                    // Report header with icon
                    div { class: "bg-blue-50 dark:bg-blue-900/20 p-4 border-b border-gray-200 dark:border-gray-700",
                        div { class: "flex items-center",
                            div { class: "bg-blue-100 dark:bg-blue-800 p-2 rounded-lg mr-3",
                                // Icon placeholder
                                div { class: "text-blue-600 dark:text-blue-300 text-xl", "📊" }
                            }
                            motion::h3 {
                                class: "text-lg font-semibold text-gray-800 dark:text-white",
                                "Monthly Sales Report"
                            }
                        }
                    }

                    // Report content
                    motion::div {
                        class: "p-4",
                        motion::p {
                            class: "text-gray-600 dark:text-gray-300 text-sm mb-4",
                            "Detailed analysis of sales performance for the current month with breakdowns by product category and region."
                        }

                        // Report metadata
                        div { class: "flex justify-between items-center text-xs text-gray-500 dark:text-gray-400",
                            div { "PDF • 2.4 MB" }
                            motion::div {
                                class: "text-blue-600 dark:text-blue-400 font-medium",
                                "Last updated: Today"
                            }
                        }
                    }

                    // Report footer with action button
                    motion::div {
                        class: "bg-gray-50 dark:bg-gray-800 p-4 border-t border-gray-200 dark:border-gray-700",
                        motion::button {
                            class: "w-full py-2 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
                            while_hover: Some(AnimationTarget::new().scale(1.02)),
                            while_tap: Some(AnimationTarget::new().scale(0.98)),

                            "Download Report"
                        }
                    }
                }

                // Report card 2
                motion::article {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden border border-gray-200 dark:border-gray-700",
                    initial: Some(AnimationTarget::new().opacity(0.0).scale(0.9)),
                    animate: Some(AnimationTarget::new().opacity(1.0).scale(1.0)),
                    transition: Some(
                        TransitionConfig::new(TransitionType::Spring)
                            .stiffness(100.0)
                            .damping(15.0)
                            .delay(0.4),
                    ),
                    while_hover: Some(AnimationTarget::new().scale(1.03).y(-5.0)),

                    // Report header with icon
                    div { class: "bg-purple-50 dark:bg-purple-900/20 p-4 border-b border-gray-200 dark:border-gray-700",
                        div { class: "flex items-center",
                            div { class: "bg-purple-100 dark:bg-purple-800 p-2 rounded-lg mr-3",
                                // Icon placeholder
                                div { class: "text-purple-600 dark:text-purple-300 text-xl", "👥" }
                            }
                            motion::h3 {
                                class: "text-lg font-semibold text-gray-800 dark:text-white",
                                "User Engagement Report"
                            }
                        }
                    }

                    // Report content
                    motion::div {
                        class: "p-4",
                        motion::p {
                            class: "text-gray-600 dark:text-gray-300 text-sm mb-4",
                            "Analysis of user engagement metrics across all platforms with insights on user behavior and retention rates."
                        }

                        // Report metadata
                        div { class: "flex justify-between items-center text-xs text-gray-500 dark:text-gray-400",
                            div { "PDF • 3.1 MB" }
                            motion::div {
                                class: "text-purple-600 dark:text-purple-400 font-medium",
                                "Last updated: Yesterday"
                            }
                        }
                    }

                    // Report footer with action button
                    motion::div {
                        class: "bg-gray-50 dark:bg-gray-800 p-4 border-t border-gray-200 dark:border-gray-700",
                        motion::button {
                            class: "w-full py-2 bg-purple-600 hover:bg-purple-700 text-white text-sm font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2",
                            while_hover: Some(AnimationTarget::new().scale(1.02)),
                            while_tap: Some(AnimationTarget::new().scale(0.98)),

                            "Download Report"
                        }
                    }
                }

                // Report card 3
                motion::article {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden border border-gray-200 dark:border-gray-700",
                    initial: Some(AnimationTarget::new().opacity(0.0).scale(0.9)),
                    animate: Some(AnimationTarget::new().opacity(1.0).scale(1.0)),
                    transition: Some(
                        TransitionConfig::new(TransitionType::Spring)
                            .stiffness(100.0)
                            .damping(15.0)
                            .delay(0.5),
                    ),
                    while_hover: Some(AnimationTarget::new().scale(1.03).y(-5.0)),

                    // Report header with icon
                    div { class: "bg-green-50 dark:bg-green-900/20 p-4 border-b border-gray-200 dark:border-gray-700",
                        div { class: "flex items-center",
                            div { class: "bg-green-100 dark:bg-green-800 p-2 rounded-lg mr-3",
                                // Icon placeholder
                                div { class: "text-green-600 dark:text-green-300 text-xl", "📈" }
                            }
                            motion::h3 {
                                class: "text-lg font-semibold text-gray-800 dark:text-white",
                                "Growth Forecast"
                            }
                        }
                    }

                    // Report content
                    motion::div {
                        class: "p-4",
                        motion::p {
                            class: "text-gray-600 dark:text-gray-300 text-sm mb-4",
                            "Quarterly growth projections based on current trends and market analysis with actionable recommendations."
                        }

                        // Report metadata
                        div { class: "flex justify-between items-center text-xs text-gray-500 dark:text-gray-400",
                            div { "PDF • 1.8 MB" }
                            motion::div {
                                class: "text-green-600 dark:text-green-400 font-medium",
                                "Last updated: 3 days ago"
                            }
                        }
                    }

                    // Report footer with action button
                    motion::div {
                        class: "bg-gray-50 dark:bg-gray-800 p-4 border-t border-gray-200 dark:border-gray-700",
                        motion::button {
                            class: "w-full py-2 bg-green-600 hover:bg-green-700 text-white text-sm font-medium rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2",
                            while_hover: Some(AnimationTarget::new().scale(1.02)),
                            while_tap: Some(AnimationTarget::new().scale(0.98)),

                            "Download Report"
                        }
                    }
                }
            }
        }
    }
}
