//! `AnimationSequence<T>` - Optimized animation step sequences

use crate::animations::core::{
    Animatable, AnimationError, validate_spring_transition, validate_value,
};
use crate::prelude::{AnimationConfig, LoopMode};

use std::sync::Mutex;
use std::sync::{Arc, MutexGuard};

#[derive(Clone)]
pub struct AnimationStep<T: Animatable> {
    pub target: T,
    pub config: Arc<AnimationConfig>,
}

struct SequenceState {
    current_step: usize,
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
    /// Validates targets, configurations, and transitions whose starting values are known.
    /// Motion setup also checks transitions using the actual initial value.
    pub fn validate(&self) -> Result<(), AnimationError> {
        self.validate_from(None)
    }

    pub(crate) fn validate_from<'a>(
        &'a self,
        mut current: Option<&'a T>,
    ) -> Result<(), AnimationError> {
        for step in &self.steps {
            step.config.validate_for::<T>()?;
            validate_value(&step.target, "sequence target")?;
            if let Some(current) = current {
                validate_spring_transition(current, &step.target, step.config.mode)?;
            }
            // A finite round trip finishes at its starting value; zero counts play one leg.
            if !matches!(step.config.loop_mode, Some(LoopMode::AlternateTimes(1..))) {
                current = Some(&step.target);
            }
        }
        Ok(())
    }

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
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            steps: Vec::with_capacity(capacity),
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
    pub fn reserve(&mut self, additional: usize) {
        self.steps.reserve(additional);
    }

    /// Adds a step with its own delay, loop mode, and completion callback.
    /// Infinite loops hold the sequence at that step. Step callbacks run once after
    /// all of its legs, before the overall sequence callback.
    pub fn then(mut self, target: T, config: AnimationConfig) -> Self {
        let new_step = AnimationStep {
            target,
            config: Arc::new(config),
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
        let total_steps = self.steps.len();

        if current < total_steps.saturating_sub(1) {
            state.current_step += 1;
            true
        } else {
            false
        }
    }

    /// Gets the current step index
    pub fn current_step_index(&self) -> usize {
        self.lock_state().current_step
    }

    /// Alias for the current step index.
    pub fn current_step(&self) -> usize {
        self.current_step_index()
    }

    /// Gets the configuration for the current step
    pub fn current_config(&self) -> Option<&AnimationConfig> {
        let current = self.current_step_index();
        self.steps.get(current).map(|step| step.config.as_ref())
    }

    /// Gets the target value for the current step
    pub fn current_target(&self) -> Option<T> {
        let current = self.current_step_index();
        self.steps.get(current).map(|step| step.target.clone())
    }

    /// Gets the current step data
    pub fn current_step_data(&self) -> Option<&AnimationStep<T>> {
        let current = self.current_step_index();
        self.steps.get(current)
    }

    /// Gets all steps (for backward compatibility)
    pub fn steps(&self) -> &[AnimationStep<T>] {
        &self.steps
    }

    /// Checks if the sequence is complete (at the last step)
    pub fn is_complete(&self) -> bool {
        let current = self.current_step_index();
        let total_steps = self.steps.len();
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
        let callback = self.take_completion();
        if let Some(callback) = callback {
            callback();
        }
    }

    pub(crate) fn take_completion(&self) -> Option<Box<dyn FnOnce() + Send>> {
        self.lock_state().on_complete.take()
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
    fn sequence_indices_follow_vector_length() {
        for count in [0, 1, 2, 254, 255, 256, 257, 512, 1024] {
            let steps = (0..count)
                .map(|index| AnimationStep {
                    target: index as f32,
                    config: Arc::new(AnimationConfig::tween_ms(index as u64 + 1)),
                })
                .collect();
            let sequence = AnimationSequence::from_steps(steps);
            assert_eq!(sequence.total_steps(), count);
            if count == 0 {
                assert!(sequence.is_complete());
                assert!(!sequence.advance_step());
                assert!(sequence.current_target().is_none());
                assert!(sequence.current_config().is_none());
                assert!(sequence.current_step_data().is_none());
                continue;
            }
            for index in 0..count {
                assert_eq!(sequence.current_step_index(), index);
                assert_eq!(sequence.current_step(), index);
                assert_eq!(sequence.current_target(), Some(index as f32));
                assert_eq!(sequence.current_step_data().unwrap().target, index as f32);
                assert_eq!(
                    sequence.current_config().unwrap().get_duration(),
                    crate::Duration::from_millis(index as u64 + 1),
                );
                assert_eq!(sequence.is_complete(), index + 1 == count);
                assert_eq!(sequence.advance_step(), index + 1 < count);
            }
            sequence.reset();
            assert_eq!(sequence.current_target(), Some(0.0));
        }
    }

    #[test]
    fn completion_releases_lock_and_runs_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let sequence = Arc::new(AnimationSequence::<f32>::new());
        let weak = Arc::downgrade(&sequence);
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        sequence.lock_state().on_complete = Some(Box::new(move || {
            let sequence = weak.upgrade().unwrap();
            assert!(
                sequence.state.try_lock().is_ok(),
                "callback still holds state lock"
            );
            sequence.reset();
            sequence.execute_completion();
            callback_calls.fetch_add(1, Ordering::Relaxed);
        }));
        sequence.execute_completion();
        sequence.execute_completion();
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_animation_sequence_basic() {
        let steps = vec![
            AnimationStep {
                target: 10.0f32,
                config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                    Spring::default(),
                ))),
            },
            AnimationStep {
                target: 20.0f32,
                config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                    Spring::default(),
                ))),
            },
            AnimationStep {
                target: 30.0f32,
                config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
                    Spring::default(),
                ))),
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
        use std::sync::atomic::{AtomicUsize, Ordering};
        let calls = Arc::new(AtomicUsize::new(0));
        let completed = calls.clone();
        let original = AnimationSequence::new()
            .then(10.0f32, AnimationConfig::tween_ms(1000))
            .then(20.0, AnimationConfig::tween_ms(1000))
            .then(30.0, AnimationConfig::tween_ms(1000))
            .on_complete(move || {
                completed.fetch_add(1, Ordering::Relaxed);
            });
        assert!(original.advance_step());
        let cloned = original.clone();
        assert_eq!(cloned.current_step_index(), 1);
        assert_eq!(cloned.total_steps(), 3);
        assert!(cloned.advance_step());
        assert_eq!(cloned.current_target(), Some(30.0));
        assert_eq!(original.current_target(), Some(20.0));
        original.reset();
        assert_eq!(original.current_target(), Some(10.0));
        assert_eq!(cloned.current_target(), Some(30.0));
        cloned.execute_completion();
        assert_eq!(calls.load(Ordering::Relaxed), 0);
        original.execute_completion();
        original.execute_completion();
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    #[allow(clippy::panic)] // Sentinel: this interpolation must never execute during construction.
    fn building_steps_never_interpolates_before_validation() {
        #[derive(Clone, Default, PartialEq)]
        struct NoInterpolation(f32);
        impl std::ops::Add for NoInterpolation {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }
        impl std::ops::Sub for NoInterpolation {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }
        impl std::ops::Mul<f32> for NoInterpolation {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self {
                Self(self.0 * rhs)
            }
        }
        impl Animatable for NoInterpolation {
            fn is_finite(&self) -> bool {
                self.0.is_finite()
            }
            fn magnitude(&self) -> f32 {
                self.0.abs()
            }
            fn interpolate(&self, _: &Self, _: f32) -> Self {
                panic!("building a sequence must not interpolate values");
            }
        }
        let mut sequence = AnimationSequence::with_capacity(1024usize)
            .then(NoInterpolation(1.0), AnimationConfig::tween_ms(1000))
            .then(NoInterpolation(2.0), AnimationConfig::tween_ms(1000));
        assert!(sequence.steps.capacity() >= 1024);
        sequence.reserve(2048usize);
        assert!(sequence.steps.capacity() >= 2050);
        assert_eq!(sequence.total_steps(), 2);
        assert_eq!(sequence.validate(), Ok(()));
        let invalid = sequence.then(NoInterpolation(f32::NAN), AnimationConfig::tween_ms(1000));
        assert_eq!(
            invalid.validate(),
            Err(AnimationError::NonFiniteValue("sequence target"))
        );
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

        // Test capacity helpers.
        let _sequence_with_capacity = AnimationSequence::<f32>::with_capacity(10);

        // Reserve additional slots.
        let mut sequence_mut = sequence.clone();
        sequence_mut.reserve(5);
    }
}
