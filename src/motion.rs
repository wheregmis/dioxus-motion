use crate::Duration;
use crate::animations::core::{
    Animatable, AnimationError, AnimationMode, LoopMode, OnComplete, validate_spring_transition,
    validate_value,
};
use crate::animations::spring::{Spring, SpringState, SpringStep};
use crate::keyframes::{Keyframe, KeyframeAnimation};
use crate::prelude::AnimationConfig;
use crate::sequence::AnimationSequence;

// Returned to the store owner so user code runs after its write guard is released.
pub(crate) enum Completion {
    Animation(OnComplete),
    Sequence(Box<dyn FnOnce() + Send>),
}

impl Completion {
    pub(crate) fn run(self) -> Result<(), AnimationError> {
        match self {
            Self::Animation(callback) => {
                crate::animations::core::execute_completion_callback(&callback)?;
            }
            Self::Sequence(callback) => callback(),
        }
        Ok(())
    }
}

/// Animation state with validated setup and frame updates.
/// Read values through getters and use `set_velocity` to change spring velocity.
///
/// ```
/// use dioxus_motion::motion::Motion;
/// let mut motion = Motion::new(1.0f32).expect("finite initial value");
/// use dioxus_motion::prelude::{AnimationConfig, Spring};
/// motion.animate_to(2.0, AnimationConfig::spring(Spring::default()))
///     .expect("valid spring configuration");
/// motion.set_velocity(2.0).expect("finite velocity during spring playback");
/// assert_eq!(motion.get_value(), 1.0);
/// assert_eq!(motion.get_target(), 2.0);
/// assert_eq!(motion.get_velocity(), 2.0);
/// assert!(motion.is_running());
/// ```
///
/// ```compile_fail,E0616
/// use dioxus_motion::motion::Motion;
/// let mut motion = Motion::new(0.0f32).unwrap();
/// motion.current = f32::NAN;
/// ```
///
/// ```compile_fail,E0616
/// use dioxus_motion::motion::Motion;
/// let mut motion = Motion::new(0.0f32).unwrap();
/// motion.velocity = f32::INFINITY;
/// ```
///
/// ```compile_fail,E0616
/// use dioxus_motion::motion::Motion;
/// let mut motion = Motion::new(0.0f32).unwrap();
/// motion.running = true;
/// ```
#[derive(Clone)]
pub struct Motion<T: Animatable + Send + 'static> {
    initial: T,
    pub(crate) current: T,
    target: T,
    velocity: T,
    pub(crate) running: bool,
    elapsed: Duration,
    delay_elapsed: Duration,
    current_loop: u16,
    reverse: bool,
    config: AnimationConfig,
    spring_step: Option<(f32, SpringStep)>,
    sequence: Option<AnimationSequence<T>>,
    keyframe_animation: Option<KeyframeAnimation<T>>,
}

