use crate::Duration;
use crate::animations::core::{Animatable, AnimationMode, LoopMode};
use crate::animations::spring::{Spring, SpringState};
use crate::keyframes::KeyframeAnimation;
use crate::prelude::AnimationConfig;
use crate::sequence::AnimationSequence;

#[cfg(not(feature = "web"))]
use crate::pool::SpringIntegrator;

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
    config: AnimationConfig,
    pub sequence: Option<AnimationSequence<T>>,
    pub keyframe_animation: Option<KeyframeAnimation<T>>,
}

impl<T: Animatable + Send + 'static> Motion<T> {
    pub fn new(initial: T) -> Self {
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
            config: AnimationConfig::default(),
            sequence: None,
            keyframe_animation: None,
        }
    }

    pub fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.sequence = None;
        self.keyframe_animation = None;
        self.start_animation(target, config);
    }

    pub fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        sequence.reset();
        if let Some(first_step) = sequence.current_step_data() {
            self.start_animation(first_step.target, first_step.config.as_ref().clone());
            self.sequence = Some(sequence);
        }
    }

    pub fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        self.sequence = None;
        self.keyframe_animation = Some(animation);
        self.running = true;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::default();
        self.current_loop = 0;
        self.reverse = false;
    }

    pub fn get_value(&self) -> T {
        self.current
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn reset(&mut self) {
        self.stop();
        self.current = self.initial;
        self.target = self.initial;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.current_loop = 0;
        self.velocity = T::default();
        self.reverse = false;
        self.sequence = None;
        self.keyframe_animation = None;
    }

    pub fn delay(&mut self, duration: Duration) {
        self.config.delay = duration;
    }

    /// Gets the effective epsilon threshold for this animation.
    pub fn get_epsilon(&self) -> f32 {
        self.config.epsilon.unwrap_or_else(T::epsilon)
    }

    pub fn update(&mut self, dt: f32) -> bool {
        const MIN_DELTA: f32 = 1.0 / 240.0;

        if !self.running {
            return false;
        }

        if dt < MIN_DELTA {
            return true;
        }

        if self.delay_elapsed < self.config.delay {
            self.delay_elapsed += Duration::from_secs_f32(dt);
            return true;
        }

        if self.keyframe_animation.is_some() {
            if self.update_keyframes(dt) {
                self.finish_motion();
                return false;
            }
            return true;
        }

        let completed = match self.config.mode {
            AnimationMode::Spring(spring) => {
                let state = self.update_spring(spring, dt);
                matches!(state, SpringState::Completed)
            }
            AnimationMode::Tween(tween) => self.update_tween(tween, dt),
        };

        if !completed {
            return true;
        }

        if self.sequence.is_some() {
            return self.advance_sequence_step();
        }

        self.handle_completion()
    }

    fn start_animation(&mut self, target: T, config: AnimationConfig) {
        self.initial = self.current;
        self.target = target;
        self.running = true;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::default();
        self.current_loop = 0;
        self.reverse = false;
        self.config = config;
    }

    fn advance_sequence_step(&mut self) -> bool {
        let Some(sequence) = self.sequence.as_mut() else {
            return false;
        };

        let next_step = if sequence.advance_step() {
            sequence
                .current_step_data()
                .map(|step| (step.target, step.config.as_ref().clone()))
        } else {
            sequence.execute_completion();
            None
        };

        if let Some((target, config)) = next_step {
            self.start_animation(target, config);
            return true;
        }

        self.finish_motion();
        false
    }

    fn update_keyframes(&mut self, dt: f32) -> bool {
        let Some(animation) = self.keyframe_animation.clone() else {
            return true;
        };

        let duration_secs = animation.duration.as_secs_f32();
        let next_elapsed_secs = self.elapsed.as_secs_f32() + dt;
        let progress = if duration_secs == 0.0 {
            1.0
        } else {
            (next_elapsed_secs / duration_secs).clamp(0.0, 1.0)
        };

        let (start, end) = if animation.keyframes.is_empty() {
            return true;
        } else {
            animation
                .keyframes
                .windows(2)
                .find(|window| progress >= window[0].offset && progress <= window[1].offset)
                .map(|window| (&window[0], &window[1]))
                .unwrap_or_else(|| {
                    if progress <= animation.keyframes[0].offset {
                        let first = &animation.keyframes[0];
                        (first, first)
                    } else {
                        let last = animation
                            .keyframes
                            .last()
                            .expect("keyframe animation should have at least one frame");
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
        self.elapsed = Duration::from_secs_f32(next_elapsed_secs);

        progress >= 1.0
    }

    fn update_spring(&mut self, spring: Spring, dt: f32) -> SpringState {
        let epsilon = self.get_epsilon();
        let delta = self.target - self.current;

        if delta.magnitude() < epsilon && self.velocity.magnitude() < epsilon {
            self.current = self.target;
            self.velocity = T::default();
            return SpringState::Completed;
        }

        #[cfg(feature = "web")]
        {
            let stiffness = spring.stiffness;
            let damping = spring.damping;
            let mass_inv = 1.0 / spring.mass;

            const FIXED_DT: f32 = 1.0 / 120.0;
            let steps = ((dt / FIXED_DT) as usize).max(1);
            let step_dt = dt / steps as f32;

            for _ in 0..steps {
                let step_delta = self.target - self.current;
                let force = step_delta * stiffness;
                let damping_force = self.velocity * damping;
                self.velocity = self.velocity + (force - damping_force) * (mass_inv * step_dt);
                self.current = self.current + self.velocity * step_dt;
            }
        }

        #[cfg(not(feature = "web"))]
        {
            let mut integrator = SpringIntegrator::new();
            let (new_pos, new_vel) =
                integrator.integrate_rk4(self.current, self.velocity, self.target, &spring, dt);
            self.current = new_pos;
            self.velocity = new_vel;
        }

        self.check_spring_completion()
    }

    fn check_spring_completion(&mut self) -> SpringState {
        let epsilon = self.get_epsilon();
        let epsilon_sq = epsilon * epsilon;
        let velocity_sq = self.velocity.magnitude().powi(2);
        let delta_sq = (self.target - self.current).magnitude().powi(2);

        if velocity_sq < epsilon_sq && delta_sq < epsilon_sq {
            self.current = self.target;
            self.velocity = T::default();
            SpringState::Completed
        } else {
            SpringState::Active
        }
    }

    fn update_tween(&mut self, tween: crate::prelude::Tween, dt: f32) -> bool {
        let elapsed_secs = self.elapsed.as_secs_f32() + dt;
        self.elapsed = Duration::from_secs_f32(elapsed_secs);
        let duration_secs = tween.duration.as_secs_f32();

        let progress = if duration_secs == 0.0 {
            1.0
        } else {
            (elapsed_secs / duration_secs).min(1.0)
        };

        if progress <= 0.0 {
            self.current = self.initial;
            return false;
        }

        if progress >= 1.0 {
            self.current = self.target;
            return true;
        }

        let eased_progress = (tween.easing)(progress, 0.0, 1.0, 1.0);
        self.current = match eased_progress {
            0.0 => self.initial,
            1.0 => self.target,
            _ => self.initial.interpolate(&self.target, eased_progress),
        };

        false
    }

    fn handle_completion(&mut self) -> bool {
        match self.config.loop_mode.unwrap_or(LoopMode::None) {
            LoopMode::None => {
                self.config.execute_completion();
                self.finish_motion();
                false
            }
            LoopMode::Infinite => {
                self.restart_motion();
                true
            }
            LoopMode::Times(count) => {
                self.current_loop += 1;
                if self.current_loop >= count {
                    self.config.execute_completion();
                    self.finish_motion();
                    false
                } else {
                    self.restart_motion();
                    true
                }
            }
            LoopMode::Alternate => {
                self.reverse_motion();
                true
            }
            LoopMode::AlternateTimes(count) => {
                self.current_loop += 1;
                if self.current_loop >= count * 2 {
                    self.config.execute_completion();
                    self.finish_motion();
                    false
                } else {
                    self.reverse_motion();
                    true
                }
            }
        }
    }

    fn finish_motion(&mut self) {
        self.running = false;
        self.current_loop = 0;
        self.velocity = T::default();
        self.sequence = None;
        self.keyframe_animation = None;
    }

    fn restart_motion(&mut self) {
        self.current = self.initial;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::default();
        self.running = true;
    }

    fn reverse_motion(&mut self) {
        self.reverse = !self.reverse;
        std::mem::swap(&mut self.initial, &mut self.target);
        self.restart_motion();
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::animations::core::AnimationMode;
    use crate::animations::spring::Spring;
    use crate::prelude::Tween;
    use std::sync::{Arc, Mutex};

    fn instant_tween() -> AnimationConfig {
        AnimationConfig::new(AnimationMode::Tween(Tween::new(Duration::from_secs(0))))
    }

    #[test]
    fn test_motion_new() {
        let motion = Motion::new(0.0f32);

        assert_eq!(motion.initial, 0.0);
        assert_eq!(motion.current, 0.0);
        assert_eq!(motion.target, 0.0);
        assert!(!motion.running);
        assert!(motion.sequence.is_none());
        assert!(motion.keyframe_animation.is_none());
    }

    #[test]
    fn test_motion_animate_to() {
        let mut motion = Motion::new(0.0f32);
        motion.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );

        assert_eq!(motion.target, 100.0);
        assert!(motion.running);
        assert!(motion.sequence.is_none());
        assert!(motion.keyframe_animation.is_none());
    }

    #[test]
    fn test_motion_sequence_advances() {
        let mut motion = Motion::new(0.0f32);
        let sequence = AnimationSequence::new()
            .then(50.0f32, instant_tween())
            .then(100.0f32, instant_tween());

        motion.animate_sequence(sequence);

        assert_eq!(motion.target, 50.0);
        assert!(motion.sequence.is_some());

        assert!(motion.update(1.0 / 60.0));
        assert_eq!(motion.target, 100.0);
        assert!(motion.running);

        assert!(!motion.update(1.0 / 60.0));
        assert_eq!(motion.current, 100.0);
        assert!(!motion.running);
        assert!(motion.sequence.is_none());
    }

    #[test]
    fn test_motion_keyframes_progress_and_complete() {
        let mut motion = Motion::new(0.0f32);

        let animation = KeyframeAnimation::new(Duration::from_secs(1))
            .add_keyframe(0.0, 0.0, None)
            .unwrap()
            .add_keyframe(100.0, 1.0, None)
            .unwrap();

        motion.animate_keyframes(animation);

        assert!(motion.update(0.5));
        assert!(motion.current > 0.0);
        assert!(motion.current < 100.0);

        assert!(!motion.update(0.5));
        assert_eq!(motion.current, 100.0);
        assert!(!motion.running);
        assert!(motion.keyframe_animation.is_none());
    }

    #[test]
    fn test_motion_stop() {
        let mut motion = Motion::new(0.0f32);
        motion.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );

        motion.stop();

        assert!(!motion.running);
        assert!(motion.sequence.is_none());
        assert!(motion.keyframe_animation.is_none());
        assert_eq!(motion.velocity, 0.0);
    }

    #[test]
    fn test_motion_get_epsilon() {
        let mut motion = Motion::new(0.0f32);
        assert_eq!(motion.get_epsilon(), f32::epsilon());

        motion.animate_to(
            1.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())).with_epsilon(0.01),
        );

        assert_eq!(motion.get_epsilon(), 0.01);
    }

    #[test]
    fn test_motion_delay_prevents_early_update() {
        let mut motion = Motion::new(0.0f32);
        motion.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );
        motion.delay(Duration::from_millis(100));

        assert!(motion.update(1.0 / 60.0));
        assert_eq!(motion.current, motion.initial);
    }

    #[test]
    fn test_motion_update_tween_changes_value() {
        let mut motion = Motion::new(0.0f32);
        motion.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );

        assert!(motion.update(1.0 / 60.0));
        assert!(motion.current > 0.0);
        assert!(motion.current < 100.0);
    }

    #[test]
    fn test_motion_spring_completes_when_already_settled() {
        let mut motion = Motion::new(0.0f32);
        motion.animate_to(
            0.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
        motion.velocity = 0.0;

        assert!(!motion.update(1.0 / 60.0));
        assert_eq!(motion.current, 0.0);
        assert!(!motion.running);
    }

    #[test]
    fn test_motion_loop_mode_times() {
        let mut motion = Motion::new(0.0f32);
        motion.animate_to(100.0, instant_tween().with_loop(LoopMode::Times(2)));

        assert!(motion.update(1.0 / 60.0));
        assert_eq!(motion.current, motion.initial);
        assert!(motion.running);

        assert!(!motion.update(1.0 / 60.0));
        assert!(!motion.running);
    }

    #[test]
    fn test_motion_loop_mode_alternate() {
        let mut motion = Motion::new(0.0f32);
        motion.animate_to(100.0, instant_tween().with_loop(LoopMode::Alternate));

        assert!(motion.update(1.0 / 60.0));
        assert!(motion.running);
        assert!(motion.reverse);
        assert_eq!(motion.initial, 100.0);
        assert_eq!(motion.target, 0.0);
    }

    #[test]
    fn test_motion_completion_callback() {
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();
        let config = instant_tween().with_on_complete(move || {
            *called_clone.lock().unwrap() = true;
        });

        let mut motion = Motion::new(0.0f32);
        motion.animate_to(100.0, config);

        assert!(!motion.update(1.0 / 60.0));
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_motion_get_value_tracks_current_directly() {
        let mut motion = Motion::new(0.0f32);
        motion.current = 12.5;
        assert_eq!(motion.get_value(), 12.5);

        motion.current = 42.0;
        assert_eq!(motion.get_value(), 42.0);
    }
}
