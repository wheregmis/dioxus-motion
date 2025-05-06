use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn DashboardStatsCard() -> Element {
    rsx! {
        motion::div {
            class: "bg-white dark:bg-surface rounded-lg shadow-md p-6 flex flex-col",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(TransitionConfig::new(TransitionType::Spring).stiffness(100.0).damping(15.0)),
            while_hover: Some(AnimationTarget::new().scale(1.02)),

            // Global attributes for accessibility and testing
            role: "region",
            aria_label: "Total users statistics",
            "data-testid": "stats-card-users",

            h3 { class: "text-lg font-semibold text-gray-800 dark:text-white mb-2", "Total Users" }
            div { class: "text-3xl font-bold text-gray-900 dark:text-white", "24,532" }
            div { class: "text-sm font-medium text-green-600 dark:text-green-400 mt-2", "+12% from last month" }
        }

        motion::div {
            class: "bg-white dark:bg-surface rounded-lg shadow-md p-6 flex flex-col",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.1),
            ),
            while_hover: Some(AnimationTarget::new().scale(1.02)),

            h3 { class: "text-lg font-semibold text-gray-800 dark:text-white mb-2", "Revenue" }
            div { class: "text-3xl font-bold text-gray-900 dark:text-white", "$48,271" }
            div { class: "text-sm font-medium text-green-600 dark:text-green-400 mt-2", "+8% from last month" }
        }

        motion::div {
            class: "bg-white dark:bg-surface rounded-lg shadow-md p-6 flex flex-col",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
                    .delay(0.2),
            ),
            while_hover: Some(AnimationTarget::new().scale(1.02)),

            h3 { class: "text-lg font-semibold text-gray-800 dark:text-white mb-2", "Active Projects" }
            div { class: "text-3xl font-bold text-gray-900 dark:text-white", "16" }
            div { class: "text-sm font-medium text-red-600 dark:text-red-400 mt-2", "-2 from last month" }
        }
    }
}
