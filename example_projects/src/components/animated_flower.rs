use dioxus::prelude::*;
use dioxus_motion::{animations::Animatable, prelude::*};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct PetalTransform {
    rotate: f32,
    scale: f32,
    translate_x: f32,
    translate_y: f32,
}

impl PetalTransform {
    pub fn new(rotate: f32, scale: f32, translate_x: f32, translate_y: f32) -> Self {
        Self {
            rotate,
            scale,
            translate_x,
            translate_y,
        }
    }
}

impl Animatable for PetalTransform {
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        (self.rotate * self.rotate
            + self.scale * self.scale
            + self.translate_x * self.translate_x
            + self.translate_y * self.translate_y)
            .sqrt()
    }

    fn scale(&self, factor: f32) -> Self {
        Self::new(
            self.rotate * factor,
            self.scale * factor,
            self.translate_x * factor,
            self.translate_y * factor,
        )
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(
            self.rotate + other.rotate,
            self.scale + other.scale,
            self.translate_x + other.translate_x,
            self.translate_y + other.translate_y,
        )
    }

    fn sub(&self, other: &Self) -> Self {
        Self::new(
            self.rotate - other.rotate,
            self.scale - other.scale,
            self.translate_x - other.translate_x,
            self.translate_y - other.translate_y,
        )
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        Self::new(
            self.rotate + (target.rotate - self.rotate) * t,
            self.scale + (target.scale - self.scale) * t,
            self.translate_x + (target.translate_x - self.translate_x) * t,
            self.translate_y + (target.translate_y - self.translate_y) * t,
        )
    }
}

#[component]
pub fn AnimatedFlower() -> Element {
    let mut transform = use_motion(PetalTransform::zero());

    let animate = move |_| {
        transform.animate_to(
            PetalTransform::new(
                PI / 6.0, // Rotation
                1.0,      // Scale
                5.0,      // X translation
                5.0,      // Y translation
            ),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 0.5,
                ..Default::default()
            }))
            .with_loop(LoopMode::Infinite),
        );
    };

    rsx! {
        div { class: "flex items-center justify-center",
            svg {
                width: "200",
                height: "200",
                view_box: "-25 -25 50 50",
                onmounted: animate,

                // Stem
                path {
                    d: "M 0 25 C -2 15, 2 5, 0 0",
                    stroke: "#2F855A",
                    stroke_width: "0.75",
                    fill: "none",
                }

                // Leaf
                path {
                    d: "M 0 15 C 5 12, 7 17, 2 20 C 7 17, 5 12, 0 15",
                    fill: "#48BB78",
                    transform: "rotate(-20)",
                }

                // Center of the flower
                circle {
                    cx: "0",
                    cy: "0",
                    r: "2.5",
                    fill: "#F6E05E",
                }

                // 8 petals
                {
                    (0..6)
                        .map(|i| {
                            let base_angle = (i as f32) * PI / 3.0;
                            let transform_value = transform.get_value();
                            rsx! {
                                path {
                                    key: "{i}",
                                    d: "M 0 -1 C 2 -5, 5 -7, 0 -12 C -5 -7, -2 -5, 0 -1",
                                    fill: "#F687B3",
                                    transform: "translate({transform_value.translate_x} {transform_value.translate_y})
                                                                                                                                                                                                     rotate({(base_angle + transform_value.rotate) * 180.0 / PI}) 
                                                                                                                                                                                                     scale({transform_value.scale})",
                                    opacity: "0.9",
                                }
                            }
                        })
                }
            }
        }
    }
}
