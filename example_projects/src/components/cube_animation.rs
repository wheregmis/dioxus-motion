use dioxus::prelude::*;
use dioxus_motion::{animations::Animatable, prelude::*};
use std::f32::consts::PI;

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

impl Animatable for Transform3D {
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
    }

    fn epsilon() -> f32 {
        0.001
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

    fn scale(&self, factor: f32) -> Self {
        Self::new(
            self.rotate_x * factor,
            self.rotate_y * factor,
            self.rotate_z * factor,
            self.translate_x * factor,
            self.translate_y * factor,
            self.scale * factor,
        )
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(
            self.rotate_x + other.rotate_x,
            self.rotate_y + other.rotate_y,
            self.rotate_z + other.rotate_z,
            self.translate_x + other.translate_x,
            self.translate_y + other.translate_y,
            self.scale + other.scale,
        )
    }

    fn sub(&self, other: &Self) -> Self {
        Self::new(
            self.rotate_x - other.rotate_x,
            self.rotate_y - other.rotate_y,
            self.rotate_z - other.rotate_z,
            self.translate_x - other.translate_x,
            self.translate_y - other.translate_y,
            self.scale - other.scale,
        )
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        Self::new(
            self.rotate_x + (target.rotate_x - self.rotate_x) * t,
            self.rotate_y + (target.rotate_y - self.rotate_y) * t,
            self.rotate_z + (target.rotate_z - self.rotate_z) * t,
            self.translate_x + (target.translate_x - self.translate_x) * t,
            self.translate_y + (target.translate_y - self.translate_y) * t,
            self.scale + (target.scale - self.scale) * t,
        )
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
    let mut transform = use_motion(Transform3D::zero());
    let mut glow_scale = use_motion(1.0f32);

    let animate = move |_| {
        transform.animate_to(
            Transform3D::new(
                PI / 3.0, // X rotation
                PI / 2.0, // Y rotation
                PI / 4.0, // Z rotation
                2.0,      // X translation
                -1.0,     // Y translation
                1.2,      // Scale
            ),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 35.0,
                damping: 5.0,
                mass: 1.0,
                velocity: 2.0,
            }))
            .with_loop(LoopMode::Infinite),
        );

        glow_scale.animate_to(
            1.4,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 40.0,
                damping: 4.0,
                mass: 0.5,
                velocity: 1.0,
            }))
            .with_loop(LoopMode::Infinite),
        );
    };

    let projected_vertices: Vec<(f32, f32)> = VERTICES
        .iter()
        .map(|v| {
            v.rotate_x(transform.get_value().rotate_x)
                .rotate_y(transform.get_value().rotate_y)
                .rotate_z(transform.get_value().rotate_z)
                .translate(
                    transform.get_value().translate_x,
                    transform.get_value().translate_y,
                )
                .project(50.0 * transform.get_value().scale)
        })
        .collect();

    rsx! {
        div { class: "flex items-center justify-center p-8",
            svg {
                width: "400.0",
                height: "400.0",
                view_box: "0.0 0.0 200.0 200.0",
                onmounted: animate,
                defs {
                    // Gradient definitions
                    linearGradient {
                        id: "cube-gradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "100%",
                        stop { offset: "0%", style: "stop-color:#4299e1" }
                        stop { offset: "50%", style: "stop-color:#9f7aea" }
                        stop { offset: "100%", style: "stop-color:#ed64a6" }
                    }
                    // Glow filter
                    filter { id: "glow",
                        feGaussianBlur {
                            "in": "SourceGraphic",
                            std_deviation: "4.0",
                            result: "blur",
                        }
                        feColorMatrix {
                            "in": "blur",
                            r#type: "matrix",
                            values: "1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 18 -7",
                        }
                    }
                }
                //Glowing background circle
                circle {
                    cx: "100.0",
                    cy: "100.0",
                    r: "{30.0 * glow_scale.get_value()}",
                    fill: "url(#cube-gradient)",
                    filter: "url(#glow)",
                    opacity: "0.3",
                }
                // Enhanced rope with gradient
                path {
                    d: "M 100 20 Q {projected_vertices[4].0} {projected_vertices[4].1 - 20.0}
                       {projected_vertices[4].0} {projected_vertices[4].1}",
                    stroke: "url(#cube-gradient)",
                    stroke_width: "1",
                    fill: "none",
                    stroke_dasharray: "4,4",
                }
                // Enhanced cube faces with gradients and animations
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
                                    // Shadow effect
                                    path {
                                        d: "{path}",
                                        fill: "rgba(0,0,0,0.2)",
                                        transform: "translate(2.0 2.0)",
                                    }
                                    // Main face
                                    path {
                                        d: "{path}",
                                        fill: "url(#cube-gradient)",
                                        stroke: "#ffffff",
                                        stroke_width: "0.5",
                                        opacity: "{0.7 + (i as f32 * 0.05)}",
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
