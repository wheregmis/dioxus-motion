use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn LayoutDemo() -> Element {
    let mut items = use_signal(|| vec![1_usize, 2, 3]);
    rsx! {
        div { class: "lesson-controls",
            div { class: "lesson-actions",
                button { class: "button primary", disabled: items().is_empty(), onclick: move |_| {
                    if !items.peek().is_empty() { items.write().remove(0); }
                }, "Remove first item" }
                button { class: "button secondary", onclick: move |_| items.set(vec![1, 2, 3]), "Restore items" }
            }
            div { style: "display:flex;flex-wrap:wrap;gap:12px;min-height:100px;position:relative",
                AnimatePresence { initial: false, mode: PresenceMode::PopLayout,
                    for id in items() { LayoutItem { key: "{id}", id } }
                }
            }
        }
    }
}

#[component]
fn LayoutItem(id: usize) -> Element {
    let style = use_presence_style(presence_style! {
        initial: { opacity: 0.0, scale: 0.9 },
        animate: { opacity: 1.0, scale: 1.0 },
        exit: { opacity: 0.0, scale: 0.9 },
        layout: size,
        transition: tween { duration: 350.0 },
        layout_transition: tween { duration: 250.0 },
    })?;
    rsx! {
        div { style: "width:120px;padding:20px;border-radius:12px;background:#b9f078;color:#0b0d0f;{style.get_value()}", "Item {id}" }
    }
}
