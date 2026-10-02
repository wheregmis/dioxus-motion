use crate::Duration;
use crate::animations::core::{Animatable, AnimationError, validate_value};
use crate::keyframes::KeyframeAnimation;
use crate::motion::Motion;
use crate::prelude::AnimationConfig;
use crate::sequence::AnimationSequence;

use dioxus::{
    prelude::{ReadStore, Store, use_hook},
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

pub struct MotionHandle<T: Animatable + Send + 'static> {
    state: Store<Motion<T>>,
}

impl<T: Animatable + Send + 'static> Clone for MotionHandle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Animatable + Send + 'static> Copy for MotionHandle<T> {}

impl<T: Animatable + Send + 'static> MotionHandle<T> {
    pub(crate) fn new_hook(initial: T) -> Result<Self, AnimationError> {
        use_hook(|| Self::new_detached(initial))
    }

    fn new_detached(initial: T) -> Result<Self, AnimationError> {
        Motion::new(initial).map(|motion| Self {
            state: Store::new(motion),
        })
    }

    pub fn current(self) -> ReadStore<T> {
        let scope =
            self.state
                .into_selector()
                .child(CURRENT_SCOPE, current_ref::<T>, current_mut::<T>);
        let store: Store<T, _> = scope.into();
        store.into()
    }

    pub fn running(self) -> ReadStore<bool> {
        let scope =
            self.state
                .into_selector()
                .child(RUNNING_SCOPE, running_ref::<T>, running_mut::<T>);
        let store: Store<bool, _> = scope.into();
        store.into()
    }

    pub(crate) fn epsilon(&self) -> f32 {
        self.state.peek().get_epsilon()
    }

    pub(crate) fn set_current(&mut self, value: T) -> Result<(), AnimationError> {
        validate_value(&value, "current value")?;
        self.write_motion(|motion| {
            motion.current = value;
        });
        Ok(())
    }

    fn write_motion<R>(&mut self, f: impl FnOnce(&mut Motion<T>) -> R) -> R {
        let selector = self.state.into_selector();
        let mut motion = selector.write_untracked();
        let previous_current = motion.current.clone();
        let previous_running = motion.running;

        let result = f(&mut motion);
        let next_current = motion.current.clone();
        let next_running = motion.running;
        drop(motion);
        let epsilon = self.epsilon();

        if (next_current - previous_current).magnitude() > epsilon {
            selector.child_unmapped(CURRENT_SCOPE).mark_dirty();
        }

        if next_running != previous_running {
            selector.child_unmapped(RUNNING_SCOPE).mark_dirty();
        }

        result
    }
}

pub trait AnimationManager<T: Animatable + Send + 'static>: Clone + Copy {
    fn new(initial: T) -> Result<Self, AnimationError>;
    fn animate_to(&mut self, target: T, config: AnimationConfig) -> Result<(), AnimationError>;
    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) -> Result<(), AnimationError>;
    fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) -> Result<(), AnimationError>;
    fn update(&mut self, dt: f32) -> Result<bool, AnimationError>;
    /// Changes spring velocity in value units per second without restarting playback.
    fn set_velocity(&mut self, velocity: T) -> Result<(), AnimationError>;
    fn get_value(&self) -> T;
    fn is_running(&self) -> bool;
    fn reset(&mut self);
    fn stop(&mut self);
    fn delay(&mut self, duration: Duration);
}

