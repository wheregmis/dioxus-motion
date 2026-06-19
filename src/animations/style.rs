//! Animatable CSS style type for opacity, transforms, and arbitrary CSS properties.

use std::{collections::BTreeMap, fmt};

use crate::animations::{
    core::Animatable,
    css::{CssValue, IntoCssValue},
};

fn normalize_style_property(property: &str) -> String {
    let mut normalized = String::new();

    for (index, character) in property.chars().enumerate() {
        if character == '_' {
            normalized.push('-');
        } else if character.is_ascii_uppercase() {
            if index > 0 {
                normalized.push('-');
            }
            normalized.push(character.to_ascii_lowercase());
        } else {
            normalized.push(character);
        }
    }

    normalized
}

fn merge_style_properties(
    left: &BTreeMap<String, CssValue>,
    right: &BTreeMap<String, CssValue>,
    merge: impl Fn(CssValue, CssValue) -> CssValue,
) -> BTreeMap<String, CssValue> {
    let mut properties = BTreeMap::new();

    for (property, left_value) in left {
        if let Some(right_value) = right.get(property) {
            properties.insert(
                property.clone(),
                merge(left_value.clone(), right_value.clone()),
            );
        } else {
            properties.insert(property.clone(), left_value.clone());
        }
    }

    for (property, right_value) in right {
        properties
            .entry(property.clone())
            .or_insert_with(|| right_value.clone());
    }

    properties
}

/// Animatable CSS style value: opacity, transforms, and arbitrary typed CSS properties.
#[derive(Debug, Clone, PartialEq)]
pub struct MotionStyle {
    /// Element opacity.
    pub opacity: f32,
    /// X translation in pixels.
    pub x: f32,
    /// Y translation in pixels.
    pub y: f32,
    /// Z translation in pixels.
    pub z: f32,
    /// Uniform scale.
    pub scale: f32,
    /// X-axis scale.
    pub scale_x: f32,
    /// Y-axis scale.
    pub scale_y: f32,
    /// Z-axis scale.
    pub scale_z: f32,
    /// Rotation in degrees.
    pub rotate: f32,
    /// X-axis rotation in degrees.
    pub rotate_x: f32,
    /// Y-axis rotation in degrees.
    pub rotate_y: f32,
    /// Z-axis rotation in degrees.
    pub rotate_z: f32,
    /// Uniform skew in degrees.
    pub skew: f32,
    /// X-axis skew in degrees.
    pub skew_x: f32,
    /// Y-axis skew in degrees.
    pub skew_y: f32,
    /// Transform perspective in pixels. A value of 0 omits perspective.
    pub perspective: f32,
    /// Additional animated CSS properties keyed by kebab-case CSS property name.
    pub properties: BTreeMap<String, CssValue>,
}

impl MotionStyle {
    /// Creates a style with the given opacity and identity transform values.
    pub fn new(opacity: f32) -> Self {
        Self {
            opacity,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            scale: 1.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,
            rotate: 0.0,
            rotate_x: 0.0,
            rotate_y: 0.0,
            rotate_z: 0.0,
            skew: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
            perspective: 0.0,
            properties: BTreeMap::new(),
        }
    }

    /// Sets the X translation in pixels.
    pub fn x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    /// Sets the Y translation in pixels.
    pub fn y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    /// Sets the Z translation in pixels.
    pub fn z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    /// Sets the uniform scale.
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Sets the X-axis scale.
    pub fn scale_x(mut self, scale_x: f32) -> Self {
        self.scale_x = scale_x;
        self
    }

    /// Sets the Y-axis scale.
    pub fn scale_y(mut self, scale_y: f32) -> Self {
        self.scale_y = scale_y;
        self
    }

    /// Sets the Z-axis scale.
    pub fn scale_z(mut self, scale_z: f32) -> Self {
        self.scale_z = scale_z;
        self
    }

    /// Sets the rotation in degrees.
    pub fn rotate(mut self, rotate: f32) -> Self {
        self.rotate = rotate;
        self
    }

