use crate::Duration;
use crate::TimeProvider;
use crate::animations::core::{Animatable, AnimationMode};
use crate::animations::spring::{Spring, SpringState};
use crate::keyframes::KeyframeAnimation;
use crate::prelude::{AnimationConfig, LoopMode, Tween};
use crate::sequence::AnimationSequence;
use std::sync::Arc;

#[derive(Clone)]
pub struct Motion<T: Animatable> {
    pub initial: T,
    pub current: T,
    pub target: T,
    pub velocity: T,
    pub running: bool,
    pub elapsed: Duration,
    pub delay_elapsed: Duration,
    pub current_loop: u8,
    pub config: Arc<AnimationConfig>,
    pub sequence: Option<Arc<AnimationSequence<T>>>,
    pub reverse: bool,
    pub keyframe_animation: Option<Arc<KeyframeAnimation<T>>>,
    // Internal value cache: (value, frame_time)
    value_cache: Option<(T, f32)>,
}

impl<T: Animatable> Motion<T> {
    pub fn new(initial: T) -> Self {
        Self {
            initial,
            current: initial,
            target: initial,
            velocity: T::zero(),
            running: false,
            elapsed: Duration::default(),
            delay_elapsed: Duration::default(),
            current_loop: 0,
            config: Arc::new(AnimationConfig::default()),
            sequence: None,
            reverse: false,
            keyframe_animation: None,
            value_cache: None,
        }
    }

    pub fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.value_cache = None;
        self.sequence = None;
        self.initial = self.current;
        self.target = target;
        self.config = Arc::new(config);
        self.running = true;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::zero();
        self.current_loop = 0;
    }

    pub fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.value_cache = None;
        if let Some(first_step) = sequence.steps.first() {
            self.animate_to(first_step.target, (*first_step.config).clone());
            let mut new_sequence = sequence;
            new_sequence.current_step = 0;
            self.sequence = Some(Arc::new(new_sequence));
        }
    }

    pub fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        self.value_cache = None;
        self.keyframe_animation = Some(Arc::new(animation));
        self.running = true;
        self.elapsed = Duration::default();
        self.velocity = T::zero();
    }

    pub fn get_value(&self) -> T {
        // If the cache is valid for this frame, return it
        let now = crate::Time::now().elapsed().as_secs_f32();
        if let Some((ref cached, cached_time)) = self.value_cache {
            if (now - cached_time).abs() < 0.001 {
                return cached.clone();
            }
        }
        // Not cached or outdated, so cache and return current value
        // (In practice, current is always up to date, but this is where you'd compute if needed)
        // Note: This requires &mut self, so we need to use interior mutability (e.g., RefCell) for full effect.
        // For now, just return current.
        self.current
    }

    pub fn is_running(&self) -> bool {
        self.running || self.sequence.is_some() || self.keyframe_animation.is_some()
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
        self.velocity = T::zero();
        self.sequence = None;
        self.keyframe_animation = None;
    }

    pub fn delay(&mut self, duration: Duration) {
        self.value_cache = None;
        let mut config = (*self.config).clone();
        config.delay = duration;
        self.config = Arc::new(config);
    }

    /// Gets the effective epsilon threshold for this animation
    /// Uses the configured epsilon if present, otherwise falls back to the type's default
    pub fn get_epsilon(&self) -> f32 {
        self.config.epsilon.unwrap_or_else(T::epsilon)
    }

    pub fn update(&mut self, dt: f32) -> bool {
        // Invalidate value cache on update
        self.value_cache = None;
        if !self.running && self.sequence.is_none() && self.keyframe_animation.is_none() {
            return false;
        }

        // Sequence support
        if let Some(sequence) = &self.sequence {
            if !self.running {
                let current_step = sequence.current_step;
                let total_steps = sequence.steps.len();
                if current_step < (total_steps - 1) as u8 {
                    let mut new_sequence = (**sequence).clone();
                    new_sequence.current_step = current_step + 1;
                    let next_step = current_step + 1;
                    let step = &sequence.steps[next_step as usize];
                    let target = step.target;
                    let config = (*step.config).clone();
                    self.sequence = Some(Arc::new(new_sequence));
                    self.initial = self.current;
                    self.target = target;
                    self.config = Arc::new(config);
                    self.running = true;
                    self.elapsed = Duration::default();
                    self.delay_elapsed = Duration::default();
                    self.velocity = T::zero();
                    return true;
                } else {
                    let mut sequence_clone = (**sequence).clone();
                    if let Some(on_complete) = sequence_clone.on_complete.take() {
                        on_complete();
                    }
                    self.sequence = None;
                    self.stop();
                    return false;
                }
            }
        }

        // Keyframe animation support
        if let Some(_animation) = &self.keyframe_animation {
            return update_keyframes(self, dt);
        }

        // Skip updates for imperceptible changes
        const MIN_DELTA: f32 = 1.0 / 240.0;
        if dt < MIN_DELTA {
            return true;
        }

        if self.delay_elapsed < self.config.delay {
            self.delay_elapsed += Duration::from_secs_f32(dt);
            return true;
        }

        let completed = match self.config.mode {
            AnimationMode::Spring(spring) => {
                let spring_result = update_spring(self, spring, dt);
                matches!(spring_result, SpringState::Completed)
            }
            AnimationMode::Tween(tween) => update_tween(self, tween, dt),
        };

        if completed {
            handle_completion(self)
        } else {
            true
        }
    }
}

// --- Private helper functions for Motion<T> ---

