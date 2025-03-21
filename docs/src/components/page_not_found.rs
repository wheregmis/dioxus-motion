use dioxus::prelude::*;

#[component]
/// Renders a "Page not found" component that displays a message and the attempted navigation route.
/// 
/// This component visually informs users that the requested page does not exist by presenting a heading,
/// an apologetic message, and a formatted log of the navigation route that was attempted.
/// 
/// # Arguments
/// 
/// * `route` - A vector of strings representing the segments of the attempted navigation path.
/// 
/// # Returns
/// 
/// Returns an `Element` representing the rendered "Page not found" user interface.
/// 
/// # Examples
/// 
/// ```
/// use dioxus::prelude::*;
///
/// fn App(cx: Scope) -> Element {
///     PageNotFound(vec!["nonexistent".to_string(), "path".to_string()])
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
