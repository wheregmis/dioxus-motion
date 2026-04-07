//! `AnimationSequence<T>` - Optimized animation step sequences

use crate::animations::core::Animatable;
use crate::prelude::AnimationConfig;

use std::sync::Mutex;
use std::sync::{Arc, MutexGuard};

#[derive(Clone)]
pub struct AnimationStep<T: Animatable> {
    pub target: T,
    pub config: Arc<AnimationConfig>,
    pub predicted_next: Option<T>,
}

struct SequenceState {
    current_step: u8,
    #[allow(clippy::type_complexity)]
    on_complete: Option<Box<dyn FnOnce() + Send>>,
}

/// Animation sequence that keeps step data simple and stores only the mutable
/// execution state behind a mutex for shared access.
pub struct AnimationSequence<T: Animatable> {
    steps: Vec<AnimationStep<T>>,
    state: Mutex<SequenceState>,
}

impl<T: Animatable> AnimationSequence<T> {
    fn lock_state(&self) -> MutexGuard<'_, SequenceState> {
        match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    /// Creates a new empty animation sequence
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            state: Mutex::new(SequenceState {
                current_step: 0,
                on_complete: None,
            }),
        }
    }

    /// Creates a new animation sequence with specified capacity hint.
    pub fn with_capacity(capacity: u8) -> Self {
        Self {
            steps: Vec::with_capacity(capacity as usize),
            state: Mutex::new(SequenceState {
                current_step: 0,
                on_complete: None,
            }),
        }
    }

    /// Creates a new animation sequence from a vector of steps
    pub fn from_steps(steps: Vec<AnimationStep<T>>) -> Self {
        Self {
            steps,
            state: Mutex::new(SequenceState {
                current_step: 0,
                on_complete: None,
            }),
        }
    }

    /// Creates a new animation sequence with a completion callback
    pub fn with_on_complete<F>(steps: Vec<AnimationStep<T>>, on_complete: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            steps,
            state: Mutex::new(SequenceState {
                current_step: 0,
                on_complete: Some(Box::new(on_complete)),
            }),
        }
    }

    /// Reserve additional capacity for future steps.
    pub fn reserve(&mut self, additional: u8) {
        self.steps.reserve(additional as usize);
    }

    /// Adds a new step to the sequence and returns a new sequence
    pub fn then(mut self, target: T, config: AnimationConfig) -> Self {
        let predicted_next = if self.steps.is_empty() {
            None
        } else {
            self.steps
                .last()
                .map(|last_step| last_step.target.interpolate(&target, 0.5))
        };

        let new_step = AnimationStep {
            target,
            config: Arc::new(config),
            predicted_next,
        };

        self.steps.push(new_step);
        self
    }

    /// Sets a completion callback
    pub fn on_complete<F: FnOnce() + Send + 'static>(self, f: F) -> Self {
        let mut state = self.lock_state();
        state.on_complete = Some(Box::new(f));
        drop(state);
        self
    }

    /// Advances to the next step in the sequence
    /// Returns true if advanced, false if already at the end
    pub fn advance_step(&self) -> bool {
        let mut state = self.lock_state();
        let current = state.current_step;
        let total_steps = self.steps.len() as u8;

        if current < total_steps.saturating_sub(1) {
            state.current_step += 1;
            true
        } else {
            false
        }
    }

    /// Gets the current step index
    pub fn current_step_index(&self) -> u8 {
        self.lock_state().current_step
    }

    /// Gets the current step index (kept for backward compatibility)
    pub fn current_step(&self) -> u8 {
        self.current_step_index()
    }

    /// Gets the configuration for the current step
    pub fn current_config(&self) -> Option<&AnimationConfig> {
        let current = self.current_step_index() as usize;
        self.steps.get(current).map(|step| step.config.as_ref())
    }

    /// Gets the target value for the current step
    pub fn current_target(&self) -> Option<T> {
        let current = self.current_step_index() as usize;
        self.steps.get(current).map(|step| step.target)
    }

    /// Gets the current step data
    pub fn current_step_data(&self) -> Option<&AnimationStep<T>> {
        let current = self.current_step_index() as usize;
        self.steps.get(current)
    }

    /// Gets all steps (for backward compatibility)
    pub fn steps(&self) -> &[AnimationStep<T>] {
        &self.steps
    }

    /// Checks if the sequence is complete (at the last step)
    pub fn is_complete(&self) -> bool {
        let current = self.current_step_index();
        let total_steps = self.steps.len() as u8;
        current >= total_steps.saturating_sub(1)
    }

    /// Gets the total number of steps
    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }

    /// Resets the sequence to the first step
    pub fn reset(&self) {
        self.lock_state().current_step = 0;
    }

    /// Executes the completion callback if present
    pub fn execute_completion(&self) {
        if let Some(callback) = self.lock_state().on_complete.take() {
            callback();
        }
    }
}

/// Cloning `AnimationSequence` preserves the queued steps and current_step_index,
/// but resets the inner `SequenceState::on_complete` callback to `None`.
/// Callers that clone an `AnimationSequence` must re-register `on_complete`
/// on the cloned instance when they need completion behavior there too.
impl<T: Animatable> Clone for AnimationSequence<T> {
    fn clone(&self) -> Self {
        let current_step = self.current_step_index();
        Self {
            steps: self.steps.clone(),
            state: Mutex::new(SequenceState {
                current_step,
                on_complete: None,
            }),
        }
    }
}