fn update_spring<T: Animatable>(motion: &mut Motion<T>, spring: Spring, dt: f32) -> SpringState {
    let epsilon = motion.get_epsilon();
    let stiffness = spring.stiffness;
    let damping = spring.damping;
    let mass_inv = 1.0 / spring.mass;

    // Check for completion first
    let delta = motion.target.sub(&motion.current);
    if delta.magnitude() < epsilon && motion.velocity.magnitude() < epsilon {
        motion.current = motion.target;
        motion.velocity = T::zero();
        return SpringState::Completed;
    }

    #[cfg(feature = "web")]
    {
        // Web: Use fixed timestep for better performance
        const FIXED_DT: f32 = 1.0 / 120.0;
        let steps = ((dt / FIXED_DT) as usize).max(1);
        let step_dt = dt / steps as f32;

        for _ in 0..steps {
            let force = delta.scale(stiffness);
            let damping_force = motion.velocity.scale(damping);
            motion.velocity = motion
                .velocity
                .add(&(force.sub(&damping_force)).scale(mass_inv * step_dt));
            motion.current = motion.current.add(&motion.velocity.scale(step_dt));
        }
    }

    #[cfg(not(feature = "web"))]
    {
        // Native: Use RK4 for better accuracy
        struct State<T> {
            pos: T,
            vel: T,
        }

        let derive = |state: &State<T>| -> State<T> {
            let delta = motion.target.sub(&state.pos);
            let force = delta.scale(stiffness);
            let damping_force = state.vel.scale(damping);
            let acc = (force.sub(&damping_force)).scale(mass_inv);
            State {
                pos: state.vel.clone(),
                vel: acc,
            }
        };

        let mut state = State {
            pos: motion.current.clone(),
            vel: motion.velocity.clone(),
        };

        let k1 = derive(&state);
        let k2 = derive(&State {
            pos: state.pos.add(&k1.pos.scale(dt * 0.5)),
            vel: state.vel.add(&k1.vel.scale(dt * 0.5)),
        });
        let k3 = derive(&State {
            pos: state.pos.add(&k2.pos.scale(dt * 0.5)),
            vel: state.vel.add(&k2.vel.scale(dt * 0.5)),
        });
        let k4 = derive(&State {
            pos: state.pos.add(&k3.pos.scale(dt)),
            vel: state.vel.add(&k3.vel.scale(dt)),
        });

        const SIXTH: f32 = 1.0 / 6.0;
        motion.current = state.pos.add(
            &(k1.pos
                .add(&k2.pos.scale(2.0))
                .add(&k3.pos.scale(2.0))
                .add(&k4.pos))
            .scale(dt * SIXTH),
        );
        motion.velocity = state.vel.add(
            &(k1.vel
                .add(&k2.vel.scale(2.0))
                .add(&k3.vel.scale(2.0))
                .add(&k4.vel))
            .scale(dt * SIXTH),
        );
    }

    check_spring_completion(motion)
}

fn check_spring_completion<T: Animatable>(motion: &mut Motion<T>) -> SpringState {
    let epsilon = motion.get_epsilon();
    let epsilon_sq = epsilon * epsilon;
    let velocity_sq = motion.velocity.magnitude().powi(2);
    let delta = motion.target.sub(&motion.current);
    let delta_sq = delta.magnitude().powi(2);
    if velocity_sq < epsilon_sq && delta_sq < epsilon_sq {
        motion.current = motion.target;
        motion.velocity = T::zero();
        SpringState::Completed
    } else {
        SpringState::Active
    }
}

fn update_tween<T: Animatable>(motion: &mut Motion<T>, tween: Tween, dt: f32) -> bool {
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

fn handle_completion<T: Animatable>(motion: &mut Motion<T>) -> bool {
    let should_continue = match motion.config.loop_mode.unwrap_or(LoopMode::None) {
        LoopMode::None => {
            motion.running = false;
            false
        }
        LoopMode::Infinite => {
            motion.current = motion.initial;
            motion.elapsed = Duration::default();
            motion.velocity = T::zero();
            true
        }
        LoopMode::Times(count) => {
            motion.current_loop += 1;
            if motion.current_loop >= count {
                motion.stop();
                false
            } else {
                motion.current = motion.initial;
                motion.elapsed = Duration::default();
                motion.velocity = T::zero();
                true
            }
        }
        LoopMode::Alternate => {
            motion.reverse = !motion.reverse;
            if motion.reverse {
                std::mem::swap(&mut motion.initial, &mut motion.target);
            }
            motion.elapsed = Duration::default();
            motion.velocity = T::zero();
            true
        }
        LoopMode::AlternateTimes(count) => {
            motion.current_loop += 1;
            if motion.current_loop >= count * 2 {
                motion.stop();
                false
            } else {
                motion.reverse = !motion.reverse;
                if motion.reverse {
                    std::mem::swap(&mut motion.initial, &mut motion.target);
                }
                motion.elapsed = Duration::default();
                motion.velocity = T::zero();
                true
            }
        }
    };
    if !should_continue {
        if let Some(ref f) = motion.config.on_complete {
            if let Ok(mut guard) = f.lock() {
                guard();
            }
        }
    }
    should_continue
}

fn update_keyframes<T: Animatable>(motion: &mut Motion<T>, dt: f32) -> bool {
    if let Some(animation) = &motion.keyframe_animation {
        let progress =
            (motion.elapsed.as_secs_f32() / animation.duration.as_secs_f32()).clamp(0.0, 1.0);
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
            handle_completion(motion)
        } else {
            true
        }
    } else {
        false
    }
}
