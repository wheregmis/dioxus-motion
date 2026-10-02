use crate::components::{code_block::CodeBlock, motion_lab::MotionLab, quick_start::QuickStart};
use crate::utils::router::Route;
use dioxus::prelude::*;
use dioxus_code::{CodeOptions, Language, code, code_str};

#[component]
fn DocsNavigation(#[props(default)] on_navigate: EventHandler<()>) -> Element {
    let current: Route = use_route();
    let links = [
        ("Start here", Route::DocsLanding {}),
        ("Values & springs", Route::BasicAnimationGuide {}),
        ("Loops & sequences", Route::IntermediateAnimationGuide {}),
        ("Animating CSS", Route::MotionStyleGuide {}),
        ("Custom types", Route::ComplexAnimationGuide {}),
        ("Enter & exit", Route::PresenceGuide {}),
        ("Page transitions", Route::PageTransition {}),
    ];
    rsx! {
        nav { class: "docs-navigation", aria_label: "Documentation",
            p { class: "eyebrow", "LEARN" }
            for (label, to) in links {
                Link { class: if current == to { "selected" } else { "" }, to, onclick: move |_| on_navigate.call(()), {label} }
            }
            p { class: "eyebrow nav-group", "REFERENCE" }
            a { href: "https://docs.rs/dioxus-motion", "Published API ↗" }
            a { href: "https://github.com/wheregmis/dioxus-motion/blob/main/CHANGELOG.md", "Release notes ↗" }
        }
    }
}

#[component]
pub fn Docs() -> Element {
    let mut menu_open = use_signal(|| false);
    let route: Route = use_route();
    let title = match route {
        Route::BasicAnimationGuide {} => "Values & springs",
        Route::IntermediateAnimationGuide {} => "Loops & sequences",
        Route::MotionStyleGuide {} => "Animating CSS",
        Route::ComplexAnimationGuide {} => "Custom types",
        Route::PresenceGuide {} => "Enter & exit",
        Route::PageTransition {} => "Page transitions",
        _ => "Getting started",
    };
    rsx! {
        document::Title { "{title} · Dioxus Motion" }
        div { class: "docs-shell",
            aside { class: "desktop-sidebar", DocsNavigation {} }
            div { class: "mobile-sidebar",
                button { r#type: "button", aria_expanded: menu_open(), aria_controls: "docs-menu", onclick: move |_| menu_open.toggle(), "Documentation menu" }
                if menu_open() { div { id: "docs-menu", DocsNavigation { on_navigate: move |_| menu_open.set(false) } } }
            }
            main { id: "main-content", class: "docs-content", Outlet::<Route> {} }
        }
    }
}

#[component]
pub fn DocsLanding() -> Element {
    rsx! {
        document::Title { "Getting started · Dioxus Motion" }
        article { class: "getting-started",
            p { class: "eyebrow", "GETTING STARTED" }
            h1 { "Your first spring." }
            p { class: "lead", "Create a value, give it a destination, and let the motion handle update your component. Start with a working example, then tune the feel." }
            div { class: "callout",
                strong { "You’re reading the next release." }
                p { "These examples compile against this repository and Dioxus 0.7.10. The published 0.3.6 API differs; use the branch below while trying the new API." }
            }
            section { id: "install",
                h2 { "01 / Add the library" }
                p { "For a Dioxus web app, add the web feature. Commit Cargo.lock to keep the Git dependency reproducible." }
                CodeBlock {
                    language: "Cargo.toml".to_string(),
                    code: dioxus_code::advanced::HighlightedSource::from_static_parts(
                        r#"[dependencies]
dioxus = { version = "0.7.10", features = ["web"] }
dioxus-motion = { git = "https://github.com/wheregmis/dioxus-motion", branch = "main", default-features = false, features = ["web"] }"#, Language::Rust, &[]),
                }
            }
            section {
                h2 { "02 / Make something move" }
                p { "This is the source of the live example below. use_motion and animate_to return typed errors, which you can propagate from a component with ?." }
                div { class: "example-preview", QuickStart {} }
                CodeBlock { language: "Rust".to_string(), code: code!("/src/components/quick_start.rs") }
            }
            section {
                h2 { "03 / Find the right feel" }
                p { "Stiffness pulls toward the target. Damping removes the bounce. Try changing both, then move the target while the spring is still running." }
                MotionLab {}
            }
            section {
                h2 { "Choose your next step" }
                div { class: "learning-grid",
                    Link { class: "learning-card", to: Route::BasicAnimationGuide {}, strong { "Values & springs" } span { "Start with scalar values, then animate a transform." } }
                    Link { class: "learning-card", to: Route::MotionStyleGuide {}, strong { "Animating CSS" } span { "Keep CSS units and properties in typed values." } }
                    Link { class: "learning-card", to: Route::PresenceGuide {}, strong { "Enter & exit" } span { "Keep a child mounted until its exit completes." } }
                }
            }
            section {
                h2 { "A few rules that keep motion predictable" }
                ul {
                    li { "Use springs for compatible numeric values. CSS units and complex shapes must match between shared properties." }
                    li { "Use a tween when changing units, keywords, or discrete values." }
                    li { "Custom types implement Clone + PartialEq + Animatable. Equality controls reactive updates; epsilon controls spring completion." }
                    li { "Validate user input and handle AnimationError. Rejected setup preserves existing playback." }
                }
                CodeBlock { language: "Rust".to_string(), code: code_str!("motion.animate_to(target, AnimationConfig::spring(Spring::default()))?;", CodeOptions::builder().with_language(Language::Rust)) }
            }
        }
    }
}
