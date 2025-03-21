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
    /// Creates a new `PetalTransform` with the specified rotation, scale, and translation values.
    ///
    /// # Arguments
    ///
    /// * `rotate` - The rotation value.
    /// * `scale` - The scaling factor.
    /// * `translate_x` - The offset for horizontal translation.
    /// * `translate_y` - The offset for vertical translation.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = PetalTransform::new(30.0, 1.0, 5.0, 10.0);
    /// assert_eq!(transform.rotate, 30.0);
    /// assert_eq!(transform.scale, 1.0);
    /// assert_eq!(transform.translate_x, 5.0);
    /// assert_eq!(transform.translate_y, 10.0);
    /// ```
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
    /// Returns a new `PetalTransform` with all transformation values initialized to zero.
    ///
    /// This function provides a default state for a transformation, where the rotation, scaling,
    /// and both translation components are set to `0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = PetalTransform::zero();
    /// assert_eq!(transform.rotate, 0.0);
    /// assert_eq!(transform.scale, 0.0);
    /// assert_eq!(transform.translate_x, 0.0);
    /// assert_eq!(transform.translate_y, 0.0);
    /// ```
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Returns a small epsilon value for floating point precision comparisons.
    ///
    /// This value (0.001) can be used as a threshold when determining if two floating point numbers are nearly equal.
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

    /// Computes the Euclidean magnitude of the transform.
    ///
    /// The magnitude is calculated as the square root of the sum of the squares of
    /// the rotation, scale, and translation components.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = PetalTransform::new(3.0, 4.0, 0.0, 0.0);
    /// assert_eq!(transform.magnitude(), 5.0);
    /// ```
    fn magnitude(&self) -> f32 {
        (self.rotate * self.rotate
            + self.scale * self.scale
            + self.translate_x * self.translate_x
            + self.translate_y * self.translate_y)
            .sqrt()
    }

    /// Scales each transformation property by the given factor, returning a new scaled transform.
    /// 
    /// This method multiplies the `rotate`, `scale`, `translate_x`, and `translate_y` properties by the specified
    /// factor, producing a new `PetalTransform` with the updated values.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let transform = PetalTransform::new(1.0, 1.0, 1.0, 1.0);
    /// let scaled_transform = transform.scale(2.0);
    /// assert_eq!(scaled_transform.rotate, 2.0);
    /// assert_eq!(scaled_transform.scale, 2.0);
    /// assert_eq!(scaled_transform.translate_x, 2.0);
    /// assert_eq!(scaled_transform.translate_y, 2.0);
    /// ```
    fn scale(&self, factor: f32) -> Self {
        Self::new(
            self.rotate * factor,
            self.scale * factor,
            self.translate_x * factor,
            self.translate_y * factor,
        )
    }

    /// Returns a new `PetalTransform` instance representing the component-wise sum of two transforms.
    ///
    /// The resulting transform's fields are computed by adding the corresponding fields of `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// let t1 = PetalTransform::new(10.0, 1.5, 5.0, 2.5);
    /// let t2 = PetalTransform::new(15.0, 2.0, 3.0, 4.0);
    /// let sum = t1.add(&t2);
    /// assert_eq!(sum.rotate, 25.0);
    /// assert_eq!(sum.scale, 3.5);
    /// assert_eq!(sum.translate_x, 8.0);
    /// assert_eq!(sum.translate_y, 6.5);
    /// ```
    fn add(&self, other: &Self) -> Self {
        Self::new(
            self.rotate + other.rotate,
            self.scale + other.scale,
            self.translate_x + other.translate_x,
            self.translate_y + other.translate_y,
        )
    }

    /// Returns a new `PetalTransform` produced by subtracting each corresponding property of `other` from `self`.
    ///
    /// This method performs an element-wise subtraction on the `rotate`, `scale`, `translate_x`, and `translate_y` properties.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = PetalTransform::new(10.0, 5.0, 7.0, 3.0);
    /// let b = PetalTransform::new(3.0, 2.0, 4.0, 1.0);
    /// let result = a.sub(&b);
    /// assert_eq!(result.rotate, 7.0);
    /// assert_eq!(result.scale, 3.0);
    /// assert_eq!(result.translate_x, 3.0);
    /// assert_eq!(result.translate_y, 2.0);
    /// ```
    fn sub(&self, other: &Self) -> Self {
        Self::new(
            self.rotate - other.rotate,
            self.scale - other.scale,
            self.translate_x - other.translate_x,
            self.translate_y - other.translate_y,
        )
    }

    /// Linearly interpolates between this transform and a target transform.
    /// 
    /// Computes a weighted average of the transformation fields (`rotate`, `scale`, `translate_x`, `translate_y`)
    /// using the factor `t`. A factor of 0.0 returns this transform while 1.0 returns the target transform.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let start = PetalTransform::new(0.0, 1.0, 2.0, 3.0);
    /// let end = PetalTransform::new(90.0, 2.0, 4.0, 6.0);
    /// let mid = start.interpolate(&end, 0.5);
    /// 
    /// assert_eq!(mid.rotate, 45.0);
    /// assert_eq!(mid.scale, 1.5);
    /// assert_eq!(mid.translate_x, 3.0);
    /// assert_eq!(mid.translate_y, 4.5);
    /// ```
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
/// Renders an animated SVG flower component using the Dioxus framework.
///
/// This function creates an animated flower by initializing motion states for petals, leaves, center, and stem.
/// It triggers spring-based animations on component mount: first animating the leaves (and stem) to simulate growth,
/// then, once the leaves have fully grown, starting continuous motions for the petals and center circle.
/// The returned element is an SVG that dynamically displays the animated transformations.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
/// use your_crate::AnimatedFlower; // Update with the actual module path
///
/// fn main() {
///     dioxus::desktop::launch(App);
/// }
///
/// fn App(cx: Scope) -> Element {
///     AnimatedFlower()
/// }
/// ```
pub fn AnimatedFlower() -> Element {
    let mut petal_transform = use_motion(PetalTransform::zero());
    let mut leaf_transform = use_motion(PetalTransform::zero());
    let mut center_scale = use_motion(0.0f32);
    let mut is_leaves_grown = use_signal_sync(|| false);

    let mut stem_length = use_motion(100.0f32);

    let stem_transform = use_motion(PetalTransform::zero());

    let animate_leaves = move |_: Event<MountedData>| {
        stem_length.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 35.0,
                damping: 5.0,
                mass: 0.3,
                velocity: 0.0,
            })),
        );

        leaf_transform.animate_to(
            PetalTransform::new(
                PI / 6.0, // rotation
                1.0,      // initial scale
                0.0,      // x position
                -20.0,    // move up from bottom
            ),
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 40.0,
                damping: 5.0,
                mass: 0.3,
                velocity: 2.0, // increased initial velocity for upward motion
            }))
            .with_on_complete(move || {
                is_leaves_grown.set(true);
            }),
        );
    };

    let mut animate_petals = move || {
        if *is_leaves_grown.read() {
            petal_transform.animate_to(
                PetalTransform::new(PI / 4.0, 1.2, 3.0, 3.0),
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 60.0,
                    damping: 8.0,
                    mass: 0.5,
                    velocity: 1.0,
                }))
                .with_loop(LoopMode::Infinite),
            );

            center_scale.animate_to(
                1.2,
                AnimationConfig::new(AnimationMode::Spring(Spring {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    velocity: 0.0,
                }))
                .with_loop(LoopMode::Infinite),
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
            svg {
                width: "300",
                height: "300",
                view_box: "-50 -50 100 100",
                onmounted: animate_leaves,

                {
                    (0..8)
                        .map(|i| {
                            rsx! {
                                path {
                                    key: "leaf_{i}",
                                    d: "M 0 0 C 5 -3, 8 0, 5 5 C 8 0, 5 -3, 0 0",
                                    fill: "#48BB78",
                                    transform: "translate(0 {25.0 + leaf_transform.get_value().translate_y + (i as f32 * 5.0)})
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              rotate({-20.0 + (i as f32 * 15.0)}) 
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              scale({leaf_transform.get_value().scale})",
                                    opacity: "0.9",
                                }
                            }
                        })
                }


                // Enhanced stem with curve
                path {
                    d: "M 0 25 C -4 20, 4 15, -2 10 C 4 5, -4 0, 0 -2",
                    stroke: "#2F855A",
                    stroke_width: "1.2",
                    fill: "none",
                    stroke_dasharray: "100",
                    stroke_dashoffset: "{stem_length.get_value()}",
                    transform: "translate(0 {stem_transform.get_value().translate_y})",
                }

                circle {
                    cx: "0",
                    cy: "0",
                    r: "{2.5 * center_scale.get_value()}",
                    fill: "url(#center_gradient)",
                }

                // More petals with gradient
                {
                    (0..8)
                        .map(|i| {
                            let base_angle = (i as f32) * PI / 4.0;
                            let transform_value = petal_transform.get_value();
                            let hue = 340.0 + (i as f32 * 5.0);
                            rsx! {
                                path {
                                    key: "petal_{i}",
                                    d: "M 0 -1 C 3 -6, 6 -8, 0 -14 C -6 -8, -3 -6, 0 -1",
                                    fill: "hsl({hue}, 70%, 80%)",
                                    transform: "translate({transform_value.translate_x} {transform_value.translate_y})
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         rotate({(base_angle + transform_value.rotate) * 180.0 / PI}) 
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         scale({transform_value.scale})",
                                    opacity: "0.85",
                                }
                            }
                        })
                }
            }
        }
    }
}
