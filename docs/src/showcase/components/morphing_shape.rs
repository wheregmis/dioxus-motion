use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

struct Shape {
    name: &'static str,
    clip: &'static str,
    colors: (&'static str, &'static str),
}

// Every silhouette is a 12-vertex polygon so clip-path can interpolate
// between any pair — mismatched point counts snap instead of morphing.
const SHAPES: [Shape; 5] = [
    Shape {
        name: "orb",
        clip: "polygon(98% 50%, 91.6% 74%, 74% 91.6%, 50% 98%, 26% 91.6%, 8.4% 74%, 2% 50%, 8.4% 26%, 26% 8.4%, 50% 2%, 74% 8.4%, 91.6% 26%)",
        colors: ("#b9f078", "#6ee7b7"),
    },
    Shape {
        name: "hex",
        clip: "polygon(98% 50%, 86% 70.8%, 74% 91.6%, 50% 91.6%, 26% 91.6%, 14% 70.8%, 2% 50%, 14% 29.2%, 26% 8.4%, 50% 8.4%, 74% 8.4%, 86% 29.2%)",
        colors: ("#6ee7b7", "#a5f3fc"),
    },
    Shape {
        name: "star",
        clip: "polygon(98% 50%, 69.1% 61%, 74% 91.6%, 50% 72%, 26% 91.6%, 30.9% 61%, 2% 50%, 30.9% 39%, 26% 8.4%, 50% 28%, 74% 8.4%, 69.1% 39%)",
        colors: ("#a5f3fc", "#c4b5fd"),
    },
    Shape {
        name: "diamond",
        clip: "polygon(50% 0%, 65% 16.7%, 80% 33.3%, 95% 50%, 80% 66.7%, 65% 83.3%, 50% 100%, 35% 83.3%, 20% 66.7%, 5% 50%, 20% 33.3%, 35% 16.7%)",
        colors: ("#c4b5fd", "#f0abfc"),
    },
    Shape {
        name: "bolt",
        clip: "polygon(58% 0%, 35% 29%, 12% 58%, 27% 58%, 42% 58%, 40% 79%, 38% 100%, 63% 70%, 88% 40%, 72% 40%, 56% 40%, 57% 20%)",
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
