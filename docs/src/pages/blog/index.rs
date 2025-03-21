use dioxus::prelude::*;

#[component]
/// Renders a Dioxus blog component with a welcoming header.
///
/// This component returns an `Element` containing a styled container with an `h1`
/// element that displays the message "Welcome to the Dioxus Blog!".
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// // Create the blog component element
/// let blog_element: Element = Blog();
/// // The returned element can be integrated into a Dioxus application view.
/// ```
pub fn Blog() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Welcome to the Dioxus Blog!" }
        }
    }
}
