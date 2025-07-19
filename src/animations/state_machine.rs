//! Animation state machine for efficient dispatch and reduced branching
//!
//! This module implements a state machine pattern to replace complex nested conditionals
//! in the animation update loop, providing better performance through efficient dispatch.

use crate::animations::core::{Animatable, AnimationMode};
use crate::animations::spring::{Spring, SpringState};
use crate::keyframes::KeyframeAnimation;
use crate::sequence::AnimationSequence;
use crate::pool::{ConfigHandle, global};
use crate::prelude::{AnimationConfig, LoopMode, Tween};
use crate::Duration;
use std::sync::Arc;

/// Animation state enum that represents the current mode of animation
/// This replaces complex branching logic with efficient state dispatch
#[derive(Clone)]
pub enum AnimationState<T: Animatable> {
    /// Animation is not running
    Idle,
    /// Single animation is running with specified mode
    Running { 
        mode: AnimationMode,
        config_handle: ConfigHandle,
    },
    /// Animation sequence is active
    Sequence { 
        sequence: Arc<AnimationSequence<T>>,
        config_handle: ConfigHandle,
    },
    /// Keyframe animation is active
    Keyframes { 
        animation: Arc<KeyframeAnimation<T>>,
        config_handle: ConfigHandle,
    },
}

