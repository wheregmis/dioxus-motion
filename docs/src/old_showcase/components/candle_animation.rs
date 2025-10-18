use dioxus::prelude::*;
use dioxus_motion::{animations::core::Animatable, prelude::*};
use wide::f32x4;

#[derive(Debug, Clone, Copy)]
pub struct FlameTransform {
    scale_x: f32,
    scale_y: f32,
    rotate: f32,
    translate_x: f32,
    translate_y: f32,
    opacity: f32,
}

impl FlameTransform {
    pub fn new(
        scale_x: f32,
        scale_y: f32,
        rotate: f32,
        translate_x: f32,
        translate_y: f32,
        opacity: f32,
    ) -> Self {
        Self {
            scale_x,
            scale_y,
            rotate,
            translate_x,
            translate_y,
            opacity,
        }
    }
}

impl Default for FlameTransform {
    fn default() -> Self {
        Self::new(1.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }
}

impl std::ops::Add for FlameTransform {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.scale_x + other.scale_x,
            self.scale_y + other.scale_y,
            self.rotate + other.rotate,
            self.translate_x + other.translate_x,
            self.translate_y + other.translate_y,
            self.opacity + other.opacity,
        )
    }
}

impl std::ops::Sub for FlameTransform {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.scale_x - other.scale_x,
            self.scale_y - other.scale_y,
            self.rotate - other.rotate,
            self.translate_x - other.translate_x,
            self.translate_y - other.translate_y,
            self.opacity - other.opacity,
        )
    }
}

impl std::ops::Mul<f32> for FlameTransform {
    type Output = Self;
    fn mul(self, factor: f32) -> Self {
        Self::new(
            self.scale_x * factor,
            self.scale_y * factor,
            self.rotate * factor,
            self.translate_x * factor,
            self.translate_y * factor,
            self.opacity * factor,
        )
    }
}

impl Animatable for FlameTransform {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let a = [
            self.scale_x,
            self.scale_y,
            self.rotate,
            self.translate_x,
            self.translate_y,
            self.opacity,
        ];
        let b = [
            target.scale_x,
            target.scale_y,
            target.rotate,
            target.translate_x,
            target.translate_y,
            target.opacity,
        ];

        // Use SIMD for the first 4 fields
        let va1 = f32x4::new([a[0], a[1], a[2], a[3]]);
        let vb1 = f32x4::new([b[0], b[1], b[2], b[3]]);
        let vt = f32x4::splat(t);
        let result1 = va1 + (vb1 - va1) * vt;
        let out1 = result1.to_array();

        // Handle the last 2 fields
        let va2 = f32x4::new([a[4], a[5], 0.0, 0.0]);
        let vb2 = f32x4::new([b[4], b[5], 0.0, 0.0]);
        let result2 = va2 + (vb2 - va2) * vt;
        let out2 = result2.to_array();

        Self::new(out1[0], out1[1], out1[2], out1[3], out2[0], out2[1])
    }

    fn magnitude(&self) -> f32 {
        (self.scale_x * self.scale_x
            + self.scale_y * self.scale_y
            + self.rotate * self.rotate
            + self.translate_x * self.translate_x
            + self.translate_y * self.translate_y
            + self.opacity * self.opacity)
            .sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WaxTransform {
    scale_y: f32,
    translate_y: f32,
    opacity: f32,
    drip_length: f32,
}

impl WaxTransform {
    pub fn new(scale_y: f32, translate_y: f32, opacity: f32, drip_length: f32) -> Self {
        Self {
            scale_y,
            translate_y,
            opacity,
            drip_length,
        }
    }
}

impl Default for WaxTransform {
    fn default() -> Self {
        Self::new(1.0, 0.0, 1.0, 0.0)
    }
}

impl std::ops::Add for WaxTransform {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.scale_y + other.scale_y,
            self.translate_y + other.translate_y,
            self.opacity + other.opacity,
            self.drip_length + other.drip_length,
        )
    }
}

