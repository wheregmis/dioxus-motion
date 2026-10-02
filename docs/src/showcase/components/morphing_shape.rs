use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

struct Shape {
    name: &'static str,
    clip: &'static str,
    colors: (&'static str, &'static str),
}

const SHAPES: [Shape; 5] = [
    Shape {
        name: "orb",
        clip: "circle(48% at 50% 50%)",
        colors: ("#b9f078", "#6ee7b7"),
    },
    Shape {
        name: "hex",
        clip: "polygon(25% 5%, 75% 5%, 98% 50%, 75% 95%, 25% 95%, 2% 50%)",
        colors: ("#6ee7b7", "#a5f3fc"),
    },
    Shape {
        name: "star",
        clip: "polygon(50% 2%, 61% 36%, 98% 36%, 68% 58%, 79% 94%, 50% 72%, 21% 94%, 32% 58%, 2% 36%, 39% 36%)",
        colors: ("#a5f3fc", "#c4b5fd"),
    },
    Shape {
        name: "diamond",
        clip: "polygon(50% 0%, 95% 50%, 50% 100%, 5% 50%)",
        colors: ("#c4b5fd", "#f0abfc"),
    },
    Shape {
        name: "bolt",
        clip: "polygon(58% 0%, 12% 58%, 42% 58%, 38% 100%, 88% 40%, 56% 40%)",
        colors: ("#f0abfc", "#b9f078"),
    },
];

/// A liquid morph: clip-path crossfades between silhouettes while a
/// gentle spring rotation keeps it drifting — the label tracks along.
#[component]
pub fn MorphingShape() -> Element {
    let mut rotation = use_motion(0.0f32)?;
    let mut breathe = use_motion(1.0f32)?;
    let mut index = use_signal(|| 0usize);

    use_effect(move || {
        rotation
            .animate_to(
                360.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_secs(24),
                    easing: easer::functions::Linear::ease_in_out,
                }))
                .with_loop(LoopMode::Infinite),
            )
            .expect("valid animation configuration");
        breathe
            .animate_to(
                1.08,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 30.0,
                    damping: 6.0,
                    mass: 0.6,
                }))
                .with_loop(LoopMode::Alternate),
            )
            .expect("valid animation configuration");
        spawn(async move {
            loop {
                if Time::delay(Duration::from_millis(2400)).await.is_err() {
                    break;
                }
                index.set((*index)() + 1);
            }
        });
    });

    let shape = &SHAPES[(*index)() % SHAPES.len()];

    rsx! {
        div { class: "flex flex-col items-center gap-7",
            div { class: "relative w-48 h-48 flex items-center justify-center",
                // Halo behind the morph
                div {
                    class: "absolute inset-0 rounded-full blur-2xl",
                    style: "background: radial-gradient(circle, {shape.colors.0}33, transparent 70%); \
                           transition: background 0.8s;",
                }
                div {
                    class: "absolute inset-2",
                    style: "clip-path: {shape.clip}; \
                           background: linear-gradient(140deg, {shape.colors.0}, {shape.colors.1}); \
                           transform: rotate({rotation.get_value()}deg) scale({breathe.get_value()}); \
                           transition: clip-path 0.9s cubic-bezier(0.6, 0, 0.2, 1), background 0.9s; \
                           filter: drop-shadow(0 8px 30px {shape.colors.0}55);",
                }
            }
            div { class: "text-center",
                p { class: "font-mono text-sm text-[#b9f078] tracking-[0.3em] uppercase",
                    "{shape.name}"
                }
                p { class: "text-[10px] tracking-[0.25em] text-[#59616d] uppercase mt-1",
                    "clip-path · spring"
                }
            }
        }
    }
}
