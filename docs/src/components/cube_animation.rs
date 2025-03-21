use dioxus::prelude::*;
use dioxus_motion::{animations::utils::Animatable, prelude::*};
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
    /// Creates a new 3D transformation with specified rotation angles, translation values, and scaling.
    ///
    /// The rotation angles are provided for the x, y, and z axes, and the translation values apply to the x and y axes.
    /// The scale represents a uniform scaling factor.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = Transform3D::new(30.0, 45.0, 60.0, 100.0, 200.0, 1.0);
    /// assert_eq!(transform.rotate_x, 30.0);
    /// assert_eq!(transform.rotate_y, 45.0);
    /// assert_eq!(transform.rotate_z, 60.0);
    /// assert_eq!(transform.translate_x, 100.0);
    /// assert_eq!(transform.translate_y, 200.0);
    /// assert_eq!(transform.scale, 1.0);
    /// ```
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
    /// Returns a neutral transformation with no rotation or translation and a unit scale.
    ///
    /// This method constructs a `Transform3D` instance where all rotation angles and translations are zero,
    /// and the scale is set to 1.0, effectively representing the identity transformation.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = Transform3D::zero();
    /// assert_eq!(transform.rotation_x, 0.0);
    /// assert_eq!(transform.rotation_y, 0.0);
    /// assert_eq!(transform.rotation_z, 0.0);
    /// assert_eq!(transform.translation_x, 0.0);
    /// assert_eq!(transform.translation_y, 0.0);
    /// assert_eq!(transform.scale, 1.0);
    /// ```
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
    }

    /// Returns a small constant value for floating-point precision comparisons.
    ///
    /// This value is commonly used as a threshold when performing approximate equality checks
    /// between floating-point numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// let eps = epsilon();
    /// assert_eq!(eps, 0.001);
    /// ```
    fn epsilon() -> f32 {
        0.001
    }

    /// Computes the Euclidean magnitude of the transformation.
    ///
    /// This method calculates the square root of the sum of the squares of all transformation
    /// components including rotations around the x, y, and z axes, translations along the x and y axes,
    /// and the scale factor.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = Transform3D::zero();
    /// // Since a zero transformation has no rotation or translation and a scale of 1,
    /// // the magnitude is √(0²+0²+0²+0²+0²+1²) = 1.0.
    /// assert_eq!(transform.magnitude(), 1.0);
    /// ```
    fn magnitude(&self) -> f32 {
        (self.rotate_x * self.rotate_x
            + self.rotate_y * self.rotate_y
            + self.rotate_z * self.rotate_z
            + self.translate_x * self.translate_x
            + self.translate_y * self.translate_y
            + self.scale * self.scale)
            .sqrt()
    }

    /// Scales the transformation by the given factor, returning a new `Transform3D` instance with all components multiplied by that factor.
    ///
    /// This method multiplies the rotation, translation, and scaling components of the transformation uniformly.
    /// 
    /// # Examples
    ///
    /// ```
    /// // Create an initial transformation with specific rotation, translation, and scale values.
    /// let transform = Transform3D::new(1.0, 2.0, 3.0, 4.0, 5.0, 1.0);
    ///
    /// // Scale the transformation by a factor of 2.
    /// let scaled = transform.scale(2.0);
    ///
    /// // Verify that each component has been scaled correctly.
    /// assert_eq!(scaled.rotate_x, 2.0);
    /// assert_eq!(scaled.rotate_y, 4.0);
    /// assert_eq!(scaled.rotate_z, 6.0);
    /// assert_eq!(scaled.translate_x, 8.0);
    /// assert_eq!(scaled.translate_y, 10.0);
    /// assert_eq!(scaled.scale, 2.0);
    /// ```
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

    /// Adds the corresponding components of two `Transform3D` instances.
    ///
    /// This method returns a new instance where each field is the sum of the corresponding fields
    /// (rotations, translations, and scale) from `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// let t1 = Transform3D::new(1.0, 2.0, 3.0, 4.0, 5.0, 1.0);
    /// let t2 = Transform3D::new(0.5, 1.0, 1.5, 2.0, 2.5, 2.0);
    /// let result = t1.add(&t2);
    /// assert_eq!(result, Transform3D::new(1.5, 3.0, 4.5, 6.0, 7.5, 3.0));
    /// ```
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

    /// Subtracts the corresponding components of another `Transform3D` from this transformation.
    ///
    /// Performs an element-wise subtraction of rotation angles, translation distances, and scaling factors,
    /// returning a new `Transform3D` that represents the difference between the two transformations.
    ///
    /// # Examples
    ///
    /// ```
    /// let t1 = Transform3D::new(10.0, 20.0, 30.0, 40.0, 50.0, 2.0);
    /// let t2 = Transform3D::new(5.0, 15.0, 25.0, 35.0, 45.0, 1.0);
    /// let subtracted = t1.sub(&t2);
    ///
    /// assert_eq!(subtracted.rotate_x, 5.0);
    /// assert_eq!(subtracted.rotate_y, 5.0);
    /// assert_eq!(subtracted.rotate_z, 5.0);
    /// assert_eq!(subtracted.translate_x, 5.0);
    /// assert_eq!(subtracted.translate_y, 5.0);
    /// assert_eq!(subtracted.scale, 1.0);
    /// ```
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

    /// Returns a new 3D transformation that is a linear interpolation between this transformation and a target transformation.
    ///
    /// Each component (rotation around the x, y, and z axes; translation along the x and y axes; and scale) is
    /// interpolated independently using the factor `t`. A value of `t = 0.0` yields the current transformation,
    /// while `t = 1.0` yields the target transformation. Values outside the [0.0, 1.0] range will produce an extrapolated result.
    ///
    /// # Examples
    ///
    /// ```
    /// let start = Transform3D::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// let end = Transform3D::new(90.0, 90.0, 90.0, 10.0, 10.0, 2.0);
    /// let mid = start.interpolate(&end, 0.5);
    ///
    /// assert_eq!(mid.rotate_x, 45.0);
    /// assert_eq!(mid.translate_x, 5.0);
    /// // Other components are interpolated similarly.
    /// ```
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
    /// Rotates this 3D point around the x-axis by the specified angle in radians.
    ///
    /// The x coordinate remains unchanged while the y and z coordinates are recalculated
    /// using standard rotation formulas, effectively rotating the point in the yz-plane.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Point3D { x: 0.0, y: 1.0, z: 0.0 };
    /// let rotated = p.rotate_x(std::f32::consts::PI / 2.0);
    /// // After a 90° rotation, the point should be approximately (0.0, 0.0, 1.0)
    /// assert!(rotated.y.abs() < 1e-6);
    /// assert!((rotated.z - 1.0).abs() < 1e-6);
    /// ```
    fn rotate_x(self, angle: f32) -> Self {
        Point3D {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }

    /// Rotates the point around the y-axis by the given angle in radians.
    /// 
    /// This method returns a new `Point3D` that results from rotating the original point
    /// around the y-axis using standard trigonometric transformations.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cube_animation::Point3D;
    /// 
    /// let point = Point3D { x: 1.0, y: 0.0, z: 0.0 };
    /// let rotated = point.rotate_y(std::f32::consts::FRAC_PI_2);
    /// // For a 90° rotation (pi/2 radians), the point (1, 0, 0) becomes approximately (0, 0, -1).
    /// assert!(rotated.x.abs() < 1e-6);
    /// assert_eq!(rotated.y, 0.0);
    /// assert!((rotated.z + 1.0).abs() < 1e-6);
    /// ```
    fn rotate_y(self, angle: f32) -> Self {
        Point3D {
            x: self.x * angle.cos() + self.z * angle.sin(),
            y: self.y,
            z: -self.x * angle.sin() + self.z * angle.cos(),
        }
    }

    /// Rotates the point around the Z-axis by a given angle in radians.
    /// 
    /// This method applies a 2D rotation to the point's x and y coordinates while leaving the z coordinate unchanged.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// // Create a point at (1.0, 0.0, 5.0)
    /// let point = Point3D { x: 1.0, y: 0.0, z: 5.0 };
    /// // Rotate the point 90 degrees (π/2 radians)
    /// let rotated = point.rotate_z(std::f32::consts::FRAC_PI_2);
    /// 
    /// // After a 90° rotation, the point (1.0, 0.0) becomes approximately (0.0, 1.0)
    /// assert!((rotated.x).abs() < 1e-6);
    /// assert!((rotated.y - 1.0).abs() < 1e-6);
    /// assert_eq!(rotated.z, 5.0);
    /// ```
    fn rotate_z(self, angle: f32) -> Self {
        Point3D {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,
        }
    }

    /// Translates the point by adding the specified offsets to its x and y coordinates, leaving the z coordinate unchanged.
    ///
    /// Consumes the point and returns a new instance with the updated coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let translated_point = point.translate(5.0, -3.0);
    /// assert_eq!(translated_point.x, 6.0);
    /// assert_eq!(translated_point.y, -1.0);
    /// assert_eq!(translated_point.z, 3.0);
    /// ```
    fn translate(self, tx: f32, ty: f32) -> Self {
        Point3D {
            x: self.x + tx,
            y: self.y + ty,
            z: self.z,
        }
    }

    /// Projects the 3D point onto a 2D plane using a simple perspective transformation.
    /// 
    /// The computation adds an offset of 100.0 to both the x and y coordinates and adjusts them
    /// based on the point’s depth by dividing by (z + 4.0). The provided `scale` factor controls
    /// the degree of magnification for the x and y components.
    /// 
    /// # Examples
    ///
    /// ```
    /// // Assuming a Point3D struct with fields x, y, and z is defined
    /// let point = Point3D { x: 4.0, y: 4.0, z: 0.0 };
    /// let (proj_x, proj_y) = point.project(50.0);
    /// // The projection computes: 
    /// // proj_x = 100.0 + 50.0 * 4.0 / (0.0 + 4.0) = 150.0
    /// // proj_y = 100.0 + 50.0 * 4.0 / (0.0 + 4.0) = 150.0
    /// assert_eq!(proj_x, 150.0);
    /// assert_eq!(proj_y, 150.0);
    /// ```
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
/// Renders a 3D swinging cube component with animated transformations and a glow effect.
/// 
/// This component continuously animates its rotation, translation, and scaling using spring physics,
/// projecting 3D vertices onto a 2D plane to form an SVG representation of a cube. The SVG includes
/// dynamic gradient backgrounds, shadowed faces, and a glowing background circle, all updated in real-time.
/// 
/// # Examples
/// 
/// ```
/// // Within a Dioxus component:
/// rsx! {
///     div {
///         SwingingCube()
///     }
/// }
/// ```
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
