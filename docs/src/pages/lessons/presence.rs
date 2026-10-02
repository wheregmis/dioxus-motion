use dioxus::prelude::*;
use dioxus_motion::{MotionTime, TimeProvider, prelude::*};

#[component]
pub fn PresenceDemo() -> Element {
    let mut visible = use_signal(|| false);
    let notice_key = "notice";
    rsx! {
        div { class: "lesson-controls",
            button { class: "button primary", onclick: move |_| visible.toggle(), if visible() { "Dismiss notice" } else { "Show notice" } }
            div { style: "min-height:90px",
                AnimatePresence {
                    if visible() { Notice { key: "{notice_key}", label: "Changes saved" } }
                }
            }
        }
    }
}

#[component]
fn Notice(label: String) -> Element {
    let style = use_presence_style(presence_style! {
        initial: { opacity: 0.0, y: 12.0, scale: 0.95 },
        animate: { opacity: 1.0, y: 0.0, scale: 1.0 },
        exit: { opacity: 0.0, y: -12.0, scale: 0.95 },
        transition: tween { duration: Duration::from_millis(350) },
    })?;
    rsx! { div { style: "padding:20px;border-radius:12px;background:#b9f078;color:#0b0d0f;{style.get_value()}", {label} } }
}

#[component]
pub fn PresenceModeDemo() -> Element {
    let mut second = use_signal(|| false);
    let mut wait = use_signal(|| true);
    let label = if second() { "Details" } else { "Overview" };
    rsx! {
        div { class: "lesson-controls",
            label { input { r#type: "checkbox", checked: wait(), onchange: move |event| wait.set(event.checked()) } " Wait for exit before entering" }
            button { class: "button primary", onclick: move |_| second.toggle(), "Switch panel" }
            div { style: "min-height:180px",
                AnimatePresence { mode: if wait() { PresenceMode::Wait } else { PresenceMode::Sync }, initial: false,
                    Notice { key: "{label}", label }
                }
            }
        }
    }
}

#[component]
pub fn ManualPresenceDemo() -> Element {
    let mut visible = use_signal(|| false);
    let panel_key = "manual-panel";
    rsx! {
        div { class: "lesson-controls",
            button { class: "button primary", onclick: move |_| visible.toggle(), if visible() { "Remove panel" } else { "Show panel" } }
            AnimatePresence { if visible() { ManualPanel { key: "{panel_key}" } } }
        }
    }
}

#[component]
fn ManualPanel() -> Element {
    let presence = use_presence();
    let present = presence.is_present;
    use_effect(use_reactive((&present,), move |(present,)| {
        if !present {
            let remove = presence.safe_to_remove;
            spawn(async move {
                if let Err(error) = MotionTime::delay(Duration::from_millis(500)).await {
                    dioxus::logger::tracing::warn!(%error, "removing without a timer");
                }
                remove.call(());
            });
        }
    }));
    rsx! { div { style: "padding:20px;border:1px solid #b9f078;border-radius:12px", if present { "Mounted" } else { "Finishing work before removal…" } } }
}
