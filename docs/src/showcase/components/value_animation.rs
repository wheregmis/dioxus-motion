use dioxus::prelude::*;
use dioxus_motion::prelude::*;

const SPRING: Spring = Spring {
    stiffness: 140.0,
    damping: 16.0,
    mass: 1.0,
};

/// A speedometer dial: presets spring the needle — hit a button mid-swing
/// and it retargets without snapping.
#[component]
pub fn ValueAnimationShowcase() -> Element {
    let mut value = use_motion(0.0f32)?;

    let mut target = move |v: f32| {
        value
            .animate_to(v, AnimationConfig::new(AnimationMode::Spring(SPRING)))
            .expect("valid animation configuration");
    };

    // Needle sweeps left (0) up over the dome to right (100), matching the arc
    let tip_angle = 180.0 - value.get_value() * 1.8;

    rsx! {
        div { class: "flex flex-col items-center gap-6",
            div { class: "relative",
                svg { width: "280", height: "150", view_box: "0 0 280 150",
                    defs {
                        linearGradient {
                            id: "gauge-fill",
                            x1: "0%",
                            y1: "0%",
                            x2: "100%",
                            y2: "0%",
                            stop { offset: "0%", style: "stop-color: #6ee7b7" }
                            stop { offset: "100%", style: "stop-color: #b9f078" }
                        }
                    }
                    // Track arc
                    path {
                        d: "M 30 140 A 110 110 0 0 1 250 140",
                        fill: "none",
                        stroke: "#ffffff12",
                        stroke_width: "12",
                        stroke_linecap: "round",
                    }
                    // Value arc — pathLength=100 maps the value to dash length
                    path {
                        d: "M 30 140 A 110 110 0 0 1 250 140",
                        fill: "none",
                        stroke: "url(#gauge-fill)",
                        stroke_width: "12",
                        stroke_linecap: "round",
                        path_length: "100",
                        stroke_dasharray: "{value.get_value().max(0.5)} 100",
                        style: "filter: drop-shadow(0 0 8px rgba(185,240,120,0.5))",
                    }
                    // Tick marks
                    for i in 0..=10usize {
                        line {
                            key: "{i}",
                            x1: "{140.0 + 128.0 * f32::cos((-90.0 + i as f32 * 18.0) * std::f32::consts::PI / 180.0)}",
                            y1: "{140.0 + 128.0 * f32::sin((-90.0 + i as f32 * 18.0) * std::f32::consts::PI / 180.0)}",
                            x2: "{140.0 + 120.0 * f32::cos((-90.0 + i as f32 * 18.0) * std::f32::consts::PI / 180.0)}",
                            y2: "{140.0 + 120.0 * f32::sin((-90.0 + i as f32 * 18.0) * std::f32::consts::PI / 180.0)}",
                            stroke: "#ffffff25",
                            stroke_width: "1.5",
                        }
                    }
                    // Needle
                    line {
                        x1: "140",
                        y1: "140",
                        x2: "{140.0 + 92.0 * f32::cos(tip_angle * std::f32::consts::PI / 180.0)}",
                        y2: "{140.0 - 92.0 * f32::sin(tip_angle * std::f32::consts::PI / 180.0)}",
                        stroke: "#eaffd0",
                        stroke_width: "3",
                        stroke_linecap: "round",
                    }
                    circle { cx: "140", cy: "140", r: "7", fill: "#b9f078" }
                }
                // Readout sits below the dial, clear of the needle's sweep
                div { class: "text-center -mt-1",
                    span { class: "font-mono text-3xl font-bold text-white tabular-nums",
                        "{value.get_value().round() as i32}"
                    }
                    span { class: "text-xs text-[#848e9b] ml-1", "%" }
                }
            }
            div { class: "flex gap-2",
                for preset in [0.0f32, 25.0, 60.0, 85.0, 100.0] {
                    button {
                        key: "{preset}",
                        class: "px-4 py-2 rounded-lg text-sm font-medium text-[#a9b0bb] hover:text-[#e8ffb0] transition-colors",
                        style: "background: #0d1216; border: 1px solid #ffffff12;",
                        onclick: move |_| target(preset),
                        "{preset as i32}"
                    }
                }
            }
        }
    }
}
