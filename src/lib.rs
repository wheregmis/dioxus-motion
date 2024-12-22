use dioxus_hooks::{use_coroutine, use_signal, Coroutine};
use dioxus_signals::{Readable, Signal, Writable};
use easer::functions::{Easing, Linear};
use futures_util::StreamExt;
pub use instant::Duration;
use uuid::Uuid;

mod platform;

use platform::{DesktopTime, TimeProvider, WebTime};

#[cfg(feature = "web")]
pub type Time = WebTime;

#[cfg(not(feature = "web"))]
pub type Time = DesktopTime;

/// Represents the current state of an animation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Running,
    Completed,
}

#[derive(Debug, Clone, Copy)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub velocity: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }
    }
}

/// Animation mode
#[derive(Debug, Clone, Copy)]
pub enum AnimationMode {
    Tween {
        duration: Duration,
        easing: fn(f32, f32, f32, f32) -> f32,
    },
    Spring(Spring),
}

/// Configuration for a motion animation
#[derive(Debug, Clone, Copy)]
pub struct Motion {
    id: Uuid,
    initial: f32,
    target: f32,
    duration: Duration,
    delay: Duration,
    mode: AnimationMode,
    on_complete: Option<fn()>,
}

impl Motion {
    /// Create a new Motion with default parameters
    pub fn new(initial: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            initial,
            target: initial,
            duration: Duration::from_millis(300),
            delay: Duration::from_millis(0),
            mode: AnimationMode::Tween {
                duration: Duration::from_millis(300),
                easing: Linear::ease_in_out,
            },
            on_complete: None,
        }
    }

    /// Set the target value for the animation
    pub fn to(mut self, target: f32) -> Self {
        self.target = target;
        self
    }

    /// Alias for `to` method
    pub fn animate(self, target: f32) -> Self {
        self.to(target)
    }

    /// Set the duration of the animation
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self.mode = AnimationMode::Tween {
            duration,
            easing: Linear::ease_in_out,
        };
        self
    }

    pub fn spring(mut self, config: Spring) -> Self {
        self.mode = AnimationMode::Spring(config);
        self
    }

    // Helper method to update spring physics
    fn update_spring(
        current: f32,
        target: f32,
        velocity: &mut f32,
        spring: &Spring,
        dt: f32,
    ) -> f32 {
        let force = spring.stiffness * (target - current);
        let damping = spring.damping * *velocity;
        let acceleration = (force - damping) / spring.mass;

        *velocity += acceleration * dt;
        current + *velocity * dt
    }

    /// Set a custom easing function
    pub fn easing(mut self, easing: fn(f32, f32, f32, f32) -> f32) -> Self {
        self.mode = AnimationMode::Tween {
            duration: self.duration,
            easing,
        };
        self
    }

    /// Set a callback function to be called when animation completes
    pub fn on_complete(mut self, f: fn()) -> Self {
        self.on_complete = Some(f);
        self
    }

    /// Set a delay before the animation starts
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
}

/// Represents an active motion animation
#[derive(Clone, Copy)]
pub struct UseMotion {
    id: Uuid,
    value: Signal<f32>,
    running_state: Signal<bool>,
    completion_state: Signal<AnimationState>,
    elapsed_time: Signal<Duration>,
    config: Motion,
    channel: Coroutine<()>,
    reverse_state: Signal<bool>,
}

impl UseMotion {
    /// Get the current animated value
    pub fn value(&self) -> f32 {
        *self.value.read()
    }

    /// Start the animation
    pub fn start(&mut self) {
        *self.value.write() = self.config.initial;
        *self.completion_state.write() = AnimationState::Idle;
        *self.running_state.write() = true;
        *self.reverse_state.write() = false;
        self.channel.send(());
    }

    /// Stop the animation
    pub fn stop(&mut self) {
        *self.running_state.write() = false;
        *self.completion_state.write() = AnimationState::Idle;
    }

    pub fn resume(&mut self) {
        *self.running_state.write() = true;
        self.channel.send(());
    }

