use crate::Duration;
use crate::TimeProvider;
use crate::animations::core::Animatable;
use crate::animations::state_machine::AnimationState;
use crate::keyframes::KeyframeAnimation;
use crate::pool::{ConfigHandle, SpringIntegratorHandle, global};
use crate::prelude::AnimationConfig;
use crate::sequence::AnimationSequence;
use std::sync::Arc;

#[derive(Clone)]
pub struct Motion<T: Animatable + Send + 'static> {
    pub initial: T,
    pub current: T,
    pub target: T,
    pub velocity: T,
    pub running: bool,
    pub elapsed: Duration,
    pub delay_elapsed: Duration,
    pub current_loop: u8,
    pub reverse: bool,

    // Optimized components (now the primary implementation)
    /// State machine for efficient animation dispatch
    pub animation_state: AnimationState<T>,
    /// Handle to pooled configuration
    config_handle: ConfigHandle,
    /// Handle to pooled spring integrator for spring animations
    spring_integrator_handle: Option<SpringIntegratorHandle>,
    /// Current sequence being animated (if any)
    pub sequence: Option<Arc<AnimationSequence<T>>>,
    /// Current keyframe animation (if any)
    pub keyframe_animation: Option<Arc<KeyframeAnimation<T>>>,

    // Internal value cache: (value, frame_time)
    value_cache: Option<(T, f32)>,
}

impl<T: Animatable + Send + 'static> Drop for Motion<T> {
    fn drop(&mut self) {
        // Return config handle to pool
        global::return_config(self.config_handle.clone());

        // Return spring integrator handle to pool
        if let Some(handle) = self.spring_integrator_handle.take() {
            self.try_return_spring_integrator(handle);
        }
    }
}

