use crate::utils::router::Route;
use dioxus::prelude::*;

#[component]
pub fn GuideNavigation() -> Element {
    let current = use_route::<Route>();
    let sections = [
        (Route::DocsLanding {}, "Start here"),
        (Route::BasicAnimationGuide {}, "Values & springs"),
        (Route::IntermediateAnimationGuide {}, "Loops & sequences"),
        (Route::MotionStyleGuide {}, "CSS motion"),
        (Route::ComplexAnimationGuide {}, "Custom values"),
        (Route::PresenceGuide {}, "Presence & layout"),
        (Route::PageTransition {}, "Page transitions"),
    ];
    let Some(index) = sections.iter().position(|(route, _)| *route == current) else {
        return rsx! {};
    };
    rsx! {
        nav { class: "guide-pagination", aria_label: "Lesson navigation",
            if let Some((route, title)) = index.checked_sub(1).and_then(|i| sections.get(i)) {
                Link { to: route.clone(), "← {title}" }
            }
            if let Some((route, title)) = sections.get(index + 1) {
                Link { to: route.clone(), "{title} →" }
            }
        }
    }
}
