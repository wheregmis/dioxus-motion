use crate::utils::router::Route;
use dioxus::prelude::*;

/// Styled 404 page naming the unmatched route and linking back to real pages.
#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        document::Title { "Page not found · Dioxus Motion" }
        main { id: "main-content", class: "not-found-shell",
            p { class: "eyebrow", "404 / LOST IN MOTION" }
            h1 { "This page left the timeline." }
            p { class: "lead",
                "Nothing lives at "
                code { "/{route.join(\"/\")}" }
                ". The links below all land somewhere real."
            }
            div { class: "hero-actions",
                Link { class: "button primary", to: Route::Home {}, "Back home →" }
                Link { class: "button secondary", to: Route::DocsLanding {}, "Read the docs" }
                Link { class: "button secondary", to: Route::ShowcaseGallery {}, "See examples" }
            }
        }
    }
}
