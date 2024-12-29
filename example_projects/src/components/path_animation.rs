use dioxus::prelude::*;
use dioxus_motion::{
    prelude::*,
    use_transform_motion::{use_transform_animation, Transform},
};
#[component]
pub fn PathAnimation(path: &'static str, duration: f32) -> Element {
    let mut offset = use_value_animation(
        Motion::new(1000.0)
            .to(0.0)
            .duration(Duration::from_secs_f32(duration)),
    );

    use_effect(move || {
        offset.loop_animation();
    });

    use_drop(move || {
        offset.stop_loop();
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
                    style: "stroke-dashoffset: {offset.value()}",
                }
                defs {
                    linearGradient {
                        id: "gradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "0%",
                        stop { offset: "0%", style: "stop-color: #3B82F6" }
                        stop { offset: "100%", style: "stop-color: #8B5CF6" }
                    }
                }
            }
        }
    }
}
