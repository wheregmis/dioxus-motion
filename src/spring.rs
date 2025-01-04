#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub velocity: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SpringState {
    Active,
    Completed,
}
