use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const SIZE: f32 = 190.0;
const HALF: f32 = SIZE / 2.0;

/// Spring preset for cursor tilt — snappy but not jittery.
fn tilt() -> AnimationConfig {
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 170.0,
        damping: 16.0,
        mass: 0.8,
    }))
}

/// One glass face of the cube.
#[component]
fn CubeFace(face: &'static str, label: &'static str, accent: &'static str) -> Element {
    let transform = match face {
        "front" => format!("rotateY(0deg) translateZ({HALF}px)"),
        "back" => format!("rotateY(180deg) translateZ({HALF}px)"),
        "right" => format!("rotateY(90deg) translateZ({HALF}px)"),
        "left" => format!("rotateY(-90deg) translateZ({HALF}px)"),
        "top" => format!("rotateX(90deg) translateZ({HALF}px)"),
        _ => format!("rotateX(-90deg) translateZ({HALF}px)"),
    };
    rsx! {
        div {
            // pointer-events stay on the stage so element_coordinates is
            // always measured against the fixed 190px container
            class: "absolute inset-0 grid place-items-center rounded-xl pointer-events-none",
            style: "transform: {transform}; \
                   background: linear-gradient(150deg, {accent}26, #11161cd9 55%); \
                   border: 1px solid {accent}55; \
                   box-shadow: inset 0 0 30px {accent}22;",
            span {
                class: "text-[11px] font-mono tracking-[0.25em] uppercase",
                style: "color: {accent}; text-shadow: 0 0 14px {accent}aa;",
                "{label}"
            }
        }
    }
}

/// A glass 3D cube: hover to tilt it with your cursor, click to chain a
/// spin + squash + wobble through `AnimationSequence` steps.
#[component]
pub fn InteractiveCube() -> Element {
    // rot_y carries the resting pose plus click spins; tilt_x/tilt_y are
    // transient cursor deltas so mouse movement never interrupts a spin
    let mut rot_y = use_motion(24.0f32)?;
    let mut tilt_x = use_motion(0.0f32)?;
    let mut tilt_y = use_motion(0.0f32)?;
    let mut rot_z = use_motion(0.0f32)?;
    let mut scale = use_motion(1.0f32)?;
    let mut glow = use_motion(0.25f32)?;
    let mut lift = use_motion(0.0f32)?;

    let mut fire = move || {
        rot_y
            .animate_sequence(AnimationSequence::new().then(
                rot_y.get_value() + 360.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 160.0,
                    damping: 13.0,
                    mass: 1.0,
                })),
            ))
            .expect("valid animation configuration");
        scale
            .animate_sequence(
                AnimationSequence::new()
                    .then(
                        1.28,
                        AnimationConfig::new(AnimationMode::Spring(Spring {
                            stiffness: 420.0,
                            damping: 9.0,
                            mass: 1.0,
                        })),
                    )
                    .then(
                        1.0,
                        AnimationConfig::new(AnimationMode::Spring(Spring {
                            stiffness: 300.0,
                            damping: 16.0,
                            mass: 1.0,
                        })),
                    ),
            )
            .expect("valid animation configuration");
        rot_z
            .animate_sequence(
                AnimationSequence::new()
                    .then(
                        14.0,
                        AnimationConfig::new(AnimationMode::Spring(Spring {
                            stiffness: 220.0,
                            damping: 6.0,
                            mass: 0.6,
                        })),
                    )
                    .then(
                        0.0,
                        AnimationConfig::new(AnimationMode::Spring(Spring {
                            stiffness: 220.0,
                            damping: 11.0,
                            mass: 0.6,
                        })),
                    ),
            )
            .expect("valid animation configuration");
        glow.animate_sequence(
            AnimationSequence::new()
                .then(
                    1.0,
                    AnimationConfig::new(AnimationMode::Spring(Spring::default())),
                )
                .then(
                    0.25,
                    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
                        .with_delay(Duration::from_millis(450)),
                ),
        )
        .expect("valid animation configuration");
    };

    let onclick = move |_| fire();
    let onkeydown = move |e: Event<KeyboardData>| {
        if e.code() == Code::Enter || e.code() == Code::Space {
            fire();
        }
    };

    let onmousemove = move |e: Event<MouseData>| {
        let point = e.data().element_coordinates();
        let dx = (point.x as f32 - HALF) / HALF;
        let dy = (point.y as f32 - HALF) / HALF;
        tilt_x
            .animate_to(-dy * 18.0, tilt())
            .expect("valid animation configuration");
        tilt_y
            .animate_to(dx * 18.0, tilt())
            .expect("valid animation configuration");
    };

    let onmouseenter = move |_| {
        lift.animate_to(16.0, tilt())
            .expect("valid animation configuration");
        glow.animate_to(0.6, tilt())
            .expect("valid animation configuration");
    };

    let onmouseleave = move |_| {
        lift.animate_to(0.0, tilt()).expect("valid animation");
        glow.animate_to(0.25, tilt()).expect("valid animation");
        tilt_x.animate_to(0.0, tilt()).expect("valid animation");
        tilt_y.animate_to(0.0, tilt()).expect("valid animation");
    };

    rsx! {
        div { class: "flex flex-col items-center gap-8 cursor-pointer select-none",
            div {
                class: "relative",
                style: "width: {SIZE}px; height: {SIZE}px; perspective: 900px;",
                // Ambient glow
                div {
                    class: "absolute -inset-10 rounded-full blur-3xl pointer-events-none",
                    style: "background: radial-gradient(circle, rgba(185,240,120,0.25), rgba(110,231,183,0.12), transparent 70%); \
                           opacity: {glow.get_value()};",
                }
                // Floor shadow
                div {
                    class: "absolute -bottom-14 left-1/2 w-40 h-4 rounded-full bg-black/50 blur-lg",
                    style: "transform: translateX(-50%) scale({1.0 - lift.get_value() / 60.0}, 1.0);",
                }
                div {
                    onclick,
                    onkeydown,
                    onmousemove,
                    onmouseenter,
                    onmouseleave,
                    tabindex: "0",
                    role: "button",
                    aria_label: "Spin cube",
                    class: "absolute inset-0",
                    style: "transform-style: preserve-3d; \
                           transform: translateY(-{lift.get_value()}px) \
                           rotateX({-18.0 + tilt_x.get_value()}deg) rotateY({rot_y.get_value() + tilt_y.get_value()}deg) \
                           rotateZ({rot_z.get_value()}deg) scale({scale.get_value()});",
                    CubeFace { face: "front", label: "spring", accent: "#b9f078" }
                    CubeFace { face: "back", label: "tween", accent: "#a5f3fc" }
                    CubeFace { face: "right", label: "keyframe", accent: "#6ee7b7" }
                    CubeFace { face: "left", label: "sequence", accent: "#c4b5fd" }
                    CubeFace { face: "top", label: "loop", accent: "#f0abfc" }
                    CubeFace { face: "bottom", label: "ease", accent: "#eaffd0" }
                }
            }
            p { class: "text-xs text-[#848e9b]", "Hover to tilt · click or press Enter to chain a sequence" }
        }
    }
}
