use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn LoopDemo() -> Element {
    let mut position = use_motion(0.0_f32)?;
    let mut error = use_signal(|| None::<String>);
    rsx! {
        div { class: "lesson-controls",
            div { style: "height:24px;background:#242b30;border-radius:12px;overflow:hidden",
                div { style: "height:100%;background:#b9f078;transform-origin:left;transform:scaleX({position.get_value() / 100.0})" }
            }
            output { "Position {position.get_value():.1} / 100" }
            button { class: "button primary", onclick: move |_| {
                position.reset();
                let config = AnimationConfig::tween_ms(700)
                    .with_delay(Duration::from_millis(300))
                    .with_loop(LoopMode::AlternateTimes(2));
                error.set(position.animate_to(100.0, config).err().map(|e| e.to_string()));
            }, "Run two round trips" }
            button { class: "button secondary", onclick: move |_| position.stop(), "Stop" }
            if let Some(message) = error() { p { role: "alert", {message} } }
        }
    }
}
