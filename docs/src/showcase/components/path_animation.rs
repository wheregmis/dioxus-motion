use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

/// A comet: one tween draws the trail while a glowing head rides the
/// same path on `offset-path`, leaving a fading wake behind it.
#[component]
pub fn PathAnimation(path: &'static str, duration: f32) -> Element {
    let mut progress = use_motion(0.0f32)?;
    let mut head = use_motion(0.0f32)?;

    use_effect(move || {
        progress
            .animate_to(
                1.0,
                AnimationConfig::new(AnimationMode::Tween(Tween {
                    duration: Duration::from_secs_f32(duration),
                    easing: easer::functions::Cubic::ease_in_out,
                }))
                .with_loop(LoopMode::Infinite),
            )
            .expect("valid animation configuration");
        head.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs_f32(duration),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite),
        )
        .expect("valid animation configuration");
    });

    rsx! {
        div { class: "relative", style: "width: 320px; height: 200px;",
            svg {
                class: "absolute inset-0 w-full h-full",
                view_box: "0 0 320 200",
                defs {
                    linearGradient {
                        id: "comet-trail",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "0%",
                        stop { offset: "0%", style: "stop-color: #b9f078; stop-opacity: 0.05" }
                        stop { offset: "70%", style: "stop-color: #b9f078; stop-opacity: 0.9" }
                        stop { offset: "100%", style: "stop-color: #6ee7b7" }
                    }
                    filter { id: "comet-glow",
                        feGaussianBlur { std_deviation: "3.5", result: "blur" }
                        feMerge {
                            feMergeNode { "in": "blur" }
                            feMergeNode { "in": "SourceGraphic" }
                        }
                    }
                }
                // Faint full path
                path {
                    d: path,
                    fill: "none",
                    stroke: "#ffffff10",
                    stroke_width: "1.5",
                }
                // Lit trail behind the head
                path {
                    d: path,
                    fill: "none",
                    stroke: "url(#comet-trail)",
                    stroke_width: "3",
                    stroke_linecap: "round",
                    path_length: "1000",
                    stroke_dasharray: "1000",
                    stroke_dashoffset: "{1000.0 - progress.get_value() * 1000.0}",
                    filter: "url(#comet-glow)",
                }
            }
            // Comet head rides the path with the same progress
            div {
                class: "absolute w-3 h-3 rounded-full pointer-events-none",
                style: "offset-path: path(\"{path}\"); \
                       offset-distance: {head.get_value()}%; \
                       background: radial-gradient(circle, #eaffd0, #b9f078 60%, transparent); \
                       box-shadow: 0 0 16px 4px rgba(185,240,120,0.8);",
            }
        }
    }
}
