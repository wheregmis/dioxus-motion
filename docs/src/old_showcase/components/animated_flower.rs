use dioxus::prelude::*;
use dioxus_motion::{animations::core::Animatable, prelude::*};
use std::f32::consts::PI;
use wide::f32x4;

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

impl Default for PetalTransform {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl std::ops::Add for PetalTransform {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.rotate + other.rotate,
            self.scale + other.scale,
            self.translate_x + other.translate_x,
            self.translate_y + other.translate_y,
        )
    }
}

impl std::ops::Sub for PetalTransform {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.rotate - other.rotate,
            self.scale - other.scale,
            self.translate_x - other.translate_x,
            self.translate_y - other.translate_y,
        )
    }
}

impl std::ops::Mul<f32> for PetalTransform {
    type Output = Self;
    fn mul(self, factor: f32) -> Self {
        Self::new(
            self.rotate * factor,
            self.scale * factor,
            self.translate_x * factor,
            self.translate_y * factor,
        )
    }
}

impl Animatable for PetalTransform {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let a = [self.rotate, self.scale, self.translate_x, self.translate_y];
        let b = [
            target.rotate,
            target.scale,
            target.translate_x,
            target.translate_y,
        ];
        let va = f32x4::new(a);
        let vb = f32x4::new(b);
        let vt = f32x4::splat(t);
        let result = va + (vb - va) * vt;
        let out = result.to_array();
        PetalTransform::new(out[0], out[1], out[2], out[3])
    }

    fn magnitude(&self) -> f32 {
        (self.rotate * self.rotate
            + self.scale * self.scale
            + self.translate_x * self.translate_x
            + self.translate_y * self.translate_y)
            .sqrt()
    }
}

