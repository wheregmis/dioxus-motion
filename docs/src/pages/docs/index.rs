use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::utils::router::Route;

#[component]
pub fn Docs() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Documentation" }
            AnimatedOutlet::<Route> {}
        }
    }
}

#[component]
pub fn DocsLanding() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Docs Landing Page" }
            p { class: "text-gray-600 mb-4", "This is the landing page for the documentation." }

            Link {
                class: "text-blue-600 hover:underline",
                to: Route::PageTransition {},
                "Page Transitions"
            }
            Link {
                class: "text-blue-600 hover:underline",
                to: Route::Animations {},
                "Animations"
            }
        }
    }
}
