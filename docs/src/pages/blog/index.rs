use dioxus::prelude::*;

#[component]
/// Renders a blog page component with a styled welcome header.
///
/// This function creates a Dioxus component that outputs a `div` element containing an `h1`
/// header with preset CSS classes. It is intended to serve as the main entry point for the blog view.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         Blog()
///     })
/// }
///
/// dioxus::desktop::launch(app);
/// ```
pub fn Blog() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Welcome to the Dioxus Blog!" }
        }
    }
}
