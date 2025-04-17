use std::sync::Arc;

use smallvec::SmallVec;

use super::utils::{Animatable, AnimationConfig};

#[derive(Clone)]
#[allow(dead_code)]
pub struct AnimationStep<T: Animatable> {
    pub target: T,
    pub config: Arc<AnimationConfig>,
    // Add predicted next state for smoother transitions
    pub predicted_next: Option<T>,
}

// Use a static array instead of Vec for small sequences
pub type AnimationSteps<T> = SmallVec<[AnimationStep<T>; 8]>;

pub struct AnimationSequence<T: Animatable> {
    pub steps: AnimationSteps<T>,
    pub current_step: u8,
    pub on_complete: Option<Box<dyn FnOnce()>>,
    // Add capacity hint for better allocation
    pub capacity_hint: u8,
}

impl<T: Animatable> Clone for AnimationSequence<T> {
    fn clone(&self) -> Self {
        Self {
            steps: self.steps.clone(),
            current_step: self.current_step,
            on_complete: None,
            capacity_hint: self.capacity_hint,
        }
    }
}

impl<T: Animatable> AnimationSequence<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: u8) -> Self {
        Self {
            steps: SmallVec::with_capacity(capacity as usize),
            current_step: 0,
            on_complete: None,
            capacity_hint: capacity,
        }
    }

    // Add method to reserve space upfront
    pub fn reserve(&mut self, additional: u8) {
        self.steps.reserve(additional as usize);
    }

    pub fn then(mut self, target: T, config: AnimationConfig) -> Self {
        let predicted_next = self
            .steps
            .last()
            .map(|last_step| last_step.target.interpolate(&target, 0.5));

        self.steps.push(AnimationStep {
            target,
            config: Arc::new(config),
            predicted_next,
        });
        self
    }

    pub fn on_complete<F: FnOnce() + 'static>(mut self, f: F) -> Self {
        self.on_complete = Some(Box::new(f));
        self
    }
}

impl<T: Animatable> Default for AnimationSequence<T> {
    fn default() -> Self {
        Self {
            steps: AnimationSteps::new(),
            current_step: 0,
            on_complete: None,
            capacity_hint: 0,
        }
    }
}
