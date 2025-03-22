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
    /// Constructs a new `Transform3D` with the specified rotation, translation, and scale values.
    ///
    /// This function initializes a `Transform3D` instance using the provided rotation angles (in degrees)
    /// for the x, y, and z axes, translation offsets along the x and y axes, and a scaling factor.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = Transform3D::new(15.0, 30.0, 45.0, 10.0, 20.0, 1.5);
    /// assert_eq!(transform.rotate_x, 15.0);
    /// assert_eq!(transform.rotate_y, 30.0);
    /// assert_eq!(transform.rotate_z, 45.0);
    /// assert_eq!(transform.translate_x, 10.0);
    /// assert_eq!(transform.translate_y, 20.0);
    /// assert_eq!(transform.scale, 1.5);
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
    /// Returns a `Transform3D` representing an identity transformation.
    ///
    /// All rotation and translation components are set to zero, and the scale is set to 1.
    /// This is equivalent to calling `Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)`.
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

    /// Returns a small epsilon value for floating-point precision handling.
    ///
    /// This function returns a fixed value of 0.001 which can be used as a threshold for numerical comparisons.
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

    /// Computes the Euclidean magnitude of the transformation's components.
    ///
    /// The magnitude is calculated as the square root of the sum of the squares
    /// of all transformation parameters (rotations around the x, y, and z axes,
    /// translations along the x and y axes, and scaling).
    ///
    /// # Examples
    ///
    /// ```
    /// // Create a transformation with specific component values.
    /// let transform = Transform3D {
    ///     rotate_x: 3.0,
    ///     rotate_y: 4.0,
    ///     rotate_z: 0.0,
    ///     translate_x: 0.0,
    ///     translate_y: 0.0,
    ///     scale: 0.0,
    /// };
    ///
    /// // The magnitude should be 5.0 (i.e., sqrt(3^2 + 4^2)).
    /// assert_eq!(transform.magnitude(), 5.0);
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

    /// Scales the transformation by multiplying each component by the given factor.
    ///
    /// Returns a new `Transform3D` instance with its rotation, translation, and scaling values multiplied by `factor`.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = Transform3D::new(1.0, 2.0, 3.0, 4.0, 5.0, 2.0);
    /// let scaled = transform.scale(2.0);
    /// assert_eq!(scaled.rotate_x, 2.0);
    /// assert_eq!(scaled.rotate_y, 4.0);
    /// assert_eq!(scaled.rotate_z, 6.0);
    /// assert_eq!(scaled.translate_x, 8.0);
    /// assert_eq!(scaled.translate_y, 10.0);
    /// assert_eq!(scaled.scale, 4.0);
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

    /// Returns a new `Transform3D` instance with each transformation component equal to the sum of the corresponding components in `self` and `other`.
    ///
    /// This method combines the rotations, translations, and scales from two `Transform3D` values by performing
    /// element-wise addition on all transformation properties.
    ///
    /// # Examples
    ///
    /// ```
    /// let t1 = Transform3D::new(1.0, 2.0, 3.0, 4.0, 5.0, 1.0);
    /// let t2 = Transform3D::new(0.5, 1.0, 1.5, 2.0, 3.0, 2.0);
    /// let result = t1.add(&t2);
    ///
    /// assert_eq!(result.rotate_x, 1.5);
    /// assert_eq!(result.rotate_y, 3.0);
    /// assert_eq!(result.rotate_z, 4.5);
    /// assert_eq!(result.translate_x, 6.0);
    /// assert_eq!(result.translate_y, 8.0);
    /// assert_eq!(result.scale, 3.0);
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

    /// Subtracts the corresponding transformation components of another `Transform3D` from this one, returning a new instance representing the difference.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use your_crate::Transform3D;
    /// let a = Transform3D::new(10.0, 20.0, 30.0, 40.0, 50.0, 1.5);
    /// let b = Transform3D::new(5.0, 10.0, 15.0, 20.0, 25.0, 0.5);
    /// let result = a.sub(&b);
    /// assert_eq!(result.rotate_x, 5.0);
    /// assert_eq!(result.rotate_y, 10.0);
    /// assert_eq!(result.rotate_z, 15.0);
    /// assert_eq!(result.translate_x, 20.0);
    /// assert_eq!(result.translate_y, 25.0);
    /// assert_eq!(result.scale, 1.0);
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

    /// Interpolates between the current transformation and a target transformation using linear interpolation.
    ///
    /// Each component (rotation around x, y, and z; translation along x and y; and scale) is interpolated according to the formula:
    /// `self.value + (target.value - self.value) * t`.
    ///
    /// # Arguments
    ///
    /// * `target` - The transformation to interpolate towards.
    /// * `t` - The interpolation factor, where 0.0 returns the current transformation and 1.0 returns the target transformation.
    ///
    /// # Returns
    ///
    /// A new `Transform3D` representing the interpolated transformation.
    ///
    /// # Examples
    ///
    /// ```
    /// let start = Transform3D::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// let end = Transform3D::new(90.0, 90.0, 90.0, 10.0, 10.0, 2.0);
    /// let mid = start.interpolate(&end, 0.5);
    ///
    /// // Each component is halfway between the start and end values.
    /// assert_eq!(mid.rotate_x, 45.0);
    /// assert_eq!(mid.rotate_y, 45.0);
    /// assert_eq!(mid.rotate_z, 45.0);
    /// assert_eq!(mid.translate_x, 5.0);
    /// assert_eq!(mid.translate_y, 5.0);
    /// assert_eq!(mid.scale, 1.5);
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
    /// Rotates the 3D point around the X-axis by a given angle in radians.
    ///
    /// This produces a new `Point3D` with its Y and Z values rotated, while the X coordinate remains unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `Point3D` is in scope
    /// let point = Point3D { x: 0.0, y: 1.0, z: 0.0 };
    /// // Rotate 90 degrees (π/2 radians) around the X-axis.
    /// let rotated = point.rotate_x(std::f32::consts::FRAC_PI_2);
    ///
    /// // X remains unchanged; Y becomes 0 and Z becomes 1.
    /// assert!((rotated.x - 0.0).abs() < 1e-6);
    /// assert!((rotated.y - 0.0).abs() < 1e-6);
    /// assert!((rotated.z - 1.0).abs() < 1e-6);
    /// ```
    fn rotate_x(self, angle: f32) -> Self {
        Point3D {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }

    /// Rotates this point around the Y-axis by the specified angle (in radians).
    ///
    /// The rotation uses the standard Y-axis rotation matrix:
    /// 
    ///     x' = x * cos(angle) + z * sin(angle)
    ///     y' = y
    ///     z' = -x * sin(angle) + z * cos(angle)
    ///
    /// # Examples
    ///
    /// ```
    /// let point = Point3D { x: 1.0, y: 0.0, z: 0.0 };
    /// let rotated = point.rotate_y(std::f32::consts::FRAC_PI_2);
    /// // For a 90° rotation, x' should be ~0.0 and z' should be ~-1.0.
    /// assert!(rotated.x.abs() < 1e-6);
    /// assert!((rotated.z + 1.0).abs() < 1e-6);
    /// ```
    fn rotate_y(self, angle: f32) -> Self {
        Point3D {
            x: self.x * angle.cos() + self.z * angle.sin(),
            y: self.y,
            z: -self.x * angle.sin() + self.z * angle.cos(),
        }
    }

    /// Rotates this point around the z-axis by a given angle (in radians).
    ///
    /// The rotation applies the standard 2D rotation transformation to the x and y coordinates while leaving the z-coordinate unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f32::consts::FRAC_PI_2;
    ///
    /// let point = Point3D { x: 1.0, y: 0.0, z: 5.0 };
    /// let rotated = point.rotate_z(FRAC_PI_2);
    /// // After a 90° rotation, x is ~0 and y is ~1.
    /// assert!(rotated.x.abs() < f32::EPSILON);
    /// assert!((rotated.y - 1.0).abs() < f32::EPSILON);
    /// assert_eq!(rotated.z, 5.0);
    /// ```
    fn rotate_z(self, angle: f32) -> Self {
        Point3D {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,
        }
    }

    /// Translates the point by adding offsets to the x and y coordinates.
    /// 
    /// Returns a new `Point3D` with its x-coordinate increased by `tx` and its y-coordinate increased by `ty`.
    /// The z-coordinate remains unchanged.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let translated = point.translate(0.5, 1.5);
    /// assert_eq!(translated.x, 1.5);
    /// assert_eq!(translated.y, 3.5);
    /// assert_eq!(translated.z, 3.0);
    /// ```
    fn translate(self, tx: f32, ty: f32) -> Self {
        Point3D {
            x: self.x + tx,
            y: self.y + ty,
            z: self.z,
        }
    }

    /// Projects a 3D point onto a 2D plane using perspective division.
    ///
    /// This method scales the x and y coordinates by the given `scale` factor and applies
    /// a perspective effect by dividing these values by the point's z-coordinate offset by 4.0.
    /// An offset of 100.0 is then added to both coordinates to center the projected point.
    ///
    /// # Arguments
    ///
    /// * `scale` - The scaling factor applied to the x and y coordinates during projection.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming a Point3D struct with fields `x`, `y`, and `z`
    /// let point = Point3D { x: 4.0, y: 8.0, z: 2.0 };
    /// let (proj_x, proj_y) = point.project(50.0);
    /// assert_eq!(proj_x, 100.0 + 50.0 * 4.0 / (2.0 + 4.0));
    /// assert_eq!(proj_y, 100.0 + 50.0 * 8.0 / (2.0 + 4.0));
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
/// Renders an animated 3D swinging cube as a Dioxus component.
/// 
/// This function creates an SVG element that displays a continuously animated cube.
/// The cube's transformation state—including rotations, translations, and scaling—and its glowing
/// background are animated using spring physics in an infinite loop. The animation is triggered when
/// the component mounts, and the 3D vertices are projected into 2D coordinates for rendering.
/// 
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use your_crate::SwingingCube; // Adjust the import path as needed
/// 
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         div { class: "flex items-center justify-center p-8", SwingingCube() }
///     })
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
