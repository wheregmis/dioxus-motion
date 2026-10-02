use dioxus::prelude::*;
use dioxus_motion::{KeyframeAnimation, prelude::*};
use easer::functions::Easing;

#[allow(clippy::type_complexity)]
type Keyframes<T> = Vec<(T, f32, Option<fn(f32, f32, f32, f32) -> f32>)>;

fn keyframes<T: dioxus_motion::animations::core::Animatable>(
    duration: Duration,
    frames: Keyframes<T>,
) -> Result<KeyframeAnimation<T>, dioxus_motion::keyframes::KeyframeError> {
    let mut animation = KeyframeAnimation::new(duration);
    for (value, offset, easing) in frames {
        animation = animation.add_keyframe(value, offset, easing)?;
    }
    Ok(animation)
}

/// A launch button: three keyframe tracks play together on press —
/// squash-and-stretch scale, a full spin, and a rippling glow ring.
#[component]
pub fn RotatingButton() -> Element {
    let mut scale = use_motion(1.0f32)?;
    let mut rotation = use_motion(0.0f32)?;
    let mut ripple = use_motion(0.0f32)?;

    let onclick = move |_| {
        if let Ok(anim) = keyframes(
            Duration::from_millis(900),
            vec![
                (1.0f32, 0.0, Some(easer::functions::Expo::ease_out)),
                (0.82, 0.18, Some(easer::functions::Back::ease_out)),
                (1.12, 0.55, Some(easer::functions::Back::ease_out)),
                (1.0, 1.0, Some(easer::functions::Elastic::ease_out)),
            ],
        ) {
            scale.animate_keyframes(anim).expect("valid keyframe setup");
        }
        if let Ok(anim) = keyframes(
            Duration::from_millis(900),
            vec![
                (
                    rotation.get_value(),
                    0.0,
                    Some(easer::functions::Cubic::ease_in_out),
                ),
                (
                    rotation.get_value() + 360.0,
                    1.0,
                    Some(easer::functions::Back::ease_out),
                ),
            ],
        ) {
            rotation
                .animate_keyframes(anim)
                .expect("valid keyframe setup");
        }
        if let Ok(anim) = keyframes(
            Duration::from_millis(1100),
            vec![
                (0.0f32, 0.0, Some(easer::functions::Linear::ease_in_out)),
                (1.0, 1.0, Some(easer::functions::Cubic::ease_out)),
            ],
        ) {
            ripple
                .animate_keyframes(anim)
                .expect("valid keyframe setup");
        }
    };

    let ring_scale = 1.0 + ripple.get_value() * 2.4;
    let ring_opacity = (1.0 - ripple.get_value()).max(0.0);

    rsx! {
        div { class: "relative flex items-center justify-center",
            // Ripple ring
            div {
                class: "absolute w-40 h-14 rounded-full pointer-events-none",
                style: "border: 2px solid rgba(185,240,120,0.7); \
                       transform: scale({ring_scale}); opacity: {ring_opacity};",
            }
            button {
                class: "relative px-10 py-4 rounded-2xl font-bold text-base text-[#0b0d0f]",
                style: "transform: scale({scale.get_value()}) rotate({rotation.get_value()}deg); \
                       background: linear-gradient(135deg, #b9f078, #6ee7b7); \
                       box-shadow: 0 12px 40px -10px rgba(185,240,120,0.6), inset 0 1px 0 #ffffff60;",
                onclick,
                span { class: "relative z-10 tracking-wide", "Launch" }
            }
        }
    }
}