    /// Sets the X-axis rotation in degrees.
    pub fn rotate_x(mut self, rotate_x: f32) -> Self {
        self.rotate_x = rotate_x;
        self
    }

    /// Sets the Y-axis rotation in degrees.
    pub fn rotate_y(mut self, rotate_y: f32) -> Self {
        self.rotate_y = rotate_y;
        self
    }

    /// Sets the Z-axis rotation in degrees.
    pub fn rotate_z(mut self, rotate_z: f32) -> Self {
        self.rotate_z = rotate_z;
        self
    }

    /// Sets the uniform skew in degrees.
    pub fn skew(mut self, skew: f32) -> Self {
        self.skew = skew;
        self
    }

    /// Sets the X-axis skew in degrees.
    pub fn skew_x(mut self, skew_x: f32) -> Self {
        self.skew_x = skew_x;
        self
    }

    /// Sets the Y-axis skew in degrees.
    pub fn skew_y(mut self, skew_y: f32) -> Self {
        self.skew_y = skew_y;
        self
    }

    /// Sets transform perspective in pixels.
    pub fn perspective(mut self, perspective: f32) -> Self {
        self.perspective = perspective;
        self
    }

    /// Sets an animated CSS property by value type.
    pub fn property(mut self, property: impl Into<String>, value: CssValue) -> Self {
        let property = normalize_style_property(&property.into());
        self.properties.insert(property, value);
        self
    }

    /// Adds an animated CSS property by inferring its value type from the property name.
    ///
    /// Numeric values, lengths, hex colors, `rgb()`/`rgba()`, `hsl()`/`hsla()`, and compatible
    /// complex strings are interpolated. Other CSS strings are preserved as discrete values.
    pub fn add_css_property(&mut self, property: &str, value: impl IntoCssValue) -> &mut Self {
        let property = normalize_style_property(property);
        let value = value.into_css_value(&property);
        self.properties.insert(property, value);
        self
    }

    /// Formats the style as CSS declarations.
    pub fn to_css(&self) -> String {
        self.to_string()
    }
}

impl Default for MotionStyle {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl std::ops::Add for MotionStyle {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            opacity: self.opacity + other.opacity,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            scale: self.scale + other.scale,
            scale_x: self.scale_x + other.scale_x,
            scale_y: self.scale_y + other.scale_y,
            scale_z: self.scale_z + other.scale_z,
            rotate: self.rotate + other.rotate,
            rotate_x: self.rotate_x + other.rotate_x,
            rotate_y: self.rotate_y + other.rotate_y,
            rotate_z: self.rotate_z + other.rotate_z,
            skew: self.skew + other.skew,
            skew_x: self.skew_x + other.skew_x,
            skew_y: self.skew_y + other.skew_y,
            perspective: self.perspective + other.perspective,
            properties: merge_style_properties(&self.properties, &other.properties, |a, b| {
                a.add(&b).unwrap_or(b)
            }),
        }
    }
}

impl std::ops::Sub for MotionStyle {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            opacity: self.opacity - other.opacity,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            scale: self.scale - other.scale,
            scale_x: self.scale_x - other.scale_x,
            scale_y: self.scale_y - other.scale_y,
            scale_z: self.scale_z - other.scale_z,
            rotate: self.rotate - other.rotate,
            rotate_x: self.rotate_x - other.rotate_x,
            rotate_y: self.rotate_y - other.rotate_y,
            rotate_z: self.rotate_z - other.rotate_z,
            skew: self.skew - other.skew,
            skew_x: self.skew_x - other.skew_x,
            skew_y: self.skew_y - other.skew_y,
            perspective: self.perspective - other.perspective,
            properties: merge_style_properties(&self.properties, &other.properties, |a, b| {
                a.sub(&b).unwrap_or(b)
            }),
        }
    }
}

