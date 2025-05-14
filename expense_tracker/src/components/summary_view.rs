use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::models::Category;
use crate::utils::format_currency;

#[derive(Props, Clone, PartialEq)]
pub struct SummaryViewProps {
    pub total_amount: f64,
    pub category_totals: Vec<(Category, f64)>,
    pub budgets: std::collections::HashMap<Category, f64>,
    pub remaining_budgets: std::collections::HashMap<Category, f64>,
    pub on_category_click: EventHandler<Category>,
    pub on_budget_change: EventHandler<(Category, f64)>,
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

    // State for editing budgets
    let mut editing_budget = use_signal(|| None::<Category>);
    let mut budget_input = use_signal(String::new);

    // Clone category_totals to avoid borrow checker issues
    let category_totals = props.category_totals.clone();
    let category_rows = category_totals.into_iter().enumerate().map(|(idx, (category, amount))| {
        let percentage = if props.total_amount > 0.0 {
            (amount / props.total_amount) * 100.0
        } else {
            0.0
        };
        let budget = props.budgets.get(&category).copied().unwrap_or(0.0);
        let remaining = props.remaining_budgets.get(&category).copied().unwrap_or(budget);
        let color_idx = idx % category_colors.len();
        let color_class = category_colors[color_idx];
        let category_clone = category.clone();
        let is_editing = editing_budget.read().as_ref() == Some(&category);
        rsx! {
            div {
                key: "{idx}",
                class: "cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700 p-2 rounded-lg transition-colors",
                onclick: move |_| props.on_category_click.call(category_clone.clone()),
                div { class: "flex justify-between items-center mb-1",
                    span { class: "text-gray-700 dark:text-gray-300", "{category.display_name()}" }
                    div { class: "flex items-center gap-2",
                        span { class: "text-gray-900 dark:text-white font-medium mr-2",
                            "{format_currency(amount)}"
                        }
                        span { class: "text-gray-500 dark:text-gray-400 text-sm", "{percentage:.1}%" }
                        if is_editing {
                            input {
                                r#type: "number",
                                class: "w-20 px-2 py-1 border border-gray-300 rounded text-xs",
                                value: "{budget_input.read()}",
                                oninput: move |evt| budget_input.set(evt.value().clone()),
                                onkeydown: move |evt| {
                                    if evt.key() == Key::Enter {
                                        if let Ok(val) = budget_input.read().parse::<f64>() {
                                            props.on_budget_change.call((category.clone(), val));
                                            editing_budget.set(None);
                                        }
                                    } else if evt.key() == Key::Escape {
                                        editing_budget.set(None);
                                    }
                                },
                                autofocus: true,
                            }
                        } else {
                            span { class: "text-xs text-green-600 dark:text-green-400 ml-2",
                                "Budget: {format_currency(budget)}"
                            }
                            button {
                                class: "ml-1 text-xs text-blue-500 hover:underline",
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                    budget_input.set(format!("{:.2}", budget));
                                    editing_budget.set(Some(category.clone()));
                                },
                                "Edit"
                            }
                        }
                        span { class: "text-xs text-blue-600 dark:text-blue-400 ml-2",
                            "Pending: {format_currency(remaining)}"
                        }
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
    }).collect::<Vec<_>>();

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
                div { class: "space-y-4", {category_rows.iter().cloned()} }
            }
        }
    }
}