    pub fn reset(&mut self) {
        *self.value.write() = self.config.initial;
        *self.completion_state.write() = AnimationState::Idle;
        *self.running_state.write() = false;
        *self.elapsed_time.write() = Duration::from_secs(0);
        *self.reverse_state.write() = false;
    }

    pub fn reverse(&mut self) {
        if *self.reverse_state.read() {
            self.config.initial
        } else {
            self.config.target
        };

        self.reverse_state.toggle();
        *self.completion_state.write() = AnimationState::Idle;
        *self.running_state.write() = true;
        self.channel.send(());
    }

    /// Get the current animation state
    pub fn state(&self) -> AnimationState {
        self.completion_state.read().clone()
    }

    /// Check if the animation is currently running
    pub fn is_running(&self) -> bool {
        *self.running_state.read()
    }
}

/// Create a new motion animation
pub fn use_motion(config: Motion) -> UseMotion {
    let id = Uuid::new_v4();
    let mut value = use_signal(|| config.initial);
    let mut running_state = use_signal(|| false);
    let mut completion_state = use_signal(|| AnimationState::Idle);
    let mut elapsed_time = use_signal(|| Duration::from_secs(0));
    let mut reverse_state = use_signal(|| false);

    let channel = use_coroutine(move |mut rx| async move {
        while rx.next().await.is_some() {
            Time::delay(config.delay).await;

            match config.mode {
                AnimationMode::Tween { duration, easing } => {
                    let start_time = Time::now();
                    let start_value = *value.peek();
                    let end_value = if *reverse_state.read() {
                        config.initial
                    } else {
                        config.target
                    };
                    let total_change = (end_value - start_value).abs();
                    let total_frames = total_change.ceil() as u64;
                    let initial_elapsed = *elapsed_time.read();

                    completion_state.set(AnimationState::Running);
                    running_state.set(true);

                    while *running_state.read() {
                        let current_elapsed =
                            Time::now().duration_since(start_time) + initial_elapsed;
                        elapsed_time.set(current_elapsed);

                        if current_elapsed >= duration {
                            break;
                        }

                        // Calculate remaining duration
                        let remaining_duration = duration - current_elapsed;

                        let frame_progress = (current_elapsed.as_secs_f64()
                            / duration.as_secs_f64())
                            * total_frames as f64;
                        let current_frame = frame_progress.floor() as u64;
                        let progress = current_frame as f32 / total_frames as f32;
                        let current = (easing)(progress, start_value, end_value - start_value, 1.0);

                        value.set(current);

                        // Calculate frame delay based on remaining duration
                        let frame_delay = if remaining_duration >= Duration::from_secs(2) {
                            Duration::from_secs_f64(
                                remaining_duration.as_secs_f64()
                                    / (total_frames - current_frame) as f64,
                            )
                        } else {
                            Duration::from_millis(16)
                        };

                        Time::delay(frame_delay).await;
                    }

                    // Ensure final value is set
                    if Time::now().duration_since(start_time) + initial_elapsed >= duration {
                        value.set(end_value);
                        elapsed_time.set(Duration::from_secs(0));
                        running_state.set(false);
                        completion_state.set(AnimationState::Completed);

                        if let Some(f) = config.on_complete {
                            f();
                        }
                    }
                }
                AnimationMode::Spring(spring) => {
                    let mut velocity = spring.velocity;
                    let mut current = *value.peek();
                    let target = if *reverse_state.read() {
                        config.initial
                    } else {
                        config.target
                    };

                    completion_state.set(AnimationState::Running);
                    running_state.set(true);

                    while *running_state.read() {
                        let dt = 1.0 / 60.0; // 60 FPS

                        current =
                            Motion::update_spring(current, target, &mut velocity, &spring, dt);

                        value.set(current);

                        // Check if spring has settled
                        if velocity.abs() < 0.01 && (current - target).abs() < 0.01 {
                            break;
                        }

                        Time::delay(Duration::from_millis(5)).await;
                    }

                    // Ensure we reach exact target
                    value.set(target);
                    running_state.set(false);
                    completion_state.set(AnimationState::Completed);
                }
            }
        }
    });

    UseMotion {
        id,
        value,
        running_state,
        completion_state,
        elapsed_time,
        config,
        channel,
        reverse_state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dioxus::prelude::*;
    use dioxus::{dioxus_core::NoOpMutations, prelude::*};
    use futures::FutureExt;
    use std::{cell::RefCell, rc::Rc, sync::Arc, thread::Scope};
    use tokio::runtime::Runtime;

    struct TestContext {
        runtime: Runtime,
    }

    impl TestContext {
        fn new() -> Self {
            Self {
                runtime: tokio::runtime::Runtime::new().unwrap(),
            }
        }

        fn run<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            self.runtime.block_on(async {
                f();
            });
        }
    }

    struct MockProxy {
        rerender: Arc<dyn Fn()>,
        pub generation: usize,
    }

    impl MockProxy {
        fn new() -> Self {
            let generation = generation();
            let rerender = schedule_update();

            Self {
                rerender,
                generation,
            }
        }

        pub fn rerun(&mut self) {
            (self.rerender)();
        }
    }

    fn test_hook<V: 'static>(
        initialize: impl FnMut() -> V + 'static,
        check: impl FnMut(V, MockProxy) + 'static,
        mut final_check: impl FnMut(MockProxy) + 'static,
    ) {
        #[derive(Props)]
        struct MockAppComponent<I: 'static, C: 'static> {
            hook: Rc<RefCell<I>>,
            check: Rc<RefCell<C>>,
        }

        impl<I, C> PartialEq for MockAppComponent<I, C> {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }

        impl<I, C> Clone for MockAppComponent<I, C> {
            fn clone(&self) -> Self {
                Self {
                    hook: self.hook.clone(),
                    check: self.check.clone(),
                }
            }
        }

        fn mock_app<I: FnMut() -> V, C: FnMut(V, MockProxy), V>(
            props: MockAppComponent<I, C>,
        ) -> Element {
            let value = props.hook.borrow_mut()();
            let proxy = MockProxy::new();

            // Debug generation
            println!("Current generation: {}", proxy.generation);

            props.check.borrow_mut()(value, proxy);
            rsx! {
                div {}
            }
        }

        let mut vdom = VirtualDom::new_with_props(
            mock_app,
            MockAppComponent {
                hook: Rc::new(RefCell::new(initialize)),
                check: Rc::new(RefCell::new(check)),
            },
        );

        // Initial render
        vdom.rebuild_in_place();

        // Process all work items
        while vdom.wait_for_work().now_or_never().is_some() {
            vdom.render_immediate(&mut NoOpMutations);
        }

        // Final check
        vdom.in_runtime(|| {
            ScopeId::ROOT.in_runtime(|| {
                final_check(MockProxy::new());
            })
        });
    }

    #[test]
    fn test_motion_new() {
        let motion = Motion::new(0.0);
        assert_eq!(motion.initial, 0.0);
        assert_eq!(motion.target, 0.0);
        assert_eq!(motion.duration, Duration::from_millis(300));
    }

    #[test]
    fn test_motion_to() {
        let motion = Motion::new(0.0).to(100.0);
        assert_eq!(motion.initial, 0.0);
        assert_eq!(motion.target, 100.0);
    }

    #[test]
    fn test_spring_default() {
        let spring = Spring::default();
        assert_eq!(spring.stiffness, 100.0);
        assert_eq!(spring.damping, 10.0);
        assert_eq!(spring.mass, 1.0);
        assert_eq!(spring.velocity, 0.0);
    }

    #[test]
    fn test_spring_physics() {
        let spring = Spring::default();
        let mut velocity = 0.0;
        let current = 0.0;
        let target = 100.0;
        let dt = 1.0 / 60.0;

        let new_position = Motion::update_spring(current, target, &mut velocity, &spring, dt);
        assert!(new_position > current);
        assert!(velocity > 0.0);
    }

    // // ! This test is failing
    // #[test]
    // fn test_animation_state_transitions() {
    //     let ctx = TestContext::new();

    //     ctx.run(|| {
    //         test_hook(
    //             || use_motion(Motion::new(0.0).to(100.0)),
    //             |mut motion, mut proxy| match proxy.generation {
    //                 0 => {
    //                     // Initial state
    //                     assert_eq!(motion.state(), AnimationState::Running);
    //                     motion.start();
    //                     proxy.rerun();
    //                 }
    //                 1 => {
    //                     // Started
    //                     motion.start();
    //                     assert_eq!(motion.state(), AnimationState::Running);
    //                     proxy.rerun();
    //                 }
    //                 2 => {
    //                     // Stopped
    //                     assert_eq!(motion.state(), AnimationState::Idle);
    //                     motion.start();
    //                     proxy.rerun();
    //                 }
    //                 3 => {
    //                     // Final check
    //                     assert_eq!(motion.state(), AnimationState::Running);
    //                     proxy.rerun();
    //                 }
    //                 _ => {}
    //             },
    //             |proxy| assert_eq!(proxy.generation, 1),
    //         );
    //     });
    // }

    // #[test]
    // fn test_reverse_animation() {
    //     let ctx = TestContext::new();
    //     ctx.run(|| {
    //         test_hook(
    //             || use_motion(Motion::new(0.0).to(100.0)),
    //             |mut motion, mut proxy| match proxy.generation {
    //                 0 => {
    //                     motion.start();
    //                     proxy.rerun();
    //                 }
    //                 1 => {
    //                     motion.reverse();
    //                     proxy.rerun();
    //                 }
    //                 2 => {
    //                     assert!(*motion.reverse_state.read());
    //                 }
    //                 _ => {}
    //             },
    //             |proxy| assert_eq!(proxy.generation, 3),
    //         );
    //     });
    // }
    // #[test]
    // fn test_elapsed_time_tracking() {
    //     let ctx = TestContext::new();
    //     ctx.run(|| {
    //         test_hook(
    //             || use_motion(Motion::new(0.0).to(100.0)),
    //             |mut motion, mut proxy| match proxy.generation {
    //                 0 => {
    //                     motion.start();
    //                     proxy.rerun();
    //                 }
    //                 1 => {
    //                     assert!(*motion.elapsed_time.read() >= Duration::ZERO);
    //                 }
    //                 _ => {}
    //             },
    //             |proxy| assert_eq!(proxy.generation, 2),
    //         );
    //     });
    // }

    #[test]
    fn test_signal() {
        test_hook(
            || use_signal(|| 0),
            |mut value, mut proxy| match proxy.generation {
                0 => {
                    value.set(1);
                    proxy.rerun();
                }
                1 => {
                    assert_eq!(*value.read(), 1);
                    value.set(2);
                    proxy.rerun();
                }
                2 => {
                    assert_eq!(*value.read(), 2);
                }
                _ => todo!(),
            },
            |proxy| assert_eq!(proxy.generation, 2),
        );
    }
    // #[test]
    // fn test_custom_easing() {
    //     let ctx = TestContext::new();
    //     ctx.run(|| {
    //         test_hook(
    //             || {
    //                 use_motion(
    //                     Motion::new(0.0)
    //                         .to(100.0)
    //                         .duration(Duration::from_millis(10))
    //                         .easing(Linear::ease_in_out),
    //                 )
    //             },
    //             |mut motion, mut proxy| match proxy.generation {
    //                 0 => {
    //                     motion.start();
    //                     proxy.rerun();
    //                 }
    //                 1 => {
    //                     assert!((0.0..=100.0).contains(&motion.value()));
    //                     proxy.rerun(); // Add this to reach generation 2
    //                 }
    //                 2 => {
    //                     // Final state check
    //                     assert!(motion.value() <= 100.0);
    //                 }
    //                 _ => {}
    //             },
    //             |proxy| assert_eq!(proxy.generation, 3), // Update to expect 3 generations
    //         );
    //     });
    // }
}
