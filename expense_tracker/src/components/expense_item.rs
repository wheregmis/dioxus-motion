use dioxus::events::MouseEvent;
use dioxus::prelude::*;

use crate::components::primitives::{Card, CardBody};
use crate::models::Expense;
use crate::utils::{format_currency, format_date_short};

#[derive(Props, Clone, PartialEq)]
pub struct ExpenseItemProps {
    pub expense: Expense,
    pub on_edit: EventHandler<Expense>,
    pub on_delete: EventHandler<String>,
}

#[component]
pub fn ExpenseItem(props: ExpenseItemProps) -> Element {
    let mut show_actions = use_signal(|| false);

    let expense_clone = props.expense.clone();
    let expense_for_edit = expense_clone.clone();
    let handle_edit = move |_: MouseEvent| {
        props.on_edit.call(expense_for_edit.clone());
    };

    // Clone the expense ID outside the closure to avoid borrowing issues
    let id_clone = props.expense.id.clone();
    let handle_delete = move |_: MouseEvent| {
        props.on_delete.call(id_clone.clone());
    };

    let toggle_actions = move |_: MouseEvent| {
        let current = *show_actions.read();
        show_actions.set(!current);
    };

    rsx! {
        Card { class: "mb-3", motion: true, hover_effect: true,

            CardBody {
                div { class: "flex justify-between items-start",

                    div { class: "flex-1",

                        div { class: "flex justify-between",

                            h3 { class: "text-lg font-semibold text-gray-800 dark:text-white",
                                "{expense_clone.title}"
                            }

                            span { class: "text-lg font-bold text-gray-900 dark:text-white",
                                "{format_currency(expense_clone.amount)}"
                            }
                        }

                        div { class: "flex justify-between mt-2",

                            div { class: "flex items-center",

                                span { class: "text-sm text-gray-500 dark:text-gray-400",
                                    "{format_date_short(expense_clone.date)}"
                                }

                                span { class: "mx-2 text-gray-300 dark:text-gray-600",
                                    "•"
                                }

                                span { class: "text-sm px-2 py-1 rounded-full bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300",
                                    "{expense_clone.category.display_name()}"
                                }
                            }

                            div { class: "flex space-x-2",

                                button {
                                    class: "text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300",
                                    onclick: handle_edit,

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
                                            d: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z",
                                        }
                                    }
                                }

                                button {
                                    class: "text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300",
                                    onclick: handle_delete,

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
                                            d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16",
                                        }
                                    }
                                }
                            }
                        }

                        if !expense_clone.notes.is_empty() {
                            p { class: "text-sm text-gray-600 dark:text-gray-400 mt-2",
                                "{expense_clone.notes}"
                            }
                        }
                    }
                }
            }
        }
    }
}
