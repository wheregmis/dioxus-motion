use dioxus::prelude::*;
use dioxus_motion::prelude::tween::Tween;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

#[component]
pub fn TypewriterEffect(text: &'static str) -> Element {
    let mut char_count = use_motion(0.0f32);
    let mut cursor_opacity = use_motion(1.0f32);
    let text_len = text.len() as f32;

    use_effect(move || {
        // Start typing animation
        char_count.animate_to(
            text_len,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs_f32(text_len * 0.1), // 0.1s per character
                easing: easer::functions::Linear::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite),
        );

        // Start cursor blink
        cursor_opacity.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs(1),
                easing: easer::functions::Linear::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite),
        );
    });

    let visible_text = text
        .chars()
        .take(char_count.get_value() as usize)
        .collect::<String>();

    rsx! {
        div { class: "relative font-mono text-2xl text-blue-500",
            // Text container
            span { "{visible_text}" }
            // Cursor
            span {
                class: "absolute right-0 top-0",
                style: "opacity: {cursor_opacity.get_value()};",
                "|"
            }
        }
    }
}