impl<T: Animatable + Send + 'static> Motion<T> {
    pub fn new(initial: T) -> Result<Self, AnimationError> {
        validate_value(&initial, "initial value")?;
        let velocity = T::default() * 0.0;
        validate_value(&velocity, "zero velocity")?;
        Ok(Self {
            initial: initial.clone(),
            current: initial.clone(),
            target: initial,
            velocity,
            running: false,
            elapsed: Duration::default(),
            delay_elapsed: Duration::default(),
            current_loop: 0,
            reverse: false,
            config: AnimationConfig::default(),
            spring_step: None,
            sequence: None,
            keyframe_animation: None,
        })
    }

    /// Starts an animation, leaving the current animation unchanged on invalid configuration.
    pub fn animate_to(&mut self, target: T, config: AnimationConfig) -> Result<(), AnimationError> {
        config.validate_for::<T>()?;
        validate_value(&self.current, "current value")?;
        validate_value(&target, "target")?;
        validate_spring_transition(&self.current, &target, config.mode)?;
        self.sequence = None;
        self.keyframe_animation = None;
        self.start_animation(target, config);
        Ok(())
    }

    /// Starts a sequence after validating all steps. Errors leave motion state unchanged.
    pub fn animate_sequence(
        &mut self,
        sequence: AnimationSequence<T>,
    ) -> Result<(), AnimationError> {
        if let Some(completion) = self.animate_sequence_with_completion(sequence)? {
            completion.run()?;
        }
        Ok(())
    }

    pub(crate) fn animate_sequence_with_completion(
        &mut self,
        sequence: AnimationSequence<T>,
    ) -> Result<Option<Completion>, AnimationError> {
        sequence.validate()?;
        validate_value(&self.current, "current value")?;
        if let Some(first) = sequence.steps().first() {
            validate_spring_transition(&self.current, &first.target, first.config.mode)?;
        }
        self.stop();
        sequence.reset();
        if let Some(first_step) = sequence.current_step_data() {
            self.start_animation(
                first_step.target.clone(),
                first_step.config.as_ref().clone(),
            );
            self.sequence = Some(sequence);
            Ok(None)
        } else {
            Ok(sequence.take_completion().map(Completion::Sequence))
        }
    }

    /// Starts keyframes after validation. Errors leave the active animation unchanged.
    pub fn animate_keyframes(
        &mut self,
        animation: KeyframeAnimation<T>,
    ) -> Result<(), AnimationError> {
        self.config.validate_for::<T>()?;
        validate_value(&self.current, "current value")?;
        self.sequence = None;
        self.keyframe_animation = Some(animation);
        self.running = true;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::default() * 0.0;
        self.current_loop = 0;
        self.reverse = false;
        Ok(())
    }

    pub fn get_value(&self) -> T {
        self.current.clone()
    }

    /// Returns the current animation target.
    pub fn get_target(&self) -> T {
        self.target.clone()
    }

    /// Returns the current spring velocity in value units per second.
    pub fn get_velocity(&self) -> T {
        self.velocity.clone()
    }

    /// Changes spring velocity without restarting playback. Invalid values leave state unchanged.
    /// Requires an active spring animation. Starting another animation resets velocity to zero.
    pub fn set_velocity(&mut self, velocity: T) -> Result<(), AnimationError> {
        validate_value(&velocity, "velocity")?;
        if !self.running
            || self.keyframe_animation.is_some()
            || !matches!(self.config.mode, AnimationMode::Spring(_))
        {
            return Err(AnimationError::VelocityRequiresSpring);
        }
        validate_spring_transition(&self.current, &velocity, self.config.mode)?;
        validate_spring_transition(&self.target, &velocity, self.config.mode)?;
        self.velocity = velocity;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn reset(&mut self) {
        self.stop();
        self.current = self.initial.clone();
        self.target = self.initial.clone();
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.current_loop = 0;
        self.velocity = T::default() * 0.0;
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

    /// Advances by seconds, ignoring nonpositive or nonfinite deltas.
    /// Spring simulation caps a frame at 100 ms to bound work after stalls.
    /// Nonfinite frame results stop playback without firing completion callbacks
    /// or replacing the last valid value, and return a typed error.
    /// A busy or poisoned completion callback also returns an error after playback finishes.
    pub fn update(&mut self, dt: f32) -> Result<bool, AnimationError> {
        let (running, completion) = self.update_with_completion(dt)?;
        if let Some(completion) = completion {
            completion.run()?;
        }
        Ok(running)
    }

    pub(crate) fn update_with_completion(
        &mut self,
        dt: f32,
    ) -> Result<(bool, Option<Completion>), AnimationError> {
        let result = self.advance_frame(dt);
        if result.is_err() {
            self.stop();
        }
        result
    }

    fn advance_frame(&mut self, dt: f32) -> Result<(bool, Option<Completion>), AnimationError> {
        if !self.running {
            return Ok((false, None));
        }

        if !dt.is_finite() || dt <= 0.0 {
            return Ok((true, None));
        }

        let mut delta = Duration::try_from_secs_f32(dt).unwrap_or(Duration::MAX);
        let remaining_delay = self.config.delay.saturating_sub(self.delay_elapsed);
        if delta < remaining_delay {
            self.delay_elapsed = self.delay_elapsed.saturating_add(delta);
            return Ok((true, None));
        }
        self.delay_elapsed = self.config.delay;
        delta = delta.saturating_sub(remaining_delay);

        if self.keyframe_animation.is_some() {
            if self.update_keyframes(delta)? {
                self.finish_motion();
                return Ok((false, None));
            }
            return Ok((true, None));
        }

        let completed = match self.config.mode {
            AnimationMode::Spring(spring) => {
                let state = self.update_spring(spring, delta.as_secs_f32().min(0.1))?;
                matches!(state, SpringState::Completed)
            }
            AnimationMode::Tween(tween) => self.update_tween(tween, delta)?,
        };

        if !completed {
            return Ok((true, None));
        }

        if self.sequence.is_some() {
            return Ok(self.advance_sequence_step());
        }

        Ok(self.handle_completion())
    }

    fn start_animation(&mut self, target: T, config: AnimationConfig) {
        self.spring_step = None;
        self.initial = self.current.clone();
        self.target = target;
        self.running = true;
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::default() * 0.0;
        self.current_loop = 0;
        self.reverse = false;
        self.config = config;
    }

    fn advance_sequence_step(&mut self) -> (bool, Option<Completion>) {
        let Some(sequence) = self.sequence.as_mut() else {
            return (false, None);
        };

        if sequence.advance_step()
            && let Some(step) = sequence.current_step_data()
        {
            let target = step.target.clone();
            let config = step.config.as_ref().clone();
            self.start_animation(target, config);
            return (true, None);
        }

        let completion = sequence.take_completion().map(Completion::Sequence);
        self.finish_motion();
        (false, completion)
    }

    fn update_keyframes(&mut self, delta: Duration) -> Result<bool, AnimationError> {
        let Some(animation) = self.keyframe_animation.as_ref() else {
            return Ok(true);
        };

        let (current, next_elapsed, completed) = {
            let duration_secs = animation.duration.as_secs_f32();
            let next_elapsed = self.elapsed.saturating_add(delta);
            let next_elapsed_secs = next_elapsed.as_secs_f32();
            let progress = if duration_secs == 0.0 {
                1.0
            } else {
                (next_elapsed_secs / duration_secs).clamp(0.0, 1.0)
            };

            let keyframes = animation.keyframes();
            if keyframes.is_empty() {
                return Ok(true);
            }

            let end_index = keyframe_end_index(keyframes, |frame| frame.offset < progress);
            let (start, end) = if progress < keyframes[0].offset {
                let first = &keyframes[0];
                (first, first)
            } else if end_index < keyframes.len() {
                (&keyframes[end_index - 1], &keyframes[end_index])
            } else if let Some(last) = keyframes.last() {
                (last, last)
            } else {
                return Ok(true);
            };

            let local_progress = if start.offset == end.offset {
                1.0
            } else {
                (progress - start.offset) / (end.offset - start.offset)
            };

            let current = match local_progress {
                0.0 => start.value.clone(),
                1.0 => end.value.clone(),
                _ => {
                    let eased_progress = end
                        .easing
                        .map_or(local_progress, |ease| (ease)(local_progress, 0.0, 1.0, 1.0));
                    validate_value(&eased_progress, "easing progress")?;
                    match eased_progress {
                        0.0 => start.value.clone(),
                        1.0 => end.value.clone(),
                        _ => start.value.interpolate(&end.value, eased_progress),
                    }
                }
            };
            (current, next_elapsed, progress >= 1.0)
        };

        validate_value(&current, "keyframe value")?;
        self.current = current;
        self.elapsed = next_elapsed;

        Ok(completed)
    }

    fn update_spring(&mut self, spring: Spring, dt: f32) -> Result<SpringState, AnimationError> {
        let epsilon = self.get_epsilon();
        let delta = self.target.clone() - self.current.clone();

        if delta.magnitude() < epsilon && self.velocity.magnitude() < epsilon {
            self.current = self.target.clone();
            self.velocity = T::default() * 0.0;
            return Ok(SpringState::Completed);
        }

        let step = match self.spring_step {
            Some((cached_dt, step)) if cached_dt == dt => step,
            _ => {
                let step = spring.step(dt);
                self.spring_step = Some((dt, step));
                step
            }
        };
        let velocity = self.velocity.clone();
        let current = self.current.clone()
            + delta.clone() * step.displacement
            + velocity.clone() * step.position_velocity;
        let velocity = delta * -step.velocity_position + velocity * step.velocity;
        validate_value(&current, "spring value")?;
        validate_value(&velocity, "spring velocity")?;
        self.current = current;
        self.velocity = velocity;

        Ok(self.check_spring_completion())
    }

    fn check_spring_completion(&mut self) -> SpringState {
        let epsilon = self.get_epsilon();
        if self.velocity.magnitude() < epsilon
            && (self.target.clone() - self.current.clone()).magnitude() < epsilon
        {
            self.current = self.target.clone();
            self.velocity = T::default() * 0.0;
            SpringState::Completed
        } else {
            SpringState::Active
        }
    }

    fn update_tween(
        &mut self,
        tween: crate::prelude::Tween,
        delta: Duration,
    ) -> Result<bool, AnimationError> {
        let next_elapsed = self.elapsed.saturating_add(delta);
        let elapsed_secs = next_elapsed.as_secs_f32();
        let duration_secs = tween.duration.as_secs_f32();

        let progress = if duration_secs == 0.0 {
            1.0
        } else {
            (elapsed_secs / duration_secs).min(1.0)
        };

        if progress <= 0.0 {
            self.current = self.initial.clone();
            self.elapsed = next_elapsed;
            return Ok(false);
        }

        if progress >= 1.0 {
            self.current = self.target.clone();
            self.elapsed = next_elapsed;
            return Ok(true);
        }

        let eased_progress = (tween.easing)(progress, 0.0, 1.0, 1.0);
        validate_value(&eased_progress, "easing progress")?;
        let current = match eased_progress {
            0.0 => self.initial.clone(),
            1.0 => self.target.clone(),
            _ => self.initial.interpolate(&self.target, eased_progress),
        };

        validate_value(&current, "tween value")?;
        self.current = current;
        self.elapsed = next_elapsed;
        Ok(false)
    }

    fn complete_animation(&mut self) -> (bool, Option<Completion>) {
        let completion = self.config.on_complete.clone().map(Completion::Animation);
        self.finish_motion();
        (false, completion)
    }

    fn handle_completion(&mut self) -> (bool, Option<Completion>) {
        match self.config.loop_mode.unwrap_or(LoopMode::None) {
            LoopMode::None => self.complete_animation(),
            LoopMode::Infinite => {
                self.restart_motion();
                (true, None)
            }
            LoopMode::Times(count) => {
                self.current_loop += 1;
                if self.current_loop >= u16::from(count) {
                    self.complete_animation()
                } else {
                    self.restart_motion();
                    (true, None)
                }
            }
            LoopMode::Alternate => {
                self.reverse_motion();
                (true, None)
            }
            LoopMode::AlternateTimes(count) => {
                self.current_loop += 1;
                if self.current_loop >= u16::from(count) * 2 {
                    self.complete_animation()
                } else {
                    self.reverse_motion();
                    (true, None)
                }
            }
        }
    }

    fn finish_motion(&mut self) {
        self.running = false;
        self.current_loop = 0;
        self.velocity = T::default() * 0.0;
        self.sequence = None;
        self.keyframe_animation = None;
    }

    fn restart_motion(&mut self) {
        self.current = self.initial.clone();
        self.elapsed = Duration::default();
        self.delay_elapsed = Duration::default();
        self.velocity = T::default() * 0.0;
        self.running = true;
    }

    fn reverse_motion(&mut self) {
        self.reverse = !self.reverse;
        std::mem::swap(&mut self.initial, &mut self.target);
        self.restart_motion();
    }
}

// Countable predicate keeps work bounds testable without instrumenting production benchmarks.
fn keyframe_end_index<T: Animatable>(
    keyframes: &[Keyframe<T>],
    mut before: impl FnMut(&Keyframe<T>) -> bool,
) -> usize {
    // Linear lookup stops early on small tracks; binary search bounds large-track work.
    if keyframes.len() <= 32 {
        keyframes
            .iter()
            .position(|frame| !before(frame))
            .unwrap_or(keyframes.len())
    } else {
        keyframes.partition_point(before)
    }
    .max(1)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::animations::core::AnimationMode;
    use crate::animations::spring::Spring;
    use crate::prelude::Tween;
    use std::sync::{Arc, Mutex};

    #[test]
    fn construction_rejects_nonfinite_default_velocity() {
        #[derive(Clone, PartialEq)]
        struct BadDefault(f32);
        impl Default for BadDefault {
            fn default() -> Self {
                Self(f32::NAN)
            }
        }
        impl std::ops::Add for BadDefault {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }
        impl std::ops::Sub for BadDefault {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }
        impl std::ops::Mul<f32> for BadDefault {
            type Output = Self;
            fn mul(self, factor: f32) -> Self {
                Self(self.0 * factor)
            }
        }
        impl Animatable for BadDefault {
            fn interpolate(&self, _: &Self, _: f32) -> Self {
                self.clone()
            }
            fn magnitude(&self) -> f32 {
                self.0.abs()
            }
            fn is_finite(&self) -> bool {
                self.0.is_finite()
            }
        }
        assert!(matches!(
            Motion::new(BadDefault(1.0)),
            Err(AnimationError::NonFiniteValue("zero velocity"))
        ));
    }

    #[test]
    fn nonfinite_targets_are_rejected_without_replacing_active_motion() {
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut motion = Motion::new(0.0f32).expect("finite initial value");
            motion
                .animate_to(1.0, AnimationConfig::tween_ms(1000))
                .unwrap();
            assert!(motion.update(0.25).expect("representable animation frame"));
            assert_eq!(
                motion.animate_to(bad, instant_tween()),
                Err(AnimationError::NonFiniteValue("target"))
            );
            let sequence = AnimationSequence::new()
                .then(2.0, instant_tween())
                .then(bad, instant_tween());
            assert_eq!(
                motion.animate_sequence(sequence),
                Err(AnimationError::NonFiniteValue("sequence target"))
            );
            assert_eq!(motion.current, 0.25);
            assert_eq!(motion.target, 1.0);
            assert!(!motion.update(0.75).expect("representable animation frame"));
            assert_eq!(motion.current, 1.0);
            assert!(matches!(
                Motion::new(bad),
                Err(AnimationError::NonFiniteValue("initial value"))
            ));
            let mut invalid = Motion::new(0.0f32).expect("finite initial value");
            invalid.current = bad;
            assert_eq!(
                invalid.animate_to(1.0, instant_tween()),
                Err(AnimationError::NonFiniteValue("current value"))
            );
            assert_eq!(
                invalid.animate_sequence(AnimationSequence::new().then(1.0, instant_tween())),
                Err(AnimationError::NonFiniteValue("current value"))
            );
            assert!(!invalid.running);
        }
    }

    #[test]
    fn overflowing_spring_frames_stop_without_committing_or_completing() {
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        for (initial, target, role) in [
            (-f32::MAX, f32::MAX, "spring value"),
            (0.0, f32::MAX, "spring velocity"),
        ] {
            let calls = Arc::new(AtomicUsize::new(0));
            let completed = calls.clone();
            let mut motion = Motion::new(initial).unwrap();
            motion
                .animate_sequence(
                    AnimationSequence::new()
                        .then(target, AnimationConfig::spring(Spring::default()))
                        .on_complete(move || {
                            completed.fetch_add(1, Ordering::Relaxed);
                        }),
                )
                .unwrap();
            assert_eq!(
                motion.update(1.0 / 60.0),
                Err(AnimationError::NonFiniteValue(role))
            );
            assert_eq!(motion.current, initial);
            assert_eq!(motion.velocity, 0.0);
            assert!(!motion.is_running());
            assert!(motion.sequence.is_none());
            assert_eq!(calls.load(Ordering::Relaxed), 0);
            assert_eq!(motion.update(1.0), Ok(false));
            // Error handling does not prevent a later valid animation.
            motion.animate_to(initial, instant_tween()).unwrap();
            assert_eq!(motion.update(1.0), Ok(false));
        }
    }

    #[test]
    fn invalid_easing_stops_tweens_and_keyframes_before_interpolation() {
        fn nan(_: f32, _: f32, _: f32, _: f32) -> f32 {
            f32::NAN
        }
        fn inf(_: f32, _: f32, _: f32, _: f32) -> f32 {
            f32::INFINITY
        }
        fn neg_inf(_: f32, _: f32, _: f32, _: f32) -> f32 {
            f32::NEG_INFINITY
        }
        for easing in [nan, inf, neg_inf] {
            for keyframes in [false, true] {
                let mut motion = Motion::new(0.0f32).unwrap();
                if keyframes {
                    motion
                        .animate_keyframes(
                            KeyframeAnimation::new(Duration::from_secs(1))
                                .add_keyframe(0.0, 0.0, None)
                                .unwrap()
                                .add_keyframe(1.0, 1.0, Some(easing))
                                .unwrap(),
                        )
                        .unwrap();
                } else {
                    motion
                        .animate_to(
                            1.0,
                            AnimationConfig::new(AnimationMode::Tween(
                                Tween::new(Duration::from_secs(1)).with_easing(easing),
                            )),
                        )
                        .unwrap();
                }
                assert_eq!(
                    motion.update(0.25),
                    Err(AnimationError::NonFiniteValue("easing progress"))
                );
                assert_eq!(motion.current, 0.0);
                assert_eq!(motion.elapsed, Duration::ZERO);
                assert!(!motion.running);
                assert!(motion.keyframe_animation.is_none());
            }
        }
    }

    #[test]
    fn custom_interpolation_cannot_commit_nonfinite_frames() {
        #[derive(Clone, Default, PartialEq)]
        struct BadInterpolation(f32);
        impl std::ops::Add for BadInterpolation {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }
        impl std::ops::Sub for BadInterpolation {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }
        impl std::ops::Mul<f32> for BadInterpolation {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self {
                Self(self.0 * rhs)
            }
        }
        impl Animatable for BadInterpolation {
            fn is_finite(&self) -> bool {
                self.0.is_finite()
            }
            fn magnitude(&self) -> f32 {
                self.0.abs()
            }
            fn interpolate(&self, _: &Self, _: f32) -> Self {
                Self(f32::NAN)
            }
        }
        for keyframes in [false, true] {
            let mut motion = Motion::new(BadInterpolation(0.0)).unwrap();
            if keyframes {
                motion
                    .animate_keyframes(
                        KeyframeAnimation::new(Duration::from_secs(1))
                            .add_keyframe(BadInterpolation(0.0), 0.0, None)
                            .unwrap()
                            .add_keyframe(BadInterpolation(1.0), 1.0, None)
                            .unwrap(),
                    )
                    .unwrap();
            } else {
                motion
                    .animate_to(BadInterpolation(1.0), AnimationConfig::tween_ms(1000))
                    .unwrap();
            }
            assert_eq!(
                motion.update(0.25),
                Err(AnimationError::NonFiniteValue(if keyframes {
                    "keyframe value"
                } else {
                    "tween value"
                }))
            );
            assert_eq!(motion.current.0, 0.0);
            assert_eq!(motion.elapsed, Duration::ZERO);
            assert!(!motion.running);
        }
        // Exact eased endpoints copy the value even when custom interpolation is invalid.
        for (easing, expected) in [
            ((|_, _, _, _| 0.0) as crate::keyframes::EasingFn, -f32::MAX),
            ((|_, _, _, _| 1.0) as crate::keyframes::EasingFn, f32::MAX),
        ] {
            let mut motion = Motion::new(BadInterpolation(-f32::MAX)).unwrap();
            motion
                .animate_to(
                    BadInterpolation(f32::MAX),
                    AnimationConfig::new(AnimationMode::Tween(
                        Tween::new(Duration::from_secs(1)).with_easing(easing),
                    )),
                )
                .unwrap();
            assert_eq!(motion.update(0.25), Ok(true));
            assert_eq!(motion.current.0, expected);

            motion
                .animate_keyframes(
                    KeyframeAnimation::new(Duration::from_secs(1))
                        .add_keyframe(BadInterpolation(-f32::MAX), 0.0, None)
                        .unwrap()
                        .add_keyframe(BadInterpolation(f32::MAX), 1.0, Some(easing))
                        .unwrap(),
                )
                .unwrap();
            assert_eq!(motion.update(0.25), Ok(true));
            assert_eq!(motion.current.0, expected);
        }
        // Holding a frame or hitting an offset must bypass easing and interpolation.
        for easing in [
            (|_, _, _, _| f32::NAN) as crate::keyframes::EasingFn,
            |_, _, _, _| 0.25,
        ] {
            for (dt, expected) in [
                (0.25, -f32::MAX),
                (0.5, -f32::MAX),
                (0.75, f32::MAX),
                (0.9, f32::MAX),
                (1.0, f32::MAX),
            ] {
                let mut motion = Motion::new(BadInterpolation(0.0)).unwrap();
                motion
                    .animate_keyframes(
                        KeyframeAnimation::new(Duration::from_secs(1))
                            .add_keyframe(BadInterpolation(-f32::MAX), 0.5, Some(easing))
                            .unwrap()
                            .add_keyframe(BadInterpolation(f32::MAX), 0.75, Some(easing))
                            .unwrap(),
                    )
                    .unwrap();
                assert_eq!(motion.update(dt), Ok(dt < 1.0));
                assert_eq!(motion.current.0, expected);
            }
        }
    }

    #[test]
    fn fuzz_finite_spring_values_never_poison_motion_state() {
        let mut seed = 0x91e1_0da5u32;
        let mut next = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            f32::from_bits(seed)
        };
        let mut errors = 0;
        let mut accepted_frames = 0;
        for _ in 0..4096 {
            let (initial, target) = (next(), next());
            if !initial.is_finite() || !target.is_finite() {
                continue;
            }
            let mut motion = Motion::new(initial).unwrap();
            motion
                .animate_to(target, AnimationConfig::spring(Spring::default()))
                .unwrap();
            for _ in 0..8 {
                let previous = motion.current;
                match motion.update(1.0 / 60.0) {
                    Ok(running) => {
                        accepted_frames += 1;
                        assert!(motion.current.is_finite());
                        assert!(motion.velocity.is_finite());
                        if !running {
                            break;
                        }
                    }
                    Err(_) => {
                        errors += 1;
                        assert_eq!(motion.current, previous);
                        assert!(motion.velocity.is_finite());
                        assert!(!motion.running);
                        break;
                    }
                }
            }
        }
        assert!(errors > 0);
        assert!(accepted_frames > 0);
    }

    #[test]
    fn checked_velocity_changes_preserve_state_and_drive_spring_motion() {
        let mut motion = Motion::new(5.0f32).expect("finite initial value");
        assert_eq!(motion.get_value(), 5.0);
        assert_eq!(motion.get_target(), 5.0);
        assert_eq!(motion.get_velocity(), 0.0);
        assert!(!motion.is_running());
        motion
            .animate_to(
                10.0,
                AnimationConfig::spring(Spring {
                    stiffness: 0.0,
                    damping: 0.0,
                    mass: 1.0,
                }),
            )
            .expect("valid animation configuration");
        assert_eq!(motion.set_velocity(2.0), Ok(()));
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(
                motion.set_velocity(bad),
                Err(AnimationError::NonFiniteValue("velocity"))
            );
            assert_eq!(motion.get_velocity(), 2.0);
            assert_eq!(motion.get_value(), 5.0);
            assert_eq!(motion.get_target(), 10.0);
            assert!(motion.is_running());
        }
        assert_eq!(motion.update(0.1), Ok(true));
        assert!((motion.get_value() - 5.2).abs() < 1e-6);
        assert_eq!(motion.get_velocity(), 2.0);
        motion
            .animate_to(6.0, instant_tween())
            .expect("valid animation configuration");
        assert_eq!(motion.get_velocity(), 0.0);
        assert_eq!(motion.update(0.1), Ok(false));
        assert_eq!(motion.get_value(), 6.0);
        assert_eq!(motion.get_target(), 6.0);
        assert!(!motion.is_running());
    }

    #[test]
    fn velocity_changes_require_active_spring_playback() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        for mode in 0..3 {
            match mode {
                1 => motion
                    .animate_to(1.0, AnimationConfig::tween_ms(1000))
                    .expect("valid animation configuration"),
                2 => {
                    motion
                        .animate_to(1.0, AnimationConfig::spring(Spring::default()))
                        .expect("valid animation configuration");
                    motion
                        .animate_keyframes(KeyframeAnimation::new(Duration::from_secs(1)))
                        .expect("valid keyframe setup");
                }
                _ => {}
            }
            let running = motion.is_running();
            assert_eq!(
                motion.set_velocity(2.0),
                Err(AnimationError::VelocityRequiresSpring)
            );
            assert_eq!(motion.get_value(), 0.0);
            assert_eq!(motion.get_velocity(), 0.0);
            assert_eq!(motion.is_running(), running);
        }
        motion
            .animate_to(1.0, AnimationConfig::spring(Spring::default()))
            .expect("valid animation configuration");
        assert_eq!(motion.set_velocity(2.0), Ok(()));
        motion.stop();
        assert_eq!(
            motion.set_velocity(2.0),
            Err(AnimationError::VelocityRequiresSpring)
        );
        assert_eq!(motion.get_velocity(), 0.0);
    }

    #[test]
    fn checked_compound_velocity_validates_every_component() {
        use crate::animations::colors::Color;
        let mut motion = Motion::new(Color::new(0.0, 0.0, 0.0, 1.0)).expect("finite initial value");
        motion
            .animate_to(
                Color::new(1.0, 0.0, 0.0, 1.0),
                AnimationConfig::spring(Spring::default()),
            )
            .expect("valid animation configuration");
        let velocity = Color {
            r: 2.0,
            g: -1.0,
            b: 0.5,
            a: 0.0,
        };
        assert_eq!(motion.set_velocity(velocity), Ok(()));
        for index in 0..4 {
            for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                let mut invalid = velocity;
                let channels = [
                    &mut invalid.r,
                    &mut invalid.g,
                    &mut invalid.b,
                    &mut invalid.a,
                ];
                *channels[index] = bad;
                assert_eq!(
                    motion.set_velocity(invalid),
                    Err(AnimationError::NonFiniteValue("velocity"))
                );
                assert_eq!(motion.get_velocity(), velocity);
            }
        }
    }

    #[test]
    fn repeated_frame_delta_reuses_spring_coefficients() {
        use crate::animations::spring::STEP_CALCULATIONS;
        STEP_CALCULATIONS.set(0);
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(1.0, AnimationConfig::spring(Spring::default()))
            .unwrap();
        for (dt, calculations) in [(0.02, 1), (0.02, 1), (0.01, 2), (0.01, 2)] {
            assert!(motion.update(dt).expect("representable animation frame"));
            assert_eq!(STEP_CALCULATIONS.get(), calculations);
        }
        motion
            .animate_to(
                2.0,
                AnimationConfig::spring(Spring {
                    stiffness: 200.0,
                    ..Spring::default()
                }),
            )
            .unwrap();
        assert!(motion.update(0.01).expect("representable animation frame"));
        assert_eq!(STEP_CALCULATIONS.get(), 3);
        motion.stop();
        assert!(!motion.update(0.01).expect("representable animation frame"));
        assert_eq!(STEP_CALCULATIONS.get(), 3);
    }

    #[test]
    fn overdamped_springs_preserve_small_displacements() {
        let mut light = Motion::new(0.0f32).expect("finite initial value");
        light
            .animate_to(
                1.0,
                AnimationConfig::spring(Spring {
                    mass: 1e-30,
                    ..Spring::default()
                }),
            )
            .unwrap();
        light
            .update(1.0 / 60.0)
            .expect("representable animation frame");
        // Negligible mass gives x(t)=1-exp(-10t), v(t)=10exp(-10t).
        assert!((light.current - 0.153_518_27).abs() < 1e-6);
        assert!((light.velocity - 8.464_818).abs() < 1e-5);
        let mut damped = Motion::new(0.0f32).expect("finite initial value");
        damped
            .animate_to(
                1.0,
                AnimationConfig::spring(Spring {
                    damping: 1e30,
                    ..Spring::default()
                }),
            )
            .unwrap();
        damped
            .update(1.0 / 60.0)
            .expect("representable animation frame");
        assert!(damped.current > 1e-30 && damped.current < 2e-30);
        assert!(damped.velocity > 9e-29 && damped.velocity < 1.1e-28);
    }

    #[test]
    fn extreme_springs_stay_finite_and_step_cache_tracks_configuration_and_delta() {
        for (stiffness, damping, mass) in
            [(1e6, 10.0, 1.0), (100.0, 10.0, 1e-30), (100.0, 1e30, 1.0)]
        {
            let mut motion = Motion::new(0.0f32).expect("finite initial value");
            motion
                .animate_to(
                    1.0,
                    AnimationConfig::spring(Spring {
                        stiffness,
                        damping,
                        mass,
                    }),
                )
                .unwrap();
            for _ in 0..1000 {
                motion
                    .update(1.0 / 60.0)
                    .expect("representable animation frame");
                assert!(motion.current.is_finite());
                assert!(motion.velocity.is_finite());
                if !motion.running {
                    break;
                }
            }
        }
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(1.0, AnimationConfig::spring(Spring::default()))
            .unwrap();
        motion.update(0.02).expect("representable animation frame");
        let config = AnimationConfig::spring(Spring {
            stiffness: 1e6,
            ..Spring::default()
        });
        motion.animate_to(1.0, config.clone()).unwrap();
        for dt in [0.02, 0.001, 0.1, 0.02] {
            let mut reference = Motion::new(motion.current).expect("finite initial value");
            reference.animate_to(1.0, config.clone()).unwrap();
            reference.velocity = motion.velocity;
            assert_eq!(
                motion.update(dt).expect("representable animation frame"),
                reference.update(dt).expect("representable animation frame")
            );
            assert_eq!(motion.current, reference.current);
            assert_eq!(motion.velocity, reference.velocity);
        }
    }

    #[test]
    fn spring_completion_does_not_square_extreme_thresholds() {
        for epsilon in [f32::from_bits(1), 1e-30, 0.125, 1e30, f32::MAX] {
            let mut motion = Motion::new(0.0f32).expect("finite initial value");
            motion
                .animate_to(0.0, instant_tween().with_epsilon(epsilon))
                .unwrap();
            assert_eq!(motion.check_spring_completion(), SpringState::Completed);
            motion.current = -epsilon;
            motion.velocity = 0.0;
            assert_eq!(motion.check_spring_completion(), SpringState::Active);
            motion.current = 0.0;
            motion.velocity = epsilon;
            assert_eq!(motion.check_spring_completion(), SpringState::Active);
        }
    }

    #[test]
    fn every_style_spring_component_follows_scalar_motion() {
        use crate::prelude::MotionStyle;
        fn components(style: &MotionStyle) -> [f32; 16] {
            [
                style.opacity,
                style.x,
                style.y,
                style.z,
                style.scale,
                style.scale_x,
                style.scale_y,
                style.scale_z,
                style.rotate,
                style.rotate_x,
                style.rotate_y,
                style.rotate_z,
                style.skew,
                style.skew_x,
                style.skew_y,
                style.perspective,
            ]
        }
        let make_style = |offset: f32| MotionStyle {
            opacity: offset + 1.0,
            x: offset + 2.0,
            y: offset + 3.0,
            z: offset + 4.0,
            scale: offset + 5.0,
            scale_x: offset + 6.0,
            scale_y: offset + 7.0,
            scale_z: offset + 8.0,
            rotate: offset + 9.0,
            rotate_x: offset + 10.0,
            rotate_y: offset + 11.0,
            rotate_z: offset + 12.0,
            skew: offset + 13.0,
            skew_x: offset + 14.0,
            skew_y: offset + 15.0,
            perspective: offset + 16.0,
            ..MotionStyle::default()
        };
        let initial = make_style(-4.25);
        let target = make_style(7.5);
        let config = AnimationConfig::spring(Spring::default());
        let mut scalars: Vec<_> = components(&initial)
            .into_iter()
            .zip(components(&target))
            .map(|(initial, target)| {
                let mut motion = Motion::new(initial).expect("finite scalar");
                motion
                    .animate_to(target, config.clone())
                    .expect("valid spring");
                motion
            })
            .collect();
        let mut style = Motion::new(initial).expect("finite style");
        style.animate_to(target, config).expect("valid spring");
        for frame in 0..20 {
            style.update(1.0 / 60.0).expect("finite style frame");
            for (component, scalar) in components(&style.get_value()).into_iter().zip(&mut scalars)
            {
                scalar.update(1.0 / 60.0).expect("finite scalar frame");
                assert!(
                    (component - scalar.get_value()).abs() < 0.0001,
                    "style component diverged at frame {frame}: {component} versus {}",
                    scalar.get_value()
                );
            }
        }
    }

    #[test]
    fn removing_a_numeric_style_property_springs_toward_zero() {
        use crate::prelude::{CssValue, MotionStyle};
        let initial = MotionStyle::default().property("width", CssValue::Px(100.0));
        let target = MotionStyle::default();
        let mut motion = Motion::new(initial).expect("finite initial style");
        motion
            .animate_to(
                target.clone(),
                AnimationConfig::spring(Spring {
                    stiffness: 100.0,
                    damping: 20.0,
                    mass: 1.0,
                }),
            )
            .expect("valid spring");
        motion.update(1.0 / 60.0).expect("finite frame");
        let actual = motion.get_value();
        assert!(
            matches!(actual.properties.get("width"), Some(CssValue::Px(width)) if (0.0..100.0).contains(width)),
            "removed width moved away from zero: {actual:?}"
        );
        for _ in 0..1000 {
            if !motion.update(1.0 / 60.0).expect("finite frame") {
                break;
            }
        }
        assert!(!motion.is_running());
        assert_eq!(motion.get_value(), target);
    }

    #[test]
    fn incompatible_style_springs_preserve_playback_and_allow_tweens() {
        use crate::animations::css::parse_css_string;
        use crate::prelude::{CssValue, MotionStyle};
        for (initial_value, target_value) in [
            (CssValue::Px(100.0), CssValue::Percent(10.0)),
            (CssValue::Number(10.0), CssValue::Px(10.0)),
            (CssValue::Keyword("auto".into()), CssValue::Px(10.0)),
            (
                parse_css_string("translateX(10px)"),
                parse_css_string("translateY(20px)"),
            ),
        ] {
            let initial = MotionStyle::default().property("width", initial_value);
            let target = MotionStyle::default().property("width", target_value);
            let mut motion = Motion::new(initial.clone()).expect("finite style");
            motion
                .animate_to(initial.clone(), AnimationConfig::tween_ms(1000))
                .expect("valid tween");
            assert_eq!(
                motion.animate_to(target.clone(), AnimationConfig::spring(Spring::default())),
                Err(AnimationError::IncompatibleSpringValues)
            );
            assert_eq!(motion.get_value(), initial);
            assert_eq!(motion.get_target(), initial);
            assert!(motion.is_running());
            let invalid_first = AnimationSequence::new()
                .then(target.clone(), AnimationConfig::spring(Spring::default()));
            assert_eq!(
                motion.animate_sequence(invalid_first),
                Err(AnimationError::IncompatibleSpringValues)
            );
            let invalid_later = AnimationSequence::new()
                .then(initial.clone(), AnimationConfig::tween_ms(0))
                .then(target.clone(), AnimationConfig::spring(Spring::default()));
            assert_eq!(
                invalid_later.validate(),
                Err(AnimationError::IncompatibleSpringValues)
            );
            assert_eq!(
                motion.animate_sequence(invalid_later),
                Err(AnimationError::IncompatibleSpringValues)
            );
            assert_eq!(motion.get_value(), initial);
            assert!(motion.is_running());
            motion
                .animate_to(target.clone(), AnimationConfig::tween_ms(0))
                .expect("discrete tween");
            assert_eq!(motion.update(0.01), Ok(false));
            assert_eq!(motion.get_value(), target);
        }
        let initial = MotionStyle::default().property("width", CssValue::Px(100.0));
        let mut motion = Motion::new(initial.clone()).expect("finite style");
        motion
            .animate_to(initial, AnimationConfig::spring(Spring::default()))
            .expect("compatible spring");
        let before = motion.get_velocity();
        assert_eq!(
            motion.set_velocity(MotionStyle::default().property("width", CssValue::Percent(10.0))),
            Err(AnimationError::IncompatibleSpringValues)
        );
        assert_eq!(motion.get_velocity(), before);
        assert!(motion.is_running());
    }

    #[test]
    fn style_alpha_only_springs_follow_scalar_motion() {
        use crate::prelude::{CssColor, CssValue, MotionStyle};
        for complex in [false, true] {
            let mut initial = MotionStyle::default();
            let mut target = MotionStyle::default();
            if complex {
                initial.add_css_property("box-shadow", "0px 0px 0px rgba(0, 0, 0, 0)");
                target.add_css_property("box-shadow", "0px 0px 0px rgba(0, 0, 0, 1)");
            } else {
                initial.properties.insert(
                    "color".into(),
                    CssValue::Color(CssColor::rgba(0.0, 0.0, 0.0, 0.0)),
                );
                target.properties.insert(
                    "color".into(),
                    CssValue::Color(CssColor::rgba(0.0, 0.0, 0.0, 1.0)),
                );
            }
            let mut style = Motion::new(initial.clone()).expect("finite initial value");
            let mut scalar = Motion::new(0.0f32).expect("finite initial value");
            let config = AnimationConfig::spring(Spring::default());
            style.animate_to(target.clone(), config.clone()).unwrap();
            scalar.animate_to(1.0, config).unwrap();
            for _ in 0..1000 {
                let scalar_running = scalar
                    .update(1.0 / 120.0)
                    .expect("representable animation frame");
                assert_eq!(
                    style
                        .update(1.0 / 120.0)
                        .expect("representable animation frame"),
                    scalar_running
                );
                assert!(
                    ((style.current.clone() - initial.clone()).magnitude() - scalar.current.abs())
                        .abs()
                        < 1e-5
                );
                assert!((style.velocity.magnitude() - scalar.velocity.abs()).abs() < 1e-5);
                if !scalar_running {
                    break;
                }
            }
            assert!(!style.running);
            assert_eq!(style.current, target);
        }
    }

    #[test]
    fn rejected_setup_preserves_running_animation_and_callbacks() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let calls = Arc::new(AtomicUsize::new(0));
        let completed = calls.clone();
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(
                1.0,
                AnimationConfig::tween_ms(1000).with_on_complete(move || {
                    completed.fetch_add(1, Ordering::Relaxed);
                }),
            )
            .unwrap();
        assert!(motion.update(0.25).expect("representable animation frame"));
        let rejected_calls = calls.clone();
        assert_eq!(
            motion.animate_to(
                99.0,
                instant_tween()
                    .with_epsilon(f32::NAN)
                    .with_on_complete(move || {
                        rejected_calls.fetch_add(100, Ordering::Relaxed);
                    })
            ),
            Err(AnimationError::InvalidEpsilon)
        );
        let rejected_calls = calls.clone();
        let sequence = AnimationSequence::new()
            .then(2.0, instant_tween())
            .then(
                3.0,
                AnimationConfig::spring(Spring {
                    mass: 0.0,
                    ..Spring::default()
                }),
            )
            .on_complete(move || {
                rejected_calls.fetch_add(100, Ordering::Relaxed);
            });
        assert_eq!(
            motion.animate_sequence(sequence),
            Err(AnimationError::InvalidSpringParameter("mass"))
        );
        assert_eq!(motion.current, 0.25);
        assert_eq!(motion.target, 1.0);
        assert_eq!(motion.elapsed, Duration::from_millis(250));
        assert!(motion.sequence.is_none());
        assert!(motion.keyframe_animation.is_none());
        assert!(!motion.update(0.75).expect("representable animation frame"));
        assert_eq!(motion.current, 1.0);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn invalid_type_epsilon_requires_a_valid_override() {
        #[derive(Clone, Default, PartialEq)]
        struct InvalidEpsilon;
        impl std::ops::Add for InvalidEpsilon {
            type Output = Self;
            fn add(self, _: Self) -> Self {
                self
            }
        }
        impl std::ops::Sub for InvalidEpsilon {
            type Output = Self;
            fn sub(self, _: Self) -> Self {
                self
            }
        }
        impl std::ops::Mul<f32> for InvalidEpsilon {
            type Output = Self;
            fn mul(self, _: f32) -> Self {
                self
            }
        }
        impl Animatable for InvalidEpsilon {
            fn is_finite(&self) -> bool {
                true
            }

            fn interpolate(&self, _: &Self, _: f32) -> Self {
                self.clone()
            }
            fn magnitude(&self) -> f32 {
                0.0
            }
            fn epsilon() -> f32 {
                f32::NAN
            }
        }
        let mut motion = Motion::new(InvalidEpsilon).expect("finite initial value");
        assert_eq!(
            motion.animate_to(InvalidEpsilon, instant_tween()),
            Err(AnimationError::InvalidEpsilon)
        );
        assert_eq!(
            motion.animate_sequence(AnimationSequence::new().then(InvalidEpsilon, instant_tween())),
            Err(AnimationError::InvalidEpsilon)
        );
        assert_eq!(
            motion.animate_keyframes(KeyframeAnimation::new(Duration::ZERO)),
            Err(AnimationError::InvalidEpsilon)
        );
        assert!(!motion.running);
        assert_eq!(
            motion.animate_to(InvalidEpsilon, instant_tween().with_epsilon(0.125)),
            Ok(())
        );
        assert!(!motion.update(0.01).expect("representable animation frame"));
        assert_eq!(
            motion.animate_keyframes(KeyframeAnimation::new(Duration::ZERO)),
            Ok(())
        );
        assert!(!motion.update(0.01).expect("representable animation frame"));
    }

    #[test]
    fn keyframe_setup_rejects_corrupted_current_without_replacing_playback() {
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            for keyframes in [false, true] {
                let mut motion = Motion::new(0.0f32).unwrap();
                let track = KeyframeAnimation::new(Duration::from_secs(1))
                    .add_keyframe(0.0, 0.0, None)
                    .unwrap()
                    .add_keyframe(1.0, 1.0, None)
                    .unwrap();
                if keyframes {
                    motion.animate_keyframes(track.clone()).unwrap();
                } else {
                    motion
                        .animate_sequence(
                            AnimationSequence::new().then(1.0, AnimationConfig::tween_ms(1000)),
                        )
                        .unwrap();
                }
                assert!(motion.update(0.25).expect("representable animation frame"));
                motion.current = bad;
                assert_eq!(
                    motion.animate_keyframes(track),
                    Err(AnimationError::NonFiniteValue("current value"))
                );
                assert_eq!(motion.current.to_bits(), bad.to_bits());
                assert!(motion.running);
                assert_eq!(motion.elapsed, Duration::from_millis(250));
                assert_eq!(motion.keyframe_animation.is_some(), keyframes);
                assert_eq!(motion.sequence.is_some(), !keyframes);
                // Restoring the public value lets the original playback finish.
                motion.current = 0.25;
                assert!(!motion.update(0.75).expect("representable animation frame"));
                assert_eq!(motion.current, 1.0);
            }
        }
    }

    #[test]
    fn rejected_setup_preserves_sequence_and_keyframes() {
        for keyframes in [false, true] {
            let mut motion = Motion::new(0.0f32).expect("finite initial value");
            if keyframes {
                motion
                    .animate_keyframes(
                        KeyframeAnimation::new(Duration::from_secs(1))
                            .add_keyframe(0.0, 0.0, None)
                            .unwrap()
                            .add_keyframe(1.0, 1.0, None)
                            .unwrap(),
                    )
                    .expect("valid keyframe setup");
            } else {
                motion
                    .animate_sequence(
                        AnimationSequence::new().then(1.0, AnimationConfig::tween_ms(1000)),
                    )
                    .unwrap();
            }
            assert!(motion.update(0.25).expect("representable animation frame"));
            assert_eq!(
                motion.animate_to(99.0, instant_tween().with_epsilon(0.0)),
                Err(AnimationError::InvalidEpsilon)
            );
            assert_eq!(
                motion.animate_sequence(
                    AnimationSequence::new().then(99.0, instant_tween().with_epsilon(0.0))
                ),
                Err(AnimationError::InvalidEpsilon)
            );
            assert_eq!(motion.sequence.is_some(), !keyframes);
            assert_eq!(motion.keyframe_animation.is_some(), keyframes);
            assert_eq!(motion.current, 0.25);
            assert_eq!(motion.elapsed, Duration::from_millis(250));
            assert!(!motion.update(0.75).expect("representable animation frame"));
            assert_eq!(motion.current, 1.0);
        }
    }

    fn instant_tween() -> AnimationConfig {
        AnimationConfig::new(AnimationMode::Tween(Tween::new(Duration::from_secs(0))))
    }

    #[test]
    fn velocity_is_zero_through_every_animation_lifecycle() {
        use crate::animations::{colors::Color, style::MotionStyle, transform::Transform};
        fn check<T: Animatable + Send>() {
            let mut motion = Motion::new(T::default()).expect("finite initial value");
            assert_eq!(motion.velocity.magnitude(), 0.0);
            motion
                .animate_to(T::default(), instant_tween().with_loop(LoopMode::Times(2)))
                .expect("valid animation configuration");
            assert_eq!(motion.velocity.magnitude(), 0.0);
            assert!(motion.update(0.01).expect("representable animation frame"));
            assert_eq!(motion.velocity.magnitude(), 0.0);
            assert!(!motion.update(0.01).expect("representable animation frame"));
            assert_eq!(motion.velocity.magnitude(), 0.0);
            motion
                .animate_to(T::default(), AnimationConfig::spring(Spring::default()))
                .expect("valid animation configuration");
            assert!(!motion.update(0.01).expect("representable animation frame"));
            assert_eq!(motion.velocity.magnitude(), 0.0);
            motion
                .animate_keyframes(KeyframeAnimation::new(Duration::ZERO))
                .expect("valid keyframe setup");
            assert_eq!(motion.velocity.magnitude(), 0.0);
            motion.stop();
            assert_eq!(motion.velocity.magnitude(), 0.0);
            motion.reset();
            assert_eq!(motion.velocity.magnitude(), 0.0);
        }
        check::<f32>();
        check::<Transform>();
        check::<Color>();
        check::<MotionStyle>();
    }

    #[test]
    fn compound_springs_match_scalar_motion_in_both_directions() {
        use crate::animations::{colors::Color, transform::Transform};
        for (start, target) in [(1.0f32, 0.0f32), (0.0, 1.0)] {
            let config = AnimationConfig::spring(Spring::default());
            let mut scalar = Motion::new(start).expect("finite initial value");
            let mut color =
                Motion::new(Color::new(start, 0.0, 0.0, 1.0)).expect("finite initial value");
            let mut transform =
                Motion::new(Transform::new(0.0, 0.0, start, 0.0)).expect("finite initial value");
            scalar
                .animate_to(target, config.clone())
                .expect("valid animation configuration");
            color
                .animate_to(Color::new(target, 0.0, 0.0, 1.0), config.clone())
                .expect("valid animation configuration");
            transform
                .animate_to(Transform::new(0.0, 0.0, target, 0.0), config)
                .expect("valid animation configuration");
            assert_eq!(color.velocity.magnitude(), 0.0);
            assert_eq!(transform.velocity.magnitude(), 0.0);
            for _ in 0..600 {
                let running = scalar
                    .update(1.0 / 60.0)
                    .expect("representable animation frame");
                assert_eq!(
                    color
                        .update(1.0 / 60.0)
                        .expect("representable animation frame"),
                    running
                );
                assert_eq!(
                    transform
                        .update(1.0 / 60.0)
                        .expect("representable animation frame"),
                    running
                );
                assert!((color.current.r - scalar.current).abs() < 0.000001);
                assert!((transform.current.scale - scalar.current).abs() < 0.000001);
                assert_eq!(color.current.a, 1.0);
                if !running {
                    break;
                }
            }
            assert!(!scalar.is_running());
            assert_eq!(color.current.r, target);
            assert_eq!(transform.current.scale, target);
            assert_eq!(color.velocity.magnitude(), 0.0);
            assert_eq!(transform.velocity.magnitude(), 0.0);
        }
    }

    #[test]
    fn spring_step_matches_reference_values() {
        let mut motion = Motion::new(5.0f32).expect("finite initial value");
        motion
            .animate_to(
                -3.0,
                AnimationConfig::spring(Spring {
                    stiffness: 140.0,
                    damping: 6.0,
                    mass: 2.0,
                }),
            )
            .expect("valid animation configuration");
        motion.velocity = 2.0;
        assert!(motion.update(0.02).expect("representable animation frame"));
        // Independent closed-form reference for a damped harmonic oscillator.
        let (position, velocity) = (4.929_104_5, -8.963_278);
        assert!((motion.current - position).abs() < 0.00001);
        assert!((motion.velocity - velocity).abs() < 0.00001);
    }

    #[test]
    fn spring_completion_requires_position_and_velocity_below_epsilon() {
        let static_spring = Spring {
            stiffness: 0.0,
            damping: 0.0,
            ..Spring::default()
        };
        for (offset, velocity, completed) in [
            (0.125, 0.0, false),
            (0.0, 0.125, false),
            (0.2, 0.0, false),
            (0.0, 0.2, false),
            (0.13, -0.1, true),
        ] {
            let mut motion = Motion::new(5.0f32 + offset).expect("finite initial value");
            motion
                .animate_to(
                    5.0,
                    AnimationConfig::spring(static_spring).with_epsilon(0.125),
                )
                .expect("valid animation configuration");
            motion.velocity = velocity;
            assert_eq!(
                motion.update(0.1).expect("representable animation frame"),
                !completed,
                "offset={offset}, velocity={velocity}"
            );
            if completed {
                assert_eq!(motion.current, 5.0);
                assert_eq!(motion.velocity, 0.0);
            }
        }
        let mut settled = Motion::new(5.0625f32).expect("finite initial value");
        settled
            .animate_to(
                5.0,
                AnimationConfig::spring(Spring::default()).with_epsilon(0.125),
            )
            .expect("valid animation configuration");
        settled.velocity = 0.0625;
        assert!(!settled.update(0.1).expect("representable animation frame"));
        assert_eq!(settled.current, 5.0);
        assert_eq!(settled.velocity, 0.0);
    }

    #[test]
    fn reset_restores_animation_start_and_clears_runtime_state() {
        let mut motion = Motion::new(7.0f32).expect("finite initial value");
        motion
            .animate_to(
                20.0,
                AnimationConfig::tween_ms(1000).with_delay(Duration::from_millis(50)),
            )
            .expect("valid animation configuration");
        assert!(motion.update(0.1).expect("representable animation frame"));
        assert!(motion.current > 7.0);
        motion.reset();
        assert_eq!(motion.current, 7.0);
        assert_eq!(motion.target, 7.0);
        assert_eq!(motion.elapsed, Duration::ZERO);
        assert_eq!(motion.delay_elapsed, Duration::ZERO);
        assert_eq!(motion.velocity, 0.0);
        assert_eq!(motion.current_loop, 0);
        assert!(!motion.reverse);
        assert!(!motion.is_running());
        assert!(motion.sequence.is_none());
        assert!(motion.keyframe_animation.is_none());
    }

    #[test]
    fn tween_duration_and_eased_endpoints_are_exact() {
        let mut motion = Motion::new(10.0f32).expect("finite initial value");
        motion
            .animate_to(30.0, AnimationConfig::tween(Duration::from_secs(2)))
            .expect("valid animation configuration");
        assert!(motion.update(0.5).expect("representable animation frame"));
        assert_eq!(motion.current, 15.0);

        for (easing, expected) in [
            ((|_, _, _, _| 0.0) as crate::keyframes::EasingFn, -f32::MAX),
            ((|_, _, _, _| 1.0) as crate::keyframes::EasingFn, f32::MAX),
        ] {
            let mut motion = Motion::new(-f32::MAX).expect("finite initial value");
            motion
                .animate_to(
                    f32::MAX,
                    AnimationConfig::new(AnimationMode::Tween(
                        Tween::new(Duration::from_secs(2)).with_easing(easing),
                    )),
                )
                .expect("valid animation configuration");
            assert!(motion.update(0.5).expect("representable animation frame"));
            assert_eq!(motion.current, expected);
        }
    }

    #[test]
    fn keyframe_lookup_stops_early_on_small_tracks_and_is_logarithmic_on_large_tracks() {
        for (count, progress, expected_index, maximum_comparisons) in
            [(32, 1.0 / 64.0, 1, 2), (1024, 0.5, 512, 11)]
        {
            let frames: Vec<_> = (0..count)
                .map(|index| Keyframe {
                    value: index as f32,
                    offset: index as f32 / count as f32,
                    easing: None,
                })
                .collect();
            let mut comparisons = 0;
            let index = keyframe_end_index(&frames, |frame| {
                comparisons += 1;
                frame.offset < progress
            });
            assert_eq!(index, expected_index);
            assert!(
                comparisons <= maximum_comparisons,
                "{count} frames required {comparisons} comparisons"
            );
        }
    }

    #[test]
    fn keyframe_binary_lookup_matches_linear_interpolation() {
        fn quadratic(t: f32, _: f32, _: f32, _: f32) -> f32 {
            t * t
        }

        let cases = [
            vec![0.0, 0.125, 0.25, 0.5, 0.75, 1.0],
            vec![0.0, 0.0, 0.5, 1.0, 1.0],
            vec![0.25, 0.25, 0.5, 0.75, 0.75],
            vec![0.5],
            (0..=64).map(|index| index as f32 / 64.0).collect(),
            (0..=96).map(|index| (index / 3) as f32 / 32.0).collect(),
        ];
        for offsets in cases {
            let mut animation = KeyframeAnimation::new(Duration::from_secs(2));
            for (index, offset) in offsets.iter().enumerate() {
                animation = animation
                    .add_keyframe((index + 1) as f32 * 10.0, *offset, Some(quadratic))
                    .unwrap();
            }
            for tick in 0..=1000 {
                let dt = if tick == 0 {
                    f32::MIN_POSITIVE
                } else {
                    tick as f32 / 500.0
                };
                let mut motion = Motion::new(0.0f32).expect("finite initial value");
                motion
                    .animate_keyframes(animation.clone())
                    .expect("valid keyframe setup");
                motion.update(dt).expect("representable animation frame");
                let progress = motion.elapsed.as_secs_f32() / animation.duration.as_secs_f32();
                let frames = animation.keyframes();
                let (start, end) = match frames
                    .windows(2)
                    .find(|pair| progress >= pair[0].offset && progress <= pair[1].offset)
                {
                    Some(pair) => (&pair[0], &pair[1]),
                    None if progress < frames[0].offset => (&frames[0], &frames[0]),
                    None => {
                        let last = frames.last().unwrap();
                        (last, last)
                    }
                };
                let local = if start.offset == end.offset {
                    1.0
                } else {
                    (progress - start.offset) / (end.offset - start.offset)
                };
                let expected = start
                    .value
                    .interpolate(&end.value, quadratic(local, 0.0, 1.0, 1.0));
                assert!(
                    (motion.current - expected).abs() < 0.0001,
                    "offsets={offsets:?}, progress={progress}, actual={}, expected={expected}",
                    motion.current
                );
            }
        }
    }

    #[test]
    fn sequence_replaces_keyframes_and_handles_empty_input() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_keyframes(
                KeyframeAnimation::new(Duration::from_secs(1))
                    .add_keyframe(50.0, 0.0, None)
                    .unwrap()
                    .add_keyframe(100.0, 1.0, None)
                    .unwrap(),
            )
            .expect("valid keyframe setup");
        motion
            .animate_sequence(AnimationSequence::new().then(2.0, instant_tween()))
            .expect("valid animation configuration");
        assert!(motion.keyframe_animation.is_none());
        assert!(!motion.update(0.01).expect("representable animation frame"));
        assert_eq!(motion.current, 2.0);

        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let callback_calls = calls.clone();
        motion
            .animate_to(10.0, AnimationConfig::tween_ms(1000))
            .expect("valid animation configuration");
        motion
            .animate_sequence(AnimationSequence::new().on_complete(move || {
                callback_calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }))
            .expect("valid animation configuration");
        assert!(!motion.is_running());
        assert!(motion.sequence.is_none());
        assert!(!motion.update(0.01).expect("representable animation frame"));
        assert_eq!(motion.current, 2.0);
        assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 1);
    }

    #[test]
    fn frame_delta_boundaries() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(1.0, AnimationConfig::tween(Duration::from_secs(1)))
            .expect("valid animation configuration");
        for dt in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -1.0, 0.0] {
            assert!(motion.update(dt).expect("representable animation frame"));
            assert_eq!(motion.current, 0.0);
            assert_eq!(motion.elapsed, Duration::ZERO);
        }
        assert!(
            motion
                .update(1.0 / 1000.0)
                .expect("representable animation frame")
        );
        assert!(motion.current > 0.0);
        assert!(
            !motion
                .update(f32::MAX)
                .expect("representable animation frame")
        );
        assert_eq!(motion.current, 1.0);
    }

    #[test]
    fn delay_preserves_frame_remainder() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(
                1.0,
                AnimationConfig::tween(Duration::from_secs(1))
                    .with_delay(Duration::from_millis(250)),
            )
            .expect("valid animation configuration");
        assert!(motion.update(0.125).expect("representable animation frame"));
        assert_eq!(motion.delay_elapsed, Duration::from_millis(125));
        assert_eq!(motion.current, 0.0);
        assert!(motion.update(0.375).expect("representable animation frame"));
        assert_eq!(motion.elapsed, Duration::from_millis(250));
        assert!(motion.current > 0.0);
        assert!(!motion.update(0.75).expect("representable animation frame"));
        assert_eq!(motion.current, 1.0);
    }

    #[test]
    fn instant_tween_completes_at_delay_boundary() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(1.0, instant_tween().with_delay(Duration::from_millis(250)))
            .expect("valid animation configuration");
        assert!(!motion.update(0.25).expect("representable animation frame"));
        assert_eq!(motion.current, 1.0);
    }

    #[test]
    fn alternate_loop_counts_cover_entire_u8_range() {
        for count in 0..=u8::MAX {
            let mut motion = Motion::new(0.0f32).expect("finite initial value");
            motion
                .animate_to(
                    1.0,
                    instant_tween().with_loop(LoopMode::AlternateTimes(count)),
                )
                .expect("valid animation configuration");
            let frames = (u16::from(count) * 2).max(1);
            for frame in 1..=frames {
                assert_eq!(
                    motion.update(0.01).expect("representable animation frame"),
                    frame < frames,
                    "count={count}, frame={frame}"
                );
            }
            assert_eq!(motion.current, if count == 0 { 1.0 } else { 0.0 });
        }
    }

    #[test]
    fn fuzz_frame_delta_bits_keep_default_spring_finite() {
        let mut bits = 1u32;
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        for _ in 0..4096 {
            bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
            motion
                .animate_to(1.0, AnimationConfig::spring(Spring::default()))
                .expect("valid animation configuration");
            motion
                .update(f32::from_bits(bits))
                .expect("representable animation frame");
            assert!(motion.current.is_finite(), "delta bits={bits:#x}");
            assert!(motion.velocity.is_finite(), "delta bits={bits:#x}");
        }
    }

    #[test]
    fn test_motion_new() {
        let motion = Motion::new(0.0f32).expect("finite initial value");

        assert_eq!(motion.initial, 0.0);
        assert_eq!(motion.current, 0.0);
        assert_eq!(motion.target, 0.0);
        assert!(!motion.running);
        assert!(motion.sequence.is_none());
        assert!(motion.keyframe_animation.is_none());
    }

    #[test]
    fn test_motion_animate_to() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(
                100.0,
                AnimationConfig::new(AnimationMode::Tween(Tween::default())),
            )
            .expect("valid animation configuration");

        assert_eq!(motion.target, 100.0);
        assert!(motion.running);
        assert!(motion.is_running());
        assert!(motion.sequence.is_none());
        assert!(motion.keyframe_animation.is_none());
    }

    #[test]
    fn test_motion_sequence_advances() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        let sequence = AnimationSequence::new()
            .then(50.0f32, instant_tween())
            .then(100.0f32, instant_tween());

        motion
            .animate_sequence(sequence)
            .expect("valid animation configuration");

        assert_eq!(motion.target, 50.0);
        assert!(motion.sequence.is_some());

        assert!(
            motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
        assert_eq!(motion.target, 100.0);
        assert!(motion.running);

        assert!(
            !motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
        assert_eq!(motion.current, 100.0);
        assert!(!motion.running);
        assert!(motion.sequence.is_none());
    }

    #[test]
    fn test_motion_keyframes_progress_and_complete() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");

        let animation = KeyframeAnimation::new(Duration::from_secs(1))
            .add_keyframe(0.0, 0.0, None)
            .unwrap()
            .add_keyframe(100.0, 1.0, None)
            .unwrap();

        motion
            .animate_keyframes(animation)
            .expect("valid keyframe setup");

        assert!(motion.update(0.5).expect("representable animation frame"));
        assert!(motion.current > 0.0);
        assert!(motion.current < 100.0);

        assert!(!motion.update(0.5).expect("representable animation frame"));
        assert_eq!(motion.current, 100.0);
        assert!(!motion.running);
        assert!(motion.keyframe_animation.is_none());
    }

    #[test]
    fn test_motion_stop() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(
                100.0,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .expect("valid animation configuration");

        motion.stop();

        assert!(!motion.running);
        assert!(motion.sequence.is_none());
        assert!(motion.keyframe_animation.is_none());
        assert_eq!(motion.velocity, 0.0);
    }

    #[test]
    fn test_motion_get_epsilon() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        assert_eq!(motion.get_epsilon(), f32::epsilon());

        motion
            .animate_to(
                1.0,
                AnimationConfig::new(AnimationMode::Tween(Tween::default())).with_epsilon(0.01),
            )
            .expect("valid animation configuration");

        assert_eq!(motion.get_epsilon(), 0.01);
    }

    #[test]
    fn test_motion_delay_prevents_early_update() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(
                100.0,
                AnimationConfig::new(AnimationMode::Tween(Tween::default())),
            )
            .expect("valid animation configuration");
        motion.delay(Duration::from_millis(100));

        assert!(
            motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
        assert_eq!(motion.current, motion.initial);
    }

    #[test]
    fn test_motion_update_tween_changes_value() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(
                100.0,
                AnimationConfig::new(AnimationMode::Tween(Tween::default())),
            )
            .expect("valid animation configuration");

        assert!(
            motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
        assert!(motion.current > 0.0);
        assert!(motion.current < 100.0);
    }

    #[test]
    fn test_motion_spring_completes_when_already_settled() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Spring(Spring::default())),
            )
            .expect("valid animation configuration");
        motion.velocity = 0.0;

        assert!(
            !motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
        assert_eq!(motion.current, 0.0);
        assert!(!motion.running);
    }

    #[test]
    fn test_motion_loop_mode_times() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(100.0, instant_tween().with_loop(LoopMode::Times(2)))
            .expect("valid animation configuration");

        assert!(
            motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
        assert_eq!(motion.current, motion.initial);
        assert!(motion.running);

        assert!(
            !motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
        assert!(!motion.running);
    }

    #[test]
    fn test_motion_loop_mode_alternate() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(100.0, instant_tween().with_loop(LoopMode::Alternate))
            .expect("valid animation configuration");

        assert!(
            motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
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

        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(100.0, config)
            .expect("valid animation configuration");

        assert!(
            !motion
                .update(1.0 / 60.0)
                .expect("representable animation frame")
        );
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_motion_get_value_tracks_current_directly() {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion.current = 12.5;
        assert_eq!(motion.get_value(), 12.5);

        motion.current = 42.0;
        assert_eq!(motion.get_value(), 42.0);
    }
}
