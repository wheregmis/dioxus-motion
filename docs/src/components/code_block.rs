use dioxus::prelude::*;
use dioxus_code::{Code, Theme, advanced::HighlightedSource};

/// A code block highlighted at compile time, with a copy action and text selection.
#[component]
pub fn CodeBlock(code: HighlightedSource, language: String) -> Element {
    let mut copied = use_signal(|| false);
    let mut failed = use_signal(|| false);
    let source = code.source().to_owned();
    rsx! {
        figure { class: "code-block",
            figcaption { class: "code-caption",
                span { {language} }
                button { aria_label: "Copy code", onclick: move |_| {
                    let source = source.clone();
                    spawn(async move {
                        let mut clipboard = document::eval(r#"
                            const text = await dioxus.recv();
                            try {
                                await navigator.clipboard.writeText(text);
                                dioxus.send(true);
                            } catch (_) { dioxus.send(false); }
                        "#);
                        let success = clipboard.send(source).is_ok() && clipboard.recv::<bool>().await.unwrap_or(false);
                        copied.set(success);
                        failed.set(!success);
                    });
                }, if copied() { "Copied" } else { "Copy" } }
            }
            Code { src: code, theme: Theme::GITHUB_DARK }
            if failed() { p { class: "copy-error", role: "status", "Clipboard unavailable. Select the code to copy it." } }
        }
    }
}
