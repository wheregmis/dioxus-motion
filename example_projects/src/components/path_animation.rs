use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing;

#[component]
pub fn PathAnimation(path: &'static str, duration: f32) -> Element {
    let mut dash_offset = use_animation(1000.0f32);
    let mut mounted = use_signal(|| true);

    use_effect(move || {
        dash_offset.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_secs_f32(duration),
                easing: easer::functions::Cubic::ease_in_out,
            }))
            .with_loop(LoopMode::Infinite),
        );
    });

    use_drop(move || {
        mounted.set(false);
        dash_offset.stop();
    });

    rsx! {
        div { class: "w-full h-48 flex items-center justify-center bg-white rounded-xl",
            svg { class: "w-full h-full", view_box: "0 0 200 200",
                path {
                    d: "{path}",
                    fill: "none",
                    stroke: "url(#gradient)",
                    stroke_width: "4",
                    stroke_dasharray: "1000",
                    style: "stroke-dashoffset: {dash_offset.get_value()};
                            transition: stroke-dashoffset 0.1s linear;",
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
