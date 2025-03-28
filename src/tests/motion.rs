// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::time::Duration;

//     use dioxus::prelude::*;
//     use dioxus::{dioxus_core::NoOpMutations, prelude::*};
//     use futures::FutureExt;
//     use std::{cell::RefCell, rc::Rc, sync::Arc, thread::Scope};
//     use tokio::runtime::Runtime;

//     struct TestContext {
//         runtime: Runtime,
//     }

//     impl TestContext {
//         fn new() -> Self {
//             Self {
//                 runtime: tokio::runtime::Runtime::new().unwrap(),
//             }
//         }

//         fn run<F>(&self, f: F)
//         where
//             F: FnOnce() + Send + 'static,
//         {
//             self.runtime.block_on(async {
//                 f();
//             });
//         }
//     }

//     struct MockProxy {
//         rerender: Arc<dyn Fn()>,
//         pub generation: usize,
//     }

//     impl MockProxy {
//         fn new() -> Self {
//             let generation = generation();
//             let rerender = schedule_update();

//             Self {
//                 rerender,
//                 generation,
//             }
//         }

//         pub fn rerun(&mut self) {
//             (self.rerender)();
//         }
//     }

//     fn test_hook<V: 'static>(
//         initialize: impl FnMut() -> V + 'static,
//         check: impl FnMut(V, MockProxy) + 'static,
//         mut final_check: impl FnMut(MockProxy) + 'static,
//     ) {
//         #[derive(Props)]
//         struct MockAppComponent<I: 'static, C: 'static> {
//             hook: Rc<RefCell<I>>,
//             check: Rc<RefCell<C>>,
//         }

//         impl<I, C> PartialEq for MockAppComponent<I, C> {
//             fn eq(&self, _: &Self) -> bool {
//                 true
//             }
//         }

//         impl<I, C> Clone for MockAppComponent<I, C> {
//             fn clone(&self) -> Self {
//                 Self {
//                     hook: self.hook.clone(),
//                     check: self.check.clone(),
//                 }
//             }
//         }

//         fn mock_app<I: FnMut() -> V, C: FnMut(V, MockProxy), V>(
//             props: MockAppComponent<I, C>,
//         ) -> Element {
//             let value = props.hook.borrow_mut()();
//             let proxy = MockProxy::new();

//             // Debug generation
//             println!("Current generation: {}", proxy.generation);

//             props.check.borrow_mut()(value, proxy);
//             rsx! {
//                 div {}
//             }
//         }

//         let mut vdom = VirtualDom::new_with_props(
//             mock_app,
//             MockAppComponent {
//                 hook: Rc::new(RefCell::new(initialize)),
//                 check: Rc::new(RefCell::new(check)),
//             },
//         );

//         // Initial render
//         vdom.rebuild_in_place();

//         // Process all work items
//         while vdom.wait_for_work().now_or_never().is_some() {
//             vdom.render_immediate(&mut NoOpMutations);
//         }

//         // Final check
//         vdom.in_runtime(|| {
//             ScopeId::ROOT.in_runtime(|| {
//                 final_check(MockProxy::new());
//             })
//         });
//     }

//     #[test]
//     fn test_motion_new() {
//         let motion = Motion::new(0.0);
//         assert_eq!(motion.initial, 0.0);
//         assert_eq!(motion.target, 0.0);
//         assert_eq!(motion.duration, Duration::from_millis(300));
//     }

//     #[test]
//     fn test_motion_to() {
//         let motion = Motion::new(0.0).to(100.0);
//         assert_eq!(motion.initial, 0.0);
//         assert_eq!(motion.target, 100.0);
//     }

//     #[test]
//     fn test_spring_default() {
//         let spring = Spring::default();
//         assert_eq!(spring.stiffness, 100.0);
//         assert_eq!(spring.damping, 10.0);
//         assert_eq!(spring.mass, 1.0);
//         assert_eq!(spring.velocity, 0.0);
//     }

//     #[test]
//     fn test_spring_physics() {
//         let spring = Spring::default();
//         let mut velocity = 0.0;
//         let current = 0.0;
//         let target = 100.0;
//         let dt = 1.0 / 60.0;

//         let new_position = Motion::update_spring(current, target, &mut velocity, &spring, dt);
//         assert!(new_position > current);
//         assert!(velocity > 0.0);
//     }

//     // // ! This test is failing
//     // #[test]
//     // fn test_animation_state_transitions() {
//     //     let ctx = TestContext::new();

