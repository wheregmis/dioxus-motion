use crate::platform::Time;
use crate::prelude::AnimationMode;
use crate::tween::Tween;
use crate::TimeProvider;
use crate::{
    animations::Animatable, prelude::AnimationConfig, AnimationManager, AnimationSignal,
    AnimationState,
};
use dioxus_hooks::{use_future, use_signal};
use dioxus_signals::{Readable, Signal, Writable};
use easer::functions::Easing;
use instant::Duration;
/// Animation sequence that can chain multiple animations together
pub struct AnimationSequence<T: Animatable> {
    steps: Vec<AnimationStep<T>>,
    current_step: usize,
    on_complete: Option<Box<dyn FnOnce()>>,
}

impl<T: Animatable> Clone for AnimationSequence<T> {
    fn clone(&self) -> Self {
        Self {
            steps: self.steps.clone(),
            current_step: self.current_step,
            on_complete: None,
        }
    }
}

#[derive(Clone)]
pub struct AnimationStep<T: Animatable> {
    target: T,
    config: AnimationConfig,
}

impl<T: Animatable> AnimationSequence<T> {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            on_complete: None,
        }
    }

    pub fn then(mut self, target: T, config: AnimationConfig) -> Self {
        self.steps.push(AnimationStep { target, config });
        self
    }

    pub fn on_complete<F: FnOnce() + 'static>(mut self, f: F) -> Self {
        self.on_complete = Some(Box::new(f));
        self
    }
}

/// Enhanced AnimationManager trait with new capabilities
pub trait EnhancedAnimationManager<T: Animatable>: AnimationManager<T> {
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>);
    fn animate_relative(&mut self, delta: T, config: AnimationConfig);
    fn transition_to(&mut self, target: T, transition: TransitionConfig);
    fn follow_path(&mut self, path: AnimationPath<T>, config: AnimationConfig);
}

use std::sync::Arc;

/// Configuration for smooth transitions between animations
#[derive(Clone)]
pub struct TransitionConfig {
    pub duration: Duration,
    pub easing: Arc<dyn Fn(f32) -> f32 + Send + Sync>,
    pub blend_function: Arc<dyn Fn(f32, f32) -> f32 + Send + Sync>,
}

/// Path definition for path-following animations
#[derive(Clone)]
pub struct AnimationPath<T: Animatable> {
    pub points: Vec<T>,
    pub tension: f32,
    pub closed: bool,
}

/// Dynamic frame rate controller
pub struct FrameRateController {
    target_fps: f32,
    min_fps: f32,
    max_fps: f32,
    current_fps: f32,
    smoothing: f32,
}

impl FrameRateController {
    pub fn new(target_fps: f32) -> Self {
        Self {
            target_fps,
            min_fps: 30.0,
            max_fps: 144.0,
            current_fps: target_fps,
            smoothing: 0.1,
        }
    }

    pub fn update(&mut self, frame_time: f32, animation_count: usize) {
        let load_factor = animation_count as f32 / 100.0; // Arbitrary scaling
        let target = self.target_fps * (1.0 - load_factor.min(0.5));
        let target = target.clamp(self.min_fps, self.max_fps);

        self.current_fps += (target - self.current_fps) * self.smoothing;
    }

    pub fn get_frame_time(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.current_fps)
    }
}

/// Animation batch for optimizing multiple simultaneous animations
pub struct AnimationBatch<T: Animatable> {
    animations: Vec<BatchedAnimation<T>>,
    frame_rate_controller: FrameRateController,
}

struct BatchedAnimation<T: Animatable> {
    state: AnimationState<T>,
    priority: u32,
}

impl<T: Animatable> AnimationBatch<T> {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            frame_rate_controller: FrameRateController::new(90.0),
        }
    }

    pub fn add(&mut self, animation: AnimationState<T>, priority: u32) {
        self.animations.push(BatchedAnimation {
            state: animation,
            priority,
        });
    }

    pub fn update(&mut self, dt: f32) {
        // Sort by priority
        self.animations
            .sort_by_key(|a| std::cmp::Reverse(a.priority));

        // Update frame rate based on load
        self.frame_rate_controller.update(dt, self.animations.len());

        // Update animations
        self.animations.retain_mut(|anim| anim.state.update(dt));
    }
}