impl<T: Animatable + Send + 'static> AnimationState<T> {
    /// Creates a new idle state
    pub fn new_idle() -> Self {
        Self::Idle
    }

    /// Creates a new running state with the specified mode
    pub fn new_running(mode: AnimationMode, config_handle: ConfigHandle) -> Self {
        Self::Running { mode, config_handle }
    }

    /// Creates a new sequence state
    pub fn new_sequence(sequence: Arc<AnimationSequence<T>>, config_handle: ConfigHandle) -> Self {
        Self::Sequence { sequence, config_handle }
    }

    /// Creates a new keyframes state
    pub fn new_keyframes(animation: Arc<KeyframeAnimation<T>>, config_handle: ConfigHandle) -> Self {
        Self::Keyframes { animation, config_handle }
    }

    /// Checks if the animation state is active (not idle)
    pub fn is_active(&self) -> bool {
        !matches!(self, Self::Idle)
    }

    /// Gets the current config handle if available
    pub fn config_handle(&self) -> Option<&ConfigHandle> {
        match self {
            Self::Idle => None,
            Self::Running { config_handle, .. } => Some(config_handle),
            Self::Sequence { config_handle, .. } => Some(config_handle),
            Self::Keyframes { config_handle, .. } => Some(config_handle),
        }
    }

    /// Main update dispatch method - replaces complex branching with efficient match
    /// Returns true if animation should continue, false if completed
    pub fn update(&mut self, dt: f32, motion: &mut crate::Motion<T>) -> bool {
        match self {
            Self::Idle => false,
            Self::Running { mode, config_handle } => {
                let mode = *mode;
                let config_handle = config_handle.clone();
                self.update_running(mode, &config_handle, dt, motion)
            },
            Self::Sequence { sequence, config_handle } => {
                let sequence = sequence.clone();
                let config_handle = config_handle.clone();
                self.update_sequence(sequence, &config_handle, dt, motion)
            },
            Self::Keyframes { animation, config_handle } => {
                let animation = animation.clone();
                let config_handle = config_handle.clone();
                self.update_keyframes(animation, &config_handle, dt, motion)
            },
        }
    }

    /// Updates a running animation with the specified mode
    fn update_running(
        &mut self, 
        mode: AnimationMode, 
        config_handle: &ConfigHandle, 
        dt: f32, 
        motion: &mut crate::Motion<T>
    ) -> bool {
        // Skip updates for imperceptible changes
        const MIN_DELTA: f32 = 1.0 / 240.0;
        if dt < MIN_DELTA {
            return true;
        }

        // Get config from handle
        let config = global::get_config_ref(config_handle)
            .unwrap_or_default();

        // Handle delay
        if motion.delay_elapsed < config.delay {
            motion.delay_elapsed += Duration::from_secs_f32(dt);
            return true;
        }

        let completed = match mode {
            AnimationMode::Spring(spring) => {
                let spring_result = self.update_spring(motion, spring, dt);
                matches!(spring_result, SpringState::Completed)
            }
            AnimationMode::Tween(tween) => self.update_tween(motion, tween, dt),
        };

        if completed {
            // Check if this is part of a sequence
            if let Some(ref sequence) = motion.sequence {
                // This is a sequence step completion - advance to next step
                if sequence.advance_step() {
                    // Successfully advanced to next step
                    if let Some(step) = sequence.current_step_data() {
                        let target = step.target;
                        let config = (*step.config).clone();
                        let mode = config.mode;
                        
                        // Update motion for new step
                        motion.initial = motion.current;
                        motion.target = target;
                        motion.running = true;
                        motion.elapsed = Duration::default();
                        motion.delay_elapsed = Duration::default();
                        motion.velocity = T::default();
                        
                        // Update config handle for new step
                        global::modify_config(config_handle, |pooled_config| {
                            *pooled_config = config;
                        });
                        
                        // Update state to running mode for new step
                        *self = Self::Running { 
                            mode, 
                            config_handle: config_handle.clone() 
                        };
                        
                        return true;
                    }
                } else {
                    // Sequence is complete
                    if let Ok(mut sequence_owned) = Arc::try_unwrap(sequence.clone()) {
                        sequence_owned.execute_completion();
                    }
                    motion.running = false;
                    motion.current_loop = 0;
                    motion.velocity = T::default();
                    motion.sequence = None;
                    motion.keyframe_animation = None;
                    *self = Self::Idle;
                    return false;
                }
            }
            
            // Not part of a sequence, handle normal completion
            self.handle_completion(motion, &config)
        } else {
            true
        }
    }

    /// Updates a sequence animation
    fn update_sequence(
        &mut self, 
        sequence: Arc<AnimationSequence<T>>, 
        _config_handle: &ConfigHandle, 
        dt: f32, 
        motion: &mut crate::Motion<T>
    ) -> bool {
        if !motion.running {
            if sequence.advance_step() {
                // Successfully advanced to next step
                if let Some(step) = sequence.current_step_data() {
                    let target = step.target;
                    let config = (*step.config).clone();
                    let mode = config.mode;
                    
                    // Update motion for new step
                    motion.initial = motion.current;
                    motion.target = target;
                    motion.running = true;
                    motion.elapsed = Duration::default();
                    motion.delay_elapsed = Duration::default();
                    motion.velocity = T::default();
                    
                    // Update state to running mode for this step
                    let new_config_handle = global::get_config();
                    global::modify_config(&new_config_handle, |pooled_config| {
                        *pooled_config = config;
                    });
                    
                    *self = Self::Running { 
                        mode, 
                        config_handle: new_config_handle 
                    };
                    
                    return true;
                }
            } else {
                // Sequence is complete
                let mut sequence_clone = (*sequence).clone();
                sequence_clone.execute_completion();
                *self = Self::Idle;
                motion.running = false;
                motion.current_loop = 0;
                motion.velocity = T::default();
                motion.sequence = None;
                motion.keyframe_animation = None;
                return false;
            }
        }
        
        // If we're here, the current step is still running
        // Delegate to running update
        if let Self::Running { mode, config_handle } = self {
            let mode = *mode;
            let config_handle = config_handle.clone();
            self.update_running(mode, &config_handle, dt, motion)
        } else {
            // This shouldn't happen, but handle gracefully
            false
        }
    }

    /// Updates a keyframe animation
    fn update_keyframes(
        &mut self, 
        animation: Arc<KeyframeAnimation<T>>, 
        config_handle: &ConfigHandle, 
        dt: f32, 
        motion: &mut crate::Motion<T>
    ) -> bool {
        let progress = (motion.elapsed.as_secs_f32() / animation.duration.as_secs_f32()).clamp(0.0, 1.0);
        
        let (start, end) = if animation.keyframes.is_empty() {
            // No keyframes, nothing to animate
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
        
        motion.current = start.value.interpolate(&end.value, eased_progress);
        motion.elapsed += Duration::from_secs_f32(dt);
        
        if progress >= 1.0 {
            let config = global::get_config_ref(config_handle).unwrap_or_default();
            self.handle_completion(motion, &config)
        } else {
            true
        }
    }

    /// Updates spring animation using optimized integration
    fn update_spring(&self, motion: &mut crate::Motion<T>, spring: Spring, dt: f32) -> SpringState {
        let epsilon = motion.get_epsilon();

        // Check for completion first
        let delta = motion.target - motion.current;
        if delta.magnitude() < epsilon && motion.velocity.magnitude() < epsilon {
            motion.current = motion.target;
            motion.velocity = T::default();
            return SpringState::Completed;
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
                let damping_force = motion.velocity * damping;
                motion.velocity = motion.velocity + (force - damping_force) * (mass_inv * step_dt);
                motion.current = motion.current + motion.velocity * step_dt;
            }
        }

        #[cfg(not(feature = "web"))]
        {
            // Native: Use RK4 for better accuracy with pooled integrator
            let (new_pos, new_vel) = self.perform_rk4_integration(
                motion.current,
                motion.velocity,
                motion.target,
                &spring,
                dt,
            );
            motion.current = new_pos;
            motion.velocity = new_vel;
        }

        self.check_spring_completion(motion)
    }

    /// Checks if spring animation is complete
    fn check_spring_completion(&self, motion: &mut crate::Motion<T>) -> SpringState {
        let epsilon = motion.get_epsilon();
        let epsilon_sq = epsilon * epsilon;
        let velocity_sq = motion.velocity.magnitude().powi(2);
        let delta = motion.target - motion.current;
        let delta_sq = delta.magnitude().powi(2);
        
        if velocity_sq < epsilon_sq && delta_sq < epsilon_sq {
            motion.current = motion.target;
            motion.velocity = T::default();
            SpringState::Completed
        } else {
            SpringState::Active
        }
    }

    /// Updates tween animation
    fn update_tween(&self, motion: &mut crate::Motion<T>, tween: Tween, dt: f32) -> bool {
        let elapsed_secs = motion.elapsed.as_secs_f32() + dt;
        motion.elapsed = Duration::from_secs_f32(elapsed_secs);
        let duration_secs = tween.duration.as_secs_f32();
        
        let progress = if duration_secs == 0.0 {
            1.0
        } else {
            (elapsed_secs * (1.0 / duration_secs)).min(1.0)
        };
        
        if progress <= 0.0 {
            motion.current = motion.initial;
            return false;
        } else if progress >= 1.0 {
            motion.current = motion.target;
            return true;
        }
        
        let eased_progress = (tween.easing)(progress, 0.0, 1.0, 1.0);
        match eased_progress {
            0.0 => motion.current = motion.initial,
            1.0 => motion.current = motion.target,
            _ => motion.current = motion.initial.interpolate(&motion.target, eased_progress),
        }
        
        progress >= 1.0
    }

    /// Handles animation completion and loop logic
    fn handle_completion(&mut self, motion: &mut crate::Motion<T>, config: &AnimationConfig) -> bool {
        let should_continue = match config.loop_mode.unwrap_or(LoopMode::None) {
            LoopMode::None => {
                motion.running = false;
                *self = Self::Idle;
                false
            }
            LoopMode::Infinite => {
                motion.current = motion.initial;
                motion.elapsed = Duration::default();
                motion.velocity = T::default();
                true
            }
            LoopMode::Times(count) => {
                motion.current_loop += 1;
                if motion.current_loop >= count {
                    motion.running = false;
                    motion.current_loop = 0;
                    motion.velocity = T::default();
                    motion.sequence = None;
                    motion.keyframe_animation = None;
                    *self = Self::Idle;
                    false
                } else {
                    motion.current = motion.initial;
                    motion.elapsed = Duration::default();
                    motion.velocity = T::default();
                    true
                }
            }
            LoopMode::Alternate => {
                motion.reverse = !motion.reverse;
                if motion.reverse {
                    std::mem::swap(&mut motion.initial, &mut motion.target);
                }
                motion.elapsed = Duration::default();
                motion.velocity = T::default();
                true
            }
            LoopMode::AlternateTimes(count) => {
                motion.current_loop += 1;
                if motion.current_loop >= count * 2 {
                    motion.running = false;
                    motion.current_loop = 0;
                    motion.velocity = T::default();
                    motion.sequence = None;
                    motion.keyframe_animation = None;
                    *self = Self::Idle;
                    false
                } else {
                    motion.reverse = !motion.reverse;
                    if motion.reverse {
                        std::mem::swap(&mut motion.initial, &mut motion.target);
                    }
                    motion.elapsed = Duration::default();
                    motion.velocity = T::default();
                    true
                }
            }
        };
        
        if !should_continue {
            if let Some(ref f) = config.on_complete {
                if let Ok(mut guard) = f.lock() {
                    guard();
                }
            }
        }
        
        should_continue
    }

    /// Performs RK4 integration using a local integrator
    #[cfg(not(feature = "web"))]
    fn perform_rk4_integration(
        &self,
        current_pos: T,
        current_vel: T,
        target: T,
        spring: &Spring,
        dt: f32,
    ) -> (T, T) {
        // Use a local integrator for now - pooling can be added later
        use crate::pool::SpringIntegrator;
        let mut integrator = SpringIntegrator::new();
        integrator.integrate_rk4(current_pos, current_vel, target, spring, dt)
    }
}

