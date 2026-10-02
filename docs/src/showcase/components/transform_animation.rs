use dioxus::prelude::*;
use dioxus_motion::prelude::*;

/// A magnetic card: springs tilt it toward the cursor and it settles
/// back to rest on leave — interruptible at every frame.
#[component]
pub fn TransformAnimationShowcase() -> Element {
    let mut tilt_x = use_motion(0.0f32)?;
    let mut tilt_y = use_motion(0.0f32)?;
    let mut lift = use_motion(Transform::identity())?;
    let mut glare_x = use_signal(|| 50.0f32);
    let mut glare_y = use_signal(|| 50.0f32);

    let spring = || {
        AnimationConfig::new(AnimationMode::Spring(Spring {
            stiffness: 260.0,
            damping: 18.0,
            mass: 0.9,
        }))
    };

    let onmousemove = move |e: Event<MouseData>| {
        let point = e.data().element_coordinates();
        let dx = (point.x as f32 - 140.0) / 140.0;
        let dy = (point.y as f32 - 170.0) / 170.0;
        glare_x.set(((dx + 1.0) * 50.0).clamp(0.0, 100.0));
        glare_y.set(((dy + 1.0) * 50.0).clamp(0.0, 100.0));
        tilt_y
            .animate_to(dx * 14.0, spring())
            .expect("valid animation configuration");
        tilt_x
            .animate_to(-dy * 14.0, spring())
            .expect("valid animation configuration");
        lift.animate_to(
            Transform {
                x: dx * 6.0,
                y: -10.0 + dy * 4.0,
                scale: 1.04,
                rotation: dx * 0.02,
            },
            spring(),
        )
        .expect("valid animation configuration");
    };

    let onmouseleave = move |_| {
        tilt_x.animate_to(0.0, spring()).expect("valid animation");
        tilt_y.animate_to(0.0, spring()).expect("valid animation");
        lift.animate_to(Transform::identity(), spring())
            .expect("valid animation configuration");
    };

    let t = lift.get_value();

    rsx! {
        div {
            class: "flex items-center justify-center cursor-pointer select-none",
            style: "perspective: 900px",
            onmousemove,
            onmouseleave,
            div {
                class: "relative w-72 h-80 rounded-3xl overflow-hidden",
                style: "transform: translate({t.x}px, {t.y}px) rotateX({tilt_x.get_value()}deg) \
                       rotateY({tilt_y.get_value()}deg) scale({t.scale}) rotate({t.rotation}rad); \
                       background: linear-gradient(155deg, #1a2129 0%, #11161c 55%, #0d1216 100%); \
                       border: 1px solid rgba(185,240,120,0.25); \
                       box-shadow: 0 30px 80px -20px rgba(0,0,0,0.8), 0 0 40px -10px rgba(185,240,120,0.15);",
                // Glare that tracks the cursor
                div {
                    class: "absolute inset-0 pointer-events-none",
                    style: "background: radial-gradient(280px circle at {glare_x()}% {glare_y()}%, \
                           rgba(185,240,120,0.18), transparent 60%);",
                }
                // Card content
                div { class: "relative h-full flex flex-col justify-between p-7",
                    div { class: "flex items-start justify-between",
                        div { class: "w-11 h-11 rounded-xl grid place-items-center text-xl font-black text-[#0b0d0f]",
                            style: "background: linear-gradient(135deg, #b9f078, #6ee7b7)",
                            "m"
                        }
                        span { class: "text-[10px] tracking-[0.3em] text-[#848e9b] uppercase", "Spring" }
                    }
                    div {
                        p { class: "text-[10px] tracking-[0.3em] text-[#b9f078] uppercase mb-2", "Dioxus Motion" }
                        h3 { class: "text-2xl font-bold text-white leading-tight", "Magnetic tilt card" }
                        p { class: "text-sm text-[#a9b0bb] mt-2 leading-relaxed",
                            "Three springs retarget every frame — the card chases your cursor and never snaps."
                        }
                    }
                    div { class: "flex items-center justify-between text-[#848e9b]",
                        span { class: "text-[10px] tracking-[0.2em] uppercase", "stiffness 260" }
                        span { class: "text-[10px] tracking-[0.2em] uppercase", "damping 18" }
                    }
                }
            }
        }
    }
}
