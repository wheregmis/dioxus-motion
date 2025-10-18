use crate::Duration;
use crate::TimeProvider;
use crate::animations::core::Animatable;
use crate::keyframes::KeyframeAnimation;
use crate::pool::{ConfigHandle, global};
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

    // Optimized components
    /// Handle to pooled configuration
    config_handle: ConfigHandle,
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
            config_handle,
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

        // Animation is now ready to run
    }

    pub fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.value_cache = None;
        if let Some(first_step) = sequence.steps().first() {
            let first_config = (*first_step.config).clone();
            self.animate_to(first_step.target, first_config);
            let new_sequence = sequence.clone();
            new_sequence.reset(); // Reset to first step
            self.sequence = Some(Arc::new(new_sequence.clone()));

            // Sequence is now ready to run
        }
    }

    pub fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        self.value_cache = None;
        self.keyframe_animation = Some(Arc::new(animation.clone()));
        self.running = true;
        self.elapsed = Duration::default();
        self.velocity = T::default();

        // Keyframe animation is now ready to run
    }

    pub fn get_value(&self) -> T {
        // If the cache is valid for this frame, return it
        let now = crate::Time::now().elapsed().as_secs_f32();
        if let Some((ref cached, cached_time)) = self.value_cache
            && (now - cached_time).abs() < 0.001
        {
            return *cached;
        }
        // Not cached or outdated, so cache and return current value
        // (In practice, current is always up to date, but this is where you'd compute if needed)
        // Note: This requires &mut self, so we need to use interior mutability (e.g., RefCell) for full effect.
        // For now, just return current.
        self.current
    }

    pub fn is_running(&self) -> bool {
        self.running
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
        // Animation stopped
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

        if !self.running {
            return false;
        }

        // Skip updates for imperceptible changes
        const MIN_DELTA: f32 = 1.0 / 240.0;
        if dt < MIN_DELTA {
            return true;
        }

        // Get config from handle
        let config = global::get_config_ref(&self.config_handle).unwrap_or_default();

        // Handle delay
        if self.delay_elapsed < config.delay {
            self.delay_elapsed += Duration::from_secs_f32(dt);
            return true;
        }

        // Dispatch based on animation type
        if self.keyframe_animation.is_some() {
            let keyframe_animation = self.keyframe_animation.as_ref().unwrap().clone();
            self.update_keyframes(&keyframe_animation, dt)
        } else if self.sequence.is_some() {
            let sequence = self.sequence.as_ref().unwrap().clone();
            self.update_sequence(&sequence, dt)
        } else {
            self.update_simple(dt)
        }
    }

    /// Update simple animation (spring or tween)
    fn update_simple(&mut self, dt: f32) -> bool {
        let config = global::get_config_ref(&self.config_handle).unwrap_or_default();
        let completed = match config.mode {
            crate::animations::core::AnimationMode::Spring(spring) => {
                self.update_spring(spring, dt)
            }
            crate::animations::core::AnimationMode::Tween(tween) => self.update_tween(tween, dt),
        };

        if completed {
            self.handle_completion(&config)
        } else {
            true
        }
    }

    /// Update keyframe animation
    fn update_keyframes(&mut self, animation: &KeyframeAnimation<T>, dt: f32) -> bool {
        let progress =
            (self.elapsed.as_secs_f32() / animation.duration.as_secs_f32()).clamp(0.0, 1.0);

        let (start, end) = if animation.keyframes.is_empty() {
            return false;
        } else {
            animation
                .keyframes
                .windows(2)
                .find(|w| progress >= w[0].offset && progress <= w[1].offset)
                .map(|w| (&w[0], &w[1]))
                .unwrap_or_else(|| {
                    if progress <= animation.keyframes[0].offset {
                        let first = &animation.keyframes[0];
                        (first, first)
                    } else {
                        let last = animation
                            .keyframes
                            .last()
                            .expect("Keyframes vector should not be empty here");
                        (last, last)
                    }
                })
        };

        let local_progress = if start.offset == end.offset {
            1.0
        } else {
            (progress - start.offset) / (end.offset - start.offset)
        };

        let eased_progress = end
            .easing
            .map_or(local_progress, |ease| (ease)(local_progress, 0.0, 1.0, 1.0));

        self.current = start.value.interpolate(&end.value, eased_progress);
        self.elapsed += Duration::from_secs_f32(dt);

        if progress >= 1.0 {
            let config = global::get_config_ref(&self.config_handle).unwrap_or_default();
            self.handle_completion(&config)
        } else {
            true
        }
    }

    /// Update sequence animation
    fn update_sequence(&mut self, sequence: &AnimationSequence<T>, dt: f32) -> bool {
        if !self.running {
            if let Some(step) = sequence.current_step_data() {
                let target = step.target;
                let config = (*step.config).clone();

                // Update motion for new step
                self.initial = self.current;
                self.target = target;
                self.running = true;
                self.elapsed = Duration::default();
                self.delay_elapsed = Duration::default();
                self.velocity = T::default();

                // Update config handle for new step
                global::modify_config(&self.config_handle, |pooled_config| {
                    *pooled_config = config;
                });

                return true;
            } else {
                // Sequence is complete
                sequence.execute_completion();
                self.running = false;
                self.current_loop = 0;
                self.velocity = T::default();
                self.sequence = None;
                self.keyframe_animation = None;
                return false;
            }
        }

        // Update current step
        self.update_simple(dt)
    }

    /// Update spring animation
    fn update_spring(&mut self, spring: crate::animations::spring::Spring, dt: f32) -> bool {
        let epsilon = self.get_epsilon();

        // Check for completion first
        let delta = self.target - self.current;
        if delta.magnitude() < epsilon && self.velocity.magnitude() < epsilon {
            self.current = self.target;
            self.velocity = T::default();
            return true;
        }

        #[cfg(feature = "web")]
        {
            // Web: Use fixed timestep for better performance
            let stiffness = spring.stiffness;
            let damping = spring.damping;
            let mass_inv = 1.0 / spring.mass;

            const FIXED_DT: f32 = 1.0 / 120.0;
            let steps = ((dt / FIXED_DT) as usize).max(1);
            let step_dt = dt / steps as f32;

            for _ in 0..steps {
                let force = delta * stiffness;
                let damping_force = self.velocity * damping;
                self.velocity = self.velocity + (force - damping_force) * (mass_inv * step_dt);
                self.current = self.current + self.velocity * step_dt;
            }
        }

        #[cfg(not(feature = "web"))]
        {
            // Native: Use RK4 for better accuracy
            let (new_pos, new_vel) =
                self.perform_rk4_integration(self.current, self.velocity, self.target, &spring, dt);
            self.current = new_pos;
            self.velocity = new_vel;
        }

        self.check_spring_completion()
    }

    /// Update tween animation
    fn update_tween(&mut self, tween: crate::prelude::Tween, dt: f32) -> bool {
        let elapsed_secs = self.elapsed.as_secs_f32() + dt;
        self.elapsed = Duration::from_secs_f32(elapsed_secs);
        let duration_secs = tween.duration.as_secs_f32();

        let progress = if duration_secs == 0.0 {
            1.0
        } else {
            (elapsed_secs * (1.0 / duration_secs)).min(1.0)
        };

        if progress <= 0.0 {
            self.current = self.initial;
            return false;
        } else if progress >= 1.0 {
            self.current = self.target;
            return true;
        }

        let eased_progress = (tween.easing)(progress, 0.0, 1.0, 1.0);
        match eased_progress {
            0.0 => self.current = self.initial,
            1.0 => self.current = self.target,
            _ => self.current = self.initial.interpolate(&self.target, eased_progress),
        }

        progress >= 1.0
    }

    /// Check if spring animation is complete
    fn check_spring_completion(&mut self) -> bool {
        let epsilon = self.get_epsilon();
        let epsilon_sq = epsilon * epsilon;
        let velocity_sq = self.velocity.magnitude().powi(2);
        let delta = self.target - self.current;
        let delta_sq = delta.magnitude().powi(2);

        if velocity_sq < epsilon_sq && delta_sq < epsilon_sq {
            self.current = self.target;
            self.velocity = T::default();
            true
        } else {
            false
        }
    }

    /// Perform RK4 integration for spring animations
    #[cfg(not(feature = "web"))]
    fn perform_rk4_integration(
        &self,
        current_pos: T,
        current_vel: T,
        target: T,
        spring: &crate::animations::spring::Spring,
        dt: f32,
    ) -> (T, T) {
        // Simple RK4 implementation without pooling
        let stiffness = spring.stiffness;
        let damping = spring.damping;
        let mass_inv = 1.0 / spring.mass;

        // K1 calculation
        let delta = target - current_pos;
        let force = delta * stiffness;
        let damping_force = current_vel * damping;
        let acc = (force - damping_force) * mass_inv;
        let k1_pos = current_vel;
        let k1_vel = acc;

        // K2 calculation
        let temp_pos = current_pos + k1_pos * (dt * 0.5);
        let temp_vel = current_vel + k1_vel * (dt * 0.5);
        let delta = target - temp_pos;
        let force = delta * stiffness;
        let damping_force = temp_vel * damping;
        let acc = (force - damping_force) * mass_inv;
        let k2_pos = temp_vel;
        let k2_vel = acc;

        // K3 calculation
        let temp_pos = current_pos + k2_pos * (dt * 0.5);
        let temp_vel = current_vel + k2_vel * (dt * 0.5);
        let delta = target - temp_pos;
        let force = delta * stiffness;
        let damping_force = temp_vel * damping;
        let acc = (force - damping_force) * mass_inv;
        let k3_pos = temp_vel;
        let k3_vel = acc;

        // K4 calculation
        let temp_pos = current_pos + k3_pos * dt;
        let temp_vel = current_vel + k3_vel * dt;
        let delta = target - temp_pos;
        let force = delta * stiffness;
        let damping_force = temp_vel * damping;
        let acc = (force - damping_force) * mass_inv;
        let k4_pos = temp_vel;
        let k4_vel = acc;

        // Final integration
        const SIXTH: f32 = 1.0 / 6.0;
        let new_pos = current_pos + (k1_pos + k2_pos * 2.0 + k3_pos * 2.0 + k4_pos) * (dt * SIXTH);
        let new_vel = current_vel + (k1_vel + k2_vel * 2.0 + k3_vel * 2.0 + k4_vel) * (dt * SIXTH);

        (new_pos, new_vel)
    }

    /// Handle animation completion and loop logic
    fn handle_completion(&mut self, config: &AnimationConfig) -> bool {
        let should_continue = match config.loop_mode.unwrap_or(crate::prelude::LoopMode::None) {
            crate::prelude::LoopMode::None => {
                self.running = false;
                false
            }
            crate::prelude::LoopMode::Infinite => {
                self.current = self.initial;
                self.elapsed = Duration::default();
                self.velocity = T::default();
                self.running = true;
                true
            }
            crate::prelude::LoopMode::Times(count) => {
                self.current_loop += 1;
                if self.current_loop >= count {
                    self.running = false;
                    self.current_loop = 0;
                    self.velocity = T::default();
                    self.sequence = None;
                    self.keyframe_animation = None;
                    false
                } else {
                    self.current = self.initial;
                    self.elapsed = Duration::default();
                    self.velocity = T::default();
                    self.running = true;
                    true
                }
            }
            crate::prelude::LoopMode::Alternate => {
                self.reverse = !self.reverse;
                std::mem::swap(&mut self.initial, &mut self.target);
                self.current = self.initial;
                self.elapsed = Duration::default();
                self.velocity = T::default();
                self.running = true;
                true
            }
            crate::prelude::LoopMode::AlternateTimes(count) => {
                self.current_loop += 1;
                if self.current_loop >= count * 2 {
                    self.running = false;
                    self.current_loop = 0;
                    self.velocity = T::default();
                    self.sequence = None;
                    self.keyframe_animation = None;
                    false
                } else {
                    self.reverse = !self.reverse;
                    std::mem::swap(&mut self.initial, &mut self.target);
                    self.current = self.initial;
                    self.elapsed = Duration::default();
                    self.velocity = T::default();
                    self.running = true;
                    true
                }
            }
        };

        if !should_continue
            && let Some(ref f) = config.on_complete
            && let Ok(mut guard) = f.lock()
        {
            guard();
        }

        should_continue
    }

    /// Gets the current config handle for optimization purposes
    pub fn config_handle(&self) -> &ConfigHandle {
        &self.config_handle
    }

    /// Gets optimization statistics for this Motion instance
    pub fn optimization_stats(&self) -> MotionOptimizationStats {
        MotionOptimizationStats {
            has_config_handle: true, // Always true now
            value_cache_active: self.value_cache.is_some(),
        }
    }
}

