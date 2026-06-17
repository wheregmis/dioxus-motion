use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::components::code_block::CodeBlock;
use crate::components::guide_navigation::GuideNavigation;

/// Renders the AnimatePresence documentation guide.
#[component]
pub fn PresenceGuide() -> Element {
    rsx! {
        div { class: "space-y-12",
            PresenceHero {}
            MentalModel {}
            FirstExitExample {}
            SpringExitExample {}
            StableKeysExample {}
            ModeGuideExample {}
            LayoutPopExample {}
            CustomDataExample {}
            ManualRemovalExample {}
            ApiReference {}
            GuideNavigation {}
        }
    }
}

#[component]
fn PresenceHero() -> Element {
    rsx! {
        section { class: "space-y-6",
            div { class: "space-y-3",
                h2 { class: "text-3xl font-semibold text-text-primary", "AnimatePresence" }
                p { class: "max-w-3xl text-text-secondary",
                    "AnimatePresence lets keyed Dioxus children finish an exit animation after your RSX stops rendering them. Use it for disappearing panels, notification stacks, route-like view swaps, and list items that need to leave cleanly instead of vanishing."
                }
            }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-3",
                ConceptCard {
                    title: "Keep exits alive",
                    body: "Removed keyed children stay mounted while their exit work runs.",
                }
                ConceptCard {
                    title: "Choose the hook",
                    body: "Use style, motion, or manual presence based on what has to animate.",
                }
                ConceptCard {
                    title: "Sequence deliberately",
                    body: "Pick sync, wait, or pop layout behavior to match the UI transition.",
                }
            }
        }
    }
}

#[component]
fn ConceptCard(title: &'static str, body: &'static str) -> Element {
    rsx! {
        div { class: "rounded-lg border border-primary/10 bg-primary/5 p-4",
            h3 { class: "font-medium text-text-primary", "{title}" }
            p { class: "mt-2 text-sm text-text-secondary", "{body}" }
        }
    }
}

#[component]
fn MentalModel() -> Element {
    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "Mental Model" }
            p { class: "text-text-secondary",
                "Presence is a small lifecycle around direct keyed children. The key is the identity. When the key disappears from your RSX, AnimatePresence keeps the old child around, marks it as exiting, waits for completion, then removes it."
            }
            div { class: "grid grid-cols-1 lg:grid-cols-4 gap-3",
                LifecycleStep { number: "1", title: "Render", body: "A direct child appears with a stable key." }
                LifecycleStep { number: "2", title: "Remove", body: "Your state stops rendering that key." }
                LifecycleStep { number: "3", title: "Exit", body: "The retained child sees is_present = false." }
                LifecycleStep { number: "4", title: "Complete", body: "safe_to_remove runs automatically or manually." }
            }
            div { class: "rounded-lg border border-secondary/20 bg-secondary/5 p-4",
                h4 { class: "font-medium text-text-primary", "Pick one path" }
                ul { class: "mt-2 list-disc list-inside space-y-1 text-sm text-text-secondary",
                    li { code { class: "text-primary", "use_presence_style" } " for CSS-ready opacity, transforms, width, height, and related style values." }
                    li { code { class: "text-primary", "use_presence_motion" } " for one typed animated value, such as a number or color." }
                    li { code { class: "text-primary", "use_presence" } " when custom async work decides when the child is safe to remove." }
                }
            }
        }
    }
}

#[component]
fn LifecycleStep(number: &'static str, title: &'static str, body: &'static str) -> Element {
    rsx! {
        div { class: "rounded-lg border border-primary/10 bg-dark-200/30 p-4",
            div { class: "mb-3 flex h-8 w-8 items-center justify-center rounded-full bg-primary/20 text-sm font-semibold text-primary",
                "{number}"
            }
            h4 { class: "font-medium text-text-primary", "{title}" }
            p { class: "mt-2 text-sm text-text-secondary", "{body}" }
        }
    }
}

