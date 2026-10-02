use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn QuickStart() -> Element {
    let mut position = use_motion(0.0_f32)?;
    let mut forward = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    rsx! {
        div { class: "quick-start",
            button {
                class: "button primary",
                onclick: move |_| {
                    let target = if forward() { 0.0 } else { 160.0 };
                    match position.animate_to(target, AnimationConfig::spring(Spring::default())) {
                        Ok(()) => { forward.toggle(); error.set(None); }
                        Err(problem) => error.set(Some(problem.to_string())),
                    }
                },
                "Move the spring →"
            }
            div { class: "quick-track", style: "position: relative; width: 220px; height: 50px; background: #ffffff12; border-radius: 8px;",
                div { class: "quick-orb", style: "position: absolute; top: 5px; left: 10px; width: 40px; height: 40px; background: #b9f078; border-radius: 12px; transform: translateX({position.get_value()}px)", aria_hidden: "true" }
            }
            if let Some(problem) = error() { p { role: "alert", {problem} } }
        }
    }
}
