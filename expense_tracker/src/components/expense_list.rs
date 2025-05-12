use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::components::ExpenseItem;
use crate::models::Expense;
use crate::utils::format_currency_with_commas;

#[derive(Props, Clone, PartialEq)]
pub struct ExpenseListProps {
    expenses: Vec<Expense>,
    on_edit: EventHandler<Expense>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn ExpenseList(props: ExpenseListProps) -> Element {
    let total_amount: f64 = props.expenses.iter().map(|e| e.amount).sum();
    let mut list_opacity = use_motion(0.0f32);

    // Animate the list entry
    use_effect(move || {
        list_opacity.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    });

    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm p-6 border border-gray-200 dark:border-gray-700",
            style: "opacity: {list_opacity.get_value()}",

            div {
                class: "flex justify-between items-center mb-6",

                h2 {
                    class: "text-xl font-bold text-gray-800 dark:text-white",
                    "Expenses"
                }

                if !props.expenses.is_empty() {
                    div {
                        class: "text-sm text-gray-600 dark:text-gray-400",

                        span {
                            class: "font-medium",
                            "{props.expenses.len()} "
                        }

                        "items, total "

                        span {
                            class: "font-medium text-gray-900 dark:text-white",
                            "{format_currency_with_commas(total_amount)}"
                        }
                    }
                }
            }

            if props.expenses.is_empty() {
                div {
                    class: "p-8 text-center text-gray-500 dark:text-gray-400",

                    p {
                        class: "text-lg mb-4",
                        "No expenses found"
                    }

                    p {
                        class: "text-sm",
                        "Add a new expense to get started"
                    }
                }
            } else {
                div {
                    class: "space-y-2",

                    for expense in props.expenses.iter() {
                        ExpenseItem {
                            key: "{expense.id}",
                            expense: expense.clone(),
                            on_edit: props.on_edit.clone(),
                            on_delete: props.on_delete.clone(),
                        }
                    }
                }
            }
        }
    }
}
