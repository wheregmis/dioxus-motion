use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::models::Category;
use crate::utils::format_currency;

#[derive(Props, Clone, PartialEq)]
pub struct SummaryViewProps {
    pub total_amount: f64,
    pub category_totals: Vec<(Category, f64)>,
    pub on_category_click: EventHandler<Category>,
}

#[component]
pub fn SummaryView(props: SummaryViewProps) -> Element {
    let mut chart_opacity = use_motion(0.0f32);

    // Animate the chart entry
    use_effect(move || {
        chart_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    });

    // Calculate the percentage for each category
    let category_percentages = props
        .category_totals
        .iter()
        .map(|(category, amount)| {
            let percentage = if props.total_amount > 0.0 {
                (amount / props.total_amount) * 100.0
            } else {
                0.0
            };
            (category.clone(), *amount, percentage)
        })
        .collect::<Vec<_>>();

    // Get a color for each category
    let category_colors = [
        "bg-blue-500",
        "bg-green-500",
        "bg-yellow-500",
        "bg-red-500",
        "bg-purple-500",
        "bg-pink-500",
        "bg-indigo-500",
        "bg-teal-500",
        "bg-orange-500",
        "bg-cyan-500",
    ];

    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 border border-gray-200 dark:border-gray-700",
            style: "opacity: {chart_opacity.get_value()}",
            h2 { class: "text-xl font-bold text-gray-800 dark:text-white mb-4", "Expense Summary" }
            div { class: "mb-6",
                div { class: "flex justify-between items-center mb-2",
                    span { class: "text-gray-600 dark:text-gray-400", "Total Expenses" }
                    span { class: "text-xl font-bold text-gray-900 dark:text-white",
                        "{format_currency(props.total_amount)}"
                    }
                }
                div { class: "h-2 w-full bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden",
                    div {
                        class: "h-full bg-blue-600 rounded-full",
                        style: "width: 100%",
                    }
                }
            }
            if props.category_totals.is_empty() {
                div { class: "p-8 text-center text-gray-500 dark:text-gray-400",
                    p { class: "text-lg", "No expense data available" }
                }
            } else {
                div { class: "space-y-4",
                    {
                        category_percentages
                            .iter()
                            .enumerate()
                            .map(|(idx, (category, amount, percentage))| {
                                let color_idx = idx % category_colors.len();
                                let color_class = category_colors[color_idx];
                                let category_clone = category.clone();
                                rsx! {
                                    div {
                                        key: "{idx}",
                                        class: "cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700 p-2 rounded-lg transition-colors",
                                        onclick: move |_| props.on_category_click.call(category_clone.clone()),
                                        div { class: "flex justify-between items-center mb-1",
                                            span { class: "text-gray-700 dark:text-gray-300", "{category.display_name()}" }
                                            div { class: "flex items-center",
                                                span { class: "text-gray-900 dark:text-white font-medium mr-2", "{format_currency(*amount)}" }
                                                span { class: "text-gray-500 dark:text-gray-400 text-sm", "{percentage:.1}%" }
                                            }
                                        }
                                        div { class: "h-2 w-full bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden",
                                            div {
                                                class: "h-full {color_class} rounded-full",
                                                style: "width: {percentage}%",
                                            }
                                        }
                                    }
                                }
                            })
                    }
                }
            }
        }
    }
}