impl<T: Animatable> Default for AnimationState<T> {
    fn default() -> Self {
        Self::Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animations::core::AnimationMode;
    use crate::animations::spring::Spring;
    use crate::keyframes::KeyframeAnimation;
    use crate::sequence::{AnimationSequence, AnimationStep};
    use crate::prelude::Tween;
    use crate::Motion;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_animation_state_idle() {
        let state = AnimationState::<f32>::new_idle();
        assert!(!state.is_active());
        assert!(state.config_handle().is_none());
        
        let mut motion = Motion::new(0.0f32);
        let mut state = state;
        assert!(!state.update(1.0/60.0, &mut motion));
    }

    #[test]
    fn test_animation_state_running() {
        let config_handle = global::get_config();
        let mode = AnimationMode::Tween(Tween::default());
        
        let state = AnimationState::<f32>::new_running(mode, config_handle.clone());
        assert!(state.is_active());
        assert!(state.config_handle().is_some());
        
        let mut motion = Motion::new(0.0f32);
        motion.target = 100.0f32;
        motion.running = true;
        
        let mut state = state;
        let should_continue = state.update(1.0/60.0, &mut motion);
        
        // Should continue animating
        assert!(should_continue);
        
        global::return_config(config_handle);
    }

    #[test]
    fn test_animation_state_sequence() {
        let steps = vec![
            AnimationStep {
                target: 10.0f32,
                config: Arc::new(AnimationConfig::default()),
                predicted_next: None,
            },
            AnimationStep {
                target: 20.0f32,
                config: Arc::new(AnimationConfig::default()),
                predicted_next: None,
            },
        ];
        
        let sequence = Arc::new(AnimationSequence::from_steps(steps));
        let config_handle = global::get_config();
        
        let state = AnimationState::<f32>::new_sequence(sequence, config_handle.clone());
        assert!(state.is_active());
        
        let mut motion = Motion::new(0.0f32);
        motion.running = false; // Sequence should advance to next step
        
        let mut state = state;
        let should_continue = state.update(1.0/60.0, &mut motion);
        
        // Should continue and transition to running state
        assert!(should_continue);
        assert!(matches!(state, AnimationState::Running { .. }));
        
        global::return_config(config_handle);
    }

    #[test]
    fn test_animation_state_keyframes() {
        let mut animation = KeyframeAnimation::new(Duration::from_secs(1));
        animation = animation.add_keyframe(0.0f32, 0.0, None).unwrap();
        animation = animation.add_keyframe(100.0f32, 1.0, None).unwrap();
        
        let config_handle = global::get_config();
        let state = AnimationState::<f32>::new_keyframes(Arc::new(animation), config_handle.clone());
        assert!(state.is_active());
        
        let mut motion = Motion::new(0.0f32);
        motion.elapsed = Duration::from_millis(500); // Halfway through
        
        let mut state = state;
        let should_continue = state.update(1.0/60.0, &mut motion);
        
        // Should continue animating
        assert!(should_continue);
        // Value should be interpolated
        assert!(motion.current > 0.0 && motion.current < 100.0);
        
        global::return_config(config_handle);
    }

    #[test]
    fn test_animation_state_spring_completion() {
        let config_handle = global::get_config();
        global::modify_config(&config_handle, |config| {
            config.mode = AnimationMode::Spring(Spring::default());
        });
        
        let mode = AnimationMode::Spring(Spring::default());
        let state = AnimationState::<f32>::new_running(mode, config_handle.clone());
        
        let mut motion = Motion::new(0.0f32);
        motion.target = 0.0f32; // Already at target
        motion.velocity = 0.0f32; // No velocity
        motion.running = true;
        
        let mut state = state;
        let should_continue = state.update(1.0/60.0, &mut motion);
        
        // Should complete immediately since already at target with no velocity
        assert!(!should_continue);
        assert!(matches!(state, AnimationState::Idle));
        
        global::return_config(config_handle);
    }

    #[test]
    fn test_animation_state_tween_completion() {
        let config_handle = global::get_config();
        global::modify_config(&config_handle, |config| {
            config.mode = AnimationMode::Tween(Tween::default());
        });
        
        let mode = AnimationMode::Tween(Tween::default());
        let state = AnimationState::<f32>::new_running(mode, config_handle.clone());
        
        let mut motion = Motion::new(0.0f32);
        motion.target = 100.0f32;
        motion.running = true;
        motion.elapsed = Duration::from_secs(2); // Past default duration
        
        let mut state = state;
        let should_continue = state.update(1.0/60.0, &mut motion);
        
        // Should complete
        assert!(!should_continue);
        assert!(matches!(state, AnimationState::Idle));
        assert_eq!(motion.current, motion.target);
        
        global::return_config(config_handle);
    }

    #[test]
    fn test_animation_state_loop_infinite() {
        let config_handle = global::get_config();
        global::modify_config(&config_handle, |config| {
            config.mode = AnimationMode::Tween(Tween::default());
            config.loop_mode = Some(LoopMode::Infinite);
        });
        
        let mode = AnimationMode::Tween(Tween::default());
        let state = AnimationState::<f32>::new_running(mode, config_handle.clone());
        
        let mut motion = Motion::new(0.0f32);
        motion.target = 100.0f32;
        motion.running = true;
        motion.elapsed = Duration::from_secs(2); // Past default duration
        
        let mut state = state;
        let should_continue = state.update(1.0/60.0, &mut motion);
        
        // Should continue looping
        assert!(should_continue);
        assert!(matches!(state, AnimationState::Running { .. }));
        // Should reset to initial position
        assert_eq!(motion.current, motion.initial);
        assert_eq!(motion.elapsed, Duration::default());
        
        global::return_config(config_handle);
    }

    #[test]
    fn test_animation_state_completion_callback() {
        let callback_executed = Arc::new(Mutex::new(false));
        let callback_executed_clone = callback_executed.clone();
        
        let config_handle = global::get_config();
        global::modify_config(&config_handle, |config| {
            config.mode = AnimationMode::Tween(Tween::default());
            config.on_complete = Some(Arc::new(Mutex::new(Box::new(move || {
                *callback_executed_clone.lock().unwrap() = true;
            }))));
        });
        
        let mode = AnimationMode::Tween(Tween::default());
        let state = AnimationState::<f32>::new_running(mode, config_handle.clone());
        
        let mut motion = Motion::new(0.0f32);
        motion.target = 100.0f32;
        motion.running = true;
        motion.elapsed = Duration::from_secs(2); // Past default duration
        
        let mut state = state;
        let should_continue = state.update(1.0/60.0, &mut motion);
        
        // Should complete and execute callback
        assert!(!should_continue);
        assert!(*callback_executed.lock().unwrap());
        
        global::return_config(config_handle);
    }

    #[test]
    fn test_animation_state_delay_handling() {
        let config_handle = global::get_config();
        global::modify_config(&config_handle, |config| {
            config.mode = AnimationMode::Tween(Tween::default());
            config.delay = Duration::from_millis(100);
        });
        
        let mode = AnimationMode::Tween(Tween::default());
        let state = AnimationState::<f32>::new_running(mode, config_handle.clone());
        
        let mut motion = Motion::new(0.0f32);
        motion.target = 100.0f32;
        motion.running = true;
        motion.delay_elapsed = Duration::from_millis(50); // Still in delay
        
        let mut state = state;
        let should_continue = state.update(1.0/60.0, &mut motion);
        
        // Should continue but not animate yet (still in delay)
        assert!(should_continue);
        assert_eq!(motion.current, motion.initial); // Shouldn't have moved
        
        global::return_config(config_handle);
    }
}