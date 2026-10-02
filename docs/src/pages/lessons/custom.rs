use dioxus::prelude::*;
use dioxus_motion::{animations::core::Animatable, prelude::*};
use std::ops::{Add, Mul, Sub};

// Signed coordinates also represent displacement and velocity during a spring.
#[derive(Clone, Copy, Default, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}
impl Animatable for Point {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        Self {
            x: self.x.interpolate(&target.x, t),
            y: self.y.interpolate(&target.y, t),
        }
    }
    fn magnitude(&self) -> f32 {
        self.x.hypot(self.y)
    }
    fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

#[component]
pub fn CustomPointDemo() -> Element {
    let mut point = use_motion(Point::default())?;
    let mut forward = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let value = point.get_value();
    rsx! {
        div { class: "lesson-controls",
            div { style: "height:130px;padding:10px;background:#242b30;border-radius:12px",
                div { style: "width:24px;height:24px;border-radius:50%;background:#b9f078;transform:translate({value.x}px,{value.y}px)" }
            }
            output { "Point ({value.x:.1}, {value.y:.1})" }
            button { class: "button primary", onclick: move |_| {
                let target = if forward() { Point::default() } else { Point { x: 120.0, y: 70.0 } };
                match point.animate_to(target, AnimationConfig::spring(Spring::default())) {
                    Ok(()) => { forward.set(!forward()); error.set(None); }
                    Err(e) => error.set(Some(e.to_string())),
                }
            }, "Retarget point" }
            if let Some(message) = error() { p { role: "alert", {message} } }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_preserves_signed_arithmetic_and_validates_each_component() {
        let point = Point { x: -3.0, y: 4.0 };
        assert_eq!(point.magnitude(), 5.0);
        assert_eq!((point * -2.0).x, 6.0);
        assert_eq!((Point::default() - point).y, -4.0);
        assert!(
            !Point {
                x: f32::NAN,
                y: 0.0
            }
            .is_finite()
        );
        assert_eq!(
            Point {
                x: -f32::MAX,
                y: 0.0
            }
            .interpolate(
                &Point {
                    x: f32::MAX,
                    y: 0.0
                },
                0.5
            )
            .x,
            0.0
        );
    }
}
