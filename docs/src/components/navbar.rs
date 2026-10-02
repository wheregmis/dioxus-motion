use crate::utils::router::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        a { class: "skip-link", href: "#main-content", "Skip to content" }
        header { class: "site-header",
            nav { class: "header-inner", aria_label: "Primary",
                Link { class: "wordmark", to: Route::Home {},
                    span { class: "brand-mark", aria_hidden: "true", "m" }
                    "dioxus-motion"
                }
                div { class: "header-links",
                    Link { to: Route::DocsLanding {}, "Docs" }
                    Link { to: Route::ShowcaseGallery {}, "Examples" }
                    Link { to: Route::Blog {}, "Blog" }
                    a { href: "https://github.com/wheregmis/dioxus-motion", "GitHub ↗" }
                }
            }
        }
        Outlet::<Route> {}
        footer { class: "site-footer",
            span { "Built with Rust. Made to move." }
            a { href: "https://github.com/wheregmis/dioxus-motion/blob/main/LICENSE", "MIT" }
        }
    }
}
