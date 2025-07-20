//! `AnimationSequence<T>` - Optimized animation step sequences

use crate::animations::core::Animatable;
use crate::prelude::AnimationConfig;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone)]
pub struct AnimationStep<T: Animatable> {
    pub target: T,
    pub config: Arc<AnimationConfig>,
    pub predicted_next: Option<T>,
}

/// Optimized animation sequence that uses shared immutable data and atomic counters
/// to avoid cloning sequences on step transitions
pub struct AnimationSequence<T: Animatable> {
    /// Immutable shared steps - no cloning needed
    steps: Arc<[AnimationStep<T>]>,
    /// Atomic counter for current step - thread-safe without locks
    current_step: AtomicU8,
    /// Completion callback - thread-safe with Mutex to allow execution without ownership
    #[allow(clippy::type_complexity)]
    on_complete: Arc<Mutex<Option<Box<dyn FnOnce() + Send>>>>,
}

impl<T: Animatable> AnimationSequence<T> {
    /// Creates a new empty animation sequence
    pub fn new() -> Self {
        Self {
            steps: Arc::new([]),
            current_step: AtomicU8::new(0),
            on_complete: Arc::new(Mutex::new(None)),
        }
    }

    /// Creates a new animation sequence with specified capacity hint
    /// Note: This is kept for API compatibility but doesn't pre-allocate since we use Arc<\[T\]>
    pub fn with_capacity(_capacity: u8) -> Self {
        Self::new()
    }

    /// Creates a new animation sequence from a vector of steps
    pub fn from_steps(steps: Vec<AnimationStep<T>>) -> Self {
        Self {
            steps: steps.into(),
            current_step: AtomicU8::new(0),
            on_complete: Arc::new(Mutex::new(None)),
        }
    }

    /// Creates a new animation sequence with a completion callback
    pub fn with_on_complete<F>(steps: Vec<AnimationStep<T>>, on_complete: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            steps: steps.into(),
            current_step: AtomicU8::new(0),
            on_complete: Arc::new(Mutex::new(Some(Box::new(on_complete)))),
        }
    }

    /// Reserve additional capacity (kept for API compatibility, but no-op since we use Arc<\[T\]>)
    pub fn reserve(&mut self, _additional: u8) {
        // No-op for Arc<[T]> - kept for backward compatibility
    }

    /// Adds a new step to the sequence and returns a new sequence
    /// This creates a new Arc with all steps to maintain immutability
    pub fn then(self, target: T, config: AnimationConfig) -> Self {
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

        // Create new vector with existing steps plus the new one
        let mut new_steps: Vec<AnimationStep<T>> = self.steps.iter().cloned().collect();
        new_steps.push(new_step);

        Self {
            steps: new_steps.into(),
            current_step: AtomicU8::new(self.current_step.load(Ordering::Relaxed)),
            on_complete: self.on_complete,
        }
    }

    /// Sets a completion callback
    pub fn on_complete<F: FnOnce() + Send + 'static>(self, f: F) -> Self {
        if let Ok(mut guard) = self.on_complete.lock() {
            *guard = Some(Box::new(f));
        }
        self
    }

    /// Advances to the next step in the sequence
    /// Returns true if advanced, false if already at the end
    pub fn advance_step(&self) -> bool {
        let current = self.current_step.load(Ordering::Relaxed);
        let total_steps = self.steps.len() as u8;

        if current < total_steps.saturating_sub(1) {
            self.current_step.store(current + 1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    /// Gets the current step index
    pub fn current_step_index(&self) -> u8 {
        self.current_step.load(Ordering::Relaxed)
    }

    /// Gets the current step index (kept for backward compatibility)
    pub fn current_step(&self) -> u8 {
        self.current_step_index()
    }

    /// Gets the configuration for the current step
    pub fn current_config(&self) -> Option<&AnimationConfig> {
        let current = self.current_step.load(Ordering::Relaxed) as usize;
        self.steps.get(current).map(|step| step.config.as_ref())
    }

    /// Gets the target value for the current step
    pub fn current_target(&self) -> Option<T> {
        let current = self.current_step.load(Ordering::Relaxed) as usize;
        self.steps.get(current).map(|step| step.target)
    }

    /// Gets the current step data
    pub fn current_step_data(&self) -> Option<&AnimationStep<T>> {
        let current = self.current_step.load(Ordering::Relaxed) as usize;
        self.steps.get(current)
    }

    /// Gets all steps (for backward compatibility)
    pub fn steps(&self) -> &[AnimationStep<T>] {
        &self.steps
    }

    /// Checks if the sequence is complete (at the last step)
    pub fn is_complete(&self) -> bool {
        let current = self.current_step.load(Ordering::Relaxed);
        let total_steps = self.steps.len() as u8;
        current >= total_steps.saturating_sub(1)
    }

    /// Gets the total number of steps
    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }

    /// Resets the sequence to the first step
    pub fn reset(&self) {
        self.current_step.store(0, Ordering::Relaxed);
    }

    /// Executes the completion callback if present
    /// This method is thread-safe and can be called without ownership
    pub fn execute_completion(&self) {
        if let Ok(mut guard) = self.on_complete.lock() {
            if let Some(callback) = guard.take() {
                callback();
            }
        }
    }
}

impl<T: Animatable> Clone for AnimationSequence<T> {
    fn clone(&self) -> Self {
        Self {
            steps: self.steps.clone(), // Arc clone is cheap
            current_step: AtomicU8::new(self.current_step.load(Ordering::Relaxed)),
            on_complete: Arc::new(Mutex::new(None)), // Callbacks can't be cloned
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
