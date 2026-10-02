use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn StyleDemo() -> Element {
    let mut expanded = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut style = use_motion(
        motion_style! { scale: 1.0, rotate: 0.0, border_radius: 12.0, background_color: "#b9f078" },
    )?;
    rsx! {
        div { class: "lesson-controls",
            div { style: "min-height:140px;display:grid;place-items:center;overflow:hidden",
                div { style: "width:160px;height:90px;display:grid;place-items:center;color:#0b0d0f;{style.get_value()}", "One visual state" }
            }
            button { class: "button primary", onclick: move |_| {
                let target = if expanded() {
                    motion_style! { scale: 1.0, rotate: 0.0, border_radius: 12.0, background_color: "#b9f078" }
                } else {
                    motion_style! { scale: 1.1, rotate: 8.0, border_radius: 28.0, background_color: "#78c7f0" }
                };
                match style.animate_to(target, AnimationConfig::spring(Spring::default())) {
                    Ok(()) => { expanded.toggle(); error.set(None); }
                    Err(e) => error.set(Some(e.to_string())),
                }
            }, "Change card state" }
            if let Some(message) = error() { p { role: "alert", {message} } }
        }
    }
}

#[component]
pub fn UnitDemo() -> Element {
    let mut style = use_motion(MotionStyle::default().property("width", CssValue::Px(120.0)))?;
    let mut message = use_signal(|| "Starts at 120px.".to_string());
    rsx! {
        div { class: "lesson-controls",
            div { style: "height:32px;background:#b9f078;border-radius:8px;{style.get_value()}" }
            button { class: "button secondary", onclick: move |_| {
                let target = MotionStyle::default().property("width", CssValue::Percent(80.0));
                message.set(match style.animate_to(target, AnimationConfig::spring(Spring::default())) {
                    Ok(()) => "Spring started".into(),
                    Err(e) => format!("{e}. The existing width is preserved."),
                });
            }, "Try a spring from px to %" }
            button { class: "button primary", onclick: move |_| {
                let target = MotionStyle::default().property("width", CssValue::Percent(80.0));
                message.set(match style.animate_to(target, AnimationConfig::tween_ms(600)) {
                    Ok(()) => "Tween accepted. Incompatible units switch discretely at the endpoint.".into(),
                    Err(e) => e.to_string(),
                });
            }, "Use a tween instead" }
            button { class: "button secondary", onclick: move |_| { let initial = MotionStyle::default().property("width", CssValue::Px(120.0));
                message.set(match style.animate_to(initial, AnimationConfig::tween_ms(0)) {
                    Ok(()) => "Starts at 120px.".into(),
                    Err(e) => e.to_string(),
                }); }, "Reset width" }
            p { role: "status", {message} }
        }
    }
}