/// Implementation of EnhancedAnimationManager that wraps the base AnimationManager
#[derive(Clone, Copy)]
pub struct EnhancedMotionState<T: Animatable> {
    base: AnimationSignal<T>,
    sequence: Signal<Option<AnimationSequence<T>>>,
    batch: Signal<AnimationBatch<T>>,
    path_state: Signal<Option<PathState<T>>>,
    transition_state: Signal<Option<TransitionState<T>>>,
}
#[derive(Clone)]
struct PathState<T: Animatable> {
    path: AnimationPath<T>,
    progress: f32,
    config: AnimationConfig,
}

#[derive(Clone)]
struct TransitionState<T: Animatable> {
    start: T,
    end: T,
    progress: f32,
    config: TransitionConfig,
}

impl<T: Animatable> EnhancedAnimationManager<T> for EnhancedMotionState<T> {
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        if sequence.steps.is_empty() {
            return;
        }

        // Start first animation
        let first_step = &sequence.steps[0];
        self.base
            .animate_to(first_step.target, first_step.config.clone());

        // Store sequence for later steps
        self.sequence.set(Some(sequence));
    }

    fn animate_relative(&mut self, delta: T, config: AnimationConfig) {
        let current = self.base.get_value();
        let target = current.add(&delta);
        self.base.animate_to(target, config);
    }

    fn transition_to(&mut self, target: T, config: TransitionConfig) {
        let start = self.base.get_value();
        self.transition_state.set(Some(TransitionState {
            start,
            end: target,
            progress: 0.0,
            config,
        }));
    }

    fn follow_path(&mut self, path: AnimationPath<T>, config: AnimationConfig) {
        self.path_state.set(Some(PathState {
            path,
            progress: 0.0,
            config,
        }));
    }
}

// Implement base AnimationManager trait
impl<T: Animatable> AnimationManager<T> for EnhancedMotionState<T> {
    fn new(initial: T) -> Self {
        Self {
            base: AnimationSignal::new(initial),
            sequence: Signal::new(None),
            batch: Signal::new(AnimationBatch::new()),
            path_state: Signal::new(None),
            transition_state: Signal::new(None),
        }
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        // Clear any ongoing sequence/path/transition
        self.sequence.set(None);
        self.path_state.set(None);
        self.transition_state.set(None);

        self.base.animate_to(target, config);
    }

    fn update(&mut self, dt: f32) -> bool {
        let mut still_animating = false;

        // Handle sequences
        let sequence_clone = (*self.sequence.read()).clone();
        if let Some(mut sequence) = sequence_clone {
            if !self.base.is_running() && sequence.current_step < sequence.steps.len() - 1 {
                // Move to next step in sequence
                sequence.current_step += 1;
                let step = &sequence.steps[sequence.current_step];
                self.base.animate_to(step.target, step.config.clone());
                self.sequence.set(Some(sequence));
                still_animating = true;
            } else if !self.base.is_running() && sequence.current_step == sequence.steps.len() - 1 {
                // Sequence complete
                if let Some(on_complete) = sequence.on_complete.take() {
                    on_complete();
                }
                self.sequence.set(None);
            } else {
                still_animating = true;
            }
        }

        // Handle path following
        let path_state_value = (*self.path_state.read()).clone();
        if let Some(mut path_state) = path_state_value {
            path_state.progress += dt / path_state.config.get_duration().as_secs_f32();

            if path_state.progress >= 1.0 {
                if path_state.path.closed {
                    path_state.progress -= 1.0;
                } else {
                    self.path_state.set(None);
                    return still_animating;
                }
            }

            if let Some(point) = evaluate_path_point(&path_state.path, path_state.progress) {
                self.base.animate_to(point, path_state.config.clone());
            }

            self.path_state.set(Some(path_state));
            still_animating = true;
        }

        // Handle transitions
        let transition_state_value = (*self.transition_state.read()).clone();
        if let Some(mut transition_state) = transition_state_value {
            transition_state.progress += dt / transition_state.config.duration.as_secs_f32();

            if transition_state.progress >= 1.0 {
                self.base
                    .animate_to(transition_state.end, AnimationConfig::default());
                self.transition_state.set(None);
            } else {
                let progress = (transition_state.config.easing)(transition_state.progress);
                let current = transition_state
                    .start
                    .interpolate(&transition_state.end, progress);
                self.base.animate_to(current, AnimationConfig::default());
                self.transition_state.set(Some(transition_state));
                still_animating = true;
            }
        }

        // Update base animation
        still_animating |= self.base.update(dt);

        // Update animation batch
        self.batch.write().update(dt);

        still_animating
    }

    fn get_value(&self) -> T {
        self.base.get_value()
    }

    fn is_running(&self) -> bool {
        self.base.is_running()
            || self.sequence.read().is_some()
            || self.path_state.read().is_some()
            || self.transition_state.read().is_some()
    }

