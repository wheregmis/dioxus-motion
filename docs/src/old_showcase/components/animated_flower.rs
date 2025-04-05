use dioxus::prelude::*;
use dioxus_motion::{animations::utils::Animatable, prelude::*};
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
    let mut petal_transform = use_motion(PetalTransform::zero());
    let mut leaf_transform = use_motion(PetalTransform::zero());
    let mut center_scale = use_motion(1.0f32); // Start from 1.0 instead of 0.0
    let mut center_rotate = use_motion(0.0f32);
    let mut is_leaves_grown = use_signal_sync(|| false);
    let mut stem_length = use_motion(100.0f32);
    let mut stem_sway = use_motion(0.0f32);
    let mut glow_opacity = use_motion(0.0f32);

    let animate_leaves = move |_: Event<MountedData>| {
        // Enhanced stem animation with natural growth
        stem_length.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 25.0, // Slower for more organic movement
                damping: 8.0,
                mass: 0.4,
                velocity: 0.5,
            })),
        );

        // Add gentle stem sway
        stem_sway.animate_to(
            5.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 15.0,
                damping: 3.0,
                mass: 0.3,
                velocity: 0.0,
            }))
            .with_loop(LoopMode::Alternate),
        );

        // Enhanced leaf growth animation
        leaf_transform.animate_to(
            PetalTransform::new(
                PI / 5.0,
                1.2,   // Slightly larger scale
                2.0,   // Add some x movement
                -22.0, // Higher up
            ),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 35.0,
                damping: 6.0,
                mass: 0.4,
                velocity: 2.5,
            }))
            .with_on_complete(move || {
                is_leaves_grown.set(true);
            }),
        );
    };

    let mut animate_petals = move || {
        if *is_leaves_grown.read() {
            // More dynamic petal animation
            petal_transform.animate_to(
                PetalTransform::new(PI / 3.5, 1.3, 4.0, 4.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 45.0,
                    damping: 7.0,
                    mass: 0.4,
                    velocity: 1.5,
                }))
                .with_loop(LoopMode::Alternate),
            );

            // Add rotation to center
            center_rotate.animate_to(
                360.0,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 20.0,
                    damping: 5.0,
                    mass: 0.3,
                    velocity: 0.5,
                }))
                .with_loop(LoopMode::Infinite),
            );

            // Modified center scaling animation
            center_scale.animate_to(
                1.4,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 60.0, // Reduced stiffness
                    damping: 12.0,   // Increased damping
                    mass: 1.0,       // Increased mass
                    velocity: 0.0,   // Start with zero velocity
                }))
                .with_loop(LoopMode::Alternate),
            );

            // Add subtle glow effect
            glow_opacity.animate_to(
                0.6,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 40.0,
                    damping: 6.0,
                    mass: 0.5,
                    velocity: 0.0,
                }))
                .with_loop(LoopMode::Alternate),
            );
        }
    };

    use_effect(move || {
        if *is_leaves_grown.read() {
            animate_petals();
        }
    });

    rsx! {
        div { class: "flex items-center justify-center p-8",
            // Add subtle glow behind the flower
            div {
                class: "absolute",
                style: "filter: blur(20px); opacity: {glow_opacity.get_value()}",
                svg {
                    width: "300",
                    height: "300",
                    view_box: "-50 -50 100 100",
                    circle {
                        cx: "0",
                        cy: "0",
                        r: "30",
                        fill: "url(#glow_gradient)",
                    }
                }
            }
            svg {
                width: "300",
                height: "300",
                view_box: "-50 -50 100 100",
                onmounted: animate_leaves,

                // Enhanced leaves with gradient
                {
                    (0..8)
                        .map(|i| {
                            rsx! {
                                path {
                                    key: "leaf_{i}",
                                    d: "M 0 0 C 5 -3, 8 0, 5 5 C 8 0, 5 -3, 0 0",
                                    fill: "url(#leaf_gradient)",
                                    transform: "translate(0 {25.0 + leaf_transform.get_value().translate_y + (i as f32 * 5.0)})
                                                                                                                                                                                                          rotate({-20.0 + (i as f32 * 15.0) + stem_sway.get_value()}) 
                                                                                                                                                                                                          scale({leaf_transform.get_value().scale})",
                                    opacity: "0.95",
                                    style: "filter: drop-shadow(0 2px 3px rgba(0,0,0,0.2))",
                                }
                            }
                        })
                }

                // Enhanced stem with dynamic curve
                path {
                    d: "M 0 25 C {-4.0 + stem_sway.get_value()} 20, {4.0 - stem_sway.get_value()} 15, {-2.0 + stem_sway.get_value()} 10 C {4.0 - stem_sway.get_value()} 5, {-4.0 + stem_sway.get_value()} 0, 0 -2",
                    stroke: "#2F855A",
                    stroke_width: "1.4",
                    fill: "none",
                    stroke_dasharray: "100",
                    stroke_dashoffset: "{stem_length.get_value()}",
                    style: "filter: drop-shadow(0 2px 2px rgba(0,0,0,0.1))",
                }

                // Enhanced center with rotation
                circle {
                    cx: "0",
                    cy: "0",
                    r: "{(3.0 * center_scale.get_value()).max(0.1)}", // Added minimum radius
                    fill: "url(#center_gradient)",
                    transform: "rotate({center_rotate.get_value()})",
                    style: "filter: drop-shadow(0 2px 4px rgba(0,0,0,0.2))",
                }

                // Enhanced petals with gradients
                {
                    (0..8)
                        .map(|i| {
                            let base_angle = (i as f32) * PI / 4.0;
                            let transform_value = petal_transform.get_value();
                            let hue = 340.0 + (i as f32 * 8.0);
                            rsx! {
                                path {
                                    key: "petal_{i}",
                                    d: "M 0 -1 C 3 -6, 6 -8, 0 -14 C -6 -8, -3 -6, 0 -1",
                                    fill: "hsl({hue}, 85%, 75%)",
                                    transform: "translate({transform_value.translate_x} {transform_value.translate_y})
                                                                                                                                                                                                          rotate({(base_angle + transform_value.rotate) * 180.0 / PI}) 
                                                                                                                                                                                                          scale({transform_value.scale})",
                                    opacity: "0.9",
                                    style: "filter: drop-shadow(0 2px 3px rgba(0,0,0,0.15))",
                                }
                            }
                        })
                }
            }
        }
    }
}
