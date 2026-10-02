use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const FLIP: Spring = Spring {
    stiffness: 220.0,
    damping: 22.0,
    mass: 0.9,
};

/// A holo card: click springs the flip, and the shine sweeps across at
/// the same angle so it reads like a foil.
#[component]
pub fn Card3DFlip() -> Element {
    let mut rotation = use_motion(0.0f32)?;
    let mut flipped = use_signal(|| false);

    let mut flip = move || {
        let target = if *flipped.read() { 0.0 } else { 180.0 };
        rotation
            .animate_to(target, AnimationConfig::new(AnimationMode::Spring(FLIP)))
            .expect("valid animation configuration");
        flipped.toggle();
    };

    let onclick = move |_| flip();
    let onkeydown = move |e: Event<KeyboardData>| {
        if !e.is_auto_repeating() && (e.code() == Code::Enter || e.code() == Code::Space) {
            flip();
        }
    };

    let angle = rotation.get_value();
    let shine = ((angle / 180.0) * std::f32::consts::PI).sin().abs();

    rsx! {
        div { class: "cursor-pointer select-none", style: "perspective: 1200px",
            tabindex: "0",
            role: "button",
            aria_label: "Flip card",
            onclick,
            onkeydown,
            div {
                class: "relative w-72 h-44",
                style: "transform-style: preserve-3d; transform: rotateY({angle}deg);",
                // Front
                div {
                    class: "absolute inset-0 rounded-2xl p-6 flex flex-col justify-between overflow-hidden",
                    style: "backface-visibility: hidden; \
                           background: linear-gradient(135deg, #1c2620 0%, #11161c 60%, #0d1216 100%); \
                           border: 1px solid rgba(185,240,120,0.35); \
                           box-shadow: 0 24px 60px -18px rgba(0,0,0,0.8);",
                    div {
                        class: "absolute inset-0 pointer-events-none",
                        style: "background: radial-gradient(320px circle at 30% 20%, \
                               rgba(185,240,120,{0.10 + shine * 0.25}), transparent 60%);",
                    }
                    div { class: "flex justify-between items-start",
                        div {
                            class: "w-10 h-10 rounded-lg grid place-items-center text-lg font-black text-[#0b0d0f]",
                            style: "background: linear-gradient(135deg, #b9f078, #6ee7b7)",
                            "m"
                        }
                        span { class: "text-[9px] tracking-[0.3em] text-[#848e9b]", "HOLO·01" }
                    }
                    div {
                        p { class: "font-mono text-[13px] tracking-[0.35em] text-[#a9b0bb]",
                            "DIOXUS · MOTION"
                        }
                        p { class: "text-lg font-bold text-white mt-1", "Spring Card" }
                    }
                }
                // Back
                div {
                    class: "absolute inset-0 rounded-2xl p-6 flex flex-col justify-between overflow-hidden",
                    style: "backface-visibility: hidden; transform: rotateY(180deg); \
                           background: linear-gradient(225deg, #20262f 0%, #11161c 60%, #0d1216 100%); \
                           border: 1px solid rgba(110,231,183,0.3); \
                           box-shadow: 0 24px 60px -18px rgba(0,0,0,0.8);",
                    div { class: "h-7 -mx-6 mt-1", style: "background: linear-gradient(90deg, #0b0d0f, #b9f07833, #0b0d0f)" }
                    div { class: "flex items-end justify-between",
                        div {
                            p { class: "text-[9px] tracking-[0.3em] text-[#848e9b] uppercase", "Serial" }
                            p { class: "font-mono text-sm text-[#b9f078]", "000·SPRN·180" }
                        }
                        span { class: "text-[10px] text-[#848e9b]", "click to flip back" }
                    }
                }
            }
        }
    }
}