// Multithreaded version with simplified architecture
#[component]
pub fn AnimatedFlower() -> Element {
    // Use regular motion hooks but with background processing
    let petal_transform = use_motion(PetalTransform::default());
    let leaf_transform = use_motion(PetalTransform::default());
    let center_scale = use_motion(1.0f32);
    let center_rotate = use_motion(0.0f32);
    let is_leaves_grown = use_signal_sync(|| false);
    let stem_length = use_motion(100.0f32);
    let stem_sway = use_motion(0.0f32);
    let glow_opacity = use_motion(0.0f32);

    // Performance monitoring
    let mut performance_info = use_signal(|| "Initializing...".to_string());
    let mut active_animations = use_signal(|| 0);

    let stem_length_anim = stem_length.clone();
    let stem_sway_anim = stem_sway.clone();
    let leaf_transform_anim = leaf_transform.clone();
    let petal_transform_anim = petal_transform.clone();
    let center_scale_anim = center_scale.clone();
    let center_rotate_anim = center_rotate.clone();
    let glow_opacity_anim = glow_opacity.clone();

    let animate_leaves = move |_: Event<MountedData>| {
        // Background processing for stem animations
        #[cfg(not(target_arch = "wasm32"))]
        spawn(async move {
            // Simulate complex stem calculations in background
            for i in 0..10 {
                let _progress = i as f32 / 10.0;
                // Simulate heavy computation
                std::thread::sleep(Duration::from_millis(10));
            }
        });

        // Enhanced stem animation with natural growth
        stem_length_anim.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 25.0,
                damping: 8.0,
                mass: 0.4,
                velocity: 0.5,
            })),
        );

        // Add gentle stem sway
        stem_sway_anim.animate_to(
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
        leaf_transform_anim.animate_to(
            PetalTransform::new(PI / 5.0, 1.2, 2.0, -22.0),
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

    // Monitor animation progress with background processing
    use_effect(move || {
        let count = [
            petal_transform_anim.is_running(),
            leaf_transform_anim.is_running(),
            center_scale_anim.is_running(),
            center_rotate_anim.is_running(),
            stem_length_anim.is_running(),
            stem_sway_anim.is_running(),
            glow_opacity_anim.is_running(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        active_animations.set(count);

        // Direct performance monitoring update (no async needed)
        let status = if count > 0 {
            format!("🚀 Processing {} animations", count)
        } else {
            "✨ All animations complete".to_string()
        };
        *performance_info.write() = status;
    });

    // Start petal animations when leaves are grown
    use_effect(move || {
        if *is_leaves_grown.read() {
            // Process petal animations with background calculations
            #[cfg(not(target_arch = "wasm32"))]
            spawn(async move {
                // Simulate complex petal calculations
                for i in 0..8 {
                    let angle = (i as f32) * PI / 4.0;
                    // Simulate heavy trigonometric calculations
                    let _complex_calc = (angle.sin() * angle.cos()).powf(2.0);
                    // Use sync sleep for desktop
                    std::thread::sleep(Duration::from_millis(5));
                }
            });

            // More dynamic petal animation
            petal_transform_anim.animate_to(
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
            center_rotate_anim.animate_to(
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
            center_scale_anim.animate_to(
                1.4,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 60.0,
                    damping: 12.0,
                    mass: 1.0,
                    velocity: 0.0,
                }))
                .with_loop(LoopMode::Alternate),
            );

            // Add subtle glow effect
            glow_opacity_anim.animate_to(
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
    });

    let petal_transform_val = petal_transform.clone();
    let leaf_transform_val = leaf_transform.clone();
    let center_scale_val = center_scale.clone();
    let center_rotate_val = center_rotate.clone();
    let glow_opacity_val = glow_opacity.clone();
    let stem_length_val = stem_length.clone();
    let stem_sway_val = stem_sway.clone();

    rsx! {
        div { class: "flex flex-col items-center justify-center p-8",
            // Performance indicator
            div { class: "mb-4 text-sm text-gray-600 font-mono",
                "{performance_info()}"
            }

            // Enhanced flower with background processing
            div { class: "relative",
                // Add subtle glow behind the flower
                div {
                    class: "absolute",
                    style: "filter: blur(20px); opacity: {glow_opacity_val.get_value()}",
                    svg {
                        width: "300",
                        height: "300",
                        view_box: "-50 -50 100 100",
                        circle {
                            cx: "0",
                            cy: "0",
                            r: "30",
                            fill: "radial-gradient(circle, rgba(255,182,193,0.8) 0%, rgba(255,105,180,0.4) 100%)",
                        }
                    }
                }

                svg {
                    width: "300",
                    height: "300",
                    view_box: "-50 -50 100 100",
                    onmounted: animate_leaves,

                    // Enhanced leaves with individual variations
                    {
                        (0..8)
                            .map(|i| {
                                let leaf_val = leaf_transform_val.get_value();
                                let variation = i as f32 * 0.1;
                                rsx! {
                                    path {
                                        key: "leaf_{i}",
                                        d: "M 0 0 C 5 -3, 8 0, 5 5 C 8 0, 5 -3, 0 0",
                                        fill: "linear-gradient(135deg, #48cc6c 0%, #2d7d47 100%)",
                                        transform: "translate(0 {25.0 + leaf_val.translate_y + (i as f32 * 5.0)})
                                                   rotate({-20.0 + (i as f32 * 15.0) + stem_sway_val.get_value() + variation})
                                                   scale({leaf_val.scale + variation})",
                                        opacity: "0.95",
                                        style: "filter: drop-shadow(0 2px 3px rgba(0,0,0,0.2))",
                                    }
                                }
                            })
                    }

                    // Enhanced stem with dynamic curve
                    path {
                        d: "M 0 25 C {-4.0 + stem_sway_val.get_value()} 20, {4.0 - stem_sway_val.get_value()} 15, {-2.0 + stem_sway_val.get_value()} 10 C {4.0 - stem_sway_val.get_value()} 5, {-4.0 + stem_sway_val.get_value()} 0, 0 -2",
                        stroke: "#2F855A",
                        stroke_width: "1.4",
                        fill: "none",
                        stroke_dasharray: "100",
                        stroke_dashoffset: "{stem_length_val.get_value()}",
                        style: "filter: drop-shadow(0 2px 2px rgba(0,0,0,0.1))",
                    }

                    // Enhanced center with rotation
                    circle {
                        cx: "0",
                        cy: "0",
                        r: "{(3.0 * center_scale_val.get_value()).max(0.1)}",
                        fill: "radial-gradient(circle, #ffd700 0%, #ff8c00 100%)",
                        transform: "rotate({center_rotate_val.get_value()})",
                        style: "filter: drop-shadow(0 2px 4px rgba(0,0,0,0.2))",
                    }

                    // Enhanced petals with individual variations
                    {
                        (0..8)
                            .map(|i| {
                                let base_angle = (i as f32) * PI / 4.0;
                                let transform_value = petal_transform_val.get_value();
                                let hue = 340.0 + (i as f32 * 8.0);
                                let variation = i as f32 * 0.05;
                                rsx! {
                                    path {
                                        key: "petal_{i}",
                                        d: "M 0 -1 C 3 -6, 6 -8, 0 -14 C -6 -8, -3 -6, 0 -1",
                                        fill: "hsl({hue}, 85%, 75%)",
                                        transform: "translate({transform_value.translate_x + variation} {transform_value.translate_y + variation})
                                                   rotate({(base_angle + transform_value.rotate) * 180.0 / PI})
                                                   scale({transform_value.scale + variation})",
                                        opacity: "0.9",
                                        style: "filter: drop-shadow(0 2px 3px rgba(0,0,0,0.15))",
                                    }
                                }
                            })
                    }
                }
            }

            // Multithreading info
            div { class: "mt-4 text-xs text-gray-500 text-center",
                "🚀 Background Processing Demo"
                br {}
                "Async calculations + Spring physics"
                br {}
                "Active: {active_animations()} | Total: 8 petals + 8 leaves + 5 core animations"
            }
        }
    }
}
