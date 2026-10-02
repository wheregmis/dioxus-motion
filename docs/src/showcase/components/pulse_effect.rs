use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

fn ring_config(delay: f32) -> AnimationConfig {
    AnimationConfig::new(AnimationMode::Tween(Tween {
        duration: Duration::from_millis(2600),
        easing: easer::functions::Sine::ease_in_out,
    }))
    .with_loop(LoopMode::Infinite)
    .with_delay(Duration::from_secs_f32(delay))
}

/// One expanding sonar ring: scales out while fading to nothing.
#[component]
fn SonarRing(delay: f32) -> Element {
    let mut scale = use_motion(0.25f32)?;
    let mut opacity = use_motion(0.85f32)?;

    use_effect(move || {
        scale
            .animate_to(1.0, ring_config(delay))
            .expect("valid animation configuration");
        opacity
            .animate_to(0.0, ring_config(delay))
            .expect("valid animation configuration");
    });

    rsx! {
        div {
            class: "absolute w-56 h-56 rounded-full pointer-events-none",
            style: "border: 1.5px solid rgba(185,240,120,0.6); \
                   transform: scale({scale.get_value()}); \
                   opacity: {opacity.get_value()};",
        }
    }
}

/// A sonar sweep: a rotating gradient wedge over staggered ping rings.
#[component]
pub fn PulseEffect() -> Element {
    let mut sweep = use_motion(0.0f32)?;
    let mut core = use_motion(1.0f32)?;

    use_effect(move || {
        sweep
            .animate_to(
                360.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(3400),
                    easing: easer::functions::Linear::ease_in_out,
                }))
                .with_loop(LoopMode::Infinite),
            )
            .expect("valid animation configuration");
        core.animate_to(
            1.35,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 60.0,
                damping: 8.0,
                mass: 0.8,
            }))
            .with_loop(LoopMode::Alternate),
        )
        .expect("valid animation configuration");
    });

    rsx! {
        div { class: "relative flex items-center justify-center w-64 h-64",
            // Radar dish
            div {
                class: "absolute inset-0 rounded-full overflow-hidden",
                style: "background: radial-gradient(circle, #141a20 30%, #0d1216 100%); \
                       border: 1px solid #ffffff12; \
                       box-shadow: inset 0 0 60px rgba(0,0,0,0.6);",
                // Grid lines
                div {
                    class: "absolute inset-0 rounded-full opacity-30",
                    style: "background: \
                           linear-gradient(#b9f07818 1px, transparent 1px) 0 0 / 100% 33%, \
                           linear-gradient(90deg, #b9f07818 1px, transparent 1px) 0 0 / 33% 100%;",
                }
                // Sweep wedge
                div {
                    class: "absolute inset-0 rounded-full",
                    style: "background: conic-gradient(from {sweep.get_value()}deg, \
                           rgba(185,240,120,0.35) 0deg, transparent 70deg, transparent 360deg);",
                }
            }
            SonarRing { delay: 0.0 }
            SonarRing { delay: 0.85 }
            SonarRing { delay: 1.7 }
            // Core orb
            div {
                class: "relative w-4 h-4 rounded-full",
                style: "background: #b9f078; \
                       transform: scale({core.get_value()}); \
                       box-shadow: 0 0 18px 4px rgba(185,240,120,0.5);",
            }
        }
    }
}
