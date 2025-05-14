use chrono::Local;
use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::components::primitives::{Button, ButtonVariant};
use crate::models::{Category, Expense};
use crate::utils::{parse_currency, parse_date, today};

#[derive(Props, Clone, PartialEq)]
pub struct ExpenseFormProps {
    expense: Option<Expense>,
    on_save: EventHandler<Expense>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn ExpenseForm(props: ExpenseFormProps) -> Element {
    // Signals
    let mut title = use_signal(String::new);
    let mut amount = use_signal(String::new);
    let mut date = use_signal(today);
    let mut category = use_signal(|| Category::Other("".to_string()));
    let mut notes = use_signal(String::new);
    let mut title_error = use_signal(|| false);
    let mut amount_error = use_signal(|| false);
    let mut is_edit_mode = use_signal(|| false);
    let props_on_save = props.on_save;
    let props_expense = props.expense.clone();
    let expense = props.expense.clone();

    use_effect(use_reactive((&expense,), move |(expense,)| {
        if let Some(expense) = expense {
            tracing::info!(
                "use_effect: Updating form fields with expense data: {:?}",
                expense
            );
            title.set(expense.title.clone());
            let amount_str = format!("{:.2}", expense.amount);
            amount.set(amount_str);
            let today_date = Local::now().date_naive();
            let expense_date = if expense.date > today_date {
                today_date
            } else {
                expense.date
            };
            date.set(expense_date);
            category.set(expense.category.clone());
            notes.set(expense.notes.clone());
            is_edit_mode.set(true);
        } else {
            tracing::info!("use_effect: No expense data provided, using default values");
        }
    }));

    let mut form_scale = use_motion(1.0f32);

    // Animate the form entry
    use_effect(move || {
        form_scale.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })),
        );
    });

    let handle_submit = {
        move |_: MouseEvent| {
            // Add debug logging
            tracing::info!("Submit button clicked");

            // Validate form
            let mut is_valid = true;

            if title.read().trim().is_empty() {
                title_error.set(true);
                is_valid = false;
                tracing::warn!("Title validation failed");
            } else {
                title_error.set(false);
            }

            let parsed_amount = parse_currency(&amount.read());
            if parsed_amount.is_none() || parsed_amount.unwrap() <= 0.0 {
                amount_error.set(true);
                is_valid = false;
                tracing::warn!("Amount validation failed: {}", amount.read());
            } else {
                amount_error.set(false);
            }

            if is_valid {
                tracing::info!("Form validation passed, creating expense");
                let expense = if *is_edit_mode.read() {
                    // Use the original expense from props by cloning only here
                    let mut e = props_expense.as_ref().unwrap().clone();
                    e.title = title.read().clone();
                    e.amount = parsed_amount.unwrap();
                    e.date = *date.read();
                    e.category = category.read().clone();
                    e.notes = notes.read().clone();
                    tracing::info!("Updated expense: {}", e.title);
                    e
                } else {
                    let new_expense = Expense::new(
                        title.read().clone(),
                        parsed_amount.unwrap(),
                        *date.read(),
                        category.read().clone(),
                        notes.read().clone(),
                    );
                    tracing::info!("Created new expense: {}", new_expense.title);
                    new_expense
                };

                tracing::info!("Calling on_save handler");
                props_on_save.call(expense);
            } else {
                tracing::warn!("Form validation failed");
            }
        }
    };

    let handle_cancel = {
        let on_cancel = props.on_cancel;
        move |_| {
            on_cancel.call(());
        }
    };

    let handle_title_change = {
        move |evt: Event<FormData>| {
            let new_title = evt.value().clone();
            title.set(new_title.clone());
            if !new_title.trim().is_empty() {
                title_error.set(false);
            }
        }
    };

    let handle_amount_change = {
        move |evt: Event<FormData>| {
            amount.set(evt.value().clone());
            if parse_currency(&evt.value()).is_some() {
                amount_error.set(false);
            }
        }
    };

    let handle_date_change = {
        move |evt: Event<FormData>| {
            if let Some(parsed_date) = parse_date(&evt.value()) {
                date.set(parsed_date);
            }
        }
    };

    let handle_category_change = {
        move |evt: Event<FormData>| {
            category.set(Category::from_value(&evt.value()));
        }
    };

    let handle_notes_change = {
        move |evt: Event<FormData>| {
            notes.set(evt.value().clone());
        }
    };

    rsx! {
        div {
            class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-lg",
            style: "transform: scale({form_scale.get_value()})",

            h2 { class: "text-2xl font-bold mb-6 text-gray-800 dark:text-white",
                if *is_edit_mode.read() {
                    "Edit Expense"
                } else {
                    "Add New Expense"
                }
            }

            div { class: "space-y-4",

                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                        "Title"
                    }
                    input {
                        r#type: "text",
                        class: if *title_error.read() { "w-full px-3 py-2 border border-red-500 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white" } else { "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white" },
                        placeholder: "Expense title",
                        value: "{title.read()}",
                        oninput: handle_title_change,
                    }
                    if *title_error.read() {
                        p { class: "text-red-500 text-xs mt-1", "Title is required" }
                    }
                }

                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                        "Amount"
                    }
                    input {
                        r#type: "text",
                        class: if *amount_error.read() { "w-full px-3 py-2 border border-red-500 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white" } else { "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white" },
                        placeholder: "0.00",
                        value: "{amount.read()}",
                        oninput: handle_amount_change,
                    }
                    if *amount_error.read() {
                        p { class: "text-red-500 text-xs mt-1",
                            "Please enter a valid amount greater than zero"
                        }
                    }
                }

                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                        "Date"
                    }
                    input {
                        r#type: "date",
                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                        value: "{date.read().format(\"%Y-%m-%d\")}",
                        max: "{Local::now().date_naive().format(\"%Y-%m-%d\")}",
                        oninput: handle_date_change,
                    }
                }

                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                        "Category"
                    }
                    select {
                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                        value: category.read().as_value(),
                        oninput: handle_category_change,

                        for cat in Category::all() {
                            option {
                                value: cat.as_value(),
                                selected: *category.read() == cat,
                                "{cat.display_name()}"
                            }
                        }
                    }
                }

                div { class: "space-y-2",
                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                        "Notes"
                    }
                    textarea {
                        class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white",
                        rows: "3",
                        placeholder: "Optional notes",
                        value: "{notes.read()}",
                        oninput: handle_notes_change,
                    }
                }

                div { class: "flex justify-end space-x-3 mt-6",
                    Button {
                        variant: ButtonVariant::Outline,
                        motion: true,
                        onclick: handle_cancel,
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        motion: true,
                        button_type: "button",
                        onclick: handle_submit,
                        if *is_edit_mode.read() {
                            "Save Changes"
                        } else {
                            "Add Expense"
                        }
                    }
                }
            }
        }
    }
}
