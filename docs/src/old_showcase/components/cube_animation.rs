use dioxus::prelude::*;
use dioxus_motion::{animations::core::Animatable, prelude::*};
use wide::f32x4;

#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    rotate_x: f32,
    rotate_y: f32,
    rotate_z: f32,
    translate_x: f32,
    translate_y: f32,
    scale: f32,
}

impl Transform3D {
    pub fn new(
        rotate_x: f32,
        rotate_y: f32,
        rotate_z: f32,
        translate_x: f32,
        translate_y: f32,
        scale: f32,
    ) -> Self {
        Self {
            rotate_x,
            rotate_y,
            rotate_z,
            translate_x,
            translate_y,
            scale,
        }
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
    }
}

impl std::ops::Add for Transform3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.rotate_x + other.rotate_x,
            self.rotate_y + other.rotate_y,
            self.rotate_z + other.rotate_z,
            self.translate_x + other.translate_x,
            self.translate_y + other.translate_y,
            self.scale + other.scale,
        )
    }
}

impl std::ops::Sub for Transform3D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.rotate_x - other.rotate_x,
            self.rotate_y - other.rotate_y,
            self.rotate_z - other.rotate_z,
            self.translate_x - other.translate_x,
            self.translate_y - other.translate_y,
            self.scale - other.scale,
        )
    }
}

impl std::ops::Mul<f32> for Transform3D {
    type Output = Self;
    fn mul(self, factor: f32) -> Self {
        Self::new(
            self.rotate_x * factor,
            self.rotate_y * factor,
            self.rotate_z * factor,
            self.translate_x * factor,
            self.translate_y * factor,
            self.scale * factor,
        )
    }
}