impl std::ops::Sub for WaxTransform {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.scale_y - other.scale_y,
            self.translate_y - other.translate_y,
            self.opacity - other.opacity,
            self.drip_length - other.drip_length,
        )
    }
}

impl std::ops::Mul<f32> for WaxTransform {
    type Output = Self;
    fn mul(self, factor: f32) -> Self {
        Self::new(
            self.scale_y * factor,
            self.translate_y * factor,
            self.opacity * factor,
            self.drip_length * factor,
        )
    }
}

impl Animatable for WaxTransform {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let a = [
            self.scale_y,
            self.translate_y,
            self.opacity,
            self.drip_length,
        ];
        let b = [
            target.scale_y,
            target.translate_y,
            target.opacity,
            target.drip_length,
        ];
        let va = f32x4::new(a);
        let vb = f32x4::new(b);
        let vt = f32x4::splat(t);
        let result = va + (vb - va) * vt;
        let out = result.to_array();
        Self::new(out[0], out[1], out[2], out[3])
    }

    fn magnitude(&self) -> f32 {
        (self.scale_y * self.scale_y
            + self.translate_y * self.translate_y
            + self.opacity * self.opacity
            + self.drip_length * self.drip_length)
            .sqrt()
    }
}

#[component]
pub fn CandleAnimation() -> Element {
    let mut flame_transform = use_motion_store(FlameTransform::default());
    let mut inner_flame_transform = use_motion_store(FlameTransform::default());
    let mut glow_scale = use_motion_store(1.0f32);
    let mut wax_transform = use_motion_store(WaxTransform::default());
    let mut drip_opacity = use_motion_store(0.0f32);
    let mut ambient_glow = use_motion_store(0.3f32);
    let mut smoke_opacity = use_motion_store(0.0f32);
    let mut wick_glow = use_motion_store(0.0f32);

    let animate_flame = move |_: Event<MountedData>| {
        // Main flame flickering animation - more realistic
        flame_transform.animate_to(
            FlameTransform::new(1.15, 1.05, 1.5, 0.3, -0.5, 0.95),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 180.0,
                damping: 12.0,
                mass: 0.2,
                velocity: 8.0,
            }))
            .with_loop(LoopMode::Alternate),
        );

        // Inner flame animation (more intense flickering)
        inner_flame_transform.animate_to(
            FlameTransform::new(1.25, 1.15, 2.0, 0.4, -0.8, 0.85),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 220.0,
                damping: 8.0,
                mass: 0.15,
                velocity: 12.0,
            }))
            .with_loop(LoopMode::Alternate),
        );

        // Glow effect - subtle pulsing
        glow_scale.animate_to(
            1.2,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 120.0,
                damping: 15.0,
                mass: 0.4,
                velocity: 5.0,
            }))
            .with_loop(LoopMode::Alternate),
        );

        // Wax melting effect - gradual
        wax_transform.animate_to(
            WaxTransform::new(0.92, 3.0, 0.85, 8.0),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 25.0,
                damping: 15.0,
                mass: 1.2,
                velocity: 0.0,
            }))
            .with_loop(LoopMode::Alternate),
        );

        // Drip effect - intermittent
        drip_opacity.animate_to(
            0.7,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 60.0,
                damping: 10.0,
                mass: 0.6,
                velocity: 3.0,
            }))
            .with_loop(LoopMode::Alternate),
        );

        // Ambient glow - warm light
        ambient_glow.animate_to(
            0.5,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 80.0,
                damping: 12.0,
                mass: 0.8,
                velocity: 2.0,
            }))
            .with_loop(LoopMode::Alternate),
        );

        // Smoke effect - subtle
        smoke_opacity.animate_to(
            0.3,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 40.0,
                damping: 8.0,
                mass: 1.0,
                velocity: 1.0,
            }))
            .with_loop(LoopMode::Alternate),
        );

        // Wick glow effect
        wick_glow.animate_to(
            0.8,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 150.0,
                damping: 8.0,
                mass: 0.3,
                velocity: 6.0,
            }))
            .with_loop(LoopMode::Alternate),
        );
    };

    rsx! {
        div { class: "flex items-center justify-center p-2",
            // Background ambient glow - warm candlelight
            div {
                class: "absolute",
                style: "filter: blur(25px); opacity: {ambient_glow.store().current()()}",
                svg {
                    width: "250",
                    height: "250",
                    view_box: "-60 -60 120 120",
                    circle {
                        cx: "0",
                        cy: "0",
                        r: "45",
                        fill: "url(#ambient_gradient)",
                    }
                }
            }

            svg {
                width: "200",
                height: "250",
                view_box: "-30 -60 60 120",
                onmounted: animate_flame,

                defs {
                    // Candle body gradient - more realistic wax colors
                    linearGradient {
                        id: "candle_gradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "100%",
                        stop { offset: "0%", style: "stop-color:#fef3c7" }
                        stop { offset: "30%", style: "stop-color:#fde68a" }
                        stop { offset: "70%", style: "stop-color:#fbbf24" }
                        stop { offset: "100%", style: "stop-color:#f59e0b" }
                    }

                    // Flame outer gradient - realistic fire colors
                    radialGradient {
                        id: "flame_outer",
                        cx: "50%",
                        cy: "50%",
                        r: "50%",
                        stop { offset: "0%", style: "stop-color:#fbbf24" }
                        stop { offset: "40%", style: "stop-color:#f59e0b" }
                        stop { offset: "80%", style: "stop-color:#d97706" }
                        stop { offset: "100%", style: "stop-color:#b45309" }
                    }

                    // Flame inner gradient - bright core
                    radialGradient {
                        id: "flame_inner",
                        cx: "50%",
                        cy: "50%",
                        r: "50%",
                        stop { offset: "0%", style: "stop-color:#fef3c7" }
                        stop { offset: "50%", style: "stop-color:#fbbf24" }
                        stop { offset: "100%", style: "stop-color:#f59e0b" }
                    }

                    // Glow filter - warm light
                    filter { id: "glow",
                        feGaussianBlur {
                            "in": "SourceGraphic",
                            std_deviation: "4",
                            result: "blur",
                        }
                        feColorMatrix {
                            "in": "blur",
                            r#type: "matrix",
                            values: "1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 18 -5",
                        }
                    }

                    // Ambient glow gradient - warm candlelight
                    radialGradient {
                        id: "ambient_gradient",
                        cx: "50%",
                        cy: "50%",
                        r: "50%",
                        stop { offset: "0%", style: "stop-color:rgba(251,191,36,0.4)" }
                        stop { offset: "70%", style: "stop-color:rgba(251,191,36,0.1)" }
                        stop { offset: "100%", style: "stop-color:rgba(251,191,36,0)" }
                    }

                    // Smoke gradient - realistic
                    radialGradient {
                        id: "smoke_gradient",
                        cx: "50%",
                        cy: "50%",
                        r: "50%",
                        stop { offset: "0%", style: "stop-color:rgba(156,163,175,0.4)" }
                        stop { offset: "100%", style: "stop-color:rgba(156,163,175,0)" }
                    }

                    // Wick glow gradient
                    radialGradient {
                        id: "wick_glow",
                        cx: "50%",
                        cy: "50%",
                        r: "50%",
                        stop { offset: "0%", style: "stop-color:rgba(251,191,36,0.8)" }
                        stop { offset: "100%", style: "stop-color:rgba(251,191,36,0)" }
                    }
                }

                // Smoke effect - more realistic positioning
                {
                    (0..3).map(|i| {
                        let offset = (i as f32 - 1.0) * 4.0;
                        rsx! {
                            circle {
                                key: "smoke_{i}",
                                cx: "{offset}",
                                cy: "{-45.0 - (i as f32 * 6.0)}",
                                r: "{4.0 + (i as f32 * 1.0)}",
                                fill: "url(#smoke_gradient)",
                                opacity: "{smoke_opacity.store().current()() * (1.0 - i as f32 * 0.3)}",
                                style: "filter: blur(1px)",
                            }
                        }
                    })
                }

                // Background glow behind flame - warm light
                circle {
                    cx: "0",
                    cy: "-35",
                    r: "{15.0 * glow_scale.store().current()()}",
                    fill: "url(#ambient_gradient)",
                    filter: "url(#glow)",
                }

                // Candle body - more realistic proportions
                rect {
                    x: "-8",
                    y: "-40",
                    width: "16",
                    height: "{70.0 * wax_transform.store().current()().scale_y}",
                    fill: "url(#candle_gradient)",
                    stroke: "#d97706",
                    stroke_width: "0.3",
                    opacity: "{wax_transform.store().current()().opacity}",
                    style: "filter: drop-shadow(0 2px 4px rgba(0,0,0,0.3))",
                }

                // Candle wick with glow effect
                rect {
                    x: "-0.5",
                    y: "-42",
                    width: "1",
                    height: "4",
                    fill: "#1f2937",
                    stroke: "#374151",
                    stroke_width: "0.2",
                }

                // Wick glow
                circle {
                    cx: "0",
                    cy: "-40",
                    r: "2",
                    fill: "url(#wick_glow)",
                    opacity: "{wick_glow.store().current()()}",
                    style: "filter: blur(0.5px)",
                }

                // Wax drips - more realistic positioning
                {
                    (0..2).map(|i| {
                        let x_offset = (i as f32 - 0.5) * 6.0;
                        rsx! {
                            path {
                                key: "drip_{i}",
                                d: "M {x_offset} -40 Q {x_offset + 1.0} -36 {x_offset + 2.0} -32 Q {x_offset + 1.0} -28 {x_offset} -24",
                                stroke: "#fbbf24",
                                stroke_width: "1",
                                fill: "none",
                                opacity: "{drip_opacity.store().current()() * (0.9 - i as f32 * 0.2)}",
                                style: "filter: drop-shadow(0 1px 2px rgba(0,0,0,0.1))",
                            }
                        }
                    })
                }

                // Outer flame - teardrop shape, properly positioned above wick
                path {
                    d: "M -4 -35 Q 0 -45 4 -35 Q 3 -30 0 -28 Q -3 -30 -4 -35",
                    fill: "url(#flame_outer)",
                    opacity: "{flame_transform.store().current()().opacity}",
                    transform: "translate({flame_transform.store().current()().translate_x} {flame_transform.store().current()().translate_y})
                                rotate({flame_transform.store().current()().rotate})
                                scale({flame_transform.store().current()().scale_x} {flame_transform.store().current()().scale_y})",
                    style: "filter: drop-shadow(0 1px 3px rgba(251,191,36,0.3))",
                }

                // Inner flame (brighter core), properly positioned above wick
                path {
                    d: "M -3 -35 Q 0 -42 3 -35 Q 2 -31 0 -29 Q -2 -31 -3 -35",
                    fill: "url(#flame_inner)",
                    opacity: "{inner_flame_transform.store().current()().opacity}",
                    transform: "translate({inner_flame_transform.store().current()().translate_x} {inner_flame_transform.store().current()().translate_y})
                                rotate({inner_flame_transform.store().current()().rotate})
                                scale({inner_flame_transform.store().current()().scale_x} {inner_flame_transform.store().current()().scale_y})",
                    style: "filter: drop-shadow(0 1px 2px rgba(254,243,199,0.4))",
                }

                // Flame tip glow - bright white, properly positioned
                circle {
                    cx: "0",
                    cy: "-42",
                    r: "1.5",
                    fill: "#fef3c7",
                    opacity: "{0.9 * flame_transform.store().current()().opacity}",
                    style: "filter: blur(0.3px)",
                }

                // Base shadow - realistic, positioned below candle
                ellipse {
                    cx: "0",
                    cy: "35",
                    rx: "14",
                    ry: "4",
                    fill: "rgba(0,0,0,0.25)",
                    style: "filter: blur(1.5px)",
                }
            }
        }
    }
}
