use crate::Route;
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

use crate::components::ExpenseForm;
use crate::context::ExpenseContext;
use crate::models::Expense;
use crate::tailwind::include_tailwind_stylesheet;

#[component]
pub fn ExpenseFormPage(id: Option<String>) -> Element {
    // Get the expense context
    let expense_context = use_context::<Arc<Mutex<ExpenseContext>>>();
    let navigator = use_navigator();

    // State for UI
    let mut error_message = use_signal(|| None::<String>);
    let mut editing_expense = use_signal(|| None::<Expense>);

    // Get the loading state
    let loading = expense_context.lock().unwrap().loading();

    // Load the expense if we have an ID
    let expense_context_clone = expense_context.clone();
    use_effect(move || {
        tracing::info!("ExpenseFormPage effect running with id: {:?}", id);
        if let Some(id) = &id {
            let ctx = expense_context_clone.clone();
            let id_clone = id.clone();
            tracing::info!("Loading expense with ID: {}", id_clone);

            spawn(async move {
                tracing::info!("Inside async block, fetching expense with ID: {}", id_clone);
                let expense = ctx.lock().unwrap().get_expense(&id_clone).await;
                match expense {
                    Ok(Some(expense)) => {
                        tracing::info!("Found expense: {:?}", expense);
                        editing_expense.set(Some(expense.clone()));
                        tracing::info!(
                            "Set editing_expense signal to: {:?}",
                            editing_expense.read()
                        );
                    }
                    Ok(None) => {
                        tracing::error!("Expense with ID {} not found", id_clone);
                        error_message.set(Some(format!("Expense with ID {} not found", id_clone)));
                    }
                    Err(err) => {
                        tracing::error!("Error loading expense: {}", err);
                        error_message.set(Some(format!("Error loading expense: {}", err)));
                    }
                }
            });
        } else {
            tracing::info!("No ID provided, creating new expense");
        }
    });

    // Handle saving an expense
    let expense_context_clone = expense_context.clone();
    let handle_save_expense = move |expense: Expense| {
        tracing::info!("handle_save_expense called with expense: {}", expense.title);
        error_message.set(None);

        let ctx = expense_context_clone.clone();
        let future = async move {
            let mut ctx = ctx.lock().unwrap();
            tracing::info!("Inside async block, about to save expense");
            let result = if editing_expense.read().is_some() {
                tracing::info!("Updating existing expense");
                ctx.update_expense(expense).await
            } else {
                tracing::info!("Adding new expense");
                ctx.add_expense(expense).await
            };

            // Always reload expenses after add/update to ensure data consistency
            ctx.load_expenses().await;

            match result {
                Ok(_) => {
                    tracing::info!("Expense saved successfully, navigating to Dashboard");
                    navigator.push(Route::Dashboard {});
                }
                Err(err) => {
                    tracing::error!("Error saving expense: {}", err);
                    error_message.set(Some(format!("Error: {}", err)));
                }
            }
        };

        tracing::info!("Spawning future to save expense");
        spawn(future);
    };

    // Handle canceling the form
    let handle_cancel_form = move |_| {
        navigator.push(Route::Dashboard {});
        // Add a log to confirm navigation
        tracing::info!("Canceling form and navigating to Dashboard");
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

                // Expense form
                if *loading.read() {
                    div { class: "flex justify-center items-center p-8",
                        div { class: "animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500" }
                        p { class: "ml-4 text-gray-600 dark:text-gray-300", "Loading expense data..." }
                    }
                } else {
                    {
                        let expense_clone = editing_expense.read().clone();
                        tracing::info!("Rendering ExpenseForm with expense: {:?}", expense_clone);
                        rsx! {
                            ExpenseForm {
                                expense: expense_clone,
                                on_save: handle_save_expense,
                                on_cancel: handle_cancel_form,
                            }
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
