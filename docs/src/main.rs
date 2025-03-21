use dioxus::prelude::*;

pub mod components;
pub mod pages;
pub mod utils;

use docs::utils::router::Route;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Launches the Dioxus web application.
///
/// This function serves as the entry point of the application. It initializes the Dioxus runtime
/// by setting up the HTML document head with external stylesheets (including Google Fonts and the
/// main CSS asset) and configuring client-side routing using the Router component.
///
/// # Examples
///
/// ```rust
/// // Running the application will start the Dioxus web app:
/// main();
/// ```
fn main() {
    dioxus::launch(|| {
        rsx! {
            head {
                link {
                    rel: "stylesheet",
                    href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Inter:wght@400;500;600;700&display=swap",
                }
                link { rel: "stylesheet", href: MAIN_CSS }
            }
            Router::<Route> {}
        }
    });
}
