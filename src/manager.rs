use crate::Duration;
use crate::animations::core::Animatable;
use crate::keyframes::KeyframeAnimation;
use crate::motion::Motion;
use crate::prelude::AnimationConfig;
use crate::sequence::AnimationSequence;

use dioxus::{
    prelude::{ReadStore, Store, use_store},
    signals::ReadableExt,
};

const CURRENT_SCOPE: u16 = 0;
const RUNNING_SCOPE: u16 = 1;

fn current_ref<T: Animatable + Send + 'static>(motion: &Motion<T>) -> &T {
    &motion.current
}

fn current_mut<T: Animatable + Send + 'static>(motion: &mut Motion<T>) -> &mut T {
    &mut motion.current
}

fn running_ref<T: Animatable + Send + 'static>(motion: &Motion<T>) -> &bool {
    &motion.running
}

fn running_mut<T: Animatable + Send + 'static>(motion: &mut Motion<T>) -> &mut bool {
    &mut motion.running
}

#[derive(Clone, Copy)]
pub struct MotionHandle<T: Animatable + Send + 'static> {
    state: Store<Motion<T>>,
}

impl<T: Animatable + Send + 'static> MotionHandle<T> {
    pub(crate) fn new_hook(initial: T) -> Self {
        Self {
            state: use_store(|| Motion::new(initial)),
        }
    }

    fn new_detached(initial: T) -> Self {
        Self {
            state: Store::new(Motion::new(initial)),
        }
    }

    pub fn current(self) -> ReadStore<T> {
        let scope = self
            .state
            .into_selector()
            .child(CURRENT_SCOPE, current_ref::<T>, current_mut::<T>);
        let store: Store<T, _> = scope.into();
        store.into()
    }

    pub fn running(self) -> ReadStore<bool> {
        let scope = self
            .state
            .into_selector()
            .child(RUNNING_SCOPE, running_ref::<T>, running_mut::<T>);
        let store: Store<bool, _> = scope.into();
        store.into()
    }

    pub(crate) fn epsilon(&self) -> f32 {
        self.state.peek().get_epsilon()
    }

    fn write_motion<R>(&mut self, f: impl FnOnce(&mut Motion<T>) -> R) -> R {
        let selector = self.state.into_selector();
        let mut motion = selector.write_untracked();
        let previous_current = motion.current;
        let previous_running = motion.running;

        let result = f(&mut motion);
        let next_current = motion.current;
        let next_running = motion.running;
        drop(motion);

        if (next_current - previous_current).magnitude() > 0.0 {
            selector.child_unmapped(CURRENT_SCOPE).mark_dirty();
        }

        if next_running != previous_running {
            selector.child_unmapped(RUNNING_SCOPE).mark_dirty();
        }

        result
    }
}

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

impl<T: Animatable + Send + 'static> AnimationManager<T> for MotionHandle<T> {
    fn new(initial: T) -> Self {
        Self::new_detached(initial)
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.write_motion(|motion| motion.animate_to(target, config));
    }

    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.write_motion(|motion| motion.animate_sequence(sequence));
    }

    fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        self.write_motion(|motion| motion.animate_keyframes(animation));
    }

    fn update(&mut self, dt: f32) -> bool {
        self.write_motion(|motion| motion.update(dt))
    }

    fn get_value(&self) -> T {
        self.current().cloned()
    }

    fn is_running(&self) -> bool {
        self.running().cloned()
    }

    fn reset(&mut self) {
        self.write_motion(Motion::reset);
    }

    #[track_caller]
    fn stop(&mut self) {
        self.write_motion(Motion::stop);
    }

    fn delay(&mut self, duration: Duration) {
        self.write_motion(|motion| motion.delay(duration));
    }
}
