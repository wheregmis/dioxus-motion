use crate::{
    components::{guide_navigation::GuideNavigation, lesson::Lesson},
    pages::lessons::styles::{StyleDemo, UnitDemo},
};
use dioxus::prelude::*;

#[component]
pub fn MotionStyleGuide() -> Element {
    rsx! {
        article { class: "lesson-guide",
            header { class: "lesson-intro",
                p { class: "eyebrow", "LESSON 03 / CSS MOTION" }
                h1 { "A visual state, in one value." }
                p { "Use MotionStyle when several properties should move together. Keep separate motion values when they need independent timing or lifecycle control." }
            }
            Lesson {
                title: "1. Describe a state, then retarget it",
                summary: "motion_style! combines transform fields with typed CSS properties. This card changes scale, rotation, corner radius, and a parsed color through one spring. Formatting the current MotionStyle produces the element's inline CSS.",
                exercise: "Change state twice before the card settles. Add opacity to both targets. Transform rotations in MotionStyle are CSS degrees; do not convert them to radians.",
                source: dioxus_code::code!("/src/pages/lessons/styles.rs"), StyleDemo {}
            }
            Lesson {
                title: "2. Treat units as part of the value",
                summary: "A spring needs compatible arithmetic. A width in px cannot spring toward a percentage. Setup returns IncompatibleSpringValues and leaves current playback untouched. A tween accepts the change and switches incompatible units discretely at its endpoint.",
                exercise: "Try the rejected spring, then use the tween. Reset and change the target to CssValue::Px(200.0) in the source to make the spring compatible.",
                source: dioxus_code::code!("/src/pages/lessons/styles.rs"), UnitDemo {}
            }
            section { class: "lesson",
                h2 { "3. Pick properties with the browser in mind" }
                p { "Transform and opacity can avoid layout work. Width, height, and other geometry properties may trigger layout on every frame; use them when the layout change is the interaction you need, then profile the real page." }
                p { "Numeric lengths, parsed colors, and compatible complex strings interpolate. Keywords switch discretely. Keep matching units and complex-string shapes for springs; use tweens when states differ structurally." }
                p { "Raw MotionStyle properties remain your CSS input. The library validates numerical values; it does not make arbitrary CSS text safe for an untrusted style or HTML context." }
            }
            GuideNavigation {}
        }
    }
}