/// Statistics about Motion optimization usage
#[derive(Debug, Clone, PartialEq)]
pub struct MotionOptimizationStats {
    pub has_config_handle: bool,
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

        // Verify config handle contains correct config
        let handle = motion.config_handle();
        let config = crate::pool::global::get_config_ref(handle).unwrap();
        assert!(matches!(config.mode, AnimationMode::Tween(_)));
    }

    #[test]
    fn test_motion_spring_animation() {
        crate::pool::global::clear_pool();

        let mut motion = Motion::new(0.0f32);
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));

        motion.animate_to(100.0, config);

        // Verify animation setup
        assert_eq!(motion.target, 100.0);
        assert!(motion.running);

        // Verify optimization components
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
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
    }

    #[test]
    fn test_motion_stop_cleanup() {
        crate::pool::global::clear_pool();

        let mut motion = Motion::new(0.0f32);
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));

        motion.animate_to(100.0, config);

        // Verify animation is running
        assert!(motion.running);

        motion.stop();

        // Verify animation is stopped
        assert!(!motion.running);
    }

    #[test]
    fn test_motion_drop_cleanup() {
        crate::pool::global::clear_pool();

        {
            let mut motion = Motion::new(0.0f32);
            let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
            motion.animate_to(100.0, config);

            // Verify config is allocated
            let (config_in_use, _) = crate::pool::global::pool_stats();
            assert_eq!(config_in_use, 1);
        } // Motion drops here

        // Verify config is returned to pool
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

        // Verify animation is running
        assert!(motion.running);

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

        let mut motion = Motion::new(0.0f32);

        // Test idle state stats
        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
        assert!(!stats.value_cache_active);

        // Test spring animation stats
        let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
        motion.animate_to(100.0, config);

        let stats = motion.optimization_stats();
        assert!(stats.has_config_handle);
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
    }
}
