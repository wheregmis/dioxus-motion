use crate::{
    components::{guide_navigation::GuideNavigation, lesson::Lesson},
    pages::lessons::presence::{ManualPresenceDemo, PresenceDemo, PresenceModeDemo},
};
use dioxus::prelude::*;

#[component]
pub fn PresenceGuide() -> Element {
    rsx! {
        article { class: "lesson-guide",
            header { class: "lesson-intro",
                p { class: "eyebrow", "LESSON 05 / PRESENCE & LAYOUT" }
                h1 { "Let a goodbye finish." }
                p { "Removing an element from state normally unmounts it immediately. AnimatePresence retains a keyed child long enough for its exit work to finish, then removes it from the rendered tree." }
            }
            Lesson {
                title: "1. Own visibility outside the animated child",
                summary: "The parent owns whether the notice is present. The direct child has a stable key. Inside it, use_presence_style animates initial, animate, and exit states and reports completion to AnimatePresence.",
                exercise: "Show and dismiss the notice. Show it again before the exit finishes to see the same keyed child retarget rather than creating an unrelated notice.",
                source: dioxus_code::code!("/src/pages/lessons/presence.rs"), PresenceDemo {}
            }
            Lesson {
                title: "2. Keys define identity; modes define overlap",
                summary: "Changing a child's key creates an exiting identity and an entering identity. Wait keeps the new panel waiting for the old panel's exit. Sync lets both render at once. initial: false skips the initial animation for children present on the first render.",
                exercise: "Switch panels with Wait checked, then unchecked. Wait mode is intended for one child. For a list, use stable item IDs as keys, never its changing array index.",
                source: dioxus_code::code!("/src/pages/lessons/presence.rs"), PresenceModeDemo {}
            }
            Lesson {
                title: "3. Finish work before manual removal",
                summary: "use_presence exposes is_present and safe_to_remove. An explicit use_reactive dependency reruns the effect when that boolean changes. This panel waits 500ms after exit starts, then reports that it can unmount. If the platform timer fails, it still reports completion so the child cannot remain stuck.",
                exercise: "Remove the panel and watch its pending state. Replace the delay with your own completion signal when integrating an external animation or resource cleanup.",
                source: dioxus_code::code!("/src/pages/lessons/presence.rs"), ManualPresenceDemo {}
            }
            section { class: "lesson",
                h2 { "4. Add layout behavior deliberately" }
                p { "PresenceMode::PopLayout takes exiting children out of layout where platform measurement supports it. PresenceLayout::Size animates measured content size; it does not provide general shared-element layout transitions. Native renderers may have different measurement support." }
                p { "The presence_style! macro supports layout: size and a separate layout tween inside transition. Use propagate for nested presence boundaries that should follow an ancestor's exit; custom carries PresenceCustom data into exiting children." }
                p { "AnimatePresence also exposes initial, on_exit_complete, anchor_x, and anchor_y. Keep direct children keyed, and choose one completion owner per exit instead of mixing manual removal with a hook that already removes automatically." }
            }
            GuideNavigation {}
        }
    }
}
