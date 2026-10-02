use crate::{
    components::{guide_navigation::GuideNavigation, lesson::Lesson},
    pages::lessons::custom::CustomPointDemo,
};
use dioxus::prelude::*;

#[component]
pub fn ComplexAnimationGuide() -> Element {
    rsx! {
        article { class: "lesson-guide",
            header { class: "lesson-intro",
                p { class: "eyebrow", "LESSON 04 / CUSTOM VALUES" }
                h1 { "Animate a type you own." }
                p { "Start with f32, Transform, Color, or MotionStyle when they fit. Implement Animatable when your domain needs a different value, such as a point with two independent coordinates." }
            }
            Lesson {
                title: "1. A point is more than its position",
                summary: "The same type represents position, displacement, forces, and velocity inside a spring. Add, Sub, and Mul must preserve signed components. Default multiplied by zero must produce an additive zero value.",
                exercise: "Retarget the point while it moves. Change the target coordinates. Keep x and y signed; clamping either inside arithmetic would make the return journey incorrect.",
                source: dioxus_code::code!("/src/pages/lessons/custom.rs"), CustomPointDemo {}
            }
            section { class: "lesson",
                h2 { "2. Make every contract explicit" }
                ul {
                    li { "Clone copies the value. PartialEq compares every component, including any discrete properties; reactive subscriptions use exact equality." }
                    li { "interpolate defines tween/keyframe interpolation. This point reuses the built-in f32 implementation, including its overflow-safe interpolation." }
                    li { "magnitude measures distance from zero for spring convergence. The point uses hypot for Euclidean distance." }
                    li { "is_finite checks each numeric component directly. A magnitude can overflow even when its inputs are finite." }
                    li { "is_spring_compatible defaults to true. Override it when units or component shapes can differ; incompatible transitions should use a tween." }
                }
                p { "The complete example includes a runnable test for signed arithmetic, finite validation, and extreme interpolation. Keep a small test beside a custom value so future edits preserve these contracts." }
            }
            GuideNavigation {}
        }
    }
}
