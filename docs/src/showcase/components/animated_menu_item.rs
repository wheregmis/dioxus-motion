use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const ITEMS: [&str; 4] = ["Home", "Docs", "Examples", "Blog"];
const ITEM_W: f32 = 96.0;

fn pill_spring() -> AnimationConfig {
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 320.0,
        damping: 26.0,
        mass: 0.9,
    }))
}

/// A navbar whose pill indicator springs between items — the classic
/// "shared element" feel, done with two scalar springs.
#[component]
pub fn AnimatedMenuItem() -> Element {
    let mut pill_x = use_motion(0.0f32)?;
    let mut pill_w = use_motion(ITEM_W)?;
    let mut active = use_signal(|| 0usize);

    let mut move_to = move |index: usize| {
        let target_x = index as f32 * (ITEM_W + 8.0);
        pill_x
            .animate_to(target_x, pill_spring())
            .expect("valid animation configuration");
        pill_w
            .animate_to(ITEM_W, pill_spring())
            .expect("valid animation configuration");
        active.set(index);
    };

    let label_color = |index: usize| {
        if active() == index {
            "#e8ffb0"
        } else {
            "#848e9b"
        }
    };

    rsx! {
        div { class: "flex flex-col items-center gap-6",
            nav {
                class: "relative flex items-center gap-2 p-1.5 rounded-2xl",
                style: "background: #0d1216; border: 1px solid #ffffff10;",
                // Sliding pill
                div {
                    class: "absolute top-1.5 left-1.5 h-[calc(100%-12px)] rounded-xl pointer-events-none",
                    style: "width: {pill_w.get_value()}px; \
                           transform: translateX({pill_x.get_value()}px); \
                           background: linear-gradient(135deg, rgba(185,240,120,0.25), rgba(110,231,183,0.12)); \
                           border: 1px solid rgba(185,240,120,0.4); \
                           box-shadow: 0 0 24px -6px rgba(185,240,120,0.35);",
                }
                for (index, label) in ITEMS.iter().enumerate() {
                    button {
                        key: "{index}",
                        class: "relative z-10 h-9 text-sm font-medium transition-colors duration-200",
                        style: "width: {ITEM_W}px; color: {label_color(index)}",
                        onmouseenter: move |_| move_to(index),
                        onclick: move |_| move_to(index),
                        "{label}"
                    }
                }
            }
            p { class: "text-xs text-[#848e9b]", "Hover or click — the pill springs, never snaps." }
        }
    }
}