impl std::ops::Mul<f32> for MotionStyle {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Self {
            opacity: self.opacity * factor,
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            scale: self.scale * factor,
            scale_x: self.scale_x * factor,
            scale_y: self.scale_y * factor,
            scale_z: self.scale_z * factor,
            rotate: self.rotate * factor,
            rotate_x: self.rotate_x * factor,
            rotate_y: self.rotate_y * factor,
            rotate_z: self.rotate_z * factor,
            skew: self.skew * factor,
            skew_x: self.skew_x * factor,
            skew_y: self.skew_y * factor,
            perspective: self.perspective * factor,
            properties: self
                .properties
                .iter()
                .map(|(property, value)| (property.clone(), value.scale(factor)))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animations::css::{CssColor, CssValue};

    fn approx_eq(left: f32, right: f32) -> bool {
        (left - right).abs() < 1e-4
    }

    fn assert_color(value: Option<&CssValue>, red: f32, green: f32, blue: f32, alpha: f32) {
        let Some(CssValue::Color(color)) = value else {
            panic!("expected color value, got {value:?}");
        };

        assert!(approx_eq(color.red, red), "red was {}", color.red);
        assert!(approx_eq(color.green, green), "green was {}", color.green);
        assert!(approx_eq(color.blue, blue), "blue was {}", color.blue);
        assert!(approx_eq(color.alpha, alpha), "alpha was {}", color.alpha);
    }

    #[test]
    fn add_css_property_normalizes_and_parses_colors() {
        let mut style = MotionStyle::default();
        style.add_css_property("background_color", "#0000ff");
        style.add_css_property("borderColor", "rgba(255, 0, 0, 0.5)");

        assert_color(
            style.properties.get("background-color"),
            0.0,
            0.0,
            255.0,
            1.0,
        );
        assert_color(style.properties.get("border-color"), 255.0, 0.0, 0.0, 0.5);
    }

    #[test]
    fn property_normalizes_keys() {
        let style = MotionStyle::default().property("backgroundColor", CssValue::Px(12.0));

        assert_eq!(
            style.properties.get("background-color"),
            Some(&CssValue::Px(12.0))
        );
        assert!(!style.properties.contains_key("backgroundColor"));
    }

    #[test]
    fn interpolate_blends_motion_style_colors() {
        let mut start = MotionStyle::default();
        start.add_css_property("color", "#000000");
        start.add_css_property("background_color", "#0000ff");

        let mut target = MotionStyle::default();
        target.add_css_property("color", "#ffffff");
        target.add_css_property("background_color", "#00ff00");

        let mid = start.interpolate(&target, 0.5);

        assert_color(mid.properties.get("color"), 127.5, 127.5, 127.5, 1.0);
        assert_color(
            mid.properties.get("background-color"),
            0.0,
            127.5,
            127.5,
            1.0,
        );
    }

    #[test]
    fn interpolate_preserves_properties_missing_from_target() {
        let start = MotionStyle::default().property("borderWidth", CssValue::Px(2.0));
        let target = MotionStyle::default();

        let mid = start.interpolate(&target, 0.5);

        assert_eq!(mid.properties.get("border-width"), Some(&CssValue::Px(2.0)));
    }

    #[test]
    fn vector_math_blends_colors_for_spring_updates() {
        let mut start = MotionStyle::default();
        start.add_css_property("background_color", "#000000");

        let mut target = MotionStyle::default();
        target.add_css_property("background_color", "#ffffff");

        let halfway = start.clone() + (target - start) * 0.5;

        assert_color(
            halfway.properties.get("background-color"),
            127.5,
            127.5,
            127.5,
            1.0,
        );
    }

    #[test]
    fn vector_math_preserves_non_pixel_units() {
        let mut start = MotionStyle::default();
        start.add_css_property("width", "10%");

        let mut target = MotionStyle::default();
        target.add_css_property("width", "90%");

        let halfway = start.clone() + (target - start) * 0.5;

        assert_eq!(
            halfway.properties.get("width"),
            Some(&CssValue::Percent(50.0))
        );
    }

