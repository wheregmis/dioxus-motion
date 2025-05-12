use dioxus::prelude::*;
use dioxus_motion::{prelude::*, AnimationTarget, TransitionConfig, TransitionType};

#[component]
pub fn Header(active_section: Signal<String>) -> Element {
    rsx! {
        motion::div {
            class: "bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 px-6 py-4 shadow-sm",
            initial: Some(AnimationTarget::new().opacity(0.0).y(-10.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(TransitionConfig::new(TransitionType::Spring).stiffness(100.0).damping(15.0)),

            div { class: "flex justify-between items-center",
                motion::h1 {
                    class: "text-2xl font-bold text-gray-800 dark:text-white",
                    initial: Some(AnimationTarget::new().opacity(0.0).x(-10.0)),
                    animate: Some(AnimationTarget::new().opacity(1.0).x(0.0)),
                    transition: Some(
                        TransitionConfig::new(TransitionType::Spring)
                            .stiffness(100.0)
                            .damping(15.0)
                            .delay(0.1),
                    ),

                    if active_section() == "dashboard" {
                        "Dashboard"
                    } else if active_section() == "analytics" {
                        "Analytics"
                    } else if active_section() == "reports" {
                        "Reports"
                    } else {
                        "Settings"
                    }
                }

                // User profile section
                div { class: "flex items-center space-x-4",
                    // Notification bell
                    motion::button {
                        class: "p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800",
                        while_hover: Some(AnimationTarget::new().scale(1.1)),
                        while_tap: Some(AnimationTarget::new().scale(0.9)),

                        "🔔"
                    }

                    // User avatar
                    motion::div {
                        class: "w-10 h-10 rounded-full bg-blue-500 flex items-center justify-center text-white cursor-pointer",
                        while_hover: Some(AnimationTarget::new().scale(1.05)),

                        "JD"
                    }
                }
            }
        }
    }
}
