use super::code_block::CodeBlock;
use dioxus::prelude::*;
use dioxus_code::advanced::HighlightedSource;

/// Each preview and its complete source share one compiled Rust file.
#[component]
pub fn Lesson(
    title: String,
    summary: String,
    exercise: String,
    source: HighlightedSource,
    children: Element,
) -> Element {
    rsx! {
        section { class: "lesson",
            h2 { {title} }
            p { {summary} }
            div { class: "lesson-preview", {children} }
            p { class: "lesson-exercise", strong { "Try it: " } {exercise} }
            details { class: "lesson-source",
                summary { "Read the complete example" }
                CodeBlock { code: source, language: "Rust" }
            }
        }
    }
}