impl<T: Animatable> Default for AnimationSequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use crate::animations::core::AnimationMode;
    use crate::animations::spring::Spring;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_animation_sequence_basic() {
        let steps = vec![
            AnimationStep {
                target: 10.0f32,
                config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                    Spring::default(),
                ))),
                predicted_next: None,
            },
            AnimationStep {
                target: 20.0f32,
                config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                    Spring::default(),
                ))),
                predicted_next: None,
            },
            AnimationStep {
                target: 30.0f32,
                config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                    Spring::default(),
                ))),
                predicted_next: None,
            },
        ];

        let sequence = AnimationSequence::from_steps(steps);

        // Test initial state
        assert_eq!(sequence.current_step_index(), 0);
        assert_eq!(sequence.current_target().unwrap(), 10.0f32);
        assert!(!sequence.is_complete());
        assert_eq!(sequence.total_steps(), 3);

        // Test advancing steps
        assert!(sequence.advance_step());
        assert_eq!(sequence.current_step_index(), 1);
        assert_eq!(sequence.current_target().unwrap(), 20.0f32);
        assert!(!sequence.is_complete());

        assert!(sequence.advance_step());
        assert_eq!(sequence.current_step_index(), 2);
        assert_eq!(sequence.current_target().unwrap(), 30.0f32);
        assert!(sequence.is_complete());

        // Test can't advance past end
        assert!(!sequence.advance_step());
        assert_eq!(sequence.current_step_index(), 2);

        // Test reset
        sequence.reset();
        assert_eq!(sequence.current_step_index(), 0);
        assert!(!sequence.is_complete());
    }

    #[test]
    fn test_animation_sequence_builder_pattern() {
        let sequence = AnimationSequence::new()
            .then(
                10.0f32,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .then(
                20.0f32,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .then(
                30.0f32,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            );

        assert_eq!(sequence.total_steps(), 3);
        assert_eq!(sequence.current_target().unwrap(), 10.0f32);
        assert!(!sequence.is_complete());

        assert!(sequence.advance_step());
        assert_eq!(sequence.current_target().unwrap(), 20.0f32);

        assert!(sequence.advance_step());
        assert_eq!(sequence.current_target().unwrap(), 30.0f32);
        assert!(sequence.is_complete());
    }

    #[test]
    fn test_animation_sequence_with_callback() {
        let callback_executed = Arc::new(Mutex::new(false));
        let callback_executed_clone = callback_executed.clone();

        let steps = vec![AnimationStep {
            target: 10.0f32,
            config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                Spring::default(),
            ))),
            predicted_next: None,
        }];

        let sequence = AnimationSequence::with_on_complete(steps, move || {
            *callback_executed_clone.lock().unwrap() = true;
        });

        // Execute completion callback
        sequence.execute_completion();

        assert!(*callback_executed.lock().unwrap());
    }

    #[test]
    fn test_animation_sequence_callback_with_shared_references() {
        let callback_executed = Arc::new(Mutex::new(false));
        let callback_executed_clone = callback_executed.clone();

        let steps = vec![AnimationStep {
            target: 10.0f32,
            config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                Spring::default(),
            ))),
            predicted_next: None,
        }];

        let sequence = AnimationSequence::with_on_complete(steps, move || {
            *callback_executed_clone.lock().unwrap() = true;
        });

        // Create multiple Arc references to the sequence
        let sequence_arc1 = Arc::new(sequence);
        let sequence_arc2 = sequence_arc1.clone();
        let sequence_arc3 = sequence_arc1.clone();

        // Verify that Arc::try_unwrap would fail (multiple references exist)
        assert!(Arc::try_unwrap(sequence_arc1.clone()).is_err());

        // Execute completion callback through one of the references
        // This should work even though we can't get ownership
        sequence_arc1.execute_completion();

        // Verify the callback was executed
        assert!(*callback_executed.lock().unwrap());

        // Verify that other references still exist and are valid
        assert_eq!(sequence_arc2.current_step_index(), 0);
        assert_eq!(sequence_arc3.current_step_index(), 0);
    }

    #[test]
    fn test_animation_sequence_clone() {
        let steps = vec![AnimationStep {
            target: 10.0f32,
            config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                Spring::default(),
            ))),
            predicted_next: None,
        }];

        let sequence1 = AnimationSequence::from_steps(steps);
        sequence1.advance_step(); // This won't work since there's only one step, but let's test the clone

        let sequence2 = sequence1.clone();

        // Both sequences should have the same step data but independent counters
        assert_eq!(
            sequence1.current_step_index(),
            sequence2.current_step_index()
        );
        assert_eq!(sequence1.total_steps(), sequence2.total_steps());
        assert_eq!(sequence1.current_target(), sequence2.current_target());
    }

    #[test]
    fn test_animation_sequence_backward_compatibility() {
        // Test that the old API still works
        let sequence = AnimationSequence::new();
        let sequence = sequence.then(
            10.0f32,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        let sequence = sequence.then(
            20.0f32,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );

        // Test old method names
        assert_eq!(sequence.current_step(), 0);
        assert_eq!(sequence.steps().len(), 2);

        // Test with_capacity (should work but be a no-op)
        let _sequence_with_capacity = AnimationSequence::<f32>::with_capacity(10);

        // Test reserve (should work but be a no-op)
        let mut sequence_mut = sequence.clone();
        sequence_mut.reserve(5);
    }
}
