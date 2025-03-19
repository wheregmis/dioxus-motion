use dioxus::prelude::*;

pub mod components;
pub mod hooks;
pub mod pages;
pub mod utils;

use docs::utils::router::Route;

const MAIN_CSS: Asset = asset!("/assets/main.css");

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