#[component]
fn FirstExitExample() -> Element {
    let mut visible = use_signal(|| true);
    let box_key = "basic-box";

    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "Basic Exit With use_presence_motion" }
            p { class: "text-text-secondary",
                "Wrap the conditional child in AnimatePresence, give the direct child a stable key, and animate one value with use_presence_motion. The hook calls safe_to_remove when the exit motion stops."
            }
            ExampleGrid {
                code: {r#"#[component]
fn Example() -> Element {
    let mut visible = use_signal(|| true);

    rsx! {
        button { onclick: move |_| visible.toggle(), "Toggle" }
        AnimatePresence {
            if visible() {
                FadeBox { key: "basic-box" }
            }
        }
    }
}

#[component]
fn FadeBox() -> Element {
    let opacity = use_presence_motion(
        0.0f32,
        1.0,
        0.0,
        AnimationConfig::tween_ms(220),
    );

    rsx! {
        div { style: "opacity: {opacity.get_value()}", "Fades in and out" }
    }
}"#},
                div { class: "space-y-4",
                    button {
                        class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                        onclick: move |_| visible.toggle(),
                        if visible() { "Hide box" } else { "Show box" }
                    }
                    div { class: "min-h-24 rounded-lg bg-dark-200/30 p-4",
                        AnimatePresence {
                            if visible() {
                                FadeBox {
                                    key: "{box_key}",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FadeBox() -> Element {
    let opacity = use_presence_motion(0.0f32, 1.0, 0.0, AnimationConfig::tween_ms(220));

    rsx! {
        div {
            class: "inline-flex rounded-lg border border-primary/20 bg-primary/10 px-4 py-3 text-text-primary",
            style: "opacity: {opacity.get_value()}",
            "Fades in and out"
        }
    }
}

#[component]
fn SpringExitExample() -> Element {
    let mut visible = use_signal(|| true);
    let toast_key = "sync-toast";

    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "Spring Exit Toast" }
            p { class: "text-text-secondary",
                "Use a spring when the exit should feel physical instead of timed. The same presence lifecycle applies: the toast is removed from your state, stays mounted while the spring runs, then unmounts when motion settles."
            }
            ExampleGrid {
                code: {r#"#[component]
fn SpringToast() -> Element {
    let style = use_presence_style(presence_style! {
        initial: { opacity: 0.0, y: 30.0, scale: 0.90 },
        animate: { opacity: 1.0, y: 0.0, scale: 1.0 },
        exit: { opacity: 0.0, y: 30.0, scale: 0.80 },
        transition: spring {
            stiffness: 420.0,
            damping: 28.0,
            mass: 1.0,
        },
    });

    rsx! {
        div { style: "{style.get_value()}", "Synced to device" }
    }
}"#},
                div { class: "space-y-4",
                    div { class: "flex flex-wrap gap-2",
                        button {
                            class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                            onclick: move |_| visible.toggle(),
                            if visible() { "Dismiss toast" } else { "Show toast" }
                        }
                    }
                    div { class: "flex min-h-72 items-center justify-center overflow-hidden rounded-lg border border-primary/10 bg-dark-200/30 p-4",
                        AnimatePresence {
                            if visible() {
                                SpringToast {
                                    key: "{toast_key}",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SpringToast() -> Element {
    let style = use_presence_style(presence_style! {
        initial: { opacity: 0.0, y: 30.0, scale: 0.90 },
        animate: { opacity: 1.0, y: 0.0, scale: 1.0 },
        exit: { opacity: 0.0, y: 60.0, scale: 0.95 },
        enter_transition: spring {
            stiffness: 220.0,
            damping: 14.0,
            mass: 1.0,
        },
        exit_transition: tween {
            duration: 100.0
        }
    });

    rsx! {
        div {
            class: "w-full max-w-sm rounded-xl border border-primary/20 bg-background-secondary p-4 text-text-primary shadow-[0_24px_70px_-32px_rgba(0,0,0,0.85)]",
            style: "{style.get_value()}",
            div { class: "flex items-start justify-between gap-4",
                div {
                    p { class: "text-sm font-semibold", "Synced to device" }
                    p { class: "mt-1 text-sm text-text-secondary",
                        "Dismiss it to see the spring carry the toast offscreen."
                    }
                }
                span { class: "rounded-full bg-primary/10 px-2.5 py-1 text-xs text-primary",
                    "Live"
                }
            }
        }
    }
}

#[component]
fn StableKeysExample() -> Element {
    let mut items = use_signal(|| {
        vec![
            QueueItem::new(1, "Payment captured"),
            QueueItem::new(2, "Invite sent"),
            QueueItem::new(3, "Backup complete"),
        ]
    });
    let mut next_id = use_signal(|| 4usize);

    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "Notification Stack With Stable Keys" }
            p { class: "text-text-secondary",
                "Presence tracks children by key. A toast stack is a good place to see why durable IDs matter: dismissing one notification should animate that notification out, not whichever row inherited its index."
            }
            ExampleGrid {
                code: {r#"// Good: the key follows the data item.
AnimatePresence {
    for item in items() {
        QueueRow {
            key: "{item.id}",
            item
        }
    }
}

// Avoid index keys for presence lists. Removing the
// first item shifts every later index to a different row."#},
                div { class: "space-y-4",
                    div { class: "flex flex-wrap gap-2",
                        button {
                            class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                            onclick: move |_| {
                                if !items.read().is_empty() {
                                    items.write().remove(0);
                                }
                            },
                            "Dismiss oldest"
                        }
                        button {
                            class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                            onclick: move |_| {
                                let id = *next_id.read();
                                items.write().push(QueueItem::new(id, next_queue_label(id)));
                                *next_id.write() = id + 1;
                            },
                            "Push toast"
                        }
                        button {
                            class: "px-4 py-2 rounded-lg bg-secondary/20 text-secondary transition-colors hover:bg-secondary/30",
                            onclick: move |_| {
                                *items.write() = vec![
                                    QueueItem::new(1, "Payment captured"),
                                    QueueItem::new(2, "Invite sent"),
                                    QueueItem::new(3, "Backup complete"),
                                ];
                                *next_id.write() = 4;
                            },
                            "Reset"
                        }
                    }
                    div { class: "min-h-44 rounded-lg bg-dark-200/30 p-4",
                        div { class: "space-y-2",
                            AnimatePresence {
                                for item in items.read().iter() {
                                    QueueRow {
                                        key: "{item.id}",
                                        item: item.clone(),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct QueueItem {
    id: usize,
    label: &'static str,
}

impl QueueItem {
    fn new(id: usize, label: &'static str) -> Self {
        Self { id, label }
    }
}

fn next_queue_label(id: usize) -> &'static str {
    match id % 4 {
        0 => "Refund queued",
        1 => "Payment captured",
        2 => "Invite sent",
        _ => "Backup complete",
    }
}

#[component]
fn QueueRow(item: QueueItem) -> Element {
    let style = use_presence_style(presence_style! {
        initial: { opacity: 0.0, x: 100.0, scale: 0.8 },
        animate: { opacity: 1.0, x: 0.0 },
        exit: { opacity: 0.0, y: -100.0, scale: 0.8 },
        layout: size,
        transition: tween { duration: 333.0 },
    });

    rsx! {
        div {
            class: "flex items-center justify-between gap-4 rounded-lg border border-primary/10 bg-dark-200/30 p-4 shadow-lg",
            style: "{style.get_value()}",
            div {
                span { class: "block text-sm font-medium", "{item.label}" }
                span { class: "block text-xs text-text-muted", "stable key: {item.id}" }
            }
            span { class: "rounded-full bg-accent/10 px-2.5 py-1 text-xs text-accent", "toast" }
        }
    }
}

#[component]
fn ModeGuideExample() -> Element {
    let mut page = use_signal(|| 0usize);
    let pages = ["Profile", "Billing", "Security"];
    let active = pages[*page.read() % pages.len()];

    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "Sequencing: Sync, Wait, and PopLayout" }
            p { class: "text-text-secondary",
                "PresenceMode controls how entering children relate to exiting children. Use Wait for one-at-a-time view swaps, Sync when overlap is fine, and PopLayout when siblings should reflow while the removed item exits."
            }
            div { class: "rounded-lg bg-dark-200/50 p-3",
                CodeBlock {
                    language: "rust".to_string(),
                    code: r#"AnimatePresence { mode: PresenceMode::Sync, /* overlap */ }
AnimatePresence { mode: PresenceMode::Wait, /* exit before enter */ }
AnimatePresence { mode: PresenceMode::PopLayout, /* reflow siblings */ }"#.to_string(),
                }
            }
            div { class: "space-y-4 rounded-lg bg-dark-200/30 p-4",
                button {
                    class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                    onclick: move |_| {
                        let next = (*page.read() + 1) % pages.len();
                        *page.write() = next;
                    },
                    "Switch view"
                }
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-4",
                    ModePreview {
                        title: "Sync",
                        description: "New and old views overlap.",
                        mode: PresenceMode::Sync,
                        active,
                    }
                    ModePreview {
                        title: "Wait",
                        description: "The next view waits for exit completion.",
                        mode: PresenceMode::Wait,
                        active,
                    }
                }
            }
        }
    }
}

#[component]
fn ModePreview(
    title: &'static str,
    description: &'static str,
    mode: PresenceMode,
    active: &'static str,
) -> Element {
    rsx! {
        div { class: "rounded-lg border border-primary/10 bg-background-secondary p-4",
            h4 { class: "font-medium text-text-primary", "{title}" }
            p { class: "mt-1 text-sm text-text-secondary", "{description}" }
            div { class: "mt-4 min-h-20 overflow-hidden",
                AnimatePresence { mode,
                    ViewPanel {
                        key: "{title}-{active}",
                        label: active,
                    }
                }
            }
        }
    }
}

#[component]
fn ViewPanel(label: &'static str) -> Element {
    let style = use_presence_style(presence_style! {
        initial: { opacity: 0.0, x: 20.0 },
        animate: { opacity: 1.0, x: 0.0 },
        exit: { opacity: 0.0, x: -20.0 },
        layout: size,
        transition: tween {
            layout: tween { duration: 444.0 },
            duration: 440.0,
        },
    });

    rsx! {
        div {
            class: "rounded-lg border border-primary/20 bg-primary/10 p-4 text-text-primary",
            style: "{style.get_value()}",
            "{label}"
        }
    }
}

#[component]
fn LayoutPopExample() -> Element {
    let mut chips = use_signal(|| vec!["Alpha", "Beta", "Gamma", "Delta"]);

    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "Layout Popping" }
            p { class: "text-text-secondary",
                "PopLayout removes exiting children from layout immediately while they animate from their measured position. Add anchors when a popped child should hold its right or bottom edge instead of the default left/top edge."
            }
            ExampleGrid {
                code: {r#"AnimatePresence {
    mode: PresenceMode::PopLayout,
    anchor_x: PresenceAnchorX::Left,
    anchor_y: PresenceAnchorY::Top,
    for chip in chips() {
        Chip { key: "{chip}", label: chip }
    }
}"#},
                div { class: "space-y-4",
                    div { class: "flex flex-wrap gap-2",
                        button {
                            class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                            onclick: move |_| {
                                if !chips.read().is_empty() {
                                    chips.write().remove(0);
                                }
                            },
                            "Remove first"
                        }
                        button {
                            class: "px-4 py-2 rounded-lg bg-secondary/20 text-secondary transition-colors hover:bg-secondary/30",
                            onclick: move |_| *chips.write() = vec!["Alpha", "Beta", "Gamma", "Delta"],
                            "Reset"
                        }
                    }
                    div { class: "min-h-24 overflow-hidden rounded-lg bg-dark-200/30 p-4",
                        div { class: "flex flex-wrap gap-3",
                            AnimatePresence {
                                mode: PresenceMode::PopLayout,
                                anchor_x: PresenceAnchorX::Left,
                                anchor_y: PresenceAnchorY::Top,
                                initial: false,
                                for chip in chips.read().iter() {
                                    PopChip {
                                        key: "{chip}",
                                        label: *chip,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PopChip(label: &'static str) -> Element {
    let style = use_presence_style(presence_style! {
        initial: { opacity: 0.0, scale: 0.9 },
        animate: { opacity: 1.0, scale: 1.0 },
        exit: { opacity: 0.0, scale: 0.9 },
        layout: size,
        transition: tween { duration: 220.0 },
    });

    rsx! {
        div {
            class: "rounded-lg border border-secondary/20 bg-secondary/10 px-4 py-3 text-text-primary",
            style: "{style.get_value()}",
            "{label}"
        }
    }
}

#[component]
fn CustomDataExample() -> Element {
    let mut index = use_signal(|| 0usize);
    let mut direction = use_signal(|| 1i32);
    let slides = ["Intro", "Details", "Review"];
    let current = slides[*index.read() % slides.len()];

    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "Custom Data Captured For Exits" }
            p { class: "text-text-secondary",
                "The custom prop stores data on the presence boundary. Exiting children can still read the captured value with use_presence_data, even after the state that rendered them has moved on."
            }
            ExampleGrid {
                code: {r#"AnimatePresence {
    mode: PresenceMode::Wait,
    custom: PresenceCustom::new(direction),
    Slide {
        key: "{current}",
        label: current
    }
}

#[component]
fn Slide(label: &'static str) -> Element {
    let direction = use_presence_data::<i32>().unwrap_or(1);
    let offset = 32.0 * direction as f32;
    let x = use_presence_motion(offset, 0.0, -offset, config);
    /* render with translateX(x) */
}"#},
                div { class: "space-y-4",
                    div { class: "flex flex-wrap gap-2",
                        button {
                            class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                            onclick: move |_| {
                                *direction.write() = -1;
                                let len = slides.len();
                                let next = (*index.read() + len - 1) % len;
                                *index.write() = next;
                            },
                            "Previous"
                        }
                        button {
                            class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                            onclick: move |_| {
                                *direction.write() = 1;
                                let next = (*index.read() + 1) % slides.len();
                                *index.write() = next;
                            },
                            "Next"
                        }
                    }
                    div { class: "min-h-24 overflow-hidden rounded-lg bg-dark-200/30 p-4",
                        AnimatePresence {
                            mode: PresenceMode::Wait,
                            custom: PresenceCustom::new(*direction.read()),
                            SlideCard {
                                key: "{current}",
                                label: current,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SlideCard(label: &'static str) -> Element {
    let direction = use_presence_data::<i32>().unwrap_or(1);
    let offset = 32.0 * direction as f32;
    let x = use_presence_motion(offset, 0.0, -offset, AnimationConfig::tween_ms(240));
    let opacity = use_presence_motion(0.0f32, 1.0, 0.0, AnimationConfig::tween_ms(240));

    rsx! {
        div {
            class: "rounded-lg border border-primary/20 bg-primary/10 p-4 text-text-primary",
            style: "opacity: {opacity.get_value()}; transform: translateX({x.get_value()}px)",
            "{label}"
        }
    }
}

#[component]
fn ManualRemovalExample() -> Element {
    let mut visible = use_signal(|| true);
    let mut completed = use_signal(|| 0usize);
    let manual_key = "manual-panel";

    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "Manual Removal" }
            p { class: "text-text-secondary",
                "Use use_presence when exit completion depends on work outside the built-in motion helpers. Call safe_to_remove exactly when the child can unmount."
            }
            ExampleGrid {
                code: {r#"#[component]
fn AsyncPanel() -> Element {
    let presence = use_presence();
    let is_present = presence.is_present;

    use_effect(move || {
        if !is_present {
            let safe_to_remove = presence.safe_to_remove;
            spawn(async move {
                Time::delay(Duration::from_millis(500)).await;
                safe_to_remove.call(());
            });
        }
    });

    rsx! {
        div { class: if is_present { "open" } else { "closing" } }
    }
}"#},
                div { class: "space-y-4",
                    div { class: "flex flex-wrap items-center gap-2",
                        button {
                            class: "px-4 py-2 rounded-lg bg-primary/20 text-primary transition-colors hover:bg-primary/30",
                            onclick: move |_| visible.toggle(),
                            if visible() { "Start manual exit" } else { "Show panel" }
                        }
                        span { class: "text-sm text-text-secondary", "Completed exits: {completed}" }
                    }
                    div { class: "min-h-24 rounded-lg bg-dark-200/30 p-4",
                        AnimatePresence {
                            on_exit_complete: move |_| completed += 1,
                            if visible() {
                                ManualPanel {
                                    key: "{manual_key}",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ManualPanel() -> Element {
    let presence = use_presence();
    let is_present = presence.is_present;

    use_effect(move || {
        if !is_present {
            let safe_to_remove = presence.safe_to_remove;
            spawn(async move {
                Time::delay(Duration::from_millis(500)).await;
                safe_to_remove.call(());
            });
        }
    });

    let state_class = if is_present {
        "translate-x-0 opacity-100"
    } else {
        "-translate-x-4 opacity-0"
    };

    rsx! {
        div {
            class: "rounded-lg border border-secondary/20 bg-secondary/10 p-4 text-text-primary transition-all duration-500 {state_class}",
            if is_present { "Present" } else { "Waiting for manual safe_to_remove" }
        }
    }
}

#[component]
fn ApiReference() -> Element {
    rsx! {
        section { class: "space-y-4",
            h3 { class: "text-xl font-semibold text-text-primary", "API Reference" }
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-3",
                ReferenceItem {
                    name: "AnimatePresence",
                    detail: "Wraps direct keyed children and retains removed children until exit completion.",
                }
                ReferenceItem {
                    name: "PresenceMode::Sync",
                    detail: "Default. Entering and exiting children render at the same time.",
                }
                ReferenceItem {
                    name: "PresenceMode::Wait",
                    detail: "Entering children wait until all exiting children finish.",
                }
                ReferenceItem {
                    name: "PresenceMode::PopLayout",
                    detail: "Exiting children are popped out of layout where platform APIs support measurement.",
                }
                ReferenceItem {
                    name: "layout: size",
                    detail: "Animates measured width and height from the presence style hook.",
                }
                ReferenceItem {
                    name: "initial: false",
                    detail: "Starts newly mounted children at their animate target instead of their initial target.",
                }
                ReferenceItem {
                    name: "custom / use_presence_data",
                    detail: "Passes cloneable data into present and exiting children with type-safe retrieval.",
                }
                ReferenceItem {
                    name: "on_exit_complete",
                    detail: "Runs after the boundary has no more exiting children.",
                }
            }
            div { class: "rounded-lg border border-primary/10 bg-primary/5 p-4",
                h4 { class: "font-medium text-text-primary", "Edge cases to remember" }
                ul { class: "mt-2 list-disc list-inside space-y-1 text-sm text-text-secondary",
                    li { "Every direct child must have a stable, unique key." }
                    li { "A child with no presence-aware hook or manual subscriber is removed immediately." }
                    li { "If a key is re-added while exiting, the current child is treated as present again." }
                    li { "Missing or duplicate keys are logged as AnimatePresence errors and the boundary renders empty for that invalid shape." }
                }
            }
        }
    }
}

#[component]
fn ReferenceItem(name: &'static str, detail: &'static str) -> Element {
    rsx! {
        div { class: "rounded-lg border border-primary/10 bg-dark-200/30 p-4",
            code { class: "text-sm text-primary", "{name}" }
            p { class: "mt-2 text-sm text-text-secondary", "{detail}" }
        }
    }
}

#[component]
fn ExampleGrid(code: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
            div { class: "rounded-lg bg-dark-200/50 p-3",
                CodeBlock {
                    language: "rust".to_string(),
                    code: code.to_string(),
                }
            }
            div { class: "min-w-0", {children} }
        }
    }
}
