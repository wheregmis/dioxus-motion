use crate::Route;
use dioxus::prelude::*;

use crate::tailwind::include_tailwind_stylesheet;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    let navigator = use_navigator();

    rsx! {
        include_tailwind_stylesheet {}

        div { class: "min-h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-white flex flex-col items-center justify-center",

            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8 max-w-md w-full text-center",

                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                    "404"
                }

                p { class: "text-xl text-gray-600 dark:text-gray-400 mb-8", "Page not found" }

                pre { class: "bg-gray-100 dark:bg-gray-700 p-4 rounded mb-8 text-left overflow-auto",
                    "Attempted to navigate to: {route:?}"
                }

                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700",
                    onclick: move |_| {
                        navigator.push(Route::Dashboard {});
                    },
                    "Go to Dashboard"
                }
            }
        }
    }
}
