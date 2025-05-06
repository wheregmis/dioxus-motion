use dioxus::prelude::*;

pub fn include_tailwind_stylesheet() -> Element {
    rsx! {
        // The Stylesheet component inserts a style link into the head of the document
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/main.css"),
        }
    }
}
