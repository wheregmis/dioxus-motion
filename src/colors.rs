use crate::animations::Animatable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        )
    }
}

// Implement Animatable for Color
impl Animatable for Color {
    fn zero() -> Self {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        (self.r * self.r + self.g * self.g + self.b * self.b + self.a * self.a).sqrt()
    }

    fn scale(&self, factor: f32) -> Self {
        Color::new(
            self.r * factor,
            self.g * factor,
            self.b * factor,
            self.a * factor,
        )
    }

    fn add(&self, other: &Self) -> Self {
        Color::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
            self.a + other.a,
        )
    }

    fn sub(&self, other: &Self) -> Self {
        Color::new(
            self.r - other.r,
            self.g - other.g,
            self.b - other.b,
            self.a - other.a,
        )
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let lerp = |start: f32, end: f32, t: f32| -> f32 { start + (end - start) * t };

        Color::new(
            lerp(self.r, target.r, t),
            lerp(self.g, target.g, t),
            lerp(self.b, target.b, t),
            lerp(self.a, target.a, t),
        )
    }
}
