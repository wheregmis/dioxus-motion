use dioxus::prelude::*;
use dioxus_motion::prelude::tween::Tween;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

#[component]
pub fn PathAnimation(path: &'static str, duration: f32) -> Element {
    // Use motion hook for the dash offset animation
    let mut dash_offset = use_motion(1000.0f32);

    // Start the animation on mount
    use_effect(move || {
        // Animate the dash offset from 1000 to 0
        dash_offset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs_f32(duration),
                easing: easer::functions::Linear::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite), // Make it loop infinitely
        );
    });

    // Get the current dash offset value
    let current_offset = use_memo(move || dash_offset.get_value());

    rsx! {
        div { class: "w-full h-48 flex items-center justify-center rounded-xl",
            svg { class: "w-full h-full", view_box: "0 0 200 200",
                path {
                    d: "{path}",
                    fill: "none",
                    stroke: "url(#gradient)",
                    stroke_width: "4",
                    stroke_dasharray: "1000",
                    style: "stroke-dashoffset: {current_offset}",
                }
                defs {
                    linearGradient {
                        id: "gradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "0%",
                        stop { offset: "0%", style: "stop-color: #3B82F6;" }
                        stop { offset: "100%", style: "stop-color: #8B5CF6;" }
                    }
                }
            }
        }
    }
}