    fn reset(&mut self) {
        self.sequence.set(None);
        self.path_state.set(None);
        self.transition_state.set(None);
        self.base.reset();
    }

    fn stop(&mut self) {
        self.sequence.set(None);
        self.path_state.set(None);
        self.transition_state.set(None);
        self.base.stop();
    }

    fn delay(&mut self, duration: Duration) {
        self.base.delay(duration);
    }
}

// Helper function to evaluate a point along a path
fn evaluate_path_point<T: Animatable>(path: &AnimationPath<T>, progress: f32) -> Option<T> {
    if path.points.len() < 2 {
        return path.points.first().copied();
    }

    let total_points = path.points.len();
    let segment_f = progress * (total_points - 1) as f32;
    let i = segment_f.floor() as usize;
    let t = segment_f.fract();

    if i >= total_points - 1 {
        return Some(path.points[total_points - 1]);
    }

    Some(path.points[i].interpolate(&path.points[i + 1], t))
}
impl<T: Animatable> EnhancedMotionState<T> {
    fn update_path_animation(&mut self, dt: f32) -> bool {
        let current_state = self.path_state.read().clone();
        let path_state = match current_state {
            Some(mut state) => {
                // Update progress
                state.progress += dt / state.config.get_duration().as_secs_f32();

                // Handle completion or looping
                if state.progress >= 1.0 {
                    if state.path.closed {
                        state.progress -= state.progress.floor();
                    } else {
                        self.path_state.set(None);
                        return false;
                    }
                }

                // Get current point on path
                if let Some(point) = evaluate_path_point(&state.path, state.progress) {
                    self.base.animate_to(
                        point,
                        AnimationConfig::new(AnimationMode::Tween(Tween {
                            duration: Duration::from_millis(16),
                            easing: easer::functions::Linear::ease_in_out,
                        })),
                    );
                }

                self.path_state.set(Some(state));
                true
            }
            None => false,
        };

        path_state
    }
}

pub struct SequenceState<T: Animatable> {
    steps: Vec<(T, AnimationConfig)>,
    current_step: usize,
    elapsed: Duration,
    completion_callback: Option<Box<dyn FnOnce()>>,
}

impl<T: Animatable> SequenceState<T> {
    fn new() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            elapsed: Duration::default(),
            completion_callback: None,
        }
    }

    fn update(&mut self, dt: f32) -> bool {
        if self.current_step >= self.steps.len() {
            if let Some(callback) = self.completion_callback.take() {
                callback();
            }
            return false;
        }

        self.elapsed += Duration::from_secs_f32(dt);
        true
    }
}

impl<T: Animatable> EnhancedAnimationManager<T> for Signal<EnhancedMotionState<T>> {
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.write().animate_sequence(sequence);
    }

    fn animate_relative(&mut self, delta: T, config: AnimationConfig) {
        self.write().animate_relative(delta, config);
    }

    fn transition_to(&mut self, target: T, transition: TransitionConfig) {
        self.write().transition_to(target, transition);
    }

    fn follow_path(&mut self, path: AnimationPath<T>, config: AnimationConfig) {
        self.write().follow_path(path, config);
    }
}

impl<T: Animatable> AnimationManager<T> for Signal<EnhancedMotionState<T>> {
    fn new(initial: T) -> Self {
        Signal::new(EnhancedMotionState::new(initial))
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.write().animate_to(target, config);
    }

    fn update(&mut self, dt: f32) -> bool {
        self.write().update(dt)
    }

    fn get_value(&self) -> T {
        self.read().get_value()
    }

    fn is_running(&self) -> bool {
        self.read().is_running()
    }

    fn reset(&mut self) {
        self.write().reset();
    }

    fn stop(&mut self) {
        self.write().stop();
    }

    fn delay(&mut self, duration: Duration) {
        self.write().delay(duration);
    }
}

pub fn use_enhanced_motion<T: Animatable>(initial: T) -> impl EnhancedAnimationManager<T> {
    let mut state = use_signal(|| EnhancedMotionState::new(initial));
    let mut sequence = use_signal(|| SequenceState::<T>::new());

    use_future(move || async move {
        let mut last_frame = Time::now();

        loop {
            let now = Time::now();
            let dt = now.duration_since(last_frame).as_secs_f32();

            if state.read().is_running() || sequence.write().update(dt) {
                state.write().update(dt);
                Time::delay(Duration::from_millis(16)).await;
            } else {
                Time::delay(Duration::from_millis(50)).await;
            }

            last_frame = now;
        }
    });

    state
}
