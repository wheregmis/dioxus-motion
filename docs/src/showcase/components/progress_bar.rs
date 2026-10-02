use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

/// A tick turns lime once the fill has passed it.
fn tick_color(i: usize, p: f32) -> &'static str {
    if (i * 10) as f32 <= p {
        "#b9f078"
    } else {
        "#ffffff20"
    }
}

/// A charging bar: the fill tweens on a loop while a glowing tip and a
/// shimmer sweep ride along on the same value.
#[component]
pub fn ProgressBar() -> Element {
    let mut progress = use_motion(0.0f32)?;

    use_effect(move || {
        progress
            .animate_to(
                100.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(4200),
                    easing: easer::functions::Cubic::ease_in_out,
                }))
                .with_loop(LoopMode::Infinite),
            )
            .expect("valid animation configuration");
    });

    let p = progress.get_value();
    let tip_opacity = if p > 2.0 && p < 99.0 { 1.0 } else { 0.0 };

    rsx! {
        div { class: "w-80 flex flex-col gap-3",
            div { class: "flex items-end justify-between",
                div {
                    span { class: "text-[10px] tracking-[0.3em] text-[#848e9b] uppercase", "Charging" }
                }
                span { class: "font-mono text-sm text-[#b9f078] tabular-nums", "{p as i32}%" }
            }
            div {
                class: "relative h-3 rounded-full overflow-visible",
                style: "background: #0d1216; border: 1px solid #ffffff10;",
                // Fill
                div {
                    class: "h-full rounded-full",
                    style: "width: {p.max(1.5)}%; \
                           background: linear-gradient(90deg, #6ee7b7, #b9f078); \
                           box-shadow: 0 0 16px rgba(185,240,120,0.45);",
                }
                // Glowing tip
                div {
                    class: "absolute top-1/2 w-4 h-4 rounded-full pointer-events-none",
                    style: "left: calc({p}% - 8px); transform: translateY(-50%); \
                           background: #eaffd0; \
                           box-shadow: 0 0 14px 3px rgba(185,240,120,0.9); \
                           opacity: {tip_opacity}; \
                           transition: opacity 0.2s;",
                }
            }
            // Segment ticks
            div { class: "flex justify-between px-0.5",
                for i in 1..10usize {
                    span {
                        key: "{i}",
                        class: "w-px h-1.5",
                        style: "background: {tick_color(i, p)}; \
                               transition: background 0.3s;",
                    }
                }
            }
            div { class: "flex justify-between text-[10px] tracking-[0.2em] text-[#59616d] uppercase",
                span { "0" }
                span { "battery.motion" }
                span { "100" }
            }
        }
    }
}
