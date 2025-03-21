use dioxus::prelude::*;

#[component]
/// Renders the "Page Not Found" UI component.
///
/// This component displays an error message indicating that the requested page does not exist.
/// It includes a large heading, an apology message, and a preformatted text block that shows
/// the attempted navigation route.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn App(cx: Scope) -> Element {
///     cx.render(rsx! {
///         PageNotFound(route: vec!["/unknown/path".to_string()]),
///     })
/// }
/// ```
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Page not found" }
            p { class: "text-gray-600 mb-4",
                "We are terribly sorry, but the page you requested doesn't exist."
            }
            pre { class: "bg-red-50 text-red-600 p-4 rounded-md font-mono text-sm",
                "log:\nattemped to navigate to: {route:?}"
            }
        }
    }
}
