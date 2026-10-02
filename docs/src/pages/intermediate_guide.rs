use crate::{
    components::{guide_navigation::GuideNavigation, lesson::Lesson},
    pages::lessons::{keyframes::KeyframeDemo, loops::LoopDemo, sequence::SequenceDemo},
};
use dioxus::prelude::*;

#[component]
pub fn IntermediateAnimationGuide() -> Element {
    rsx! {
        article { class: "lesson-guide",
            header { class: "lesson-intro",
                p { class: "eyebrow", "LESSON 02 / CHOREOGRAPHY" }
                h1 { "Give motion a timeline." }
                p { "Choose repetition for a repeated action, a sequence for ordered destinations, or keyframes for a fixed timeline. Each example shows the exact Rust file used by its preview." }
            }
            Lesson {
                title: "1. Delay, repeat, stop",
                summary: "This tween waits 300ms and makes two round trips. AlternateTimes travels forward and back; Times repeats the forward path. Infinite and Alternate continue until you stop or replace playback.",
                exercise: "Start the round trips, stop halfway, then run again. Change AlternateTimes(2) to Times(2) to compare a return journey with a restarted forward journey.",
                source: dioxus_code::code!("/src/pages/lessons/loops.rs"), LoopDemo {}
            }
            Lesson {
                title: "2. Let one step finish before the next",
                summary: "A sequence first tweens to 100, springs back to 40, then tweens home. Each step has its own configuration. A spring step advances when it settles, so the sequence has no fixed total duration.",
                exercise: "Run the sequence and watch the pause in direction at 100 and 40. Change the middle spring damping and observe how it changes the time before the final step begins.",
                source: dioxus_code::code!("/src/pages/lessons/sequence.rs"), SequenceDemo {}
            }
            Lesson {
                title: "3. Put milestones on a fixed clock",
                summary: "Keyframe offsets are fractions of the total duration. This track reaches 100 at 25%, holds it through 60%, and returns to zero at 100%. Building the track validates its offsets and values before playback.",
                exercise: "Move the 0.60 offset to 0.80 to make the hold longer. Compare this fixed 1600ms timeline with the sequence above, whose middle spring controls when the last step starts.",
                source: dioxus_code::code!("/src/pages/lessons/keyframes.rs"), KeyframeDemo {}
            }
            GuideNavigation {}
        }
    }
}
