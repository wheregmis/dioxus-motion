use crate::{spring::Spring, tween::Tween};
use instant::Duration;

pub trait Animatable: Copy + 'static {
    fn zero() -> Self;
    fn epsilon() -> f32;
    fn magnitude(&self) -> f32;
    fn scale(&self, factor: f32) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn interpolate(&self, target: &Self, t: f32) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationMode {
    Tween(Tween),
    Spring(Spring),
}

impl Default for AnimationMode {
    fn default() -> Self {
        Self::Tween(Tween::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopMode {
    None,
    Infinite,
    Times(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AnimationConfig {
    pub mode: AnimationMode,
    pub loop_mode: Option<LoopMode>,
    pub delay: Duration, // Add delay field
}

impl AnimationConfig {
    pub fn new(mode: AnimationMode) -> Self {
        Self {
            mode,
            loop_mode: None,
            delay: Duration::default(),
        }
    }

    pub fn with_loop(mut self, loop_mode: LoopMode) -> Self {
        self.loop_mode = Some(loop_mode);
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
}
