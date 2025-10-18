use dioxus::prelude::*;
use dioxus_motion::prelude::*;

pub mod components;
pub mod old_showcase;
pub mod pages;
pub mod utils;

use easer::functions::Easing;

const MAIN_CSS: Asset = asset!("/assets/tailwind.css");

/// Launches the Dioxus web application.
///
/// This function serves as the entry point of the application. It initializes the Dioxus framework
/// with an HTML layout defined using the `rsx!` macro. The layout includes a head section that loads
/// external fonts from Google Fonts and a local stylesheet via the `MAIN_CSS` asset, as well as a
/// Router component parameterized with the `Route` type to handle navigation.
///

fn main() {
    dioxus::launch(|| {
        // Dynamic transition resolver for Home, Docs, ShowcaseGallery
        use dioxus_motion::prelude::TransitionVariant;
        use dioxus_motion::transitions::page_transitions::TransitionVariantResolver;
        use docs::utils::router::Route;
        let resolver: TransitionVariantResolver<Route> = std::rc::Rc::new(|from, to| {
            fn idx(route: &Route) -> i32 {
                match route {
                    Route::Home { .. } => 0,
                    Route::DocsLanding { .. } => 1,
                    Route::ShowcaseGallery { .. } => 2,
                    _ => -1,
                }
            }
            let from_idx = idx(from);
            let to_idx = idx(to);
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
        });
        use_context_provider(|| resolver);

        // To use a Tween for page transitions, provide it via context:
        let tween = use_signal(|| Tween {
            duration: std::time::Duration::from_millis(500),
            easing: easer::functions::Cubic::ease_in_out,
        });
        use_context_provider(|| tween);

        // To use a Spring instead, comment out the above and uncomment below:
        // let spring = use_signal(|| Spring {
        //     stiffness: 220.0,
        //     damping: 30.0,
        //     mass: 1.0,
        //     velocity: 0.0,
        // });
        // use_context_provider(|| spring);

        // Example: Provide a dynamic transition resolver for card navigation
        // use dioxus_motion::transitions::page_transitions::{TransitionVariantResolver};
        // use dioxus_motion::prelude::TransitionVariant;
        //
        // let resolver: TransitionVariantResolver<Route> = std::rc::Rc::new(|from, to| {
        //     // Assuming Route::Card { idx } for cards
        //     match (from, to) {
        //         (Route::Card { idx: from_idx }, Route::Card { idx: to_idx }) => {
        //             if to_idx > from_idx {
        //                 TransitionVariant::SlideLeft
        //             } else if to_idx < from_idx {
        //                 TransitionVariant::SlideRight
        //             } else {
        //                 TransitionVariant::Fade
        //             }
        //         }
        //         _ => to.get_transition(),
        //     }
        // });
        // use_context_provider(|| resolver);

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
