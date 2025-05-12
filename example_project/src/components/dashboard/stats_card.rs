use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_motion::{AnimationTarget, TransitionConfig, TransitionType};

#[component]
pub fn DashboardStatsCard() -> Element {
    let mut is_grid = use_signal(|| true);
    let cards = vec![
        (
            "card-users",
            "Total Users",
            "24,532",
            "+12% from last month",
            "text-green-600 dark:text-green-400",
        ),
        (
            "card-revenue",
            "Revenue",
            "$48,271",
            "+8% from last month",
            "text-green-600 dark:text-green-400",
        ),
        (
            "card-projects",
            "Active Projects",
            "16",
            "-2 from last month",
            "text-red-600 dark:text-red-400",
        ),
    ];
    rsx! {
        div { class: "mb-4 flex items-center",
            // Segmented toggle
            div { class: "inline-flex rounded-full bg-gray-100 dark:bg-surface border border-gray-200 dark:border-gray-700 shadow-sm overflow-hidden",
                // Grid toggle
                button {
                    class: format!(
                        "px-4 py-2 flex items-center gap-2 font-medium transition-colors duration-200 {}",
                        if is_grid() {
                            "bg-primary text-white shadow"
                        } else {
                            "text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-800"
                        },
                    ),
                    onclick: move |_| {
                        println!("DEBUG: Setting layout to grid");
                        is_grid.set(true);
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        class: "w-5 h-5",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M3.75 3.75h4.5v4.5h-4.5v-4.5zm6 0h4.5v4.5h-4.5v-4.5zm6 0h4.5v4.5h-4.5v-4.5zm-12 6h4.5v4.5h-4.5v-4.5zm6 0h4.5v4.5h-4.5v-4.5zm6 0h4.5v4.5h-4.5v-4.5zm-12 6h4.5v4.5h-4.5v-4.5zm6 0h4.5v4.5h-4.5v-4.5zm6 0h4.5v4.5h-4.5v-4.5z",
                        }
                    }
                    span { class: "hidden md:inline", "Grid" }
                }
                // List toggle
                button {
                    class: format!(
                        "px-4 py-2 flex items-center gap-2 font-medium transition-colors duration-200 {}",
                        if !is_grid() {
                            "bg-primary text-white shadow"
                        } else {
                            "text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-800"
                        },
                    ),
                    onclick: move |_| {
                        println!("DEBUG: Setting layout to list");
                        is_grid.set(false);
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        class: "w-5 h-5",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M4 6h16M4 12h16M4 18h16",
                        }
                    }
                    span { class: "hidden md:inline", "List" }
                }
            }
        }
        div {
            class: if is_grid() { "grid grid-cols-1 md:grid-cols-3 gap-6" } else { "flex flex-col gap-6" },
            // Render cards directly for better layout control
            {
                cards
                    .iter()
                    .enumerate()
                    .map(|(i, (id, title, value, subtitle, subtitle_class))| {
                        rsx! {
                            motion::div {
                                key: "{id}",
                                class: "bg-white dark:bg-surface rounded-lg shadow-md p-6 flex flex-col hover:scale-105",
                                // Use optimized spring physics for smoother animations with staggered delay
                                initial: AnimationTarget::default().opacity(0.2).scale(0.95),
                                animate: AnimationTarget::default().opacity(1.0).scale(1.0),
                                transition: TransitionConfig::default()
                                    .type_(TransitionType::Spring)
                                    .stiffness(170.0)
                                    .damping(22.0)
                                    .duration(0.6)
                                    .delay(i as f32 * 0.08), // 80ms stagger between items

                                h3 { class: "text-lg font-semibold text-gray-800 dark:text-white mb-2", "{title}" }
                                div { class: "text-3xl font-bold text-gray-900 dark:text-white", "{value}" }
                                div { class: format!("text-sm font-medium mt-2 {subtitle_class}"), "{subtitle}" }
                            }
                        }
                    })
            }
        }
    }
}