impl<T: Animatable + Send + 'static> Motion<T> {
    pub fn new(initial: T) -> Self {
        let config_handle = global::get_config();
        global::modify_config(&config_handle, |config| {
            *config = AnimationConfig::default();
        });

        Self {
            initial,
            current: initial,
            target: initial,
            velocity: T::default(),
            running: false,
            elapsed: Duration::default(),
            delay_elapsed: Duration::default(),
            current_loop: 0,
            reverse: false,

            // Optimized components
            animation_state: AnimationState::new_idle(),
            config_handle,
            spring_integrator_handle: None,
            sequence: None,
            keyframe_animation: None,

            value_cache: None,
        }
    }



    pub fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.value_cache = None;
        self.sequence = None;
        self.initial = self.current;
        self.target = target;
        self.running = true;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::default();
        self.current_loop = 0;

        // Update config handle
        global::modify_config(&self.config_handle, |pooled_config| {
            *pooled_config = config.clone();
        });

        // Set up spring integrator if needed
        if matches!(config.mode, crate::animations::core::AnimationMode::Spring(_))
            && self.spring_integrator_handle.is_none() {
            self.spring_integrator_handle = self.try_get_spring_integrator();
        }

        // Set up state machine for running animation
        self.animation_state = AnimationState::new_running(config.mode, self.config_handle.clone());
    }

    pub fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.value_cache = None;
        if let Some(first_step) = sequence.steps().first() {
            let first_config = (*first_step.config).clone();
            self.animate_to(first_step.target, first_config);
            let new_sequence = sequence;
            new_sequence.reset(); // Reset to first step
            self.sequence = Some(Arc::new(new_sequence.clone()));

            // Set up state machine for sequence animation
            self.animation_state = AnimationState::new_sequence(Arc::new(new_sequence), self.config_handle.clone());
        }
    }

    pub fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        self.value_cache = None;
        self.keyframe_animation = Some(Arc::new(animation.clone()));
        self.running = true;
        self.elapsed = Duration::default();
        self.velocity = T::default();

        // Set up state machine for keyframe animation
        self.animation_state = AnimationState::new_keyframes(Arc::new(animation), self.config_handle.clone());
    }

    pub fn get_value(&self) -> T {
        // If the cache is valid for this frame, return it
        let now = crate::Time::now().elapsed().as_secs_f32();
        if let Some((ref cached, cached_time)) = self.value_cache {
            if (now - cached_time).abs() < 0.001 {
                return *cached;
            }
        }
        // Not cached or outdated, so cache and return current value
        // (In practice, current is always up to date, but this is where you'd compute if needed)
        // Note: This requires &mut self, so we need to use interior mutability (e.g., RefCell) for full effect.
        // For now, just return current.
        self.current
    }

    pub fn is_running(&self) -> bool {
        self.animation_state.is_active()
    }

    pub fn reset(&mut self) {
        self.value_cache = None;
        self.stop();
        self.current = self.initial;
        self.elapsed = Duration::default();
    }

    pub fn stop(&mut self) {
        self.value_cache = None;
        self.running = false;
        self.current_loop = 0;
        self.velocity = T::default();
        self.sequence = None;
        self.keyframe_animation = None;
        self.animation_state = AnimationState::new_idle();

        // Return spring integrator to pool if we have one
        if let Some(handle) = self.spring_integrator_handle.take() {
            self.try_return_spring_integrator(handle);
        }
    }

    pub fn delay(&mut self, duration: Duration) {
        self.value_cache = None;
        
        // Update config handle
        global::modify_config(&self.config_handle, |pooled_config| {
            pooled_config.delay = duration;
        });
    }

    /// Gets the effective epsilon threshold for this animation
    /// Uses the configured epsilon if present, otherwise falls back to the type's default
    pub fn get_epsilon(&self) -> f32 {
        if let Some(config) = global::get_config_ref(&self.config_handle) {
            config.epsilon.unwrap_or_else(T::epsilon)
        } else {
            T::epsilon()
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        // Invalidate value cache on update
        self.value_cache = None;

        // Use state machine dispatch instead of complex branching
        // We need to temporarily take the state to avoid borrowing issues
        let mut state = std::mem::replace(&mut self.animation_state, AnimationState::new_idle());
        let result = state.update(dt, self);
        self.animation_state = state;
        result
    }

    /// Gets the current config handle for optimization purposes
    pub fn config_handle(&self) -> &ConfigHandle {
        &self.config_handle
    }

    /// Gets the current spring integrator handle for optimization purposes
    pub fn spring_integrator_handle(&self) -> Option<&SpringIntegratorHandle> {
        self.spring_integrator_handle.as_ref()
    }


    /// Gets optimization statistics for this Motion instance
    pub fn optimization_stats(&self) -> MotionOptimizationStats {
        MotionOptimizationStats {
            has_config_handle: true, // Always true now
            has_spring_integrator: self.spring_integrator_handle.is_some(),
            state_machine_active: self.animation_state.is_active(),
            value_cache_active: self.value_cache.is_some(),
        }
    }

    /// Tries to get a spring integrator handle (now always available)
    fn try_get_spring_integrator(&self) -> Option<SpringIntegratorHandle> {
        Some(crate::pool::integrator::get_integrator::<T>())
    }

    /// Tries to return a spring integrator handle (now always available)
    fn try_return_spring_integrator(&self, handle: SpringIntegratorHandle) {
        crate::pool::integrator::return_integrator::<T>(handle);
    }
}

/// Statistics about Motion optimization usage
#[derive(Debug, Clone, PartialEq)]
pub struct MotionOptimizationStats {
    pub has_config_handle: bool,
    pub has_spring_integrator: bool,
    pub state_machine_active: bool,
    pub value_cache_active: bool,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use crate::animations::core::AnimationMode;
    use crate::animations::spring::Spring;
    use crate::keyframes::KeyframeAnimation;
    use crate::prelude::Tween;
    use crate::sequence::{AnimationSequence, AnimationStep};

