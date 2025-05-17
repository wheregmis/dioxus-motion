use crate::components::primitives::ButtonVariant;
use crate::models::Category;
use crate::Route;
use crate::{components::primitives::Button, context::ExpenseContext};
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

#[component]
pub fn BudgetPage() -> Element {
    let expense_context = use_context::<Arc<Mutex<ExpenseContext>>>();
    let navigator = use_navigator();
    let budgets = expense_context.lock().unwrap().get_budgets();
    let mut editing = use_signal(|| None::<Category>);
    let mut input_value = use_signal(String::new);

    let budget_rows = Category::all()
        .into_iter()
        .map(|category| {
            let expense_context = expense_context.clone();
            let is_editing = editing.read().as_ref() == Some(&category);
            let current_budget = budgets.get(&category).copied().unwrap_or(0.0);
            rsx! {
                tr {
                    td { class: "py-2 px-4 border-b", "{category.display_name()}" }
                    td { class: "py-2 px-4 border-b",
                        if is_editing {
                            input {
                                r#type: "number",
                                class: "w-24 px-2 py-1 border border-gray-300 rounded text-sm",
                                value: "{input_value.read()}",
                                oninput: move |evt| input_value.set(evt.value().clone()),
                                autofocus: true,
                            }
                        } else {
                            span { {format!("{:.2}", current_budget)} }
                        }
                    }
                    td { class: "py-2 px-4 border-b",
                        if is_editing {
                            button {
                                class: "px-3 py-1 bg-blue-600 text-white rounded mr-2 text-sm",
                                onclick: move |_| {
                                    if let Ok(val) = input_value.read().parse::<f64>() {
                                        let mut context = expense_context.lock().unwrap();
                                        context.repository.set_budget(category.clone(), val);
                                    }
                                    editing.set(None);
                                },
                                "Save"
                            }
                            button {
                                class: "px-3 py-1 bg-gray-400 text-white rounded text-sm",
                                onclick: move |_| editing.set(None),
                                "Cancel"
                            }
                        } else {
                            button {
                                class: "px-3 py-1 bg-blue-500 text-white rounded text-sm",
                                onclick: move |_| {
                                    input_value.set(format!("{:.2}", current_budget));
                                    editing.set(Some(category.clone()));
                                },
                                "Edit"
                            }
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        div { class: "max-w-2xl mx-auto p-8 bg-white dark:bg-gray-800 rounded-lg shadow border border-gray-200 dark:border-gray-700 mt-8",
            div { class: "mb-4 flex justify-end",
                Button {
                    variant: ButtonVariant::Outline,
                    motion: true,
                    onclick: Some(
                        EventHandler::new(move |_: MouseEvent| {
                            navigator.push(Route::Dashboard {});
                        }),
                    ),
                    "Back to Dashboard"
                }
            }
            h2 { class: "text-2xl font-bold mb-6 text-gray-800 dark:text-white", "Manage Budgets" }
            table { class: "w-full text-left border-collapse",
                thead {
                    tr {
                        th { class: "py-2 px-4 border-b", "Category" }
                        th { class: "py-2 px-4 border-b", "Budget" }
                        th { class: "py-2 px-4 border-b", "Actions" }
                    }
                }
                tbody { {budget_rows.iter().cloned()} }
            }
        }
    }
}
