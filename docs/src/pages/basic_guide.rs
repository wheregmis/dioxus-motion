use crate::components::{
    guide_navigation::GuideNavigation, lesson::Lesson, motion_lab::MotionLab,
    quick_start::QuickStart,
};
use dioxus::prelude::*;

#[component]
pub fn BasicAnimationGuide() -> Element {
    rsx! {
        article { class: "lesson-guide",
            header { class: "lesson-intro",
                p { class: "eyebrow", "LESSON 01 / VALUES & SPRINGS" }
                h1 { "One value. A whole new feeling." }
                p { "Build a retargetable interaction, then learn how timing changes its feel. No animation runs until you start it." }
            }
            Lesson {
                title: "1. Create, animate, read",
                summary: "use_motion creates a reactive value. animate_to starts playback; get_value subscribes the component to changed values. In a component, propagate initialization errors with ? and handle event-handler errors in the UI.",
                exercise: "Click Move, then click again before it settles. The new target takes over from the current position. Change the 160.0 target in the source and predict the new endpoint.",
                source: dioxus_code::code!("/src/components/quick_start.rs"),
                QuickStart {}
            }
            section { class: "lesson",
                h2 { "2. Choose how it moves" }
                p { "A tween reaches its target over a duration. A spring evolves from its displacement and velocity until both are within its completion threshold. It has no fixed duration." }
                MotionLab {}
                p { class: "lesson-exercise", strong { "Try it: " } "Lower damping to see overshoot, then raise it to reduce bounce. Retarget during playback. Enable Instant motion to reach the target without an animated transition." }
                p { "Stiffness controls restoring force, damping resists velocity, and mass changes the response. Keep coefficients finite, stiffness and damping nonnegative, and mass positive." }
            }
            section { class: "lesson",
                h2 { "3. Keep invalid data outside motion" }
                p { "animate_to returns a typed error for invalid values or configuration and preserves the existing animation. Handle user-provided data with Result; reserve expect for fixed values whose validity you control. Runtime overflow stops playback at the last valid value." }
                p { "Next, keep the same value and coordinate it across time with loops, sequences, and keyframes." }
            }
            GuideNavigation {}
        }
    }
}
