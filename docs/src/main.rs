use dioxus::prelude::*;
use dioxus_motion::prelude::*;

pub mod components;
pub mod old_showcase;
pub mod pages;
pub mod utils;

use easer::functions::Easing;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn route_index(route: &docs::utils::router::Route) -> i32 {
    match route {
        docs::utils::router::Route::Home { .. } => 0,
        docs::utils::router::Route::DocsLanding { .. } => 1,
        docs::utils::router::Route::ShowcaseGallery { .. } => 2,
        _ => -1,
    }
}

fn transition_resolver() -> TransitionVariantResolver<docs::utils::router::Route> {
    std::rc::Rc::new(|from, to| {
        let from_idx = route_index(from);
        let to_idx = route_index(to);

        if from_idx != -1 && to_idx != -1 {
            if to_idx > from_idx {
                TransitionVariant::SlideLeft
            } else if to_idx < from_idx {
                TransitionVariant::SlideRight
            } else {
                TransitionVariant::Fade
            }
        } else {
            to.get_transition()
        }
    })
}

#[component]
fn App() -> Element {
    use_context_provider(transition_resolver);

    // Provide the transition animation mode through store-backed context.
    let tween = use_store(|| Tween {
        duration: std::time::Duration::from_millis(500),
        easing: easer::functions::Cubic::ease_in_out,
    });
    use_context_provider(move || tween);

    // Swap to a store-backed spring when you want physics-based transitions:
    // let spring = use_store(|| Spring {
    //     stiffness: 220.0,
    //     damping: 30.0,
    //     mass: 1.0,
    //     velocity: 0.0,
    // });
    // use_context_provider(move || spring);

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
/// The docs site wires up a route transition resolver plus an optional store-backed
/// tween or spring context, then renders the router and site stylesheet.
fn main() {
    dioxus::launch(App);
}
