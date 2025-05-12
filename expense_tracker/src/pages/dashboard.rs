use crate::Route;
use dioxus::prelude::*;

use crate::components::primitives::{Button, ButtonVariant};
use crate::components::{ExpenseChart, ExpenseList, FilterBar, SummaryView};
use crate::models::Expense;
use crate::state::{ExpenseContext, FilterType};
use crate::tailwind::include_tailwind_stylesheet;
use std::sync::{Arc, Mutex};

#[component]
pub fn DashboardPage() -> Element {
    // Get the expense context
    let expense_context = use_context::<Arc<Mutex<ExpenseContext>>>();
    let navigator = use_navigator();

    // State for UI
    let mut error_message = use_signal(|| None::<String>);
    let mut show_chart = use_signal(|| false);

    // Handle editing an expense
    let handle_edit_expense = move |expense: Expense| {
        let id = expense.id.clone();
        tracing::info!("Editing expense with ID: {}", id);
        navigator.push(Route::ExpenseFormWithId { id });
    };

    // Handle deleting an expense
    let expense_context_clone = expense_context.clone();
    let handle_delete_expense = move |id: String| {
        error_message.set(None);

        let ctx = expense_context_clone.clone();
        let future = async move {
            let mut ctx = ctx.lock().unwrap();
            match ctx.delete_expense(&id).await {
                Ok(_) => {}
                Err(err) => {
                    error_message.set(Some(format!("Error: {}", err)));
                }
            }
        };

        spawn(future);
    };

    // Handle filter changes
    let expense_context_clone2 = expense_context.clone();
    let handle_filter_change = move |filter: FilterType| {
        let mut ctx = expense_context_clone2.lock().unwrap();
        ctx.set_filter(filter);
    };

    // Handle clearing the filter
    let expense_context_clone3 = expense_context.clone();
    let handle_clear_filter = move |_| {
        let mut ctx = expense_context_clone3.lock().unwrap();
        ctx.clear_filter();
    };

    // Handle category click in summary
    let expense_context_clone4 = expense_context.clone();
    let handle_category_click = move |category| {
        let mut ctx = expense_context_clone4.lock().unwrap();
        ctx.filter_by_category(category);
    };

    // Toggle chart view
    let toggle_chart = move |_| {
        let current = *show_chart.read();
        show_chart.set(!current);
    };

    // Get the filtered expenses
    let filtered_expenses = expense_context.lock().unwrap().filtered_expenses();

    // Get the total amount
    let total_amount = expense_context.lock().unwrap().get_total_amount();

    // Get the category totals
    let category_totals = expense_context.lock().unwrap().get_total_by_category();

    // Get the current filter
    let current_filter = expense_context.lock().unwrap().current_filter();

    // Handle adding a new expense
    let handle_add_expense = move |_| {
        navigator.push(Route::ExpenseForm { id: None });
        tracing::info!("Navigating to ExpenseForm with id: None");
    };

    rsx! {
        include_tailwind_stylesheet {}

        div { class: "min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-white",

            // Header
            header { class: "bg-white dark:bg-gray-800 shadow",

                div { class: "max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8 flex justify-between items-center",

                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                        "Expense Tracker"
                    }

                    div { class: "flex space-x-2",

                        Button {
                            variant: ButtonVariant::Primary,
                            motion: true,
                            onclick: handle_add_expense,

                            div { class: "flex items-center",

                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-5 w-5 mr-1",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",

                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M12 4v16m8-8H4",
                                    }
                                }

                                span { "Add Expense" }
                            }
                        }

                        Button {
                            variant: ButtonVariant::Secondary,
                            motion: true,
                            onclick: toggle_chart,

                            div { class: "flex items-center",

                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-5 w-5 mr-1",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",

                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z",
                                    }
                                }

                                span {
                                    if *show_chart.read() {
                                        "Hide Chart"
                                    } else {
                                        "Show Chart"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Main content
            main { class: "max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8",

                // Error message
                if let Some(message) = error_message.read().as_ref() {
                    div {
                        class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6 flex justify-between items-center",
                        role: "alert",

                        span { "{message}" }

                        button {
                            class: "text-red-700",
                            onclick: move |_| error_message.set(None),

                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-5 w-5",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",

                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M6 18L18 6M6 6l12 12",
                                }
                            }
                        }
                    }
                }

                // Filter bar
                FilterBar {
                    current_filter: current_filter.read().clone(),
                    on_filter_change: handle_filter_change,
                    on_clear_filter: handle_clear_filter,
                }

                // Chart view (if enabled)
                if *show_chart.read() {
                    div { class: "mb-6",

                        ExpenseChart {
                            total_amount,
                            category_totals: category_totals.clone(),
                            on_category_click: handle_category_click.clone(),
                        }
                    }
                }

                // Main content grid
                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",

                    // Expense list
                    div { class: "lg:col-span-2",

                        ExpenseList {
                            expenses: filtered_expenses.read().clone(),
                            on_edit: handle_edit_expense,
                            on_delete: handle_delete_expense,
                        }
                    }

                    // Summary view
                    div { class: "lg:col-span-1",

                        SummaryView {
                            total_amount,
                            category_totals,
                            on_category_click: handle_category_click,
                        }
                    }
                }
            }

            // Footer
            footer { class: "bg-white dark:bg-gray-800 shadow mt-8",

                div { class: "max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8 text-center text-gray-500 dark:text-gray-400 text-sm",

                    p { "Expense Tracker - Built with Dioxus" }
                }
            }
        }
    }
}
