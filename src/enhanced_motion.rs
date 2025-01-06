use crate::platform::Time;
use crate::TimeProvider;
use crate::{animations::Animatable, prelude::AnimationConfig, AnimationManager, AnimationSignal};
use dioxus_hooks::{use_future, use_signal};
use dioxus_signals::{Readable, Signal, Writable};
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

impl<T: Animatable> Default for AnimationSequence<T> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            on_complete: None,
        }
    }
}

impl<T: Animatable> AnimationSequence<T> {
    pub fn new() -> Self {
        Self::default()
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
    // fn animate_relative(&mut self, delta: T, config: AnimationConfig);
    // fn transition_to(&mut self, target: T, transition: TransitionConfig);
}

#[derive(Clone, Copy)]
pub struct EnhancedMotionState<T: Animatable> {
    base: AnimationSignal<T>,
    sequence: Signal<Option<AnimationSequence<T>>>,
    // transition_state: Signal<Option<TransitionState<T>>>,
}

// #[derive(Clone)]
// pub struct TransitionState<T: Animatable> {
//     start: T,
//     end: T,
//     progress: f32,
//     config: TransitionConfig,
// }

/// Configuration for smooth transitions between animations
// #[derive(Clone)]
// pub struct TransitionConfig {
//     pub duration: Duration,
//     pub easing: Arc<dyn Fn(f32) -> f32 + Send + Sync>,
//     pub blend_function: Arc<dyn Fn(f32, f32) -> f32 + Send + Sync>,
// }
impl<T: Animatable> EnhancedMotionState<T> {
    fn new(initial: T) -> Self {
        Self {
            base: AnimationSignal::new(initial),
            sequence: Signal::new(None),
            // transition_state: Signal::new(None),
        }
    }
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

    // fn animate_relative(&mut self, delta: T, config: AnimationConfig) {
    //     let current = self.base.get_value();
    //     let target = current.add(&delta);
    //     self.base.animate_to(target, config);
    // }

    // fn transition_to(&mut self, target: T, config: TransitionConfig) {
    //     let start = self.base.get_value();
    //     self.transition_state.set(Some(TransitionState {
    //         start,
    //         end: target,
    //         progress: 0.0,
    //         config,
    //     }));
    // }
}

// Implement base AnimationManager trait
impl<T: Animatable> AnimationManager<T> for EnhancedMotionState<T> {
    fn new(initial: T) -> Self {
        Self {
            base: AnimationSignal::new(initial),
            sequence: Signal::new(None),
            //transition_state: Signal::new(None),
        }
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        // Clear any ongoing sequence/path/transition
        self.sequence.set(None);
        //  self.transition_state.set(None);

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

        // Handle transitions
        // let transition_state_value = (*self.transition_state.read()).clone();
        // if let Some(mut transition_state) = transition_state_value {
        //     transition_state.progress += dt / transition_state.config.duration.as_secs_f32();

        //     if transition_state.progress >= 1.0 {
        //         self.base
        //             .animate_to(transition_state.end, AnimationConfig::default());
        //         self.transition_state.set(None);
        //     } else {
        //         let progress = (transition_state.config.easing)(transition_state.progress);
        //         let current = transition_state
        //             .start
        //             .interpolate(&transition_state.end, progress);
        //         self.base.animate_to(current, AnimationConfig::default());
        //         self.transition_state.set(Some(transition_state));
        //         still_animating = true;
        //     }
        // }

        // Update base animation
        still_animating |= self.base.update(dt);

        still_animating
    }

    fn get_value(&self) -> T {
        self.base.get_value()
    }

    fn is_running(&self) -> bool {
        self.base.is_running() || self.sequence.read().is_some()
        // || self.transition_state.read().is_some()
    }

    fn reset(&mut self) {
        self.sequence.set(None);
        // self.transition_state.set(None);
        self.base.reset();
    }

    fn stop(&mut self) {
        self.sequence.set(None);
        // self.transition_state.set(None);
        self.base.stop();
    }

    fn delay(&mut self, duration: Duration) {
        self.base.delay(duration);
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

    // fn animate_relative(&mut self, delta: T, config: AnimationConfig) {
    //     self.write().animate_relative(delta, config);
    // }

    // fn transition_to(&mut self, target: T, transition: TransitionConfig) {
    //     self.write().transition_to(target, transition);
    // }
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

pub fn use_motion<T: Animatable>(initial: T) -> impl EnhancedAnimationManager<T> {
    let mut state = use_signal(|| EnhancedMotionState::new(initial));
    let mut sequence = use_signal(|| SequenceState::<T>::new());

    use_future(move || async move {
        let mut last_frame = Time::now();

        loop {
            let now = Time::now();
            let dt = now.duration_since(last_frame).as_secs_f32();

            if state.read().is_running() || sequence.write().update(dt) {
                state.write().update(dt);
                Time::delay(Duration::from_millis(8)).await;
            } else {
                Time::delay(Duration::from_millis(50)).await;
            }

            last_frame = now;
        }
    });

    state
}