//     //     ctx.run(|| {
//     //         test_hook(
//     //             || use_motion(Motion::new(0.0).to(100.0)),
//     //             |mut motion, mut proxy| match proxy.generation {
//     //                 0 => {
//     //                     // Initial state
//     //                     assert_eq!(motion.state(), AnimationState::Running);
//     //                     motion.start();
//     //                     proxy.rerun();
//     //                 }
//     //                 1 => {
//     //                     // Started
//     //                     motion.start();
//     //                     assert_eq!(motion.state(), AnimationState::Running);
//     //                     proxy.rerun();
//     //                 }
//     //                 2 => {
//     //                     // Stopped
//     //                     assert_eq!(motion.state(), AnimationState::Idle);
//     //                     motion.start();
//     //                     proxy.rerun();
//     //                 }
//     //                 3 => {
//     //                     // Final check
//     //                     assert_eq!(motion.state(), AnimationState::Running);
//     //                     proxy.rerun();
//     //                 }
//     //                 _ => {}
//     //             },
//     //             |proxy| assert_eq!(proxy.generation, 1),
//     //         );
//     //     });
//     // }

//     // #[test]
//     // fn test_reverse_animation() {
//     //     let ctx = TestContext::new();
//     //     ctx.run(|| {
//     //         test_hook(
//     //             || use_motion(Motion::new(0.0).to(100.0)),
//     //             |mut motion, mut proxy| match proxy.generation {
//     //                 0 => {
//     //                     motion.start();
//     //                     proxy.rerun();
//     //                 }
//     //                 1 => {
//     //                     motion.reverse();
//     //                     proxy.rerun();
//     //                 }
//     //                 2 => {
//     //                     assert!(*motion.reverse_state.read());
//     //                 }
//     //                 _ => {}
//     //             },
//     //             |proxy| assert_eq!(proxy.generation, 3),
//     //         );
//     //     });
//     // }
//     // #[test]
//     // fn test_elapsed_time_tracking() {
//     //     let ctx = TestContext::new();
//     //     ctx.run(|| {
//     //         test_hook(
//     //             || use_motion(Motion::new(0.0).to(100.0)),
//     //             |mut motion, mut proxy| match proxy.generation {
//     //                 0 => {
//     //                     motion.start();
//     //                     proxy.rerun();
//     //                 }
//     //                 1 => {
//     //                     assert!(*motion.elapsed_time.read() >= Duration::ZERO);
//     //                 }
//     //                 _ => {}
//     //             },
//     //             |proxy| assert_eq!(proxy.generation, 2),
//     //         );
//     //     });
//     // }

//     #[test]
//     fn test_signal() {
//         test_hook(
//             || use_signal(|| 0),
//             |mut value, mut proxy| match proxy.generation {
//                 0 => {
//                     value.set(1);
//                     proxy.rerun();
//                 }
//                 1 => {
//                     assert_eq!(*value.read(), 1);
//                     value.set(2);
//                     proxy.rerun();
//                 }
//                 2 => {
//                     assert_eq!(*value.read(), 2);
//                 }
//                 _ => todo!(),
//             },
//             |proxy| assert_eq!(proxy.generation, 2),
//         );
//     }
//     // #[test]
//     // fn test_custom_easing() {
//     //     let ctx = TestContext::new();
//     //     ctx.run(|| {
//     //         test_hook(
//     //             || {
//     //                 use_motion(
//     //                     Motion::new(0.0)
//     //                         .to(100.0)
//     //                         .duration(Duration::from_millis(10))
//     //                         .easing(Linear::ease_in_out),
//     //                 )
//     //             },
//     //             |mut motion, mut proxy| match proxy.generation {
//     //                 0 => {
//     //                     motion.start();
//     //                     proxy.rerun();
//     //                 }
//     //                 1 => {
//     //                     assert!((0.0..=100.0).contains(&motion.value()));
//     //                     proxy.rerun(); // Add this to reach generation 2
//     //                 }
//     //                 2 => {
//     //                     // Final state check
//     //                     assert!(motion.value() <= 100.0);
//     //                 }
//     //                 _ => {}
//     //             },
//     //             |proxy| assert_eq!(proxy.generation, 3), // Update to expect 3 generations
//     //         );
//     //     });
//     // }

//     #[test]
//     fn test_animation_config() {
//         let config = AnimationConfig::new(AnimationMode::Spring(Spring::default()))
//             .with_delay(Duration::from_secs(1))
//             .with_loop(LoopMode::Times(3));

//         assert_eq!(config.delay, Duration::from_secs(1));
//         assert!(matches!(config.loop_mode, Some(LoopMode::Times(3))));
//     }

//     #[test]
//     fn test_spring_configuration() {
//         let spring = Spring {
//             stiffness: 200.0,
//             damping: 20.0,
//             mass: 2.0,
//             velocity: 5.0,
//         };

