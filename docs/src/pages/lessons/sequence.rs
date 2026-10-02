use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn SequenceDemo() -> Element {
    let mut position = use_motion(0.0_f32)?;
    let mut error = use_signal(|| None::<String>);
    rsx! {
        div { class: "lesson-controls",
            div { style: "height:24px;background:#242b30;border-radius:12px;overflow:hidden",
                div { style: "height:100%;background:#b9f078;transform-origin:left;transform:scaleX({position.get_value() / 100.0})" }
            }
            output { "Position {position.get_value():.1} / 100" }
            button { class: "button primary", onclick: move |_| {
                let steps = AnimationSequence::new()
                    .then(100.0, AnimationConfig::tween_ms(600))
                    .then(40.0, AnimationConfig::spring(Spring::default()))
                    .then(0.0, AnimationConfig::tween_ms(600));
                error.set(position.animate_sequence(steps).err().map(|e| e.to_string()));
            }, "Run sequence" }
            button { class: "button secondary", onclick: move |_| position.stop(), "Stop" }
            if let Some(message) = error() { p { role: "alert", {message} } }
        }
    }
}
