use dioxus::prelude::*;

pub mod components;
pub mod pages;
pub mod utils;

use docs::utils::router::Route;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Launches the Dioxus web application.
///
/// This function serves as the entry point of the application. It initializes the Dioxus framework
/// with an HTML layout defined using the `rsx!` macro. The layout includes a head section that loads
/// external fonts from Google Fonts and a local stylesheet via the `MAIN_CSS` asset, as well as a
/// Router component parameterized with the `Route` type to handle navigation.
///
/// # Examples
///
/// ```no_run
/// // Entry point of the Dioxus application.
/// fn main() {
///     dioxus::launch(|| {
///         rsx! {
///             head {
///                 link {
///                     rel: "stylesheet",
///                     href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Inter:wght@400;500;600;700&display=swap",
///                 }
///                 link { rel: "stylesheet", href: MAIN_CSS }
///             }
///             Router::<Route> {}
///         }
///     });
/// }
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