//         let motion = Motion::new(0.0f32).with_spring(spring);

//         assert_eq!(motion.config.mode, AnimationMode::Spring(spring));
//     }

//     #[test]
//     fn test_tween_configuration() {
//         let tween = Tween {
//             duration: Duration::from_secs(2),
//             easing: easer::functions::Linear::ease_in_out,
//         };

//         let motion = Motion::new(0.0f32).with_tween(tween);

//         assert!(matches!(motion.config.mode, AnimationMode::Tween(_)));
//     }

//     #[test]
//     fn test_loop_modes() {
//         let mut motion = Motion::new(0.0f32);

//         // Test infinite loop
//         motion.config.loop_mode = Some(LoopMode::Infinite);
//         assert!(motion.handle_completion());

//         // Test finite loop
//         motion.config.loop_mode = Some(LoopMode::Times(2));
//         assert!(motion.handle_completion()); // First loop
//         assert!(!motion.handle_completion()); // Second loop (should complete)

//         // Test no loop
//         motion.config.loop_mode = Some(LoopMode::None);
//         assert!(!motion.handle_completion());
//     }

//     #[test]
//     fn test_animation_sequence() {
//         let sequence = AnimationSequence {
//             steps: vec![
//                 AnimationStep {
//                     target: 50.0,
//                     config: Arc::new(AnimationConfig::new(AnimationMode::Spring(
//                         Spring::default(),
//                     ))),
//                 },
//                 AnimationStep {
//                     target: 100.0,
//                     config: Arc::new(AnimationConfig::new(AnimationMode::Tween(Tween {
//                         duration: Duration::from_secs(1),
//                         easing: easer::functions::Linear::ease_in_out,
//                     }))),
//                 },
//             ],
//         };

//         test_hook(
//             || use_motion(0.0f32),
//             |mut motion, mut proxy| match proxy.generation {
//                 0 => {
//                     motion.animate_sequence(sequence.clone());
//                     assert_eq!(motion.get_value(), 0.0);
//                     proxy.rerun();
//                 }
//                 1 => {
//                     assert!(motion.is_running());
//                 }
//                 _ => {}
//             },
//             |proxy| assert_eq!(proxy.generation, 2),
//         );
//     }

//     #[test]
//     fn test_delay_handling() {
//         let delay = Duration::from_secs(1);
//         let mut motion = Motion::new(0.0f32);
//         motion.config.delay = delay;

//         // Test that animation doesn't progress during delay
//         assert!(motion.update(0.5));
//         assert_eq!(motion.current, 0.0);

//         // Test that animation starts after delay
//         assert!(motion.update(1.0));
//         assert!(motion.delay_elapsed >= delay);
//     }

//     #[test]
//     fn test_spring_completion_conditions() {
//         let mut motion = Motion::new(0.0f32);
//         motion.target = 100.0;
//         motion.velocity = 0.0001;
//         motion.current = 99.9999;

//         let spring = Spring::default();
//         assert!(matches!(
//             motion.check_spring_completion(),
//             SpringState::Completed
//         ));
//     }

//     #[test]
//     fn test_tween_interpolation() {
//         let mut motion = Motion::new(0.0f32);
//         motion.target = 100.0;

//         let tween = Tween {
//             duration: Duration::from_secs(1),
//             easing: easer::functions::Linear::ease_in_out,
//         };

//         // Test halfway point
//         motion.elapsed = Duration::from_millis(500);
//         assert!(!motion.update_tween(tween, 0.0));
//         assert!((motion.current - 50.0).abs() < 0.001);

//         // Test completion
//         motion.elapsed = Duration::from_secs(1);
//         assert!(motion.update_tween(tween, 0.0));
//         assert_eq!(motion.current, 100.0);
//     }

//     #[test]
//     fn test_animation_reset() {
//         let mut motion = Motion::new(0.0f32);
//         motion.target = 100.0;
//         motion.current = 50.0;
//         motion.velocity = 10.0;
//         motion.elapsed = Duration::from_secs(1);

//         motion.reset();

//         assert_eq!(motion.current, 0.0);
//         assert_eq!(motion.velocity, 0.0);
//         assert_eq!(motion.elapsed, Duration::ZERO);
//         assert!(!motion.running);
//     }

//     #[test]
//     fn test_animation_stop() {
//         let mut motion = Motion::new(0.0f32);
//         motion.running = true;
//         motion.velocity = 10.0;
//         motion.current_loop = 2;

//         motion.stop();

//         assert!(!motion.running);
//         assert_eq!(motion.velocity, 0.0);
//         assert_eq!(motion.current_loop, 0);
//         assert!(motion.sequence.is_none());
//     }
// }
