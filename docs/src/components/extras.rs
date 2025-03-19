use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::utils::router::Route;

#[component]
pub fn PageTransition() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Page Transitions" }
            AnimatedOutlet::<Route> {}
        }
    }
}

#[component]
pub fn Animations() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto px-6 py-12",
            h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Animations" }
            AnimatedOutlet::<Route> {}
        }
    }
}
