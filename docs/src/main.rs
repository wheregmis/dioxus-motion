use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    rsx! {
        head {
            link {
                rel: "stylesheet",
                href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Inter:wght@400;500;600;700&display=swap",
            }
            link { rel: "stylesheet", href: MAIN_CSS }
        }
        Router::<docs::utils::router::Route> {}
    }
}

/// Launches the Dioxus documentation app.
///
/// The docs site uses plain navigation; route motion runs inside its isolated lesson.
fn main() {
    dioxus::launch(App);
}
