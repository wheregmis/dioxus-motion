use crate::{
    components::{guide_navigation::GuideNavigation, lesson::Lesson},
    pages::lessons::transitions::TransitionDemo,
};
use dioxus::prelude::*;

#[component]
pub fn PageTransition() -> Element {
    rsx! {
        article { class: "lesson-guide",
            header { class: "lesson-intro",
                p { class: "eyebrow", "LESSON 06 / PAGE TRANSITIONS" }
                h1 { "Keep the journey connected." }
                p { "Apply motion at a route outlet. Keep navigation as normal Dioxus links, and choose the entering route's transition through MotionTransitions." }
            }
            Lesson {
                title: "1. Give the outlet a transition",
                summary: "Enable the transitions feature, derive Routable and MotionTransitions on your route enum, add transition attributes, and render AnimatedOutlet in its layout. This example uses a store-backed 450ms tween to set timing.",
                exercise: "Open Details, then return to Overview. Details uses SlideLeft; Overview uses Fade. Change one transition attribute in the complete example and observe the entering page.",
                source: dioxus_code::code!("/src/pages/lessons/transitions.rs"), TransitionDemo {}
            }
            section { class: "lesson",
                h2 { "2. Separate routing from presentation" }
                p { "The embedded example supplies MemoryHistory so its links stay inside this preview. A standalone web app normally uses browser history from its renderer. Keep ordinary Link and navigator calls; AnimatedOutlet coordinates the old and new route content." }
                p { "Place the outlet inside the layout whose children should transition. Keep persistent navigation outside it. Avoid nesting animated outlets just to animate every layout level at once." }
            }
            section { class: "lesson",
                h2 { "3. Keep motion optional" }
                p { "A Store<Tween> or Store<Spring> in context controls timing. An optional TransitionVariantResolver<Route> chooses a transition dynamically from the source and destination routes. Use it when route relationships determine direction rather than hard-coding every pair." }
                p { "Respect the user's reduced-motion preference in your application. A zero-duration tween can make the transition immediate; a plain Outlet is also appropriate when route motion does not help orientation. Keep keyboard focus and page titles tied to the new route." }
                p { "Built-in variants include Fade, SlideLeft, SlideRight, SlideUp, SlideDown, ZoomIn, and ZoomOut. Start with one consistent effect before adding a resolver." }
            }
            GuideNavigation {}
        }
    }
}