impl<T: Animatable + Send + 'static> AnimationManager<T> for MotionHandle<T> {
    fn new(initial: T) -> Result<Self, AnimationError> {
        Self::new_detached(initial)
    }

    fn animate_to(&mut self, target: T, config: AnimationConfig) -> Result<(), AnimationError> {
        self.write_motion(|motion| motion.animate_to(target, config))
    }

    fn animate_sequence(&mut self, sequence: AnimationSequence<T>) -> Result<(), AnimationError> {
        let completion =
            self.write_motion(|motion| motion.animate_sequence_with_completion(sequence))?;
        if let Some(completion) = completion {
            completion.run();
        }
        Ok(())
    }

    fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) -> Result<(), AnimationError> {
        self.write_motion(|motion| motion.animate_keyframes(animation))
    }

    fn update(&mut self, dt: f32) -> Result<bool, AnimationError> {
        let (running, completion) =
            self.write_motion(|motion| motion.update_with_completion(dt))?;
        if let Some(completion) = completion {
            completion.run();
        }
        Ok(running)
    }

    fn set_velocity(&mut self, velocity: T) -> Result<(), AnimationError> {
        self.write_motion(|motion| motion.set_velocity(velocity))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::LoopMode;
    use dioxus::prelude::*;
    use std::cell::Cell;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn hook_initialization_caches_success_and_error_without_an_invalid_store() {
        use std::rc::Rc;
        #[derive(Clone)]
        struct Props {
            initial: Rc<Cell<f32>>,
            valid: Rc<Cell<bool>>,
        }
        fn host(props: Props) -> Element {
            props
                .valid
                .set(MotionHandle::<f32>::new_hook(props.initial.get()).is_ok());
            VNode::empty()
        }
        for initial in [0.0, f32::NAN] {
            let props = Props {
                initial: Rc::new(Cell::new(initial)),
                valid: Rc::new(Cell::new(false)),
            };
            let mut dom = VirtualDom::new_with_props(host, props.clone());
            dom.rebuild_in_place();
            assert_eq!(props.valid.get(), initial.is_finite());
            props
                .initial
                .set(if initial.is_finite() { f32::NAN } else { 0.0 });
            dom.mark_dirty(ScopeId::APP);
            dom.render_immediate_to_vec();
            assert_eq!(props.valid.get(), initial.is_finite());
        }
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(matches!(
                MotionHandle::<f32>::new(value),
                Err(AnimationError::NonFiniteValue("initial value"))
            ));
        }
    }

    #[test]
    fn handle_propagates_setup_errors_without_replacing_motion() {
        let mut dom = VirtualDom::new(|| rsx! { div {} });
        dom.rebuild_in_place();
        dom.in_scope(ScopeId::APP, || {
            let mut motion = MotionHandle::new(0.0f32).expect("finite initial value");
            motion
                .animate_to(1.0, AnimationConfig::tween_ms(1000))
                .expect("valid configuration");
            assert!(motion.update(0.25).expect("representable animation frame"));
            assert_eq!(
                motion.animate_to(99.0, AnimationConfig::tween_ms(0).with_epsilon(0.0)),
                Err(AnimationError::InvalidEpsilon)
            );
            assert_eq!(
                motion.animate_sequence(
                    AnimationSequence::new()
                        .then(99.0, AnimationConfig::tween_ms(0).with_epsilon(f32::NAN))
                ),
                Err(AnimationError::InvalidEpsilon)
            );
            assert!(motion.is_running());
            assert_eq!(motion.get_value(), 0.25);
            assert!(!motion.update(0.75).expect("representable animation frame"));
            assert_eq!(motion.get_value(), 1.0);
        });
    }

    #[test]
    fn current_setter_rejects_nonfinite_values_without_changing_playback() {
        let mut dom = VirtualDom::new(|| rsx! { div {} });
        dom.rebuild_in_place();
        dom.in_scope(ScopeId::APP, || {
            let mut motion = MotionHandle::new(0.0f32).expect("finite initial value");
            motion
                .animate_to(1.0, AnimationConfig::tween_ms(1000))
                .expect("valid animation configuration");
            for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                assert_eq!(
                    motion.set_current(bad),
                    Err(AnimationError::NonFiniteValue("current value"))
                );
                assert_eq!(motion.get_value(), 0.0);
                assert!(motion.is_running());
            }
            assert_eq!(motion.set_current(0.25), Ok(()));
            assert_eq!(motion.get_value(), 0.25);
            assert!(motion.is_running());
        });
    }

    #[test]
    fn runtime_errors_propagate_through_the_store_and_allow_restart() {
        let mut dom = VirtualDom::new(|| rsx! { div {} });
        dom.rebuild_in_place();
        dom.in_scope(ScopeId::APP, || {
            let mut motion = MotionHandle::new(0.0f32).expect("finite initial value");
            motion
                .animate_to(
                    f32::MAX,
                    AnimationConfig::spring(crate::prelude::Spring::default()),
                )
                .expect("valid animation configuration");
            assert_eq!(
                motion.update(1.0 / 60.0),
                Err(AnimationError::NonFiniteValue("spring velocity"))
            );
            assert_eq!(motion.get_value(), 0.0);
            assert!(!motion.is_running());
            motion
                .animate_to(1.0, AnimationConfig::tween_ms(1000))
                .expect("valid animation configuration");
            assert_eq!(motion.update(0.5), Ok(true));
            assert_eq!(motion.get_value(), 0.5);
        });
    }

    #[test]
    fn completion_can_read_and_restart_the_same_handle() {
        // Completion callbacks are Send; Dioxus handles stay on their owner thread.
        thread_local! { static CALLBACK_MOTION: Cell<Option<MotionHandle<f32>>> = const { Cell::new(None) }; }
        let mut dom = VirtualDom::new(|| rsx! { div {} });
        dom.rebuild_in_place();
        dom.in_scope(ScopeId::APP, || {
            for case in 0..5 {
                let mut motion = MotionHandle::new(0.0f32).expect("finite initial value");
                CALLBACK_MOTION.set(Some(motion));
                let calls = Arc::new(AtomicUsize::new(0));
                let callback_calls = calls.clone();
                let expected = if case == 2 || case == 4 { 0.0 } else { 1.0 };
                let callback = move || {
                    let mut callback_motion = CALLBACK_MOTION.get().expect("current test handle");
                    assert!(!callback_motion.is_running());
                    assert_eq!(callback_motion.get_value(), expected);
                    callback_calls.fetch_add(1, Ordering::Relaxed);
                    callback_motion
                        .animate_to(2.0, AnimationConfig::tween_ms(1000))
                        .expect("valid animation configuration");
                };
                let instant = AnimationConfig::tween(Duration::ZERO);
                match case {
                    0 => motion
                        .animate_to(1.0, instant.with_on_complete(callback))
                        .expect("valid animation configuration"),
                    1 => motion
                        .animate_to(
                            1.0,
                            instant
                                .with_loop(LoopMode::Times(2))
                                .with_on_complete(callback),
                        )
                        .expect("valid animation configuration"),
                    2 => motion
                        .animate_to(
                            1.0,
                            instant
                                .with_loop(LoopMode::AlternateTimes(1))
                                .with_on_complete(callback),
                        )
                        .expect("valid animation configuration"),
                    3 => motion
                        .animate_sequence(
                            AnimationSequence::new()
                                .then(1.0, instant)
                                .on_complete(callback),
                        )
                        .expect("valid animation configuration"),
                    _ => motion
                        .animate_sequence(AnimationSequence::new().on_complete(callback))
                        .expect("valid animation configuration"),
                }
                if case == 1 || case == 2 {
                    assert!(motion.update(0.01).expect("representable animation frame"));
                    assert_eq!(calls.load(Ordering::Relaxed), 0);
                }
                if case != 4 {
                    assert!(!motion.update(0.01).expect("representable animation frame"));
                }
                assert_eq!(calls.load(Ordering::Relaxed), 1);
                assert!(motion.is_running());
                assert!(motion.update(0.5).expect("representable animation frame"));
                assert!(motion.get_value() > expected);
                assert_eq!(calls.load(Ordering::Relaxed), 1);
            }
            CALLBACK_MOTION.set(None);
        });
    }
}