impl Animatable for Transform3D {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        // SIMD for the first 4 fields
        let a1 = [
            self.rotate_x,
            self.rotate_y,
            self.rotate_z,
            self.translate_x,
        ];
        let b1 = [
            target.rotate_x,
            target.rotate_y,
            target.rotate_z,
            target.translate_x,
        ];
        let va1 = f32x4::new(a1);
        let vb1 = f32x4::new(b1);
        let vt = f32x4::splat(t);
        let result1 = va1 + (vb1 - va1) * vt;
        let out1 = result1.to_array();
        // SIMD for the last 2 fields + 2 zeros
        let a2 = [self.translate_y, self.scale, 0.0, 0.0];
        let b2 = [target.translate_y, target.scale, 0.0, 0.0];
        let va2 = f32x4::new(a2);
        let vb2 = f32x4::new(b2);
        let result2 = va2 + (vb2 - va2) * vt;
        let out2 = result2.to_array();
        Self::new(out1[0], out1[1], out1[2], out1[3], out2[0], out2[1])
    }

    fn magnitude(&self) -> f32 {
        (self.rotate_x * self.rotate_x
            + self.rotate_y * self.rotate_y
            + self.rotate_z * self.rotate_z
            + self.translate_x * self.translate_x
            + self.translate_y * self.translate_y
            + self.scale * self.scale)
            .sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3D {
    fn rotate_x(self, angle: f32) -> Self {
        Point3D {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }

    fn rotate_y(self, angle: f32) -> Self {
        Point3D {
            x: self.x * angle.cos() + self.z * angle.sin(),
            y: self.y,
            z: -self.x * angle.sin() + self.z * angle.cos(),
        }
    }

    fn rotate_z(self, angle: f32) -> Self {
        Point3D {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,
        }
    }

    fn translate(self, tx: f32, ty: f32) -> Self {
        Point3D {
            x: self.x + tx,
            y: self.y + ty,
            z: self.z,
        }
    }

    fn project(self, scale: f32) -> (f32, f32) {
        (
            100.0 + scale * self.x / (self.z + 4.0),
            100.0 + scale * self.y / (self.z + 4.0),
        )
    }
}

// Cube vertices and faces remain the same as in your original code
const VERTICES: [Point3D; 8] = [
    Point3D {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    },
    Point3D {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    },
    Point3D {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    },
    Point3D {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    },
    Point3D {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    },
    Point3D {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    },
    Point3D {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    Point3D {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    },
];

const FACES: [[usize; 4]; 6] = [
    [0, 1, 2, 3], // front
    [1, 5, 6, 2], // right
    [5, 4, 7, 6], // back
    [4, 0, 3, 7], // left
    [3, 2, 6, 7], // top
    [4, 5, 1, 0], // bottom
];

#[component]
pub fn SwingingCube() -> Element {
    let mut transform = use_motion_store(Transform3D::default());
    let mut glow_scale = use_motion_store(1.0f32);
    let mut pulse_scale = use_motion_store(1.0f32);
    let mut highlight_opacity = use_motion_store(0.0f32);

    let animate = move |_| {
        // Main swing animation - continuous loop
        transform.animate_to(
            Transform3D::new(45.0, 90.0, 0.0, 0.0, 0.0, 1.2),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 80.0,
                damping: 8.0,
                mass: 1.0,
                velocity: 15.0,
            }))
            .with_loop(LoopMode::Alternate), // Add continuous alternating loop
        );

        // Glow effect - continuous loop
        glow_scale.animate_to(
            1.5,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 120.0,
                damping: 10.0,
                mass: 0.8,
                velocity: 8.0,
            }))
            .with_loop(LoopMode::Alternate), // Add continuous alternating loop
        );

        // Pulse effect - continuous loop
        pulse_scale.animate_to(
            1.3,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 12.0,
                mass: 0.6,
                velocity: 10.0,
            }))
            .with_loop(LoopMode::Alternate), // Add continuous alternating loop
        );

        // Highlight effect - continuous loop
        highlight_opacity.animate_to(
            0.8,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 150.0,
                damping: 8.0,
                mass: 0.5,
                velocity: 5.0,
            }))
            .with_loop(LoopMode::Alternate), // Add continuous alternating loop
        );
    };

    let projected_vertices: Vec<(f32, f32)> = VERTICES
        .iter()
        .map(|v| {
            v.rotate_x(transform.store().current()().rotate_x)
                .rotate_y(transform.store().current()().rotate_y)
                .rotate_z(transform.store().current()().rotate_z)
                .translate(
                    transform.store().current()().translate_x,
                    transform.store().current()().translate_y,
                )
                .project(
                    50.0 * transform.store().current()().scale * pulse_scale.store().current()(),
                )
        })
        .collect();

    rsx! {
        div { class: "flex items-center justify-center p-8",
            svg {
                width: "400.0",
                height: "400.0",
                view_box: "-20.0 -20.0 240.0 240.0", // Adjusted viewBox for better centering
                onmounted: animate,
                defs {
                    // Enhanced gradient with more colors
                    linearGradient {
                        id: "cube-gradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "100%",
                        stop { offset: "0%", style: "stop-color:#60a5fa" }
                        stop { offset: "25%", style: "stop-color:#7c3aed" }
                        stop { offset: "50%", style: "stop-color:#db2777" }
                        stop { offset: "75%", style: "stop-color:#9333ea" }
                        stop { offset: "100%", style: "stop-color:#3b82f6" }
                    }
                    // Enhanced glow filter
                    filter { id: "glow",
                        feGaussianBlur {
                            "in": "SourceGraphic",
                            std_deviation: "6.0",
                            result: "blur-sm",
                        }
                        feColorMatrix {
                            "in": "blur-sm",
                            r#type: "matrix",
                            values: "1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 22 -7",
                        }
                    }
                    // Highlight gradient
                    radialGradient {
                        id: "highlight",
                        cx: "50%",
                        cy: "50%",
                        r: "50%",
                        stop {
                            offset: "0%",
                            style: "stop-color:rgba(255,255,255,0.8)",
                        }
                        stop {
                            offset: "100%",
                            style: "stop-color:rgba(255,255,255,0)",
                        }
                    }
                }
                // Background effects
                circle {
                    cx: "100.0",
                    cy: "100.0",
                    r: "{40.0 * glow_scale.store().current()()}",
                    fill: "url(#cube-gradient)",
                    filter: "url(#glow)",
                    opacity: "0.4",
                }
                // Enhanced rope with double line
                path {
                    d: "M 100 10 Q {projected_vertices[4].0} {projected_vertices[4].1 - 30.0}
                       {projected_vertices[4].0} {projected_vertices[4].1}",
                    stroke: "url(#cube-gradient)",
                    stroke_width: "2",
                    fill: "none",
                    stroke_dasharray: "6,6",
                    filter: "url(#glow)",
                }
                // Cube faces with enhanced effects
                {
                    FACES
                        .iter()
                        .enumerate()
                        .map(|(i, face)| {
                            let path = format!(
                                "M {} {} L {} {} L {} {} L {} {} Z",
                                projected_vertices[face[0]].0,
                                projected_vertices[face[0]].1,
                                projected_vertices[face[1]].0,
                                projected_vertices[face[1]].1,
                                projected_vertices[face[2]].0,
                                projected_vertices[face[2]].1,
                                projected_vertices[face[3]].0,
                                projected_vertices[face[3]].1,
                            );
                            rsx! {
                                g { key: "{i}",
                                    // Enhanced shadow
                                    path {
                                        d: "{path}",
                                        fill: "rgba(0,0,0,0.3)",
                                        transform: "translate(3.0 3.0)",
                                        filter: "url(#glow)",
                                    }
                                    // Main face with gradient and stroke
                                    path {
                                        d: "{path}",
                                        fill: "url(#cube-gradient)",
                                        stroke: "#ffffff",
                                        stroke_width: "1.0",
                                        opacity: "{0.8 + (i as f32 * 0.05)}",
                                    }
                                    // Highlight overlay
                                    path {
                                        d: "{path}",
                                        fill: "url(#highlight)",
                                        opacity: "{highlight_opacity.store().current()()}",
                                    }
                                }
                            }
                        })
                }
                // Additional decorative elements
                circle {
                    cx: "100.0",
                    cy: "100.0",
                    r: "80.0",
                    fill: "none",
                    stroke: "url(#cube-gradient)",
                    stroke_width: "0.5",
                    stroke_dasharray: "4,4",
                    opacity: "0.3",
                }
            }
        }
    }
}
