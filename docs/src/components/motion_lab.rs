use super::code_block::CodeBlock;
use dioxus::prelude::*;
use dioxus_code::{CodeOptions, Language, code_str};
use dioxus_motion::prelude::*;
use dioxus_primitives::tabs::{TabContent, TabList, TabTrigger, Tabs};

#[component]
pub fn MotionLab() -> Element {
    let mut position = use_motion(0.0_f32)?;
    let mut stiffness = use_signal(|| 180.0_f32);
    let mut damping = use_signal(|| 14.0_f32);
    let mut forward = use_signal(|| false);
    let mut instant = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut tab = use_signal(|| Some("preview".to_string()));
    use_future(move || async move {
        let mut preference = document::eval(
            "dioxus.send(window.matchMedia('(prefers-reduced-motion: reduce)').matches)",
        );
        if let Ok(reduced) = preference.recv::<bool>().await {
            instant.set(reduced);
        }
        // Play once on load so the playground demonstrates itself; skipped when
        // the visitor prefers reduced motion
        if !instant()
            && position
                .animate_to(
                    100.0,
                    AnimationConfig::spring(Spring {
                        stiffness: stiffness(),
                        damping: damping(),
                        mass: 1.0,
                    }),
                )
                .is_ok()
        {
            forward.set(true);
        }
    });
    let value = position.get_value();

    rsx! {
        div { class: "motion-lab",
            div { class: "lab-heading",
                span { class: "eyebrow", "MOTION PLAYGROUND" }
                span { class: "lab-status", if position.is_running() { "MOVING" } else { "AT REST" } }
            }
            Tabs { value: tab, on_value_change: move |value| tab.set(Some(value)), horizontal: true,
                TabList { class: "preview-tabs", aria_label: "Playground view",
                    TabTrigger { value: "preview", index: 0usize, id: None, class: None, "Preview" }
                    TabTrigger { value: "code", index: 1usize, id: None, class: None, "Code" }
                }
                TabContent { value: "preview", index: 0usize, id: None, class: Some("lab-panel".to_string()),
                    div { class: "lab-stage",
                        div { class: "lab-rail", aria_hidden: "true",
                            span { class: "rail-end start" }
                            span { class: "rail-end finish" }
                            div { class: "lab-orb", style: "transform: translateX(calc({value * 0.65}cqw - 50%)) rotate({value * 1.8}deg)", "m" }
                        }
                        div { class: "lab-readout", span { "POSITION" } output { "{value:.2}" } span { "/ 100" } }
                    }
                    div { class: "lab-controls",
                        label { class: "slider-label",
                            span { "Stiffness" } output { "{stiffness():.0}" }
                            input { r#type: "range", aria_label: "Stiffness", min: "40", max: "500", step: "10", value: "{stiffness()}",
                                oninput: move |event| { if let Ok(value) = event.value().parse::<f32>() { stiffness.set(value); } }
                            }
                        }
                        label { class: "slider-label",
                            span { "Damping" } output { "{damping():.0}" }
                            input { r#type: "range", aria_label: "Damping", min: "4", max: "50", step: "1", value: "{damping()}",
                                oninput: move |event| { if let Ok(value) = event.value().parse::<f32>() { damping.set(value); } }
                            }
                        }
                        div { class: "lab-actions",
                            button { class: "button primary", onclick: move |_| {
                                let target = if forward() { 0.0 } else { 100.0 };
                                let config = if instant() { AnimationConfig::tween(Duration::ZERO) }
                                    else { AnimationConfig::spring(Spring { stiffness: stiffness(), damping: damping(), mass: 1.0 }) };
                                match position.animate_to(target, config) {
                                    Ok(()) => { forward.toggle(); error.set(None); }
                                    Err(problem) => error.set(Some(problem.to_string())),
                                }
                            }, "Move target ↔" }
                            label { class: "motion-toggle", input { r#type: "checkbox", checked: instant(), onchange: move |event| instant.set(event.checked()) } "Instant motion" }
                        }
                        if let Some(problem) = error() { p { role: "alert", {problem} } }
                    }
                }
                TabContent { value: "code", index: 1usize, id: None, class: Some("lab-code".to_string()),
                    CodeBlock { language: "Rust".to_string(), code: code_str!(r#"let mut position = use_motion(0.0_f32)?;

// Call from an event handler. Retarget at any point.
position.animate_to(
    100.0,
    AnimationConfig::spring(Spring {
        stiffness: 180.0,
        damping: 14.0,
        mass: 1.0,
    }),
)?;

// Read in RSX to subscribe to value changes.
let x = position.get_value();"#, CodeOptions::builder().with_language(Language::Rust)) }
                }
            }
            p { class: "lab-footnote", "Real Rust springs. Retargetable — move the target again mid-flight." }
        }
    }
}
