use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const POP: Spring = Spring {
    stiffness: 380.0,
    damping: 16.0,
    mass: 0.7,
};
const COUNT: Spring = Spring {
    stiffness: 90.0,
    damping: 18.0,
    mass: 1.1,
};

/// A stat counter: the number springs to each new total, pops on impact,
/// and the ring beneath fills toward the next milestone.
#[component]
pub fn AnimatedCounter() -> Element {
    let mut value = use_motion(0.0f32)?;
    let mut pop = use_motion(1.0f32)?;
    let mut count = use_signal(|| 0u32);

    let increment = move |_| {
        let next = (*count)() + 1;
        count.set(next);
        value
            .animate_to(
                next as f32 * 100.0,
                AnimationConfig::new(AnimationMode::Spring(COUNT)),
            )
            .expect("valid animation configuration");
        pop.animate_sequence(
            AnimationSequence::new()
                .then(1.2, AnimationConfig::new(AnimationMode::Spring(POP)))
                .then(
                    1.0,
                    AnimationConfig::new(AnimationMode::Spring(POP))
                        .with_delay(Duration::from_millis(200)),
                ),
        )
        .expect("valid animation configuration");
    };

    let shown = value.get_value();
    let frac = ((shown % 1000.0) + 1000.0) % 1000.0 / 1000.0;

    rsx! {
        div { class: "flex flex-col items-center gap-8",
            div { class: "relative flex items-center justify-center",
                // Glow behind the number
                div {
                    class: "absolute w-64 h-64 rounded-full pointer-events-none",
                    style: "background: radial-gradient(circle, rgba(185,240,120,0.18), transparent 65%); \
                           transform: scale({pop.get_value()});",
                }
                div { class: "relative text-center",
                    p { class: "text-[10px] tracking-[0.35em] text-[#848e9b] uppercase mb-1", "Momentum" }
                    p {
                        class: "font-mono text-8xl font-black tabular-nums leading-none",
                        style: "color: #eaffd0; transform: scale({pop.get_value()}); \
                               text-shadow: 0 0 40px rgba(185,240,120,0.35);",
                        "{shown.round() as i32}"
                    }
                }
            }
            // Milestone meter
            div { class: "w-56",
                div { class: "flex justify-between text-[10px] tracking-[0.25em] text-[#848e9b] uppercase mb-2",
                    span { "Next level" }
                    span { "{(frac * 10.0).floor() as i32}/10" }
                }
                div { class: "h-1.5 rounded-full overflow-hidden", style: "background: #ffffff0e",
                    div {
                        class: "h-full rounded-full",
                        style: "width: {frac * 100.0}%; background: linear-gradient(90deg, #6ee7b7, #b9f078);",
                    }
                }
            }
            button {
                class: "px-7 py-3 rounded-xl font-semibold text-sm text-[#0b0d0f] transition-transform hover:scale-105 active:scale-95",
                style: "background: linear-gradient(135deg, #b9f078, #6ee7b7); \
                       box-shadow: 0 8px 30px -8px rgba(185,240,120,0.5);",
                onclick: increment,
                "+100"
            }
        }
    }
}
