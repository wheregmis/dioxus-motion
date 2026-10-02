use crate::components::motion_lab::MotionLab;
use crate::utils::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Dioxus Motion · Make your interface feel alive" }
        document::Meta { name: "description", content: "Typed, interruptible animations for Dioxus. Explore real springs, readable examples, CSS motion, and enter/exit transitions." }
        main { id: "main-content", class: "home-shell",
            section { class: "hero-grid",
                div { class: "hero-copy",
                    p { class: "eyebrow", span { class: "status-dot" } "RUST / DIOXUS 0.7" }
                    h1 { "Make your" br {} "interface" br {} span { "feel alive." } }
                    p { class: "lead", "Springs that respond. Types you can trust. Bring a little life to your Dioxus app, one value at a time." }
                    div { class: "hero-actions",
                        Link { class: "button primary", to: Route::DocsLanding {}, "Start building →" }
                        Link { class: "button secondary", to: Route::ShowcaseGallery {}, "Explore examples" }
                    }
                    p { class: "hero-note", "Typed values · Interruptible springs · Web & desktop" }
                }
                MotionLab {}
            }
            section { class: "principles-grid", aria_label: "Why Dioxus Motion",
                div { span { class: "eyebrow", "01 / FEEL" } h2 { "Motion with momentum." } p { "Give a spring a new destination while it’s moving. It keeps going from where it is." } }
                div { span { class: "eyebrow", "02 / SAFETY" } h2 { "Errors you can handle." } p { "Finite values, compatible units, and validated configuration. Invalid setup returns a typed error." } }
                div { span { class: "eyebrow", "03 / OWNERSHIP" } h2 { "Still just Rust." } p { "Animate scalars, transforms, colours, styles, or your own types. Keep control of the values." } }
            }
            section { class: "learn-section",
                div { class: "section-heading", div { p { class: "eyebrow", "A CLEAR PATH FORWARD" } h2 { "From first spring to full interface." } } Link { to: Route::DocsLanding {}, "Read the docs ↗" } }
                div { class: "learning-grid",
                    Link { class: "learning-card", to: Route::BasicAnimationGuide {}, span { class: "card-number", "01" } strong { "Start small" } span { "Create a motion value. Animate it. Read it in your component." } }
                    Link { class: "learning-card", to: Route::MotionStyleGuide {}, span { class: "card-number", "02" } strong { "Style in motion" } span { "Move beyond transforms with typed CSS properties." } }
                    Link { class: "learning-card", to: Route::PresenceGuide {}, span { class: "card-number", "03" } strong { "Make an entrance" } span { "Coordinate mounting, layout, and exit animations." } }
                }
            }
            section { class: "closing-note", p { "Good motion starts with a small experiment." } Link { class: "button secondary", to: Route::DocsLanding {}, "Try your first spring →" } }
        }
    }
}
