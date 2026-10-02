use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

/// A terminal that types itself: a linear tween reveals characters and a
/// second tween blinks the block cursor between letters.
#[component]
pub fn TypewriterEffect(text: &'static str) -> Element {
    let mut char_count = use_motion(0.0f32)?;
    let mut cursor = use_motion(1.0f32)?;
    let text_len = text.chars().count() as f32;

    use_effect(move || {
        char_count
            .animate_to(
                text_len,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_secs_f32(text_len * 0.09),
                    easing: easer::functions::Linear::ease_in_out,
                }))
                .with_loop(LoopMode::Infinite),
            )
            .expect("valid animation configuration");
        cursor
            .animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_millis(600),
                    easing: easer::functions::Linear::ease_in_out,
                }))
                .with_loop(LoopMode::Alternate),
            )
            .expect("valid animation configuration");
    });

    let visible = text
        .chars()
        .take(char_count.get_value() as usize)
        .collect::<String>();

    rsx! {
        div {
            class: "w-96 rounded-xl overflow-hidden",
            style: "background: #0d1216; border: 1px solid #ffffff12; \
                   box-shadow: 0 24px 60px -20px rgba(0,0,0,0.8);",
            // Title bar
            div {
                class: "flex items-center gap-2 px-4 py-2.5",
                style: "background: #141a20; border-bottom: 1px solid #ffffff0c;",
                span { class: "w-2.5 h-2.5 rounded-full", style: "background: #ff5f57" }
                span { class: "w-2.5 h-2.5 rounded-full", style: "background: #febc2e" }
                span { class: "w-2.5 h-2.5 rounded-full", style: "background: #28c840" }
                span { class: "ml-3 text-[11px] text-[#59616d] font-mono", "motion — zsh" }
            }
            // Body
            div { class: "px-4 py-5 font-mono text-sm leading-7",
                div {
                    span { class: "text-[#6ee7b7] mr-2", "➜" }
                    span { class: "text-[#b9f078] mr-2", "motion" }
                    span { class: "text-[#a9b0bb]", "{visible}" }
                    span {
                        class: "inline-block w-2 h-4 align-middle ml-0.5",
                        style: "background: #b9f078; opacity: {cursor.get_value()};",
                    }
                }
            }
        }
    }
}