    #[test]
    fn vector_math_blends_compatible_complex_css_values() {
        let mut start = MotionStyle::default();
        start.add_css_property("box_shadow", "0px 10px 20px rgba(0, 0, 0, 0.2)");

        let mut target = MotionStyle::default();
        target.add_css_property("box_shadow", "0px 30px 60px rgba(100, 150, 200, 0.6)");

        let halfway = start.clone() + (target - start) * 0.5;

        assert_eq!(
            halfway
                .properties
                .get("box-shadow")
                .map(CssValue::to_css)
                .as_deref(),
            Some("0px 20px 40px rgba(50, 75, 100, 0.4)")
        );
    }

    #[test]
    fn color_css_output_is_clamped_for_spring_overshoot() {
        assert_eq!(
            CssValue::Color(CssColor {
                red: 300.0,
                green: -20.0,
                blue: 128.0,
                alpha: 1.4,
            })
            .to_css(),
            "rgba(255, 0, 128, 1)"
        );
    }

    #[test]
    fn motion_style_macro_builds_color_properties() {
        let style = crate::motion_style! {
            color: "#dbeafe",
            background_color: "#2563eb",
            borderColor: "#60a5fa",
        };

        assert_color(style.properties.get("color"), 219.0, 234.0, 254.0, 1.0);
        assert_color(
            style.properties.get("background-color"),
            37.0,
            99.0,
            235.0,
            1.0,
        );
        assert_color(
            style.properties.get("border-color"),
            96.0,
            165.0,
            250.0,
            1.0,
        );
    }
}

impl Animatable for MotionStyle {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let mut style = self.clone() + (target.clone() - self.clone()) * t;

        for (property, target_value) in &target.properties {
            if let Some(current_value) = self.properties.get(property) {
                style
                    .properties
                    .insert(property.clone(), current_value.interpolate(target_value, t));
            } else {
                style
                    .properties
                    .insert(property.clone(), target_value.clone());
            }
        }

        for (property, current_value) in &self.properties {
            if !target.properties.contains_key(property) {
                style
                    .properties
                    .insert(property.clone(), current_value.clone());
            }
        }

        style
    }

    fn magnitude(&self) -> f32 {
        let property_magnitude: f32 = self
            .properties
            .values()
            .map(|value| value.number() * value.number())
            .sum();

        (self.opacity * self.opacity
            + self.x * self.x
            + self.y * self.y
            + self.z * self.z
            + self.scale * self.scale
            + self.scale_x * self.scale_x
            + self.scale_y * self.scale_y
            + self.scale_z * self.scale_z
            + self.rotate * self.rotate
            + self.rotate_x * self.rotate_x
            + self.rotate_y * self.rotate_y
            + self.rotate_z * self.rotate_z
            + self.skew * self.skew
            + self.skew_x * self.skew_x
            + self.skew_y * self.skew_y
            + self.perspective * self.perspective
            + property_magnitude)
            .sqrt()
    }
}

impl fmt::Display for MotionStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let perspective = if self.perspective > 0.0 {
            format!(" perspective({}px)", self.perspective)
        } else {
            String::new()
        };

        write!(
            formatter,
            "opacity: {}; transform:{} translateX({}px) translateY({}px) translateZ({}px) scale({}) scaleX({}) scaleY({}) scaleZ({}) rotate({}deg) rotateX({}deg) rotateY({}deg) rotateZ({}deg) skew({}deg) skewX({}deg) skewY({}deg)",
            self.opacity,
            perspective,
            self.x,
            self.y,
            self.z,
            self.scale,
            self.scale_x,
            self.scale_y,
            self.scale_z,
            self.rotate,
            self.rotate_x,
            self.rotate_y,
            self.rotate_z,
            self.skew,
            self.skew_x,
            self.skew_y
        )?;

        for (property, value) in &self.properties {
            write!(formatter, "; {property}: {}", value.to_css())?;
        }

        Ok(())
    }
}
