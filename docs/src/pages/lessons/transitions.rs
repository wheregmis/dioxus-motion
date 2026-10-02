use dioxus::router::components::HistoryProvider;
use dioxus::{history::MemoryHistory, prelude::*};
use dioxus_motion::prelude::*;
use std::rc::Rc;

#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
enum PreviewRoute {
    #[layout(PreviewLayout)]
        #[route("/")]
        #[transition(Fade)]
        Overview {},
        #[route("/details")]
        #[transition(SlideLeft)]
        Details {},
}

#[component]
pub fn TransitionDemo() -> Element {
    let tween = use_store(|| Tween::new(Duration::from_millis(450)));
    use_context_provider(move || tween);
    rsx! {
        // Memory history keeps this embedded example separate from the docs URL.
        // A standalone web app normally uses its renderer's browser history.
        HistoryProvider { history: |_| Rc::new(MemoryHistory::default()) as Rc<dyn History>,
            div { class: "transition-demo",
                Router::<PreviewRoute> {}
            }
        }
    }
}

#[component]
fn PreviewLayout() -> Element {
    rsx! {
        nav { class: "lesson-actions", aria_label: "Example pages",
            Link { class: "button secondary", to: PreviewRoute::Overview {}, "Overview page" }
            Link { class: "button primary", to: PreviewRoute::Details {}, "Details page" }
        }
        div { style: "min-height:160px;overflow:hidden;margin-top:20px",
            AnimatedOutlet::<PreviewRoute> {}
        }
    }
}

#[component]
fn Overview() -> Element {
    rsx! { div { class: "transition-panel", h3 { "Overview" } p { "A quiet starting point." } } }
}

#[component]
fn Details() -> Element {
    rsx! { div { class: "transition-panel", h3 { "Details" } p { "A little more depth, one route away." } } }
}
