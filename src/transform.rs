use crate::Animatable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotation: f32, // in radians
}

impl Transform {
    pub fn new(x: f32, y: f32, scale: f32, rotation: f32) -> Self {
        Self {
            x,
            y,
            scale,
            rotation,
        }
    }

    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            rotation: 0.0,
        }
    }
}

// Now let's implement Animatable for f32
impl Animatable for f32 {
    fn zero() -> Self {
        0.0
    }

    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        self.abs()
    }

    fn scale(&self, factor: f32) -> Self {
        self * factor
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        self + (target - self) * t
    }
}

// Implement Animatable for Transform
impl Animatable for Transform {
    fn zero() -> Self {
        Transform::new(0.0, 0.0, 0.0, 0.0)
    }

    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        (self.x * self.x
            + self.y * self.y
            + self.scale * self.scale
            + self.rotation * self.rotation)
            .sqrt()
    }

    fn scale(&self, factor: f32) -> Self {
        Transform::new(
            self.x * factor,
            self.y * factor,
            self.scale * factor,
            self.rotation * factor,
        )
    }

    fn add(&self, other: &Self) -> Self {
        Transform::new(
            self.x + other.x,
            self.y + other.y,
            self.scale + other.scale,
            self.rotation + other.rotation,
        )
    }

    fn sub(&self, other: &Self) -> Self {
        Transform::new(
            self.x - other.x,
            self.y - other.y,
            self.scale - other.scale,
            self.rotation - other.rotation,
        )
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        // Special handling for rotation to ensure shortest path
        let mut rotation_diff = target.rotation - self.rotation;
        if rotation_diff > std::f32::consts::PI {
            rotation_diff -= 2.0 * std::f32::consts::PI;
        } else if rotation_diff < -std::f32::consts::PI {
            rotation_diff += 2.0 * std::f32::consts::PI;
        }

        Transform::new(
            self.x + (target.x - self.x) * t,
            self.y + (target.y - self.y) * t,
            self.scale + (target.scale - self.scale) * t,
            self.rotation + rotation_diff * t,
        )
    }
}
