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
    /// This constructor initializes a new instance of `PetalTransform` using the provided parameters:
    /// - `rotate`: The rotation angle in radians.
    /// - `scale`: The scaling factor.
    /// - `translate_x`: The horizontal translation value.
    /// - `translate_y`: The vertical translation value.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = PetalTransform::new(1.57, 1.0, 5.0, 3.0);
    /// assert_eq!(transform.rotate, 1.57);
    /// assert_eq!(transform.scale, 1.0);
    /// assert_eq!(transform.translate_x, 5.0);
    /// assert_eq!(transform.translate_y, 3.0);
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
    /// Returns a `PetalTransform` with all transformation parameters set to zero.
    ///
    /// This function provides a neutral baseline where no rotation, scaling, or translation
    /// is applied, making it useful as a starting point for further transformations.
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

    /// Returns a small epsilon value (0.001) used as a precision threshold.
    ///
    /// # Examples
    ///
    /// ```
    /// let eps = epsilon();
    /// assert!((eps - 0.001).abs() < std::f32::EPSILON);
    /// ```
    fn epsilon() -> f32 {
        0.001
    }

    /// Computes the Euclidean magnitude of the transformation.
    ///
    /// The magnitude is calculated as the square root of the sum of squares of the rotation, scale,
    /// horizontal translation, and vertical translation components. This value reflects the overall
    /// intensity of the transformation.
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

    /// Returns a new transformation with each property scaled by the specified factor.
    ///
    /// The method multiplies the transformation's fields (`rotate`, `scale`, `translate_x`, and `translate_y`)
    /// by the given factor, producing a new instance with the scaled values.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform = PetalTransform::new(1.0, 2.0, 3.0, 4.0);
    /// let scaled = transform.scale(2.0);
    /// assert_eq!(scaled.rotate, 2.0);
    /// assert_eq!(scaled.scale, 4.0);
    /// assert_eq!(scaled.translate_x, 6.0);
    /// assert_eq!(scaled.translate_y, 8.0);
    /// ```
    fn scale(&self, factor: f32) -> Self {
        Self::new(
            self.rotate * factor,
            self.scale * factor,
            self.translate_x * factor,
            self.translate_y * factor,
        )
    }

    /// Returns a new `PetalTransform` by component-wise adding the corresponding transformation values of `self` and `other`.
    ///
    /// This operation sums the rotation, scale, and translation values (both horizontal and vertical) of the two transforms,
    /// resulting in a combined transformation.
    ///
    /// # Examples
    ///
    /// ```
    /// let transform1 = PetalTransform::new(1.0, 2.0, 3.0, 4.0);
    /// let transform2 = PetalTransform::new(0.5, 1.0, 1.5, 2.0);
    /// let combined = transform1.add(&transform2);
    /// assert_eq!(combined.rotate, 1.5);
    /// assert_eq!(combined.scale, 3.0);
    /// assert_eq!(combined.translate_x, 4.5);
    /// assert_eq!(combined.translate_y, 6.0);
    /// ```
    fn add(&self, other: &Self) -> Self {
        Self::new(
            self.rotate + other.rotate,
            self.scale + other.scale,
            self.translate_x + other.translate_x,
            self.translate_y + other.translate_y,
        )
    }

    /// Subtracts the corresponding components of another `PetalTransform` from this one.
    ///
    /// Returns a new `PetalTransform` where each field is the difference between the fields of `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// let t1 = PetalTransform::new(1.0, 2.0, 3.0, 4.0);
    /// let t2 = PetalTransform::new(0.5, 1.0, 1.5, 2.0);
    /// let result = t1.sub(&t2);
    /// assert_eq!(result.rotate, 0.5);
    /// assert_eq!(result.scale, 1.0);
    /// assert_eq!(result.translate_x, 1.5);
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
    /// Computes each field (rotate, scale, translate_x, and translate_y) by linearly
    /// interpolating between the current and target values using the factor `t`. When
    /// `t` is 0.0 the result is equal to the current transform, and when `t` is 1.0
    /// the result is equal to the target transform.
    ///
    /// # Examples
    ///
    /// ```
    /// let start = PetalTransform::new(0.0, 1.0, 0.0, 0.0);
    /// let target = PetalTransform::new(3.14, 2.0, 5.0, 5.0);
    /// let result = start.interpolate(&target, 0.5);
    /// // result.rotate should be approximately 1.57 (halfway between 0.0 and 3.14)
    /// assert!((result.rotate - 1.57).abs() < 0.01);
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
/// Renders an animated flower component using Dioxus.  
///  
/// This component creates and animates various parts of a flower—stem, leaves, petals, and center—using spring physics.
/// On mounting, it triggers an animation for the stem and leaves, and once the leaves have grown, it starts continuous,
/// looping animations for the petals and the flower's center scale. The returned SVG element visually represents the animated flower.
///  
/// # Examples
///  
/// ```no_run
/// use dioxus::prelude::*;
/// use your_crate::AnimatedFlower;
///  
/// fn main() {
///     dioxus::desktop::launch(App);
/// }
///  
/// fn App(cx: Scope) -> Element {
///     cx.render(rsx! {
///         AnimatedFlower()
///     })
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