    #[test]
    fn test_motion_new_with_optimizations() {
        crate::pool::global::clear_pool();

        let motion = Motion::new(0.0f32);

        // Verify basic initialization
        assert_eq!(motion.initial, 0.0);
        assert_eq!(motion.current, 0.0);
        assert_eq!(motion.target, 0.0);
        assert!(!motion.running);

        // Verify optimization components are initialized
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(!stats.has_spring_integrator); // Not needed for idle motion
        assert!(!stats.state_machine_active); // Idle state
        assert!(!stats.value_cache_active);
    }

    #[test]
    fn test_motion_animate_to_with_optimizations() {
        crate::pool::global::clear_pool();

        let mut motion = Motion::new(0.0f32);
        let config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));

        motion.animate_to(100.0, config);

        // Verify animation setup
        assert_eq!(motion.target, 100.0);
        assert!(motion.running);

        // Verify optimization components
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(!stats.has_spring_integrator); // Tween doesn't need integrator
        assert!(stats.state_machine_active);

        // Verify config handle contains correct config
        let handle = motion.config_handle();
        let config = crate::pool::global::get_config_ref(handle).unwrap();
        assert!(matches!(config.mode, AnimationMode::Tween(_)));
    }

    #[test]
    fn test_motion_spring_animation_with_integrator() {
        crate::pool::global::clear_pool();
        crate::pool::integrator::clear_pools();

        let mut motion = Motion::new(0.0f32);
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));

        motion.animate_to(100.0, config);

        // Verify spring integrator is allocated (now enabled by default)
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(stats.has_spring_integrator);
        assert!(stats.state_machine_active);

        // Verify integrator handle is allocated
        assert!(motion.spring_integrator_handle().is_some());

        // Verify pool statistics
        let (in_use, _) = crate::pool::integrator::pool_stats::<f32>();
        assert_eq!(in_use, 1);
    }

    #[test]
    fn test_motion_sequence_with_optimizations() {
        crate::pool::global::clear_pool();

        let mut motion = Motion::new(0.0f32);

        let steps = vec![
            AnimationStep {
                target: 50.0,
                config: Arc::new(AnimationConfig::default()),
                predicted_next: None,
            },
            AnimationStep {
                target: 100.0,
                config: Arc::new(AnimationConfig::default()),
                predicted_next: None,
            },
        ];

        let sequence = AnimationSequence::from_steps(steps);
        motion.animate_sequence(sequence);

        // Verify sequence setup
        assert!(motion.sequence.is_some());
        assert_eq!(motion.target, 50.0); // First step target

        // Verify optimization components
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(stats.state_machine_active);

        // Verify state machine is in sequence mode
        assert!(matches!(
            motion.animation_state,
            AnimationState::Sequence { .. }
        ));
    }

    #[test]
    fn test_motion_keyframes_with_optimizations() {
        crate::pool::global::clear_pool();

        let mut motion = Motion::new(0.0f32);

        let mut animation = KeyframeAnimation::new(Duration::from_secs(1));
        animation = animation.add_keyframe(0.0, 0.0, None).unwrap();
        animation = animation.add_keyframe(100.0, 1.0, None).unwrap();

        motion.animate_keyframes(animation);

        // Verify keyframe setup
        assert!(motion.keyframe_animation.is_some());
        assert!(motion.running);

        // Verify optimization components
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(stats.state_machine_active);

        // Verify state machine is in keyframes mode
        assert!(matches!(
            motion.animation_state,
            AnimationState::Keyframes { .. }
        ));
    }

    #[test]
    fn test_motion_stop_cleanup() {
        crate::pool::global::clear_pool();
        crate::pool::integrator::clear_pools();

        let mut motion = Motion::new(0.0f32);
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));

        motion.animate_to(100.0, config);

        // Verify resources are allocated (now enabled by default)
        assert!(motion.spring_integrator_handle().is_some());
        let (in_use_before, _) = crate::pool::integrator::pool_stats::<f32>();
        assert_eq!(in_use_before, 1);

        motion.stop();

        // Verify resources are cleaned up
        assert!(motion.spring_integrator_handle().is_none());
        assert!(!motion.running);
        assert!(matches!(motion.animation_state, AnimationState::Idle));

        let (in_use_after, available_after) = crate::pool::integrator::pool_stats::<f32>();
        assert_eq!(in_use_after, 0);
        assert_eq!(available_after, 1); // Returned to pool
    }

    #[test]
    fn test_motion_drop_cleanup() {
        crate::pool::global::clear_pool();
        crate::pool::integrator::clear_pools();

        {
            let mut motion = Motion::new(0.0f32);
            let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
            motion.animate_to(100.0, config);

            // Verify resources are allocated (both config and integrator)
            let (in_use, _) = crate::pool::integrator::pool_stats::<f32>();
            assert_eq!(in_use, 1); // Integrator pooling now enabled

            let (config_in_use, _) = crate::pool::global::pool_stats();
            assert_eq!(config_in_use, 1);
        } // Motion drops here

        // Verify resources are returned to pools
        let (in_use_after, available_after) = crate::pool::integrator::pool_stats::<f32>();
        assert_eq!(in_use_after, 0);
        assert_eq!(available_after, 1); // Returned to pool

        let (config_in_use_after, config_available_after) = crate::pool::global::pool_stats();
        assert_eq!(config_in_use_after, 0);
        assert_eq!(config_available_after, 1);
    }

    #[test]
    fn test_motion_get_epsilon_optimization() {
        crate::pool::global::clear_pool();

        let motion = Motion::new(0.0f32);

        // Test default epsilon
        assert_eq!(motion.get_epsilon(), f32::epsilon());

        // Test custom epsilon through optimized config handle
        let handle = motion.config_handle();
        crate::pool::global::modify_config(handle, |config| {
            config.epsilon = Some(0.01);
        });

        assert_eq!(motion.get_epsilon(), 0.01);
    }

    #[test]
    fn test_motion_delay_optimization() {
        crate::pool::global::clear_pool();

        let mut motion = Motion::new(0.0f32);
        let delay = Duration::from_millis(100);

        motion.delay(delay);

        // Verify config is updated through the optimized handle

        let handle = motion.config_handle();
        let config = crate::pool::global::get_config_ref(handle).unwrap();
        assert_eq!(config.delay, delay);
    }

    #[test]
    fn test_motion_update_with_state_machine() {
        crate::pool::global::clear_pool();

        let mut motion = Motion::new(0.0f32);
        let config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));

        motion.animate_to(100.0, config);

        // Verify state machine is active
        assert!(motion.animation_state.is_active());

        // Update animation
        let should_continue = motion.update(1.0 / 60.0);

        // Should continue animating
        assert!(should_continue);

        // Value should have changed
        assert!(motion.current > 0.0);
        assert!(motion.current < 100.0);
    }

    #[test]
    fn test_motion_optimization_stats() {
        crate::pool::global::clear_pool();
        crate::pool::integrator::clear_pools();

        let mut motion = Motion::new(0.0f32);

        // Test idle state stats
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(!stats.has_spring_integrator);
        assert!(!stats.state_machine_active);
        assert!(!stats.value_cache_active);

        // Test spring animation stats
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
        motion.animate_to(100.0, config);

        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(stats.has_spring_integrator); // Now enabled by default
        assert!(stats.state_machine_active);
        assert!(!stats.value_cache_active);
    }

    #[test]
    fn test_motion_backward_compatibility() {
        crate::pool::global::clear_pool();

        let mut motion = Motion::new(0.0f32);
        let config = AnimationConfig::default();

        // Test that legacy API still works
        motion.animate_to(100.0, config);

        // Core fields should still be accessible
        assert_eq!(motion.target, 100.0);
        assert!(motion.running);

        // But optimizations should also be active
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(stats.state_machine_active);
    }
}
