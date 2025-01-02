#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotate: f32,
    pub opacity: f32,
}

// Implement conversion traits needed for animation
impl From<f32> for Transform {
    fn from(value: f32) -> Self {
        Transform {
            x: value,
            y: value,
            scale: value,
            rotate: value,
            opacity: value,
        }
    }
}

impl From<Transform> for f32 {
    fn from(val: Transform) -> Self {
        // You can choose which property to use for interpolation
        // Here we're using x, but you could use any property
        val.x
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            rotate: 0.0,
            opacity: 1.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

// Implement conversion traits needed for animation
impl From<f32> for Color {
    fn from(value: f32) -> Self {
        Color {
            r: value,
            g: value,
            b: value,
        }
    }
}

impl From<Color> for f32 {
    fn from(val: Color) -> Self {
        // Using luminance formula for RGB to single value conversion
        0.299 * val.r + 0.587 * val.g + 0.114 * val.b
    }
}
