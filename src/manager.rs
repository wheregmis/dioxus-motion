use crate::Duration;
use crate::animations::core::Animatable;
use crate::keyframes::KeyframeAnimation;
use crate::motion::Motion;
use crate::prelude::AnimationConfig;
use crate::sequence::AnimationSequence;

use dioxus::prelude::{Readable, Signal, Writable};

pub trait AnimationManager<T: Animatable + Send + 'static>: Clone + Copy {
    fn new(initial: T) -> Self;
    fn animate_to(&mut self, target: T, config: AnimationConfig);
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>);
    fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>);
    fn update(&mut self, dt: f32) -> bool;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn stop(&mut self);
    fn delay(&mut self, duration: Duration);
}

impl<T: Animatable + Send + 'static> AnimationManager<T> for Signal<Motion<T>> {
    fn new(initial: T) -> Self {
        Signal::new(Motion::new(initial))
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        (*self.write()).animate_to(target, config);
    }

    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        if let Some(first_step) = sequence.steps().first() {
            let mut state = self.write();
            (*state).animate_to(first_step.target, (*first_step.config).clone());
            state.sequence = Some(sequence.into());
        }
    }

    fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        (*self.write()).animate_keyframes(animation);
    }

    fn update(&mut self, dt: f32) -> bool {
        (*self.write()).update(dt)
    }

    fn get_value(&self) -> T {
        (*self.read()).get_value()
    }

    fn is_running(&self) -> bool {
        (*self.read()).is_running()
    }

    fn reset(&mut self) {
        (*self.write()).reset();
    }

    #[track_caller]
    fn stop(&mut self) {
        (*self.write()).stop();
    }

    fn delay(&mut self, duration: Duration) {
        (*self.write()).delay(duration);
    }
}
