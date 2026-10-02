use dioxus::prelude::*;
use dioxus_motion::{KeyframeAnimation, keyframes::KeyframeError, prelude::*};

fn track() -> Result<KeyframeAnimation<f32>, KeyframeError> {
    KeyframeAnimation::new(Duration::from_millis(1600))
        .add_keyframe(0.0, 0.0, None)?
        .add_keyframe(100.0, 0.25, None)?
        .add_keyframe(100.0, 0.60, None)?
        .add_keyframe(0.0, 1.0, None)
}

#[component]
pub fn KeyframeDemo() -> Element {
    let mut position = use_motion(0.0_f32)?;
    let mut error = use_signal(|| None::<String>);
    rsx! {
        div { class: "lesson-controls",
            div { style: "height:24px;background:#242b30;border-radius:12px;overflow:hidden",
                div { style: "height:100%;background:#b9f078;transform-origin:left;transform:scaleX({position.get_value() / 100.0})" }
            }
            output { "Position {position.get_value():.1} / 100" }
            button { class: "button primary", onclick: move |_| {
                let result = track().map_err(|e| e.to_string()).and_then(|frames| {
                    position.animate_keyframes(frames).map_err(|e| e.to_string())
                });
                error.set(result.err());
            }, "Run keyframes" }
            button { class: "button secondary", onclick: move |_| position.stop(), "Stop" }
            if let Some(message) = error() { p { role: "alert", {message} } }
        }
    }
}
