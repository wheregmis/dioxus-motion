//! Performance benchmarks for platform-specific optimizations
//!
//! This module contains benchmarks to validate the performance improvements
//! from closure pooling on web platforms and sleep optimization on desktop.

#[cfg(test)]
mod tests {
    #![allow(clippy::uninlined_format_args)]
    use instant::{Duration, Instant};

    #[test]
    #[ignore = "manual performance measurement; wall-clock timing is host-dependent"]
    fn test_keyframe_lookup_cpu_usage() {
        use crate::{KeyframeAnimation, Motion};
        use std::hint::black_box;

        const ITERATIONS: usize = 100_000;
        for count in [2, 32, 1024] {
            let mut animation = KeyframeAnimation::new(Duration::from_secs(10_000));
            for index in 0..count {
                animation = animation
                    .add_keyframe(index as f32, index as f32 / (count - 1) as f32, None)
                    .expect("finite keyframe offset");
            }
            let mut samples = Vec::with_capacity(7);
            for _ in 0..7 {
                let mut motion = Motion::new(0.0f32).expect("finite initial value");
                motion
                    .animate_keyframes(animation.clone())
                    .expect("valid keyframe setup");
                let start = Instant::now();
                for _ in 0..ITERATIONS {
                    black_box(
                        motion
                            .update(black_box(1.0 / 60.0))
                            .expect("representable animation frame"),
                    );
                    black_box(motion.get_value());
                }
                samples.push(start.elapsed());
                assert!(motion.is_running());
            }
            samples.sort_unstable();
            println!(
                "{count} keyframes: {:.2} ns/update (median of 7 samples)",
                samples[3].as_nanos() as f64 / ITERATIONS as f64,
            );
        }
    }

    /// Test web closure pooling performance
    #[cfg(feature = "web")]
    #[test]
    #[ignore = "manual performance measurement; wall-clock timing is host-dependent"]
    fn test_web_closure_pooling_performance() {
        use crate::animations::closure_pool::{
            closure_pool_stats, execute_and_return_pooled_closure, register_pooled_callback,
        };

        const ITERATIONS: usize = 100;

        // Test that closure pooling doesn't significantly impact performance
        let start = Instant::now();

        // Register multiple callbacks to test pool performance
        let mut callback_ids = Vec::with_capacity(ITERATIONS);
        for i in 0..ITERATIONS {
            let callback = Box::new(move || {
                // Simple callback that captures the loop variable
                let _result = i * 2;
            });
            let id = register_pooled_callback(callback);
            callback_ids.push(id);
        }

        let registration_time = start.elapsed();

        // Execute all callbacks
        let execution_start = Instant::now();
        for id in callback_ids {
            execute_and_return_pooled_closure(id);
        }
        let execution_time = execution_start.elapsed();

        // Verify pool statistics
        let (_available, in_use) = closure_pool_stats();

        // Performance assertions
        assert!(
            registration_time < Duration::from_millis(10),
            "Callback registration took too long: {:?}",
            registration_time
        );
        assert!(
            execution_time < Duration::from_millis(10),
            "Callback execution took too long: {:?}",
            execution_time
        );

        // Pool should be clean after execution
        assert_eq!(
            in_use, 0,
            "Pool should have no callbacks in use after execution"
        );
    }

    /// Test desktop sleep optimization performance
    #[cfg(not(feature = "web"))]
    #[tokio::test]
    #[ignore = "manual scheduling measurement; latency depends on host load"]
    async fn test_desktop_sleep_performance() {
        use crate::animations::platform::{MotionTime, TimeProvider};

        let test_durations = vec![
            Duration::from_micros(500), // Short - should yield
            Duration::from_millis(1),   // Threshold - should sleep
            Duration::from_millis(5),   // Medium - should sleep
        ];

        for duration in test_durations {
            let start = Instant::now();
            MotionTime::delay(duration).await;
            let elapsed = start.elapsed();

            // Validate performance characteristics
            if duration < Duration::from_millis(1) {
                // Very short durations should complete quickly
                assert!(
                    elapsed < Duration::from_millis(2),
                    "Duration {:?} took too long: {:?}",
                    duration,
                    elapsed
                );
            } else {
                // Longer durations should be reasonably accurate
                let tolerance = Duration::from_millis(3);
                assert!(
                    elapsed >= duration.saturating_sub(tolerance),
                    "Duration {:?} was too short: {:?}",
                    duration,
                    elapsed
                );
                assert!(
                    elapsed <= duration + tolerance,
                    "Duration {:?} was too long: {:?}",
                    duration,
                    elapsed
                );
            }
        }
    }

    /// Test animation config pool performance and reuse
    #[test]
    #[ignore = "manual performance measurement; wall-clock timing is host-dependent"]
    fn test_config_pool_performance() {
        use crate::animations::core::{AnimationConfig, AnimationMode};
        use crate::animations::tween::Tween;
        use crate::pool::global;

        // Clear pool to start with known state
        global::clear_pool();

        const ITERATIONS: usize = 1000;
        let start = Instant::now();

        // Test config pool allocation and release performance
        let mut handles = Vec::with_capacity(ITERATIONS);

        // Phase 1: Allocate configs from pool
        let allocation_start = Instant::now();
        for _ in 0..ITERATIONS {
            let handle = global::get_config();
            global::modify_config(&handle, |config| {
                *config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));
            });
            handles.push(handle);
        }
        let allocation_time = allocation_start.elapsed();

        // Verify all configs are in use
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, ITERATIONS, "All configs should be in use");
        assert_eq!(available, 0, "No configs should be available");

        // Phase 2: Release configs back to pool
        let release_start = Instant::now();
        for handle in handles {
            global::return_config(handle);
        }
        let release_time = release_start.elapsed();

        // Verify all configs are returned to pool
        let (in_use, available) = global::pool_stats();
        assert_eq!(in_use, 0, "No configs should be in use after return");
        assert_eq!(available, ITERATIONS, "All configs should be available");

        // Phase 3: Test reuse performance (should be faster than initial allocation)
        let reuse_start = Instant::now();
        let mut reuse_handles = Vec::with_capacity(ITERATIONS);
        for _ in 0..ITERATIONS {
            let handle = global::get_config();
            global::modify_config(&handle, |config| {
                *config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));
            });
            reuse_handles.push(handle);
        }
        let reuse_time = reuse_start.elapsed();

        let total_time = start.elapsed();

        // Performance assertions
        assert!(
            allocation_time < Duration::from_millis(50),
            "Config allocation took too long: {allocation_time:?}"
        );

        assert!(
            release_time < Duration::from_millis(10),
            "Config release took too long: {release_time:?}"
        );

        assert!(
            reuse_time < Duration::from_millis(25),
            "Config reuse took too long: {reuse_time:?}"
        );

        assert!(
            total_time < Duration::from_millis(100),
            "Total pool operations took too long: {total_time:?}"
        );

        // Keep the benchmark honest without requiring one wall-clock sample to beat another.
        // The pool behavior itself is covered by deterministic pool tests.

        // Clean up
        for handle in reuse_handles {
            global::return_config(handle);
        }

        println!("Config pool performance:");
        println!("  Allocation: {allocation_time:?} for {ITERATIONS} configs");
        println!("  Release: {release_time:?} for {ITERATIONS} configs");
        println!("  Reuse: {reuse_time:?} for {ITERATIONS} configs");
        println!("  Total: {total_time:?}");
        println!(
            "  Reuse efficiency: {:.2}x",
            allocation_time.as_nanos() as f64 / reuse_time.as_nanos() as f64
        );
    }

    /// Reports median update cost with animations active throughout each sample.
    #[test]
    #[ignore = "manual performance measurement; wall-clock timing is host-dependent"]
    fn test_motion_update_cpu_usage() {
        use crate::Motion;
        use crate::animations::core::{Animatable, AnimationMode};
        use crate::prelude::{AnimationConfig, LoopMode, Spring, Tween};
        use crate::prelude::{Color, Transform};
        use std::hint::black_box;

        const ITERATIONS: usize = 100_000;
        const DT: f32 = 1.0 / 60.0;

        let test_cases = [
            ("idle", None),
            (
                "running_tween",
                Some(AnimationConfig::new(AnimationMode::Tween(Tween::default()))),
            ),
            (
                "running_spring",
                Some(AnimationConfig::new(AnimationMode::Spring(
                    Spring::default(),
                ))),
            ),
        ];

        fn measure<T: Animatable + Send + Copy>(
            value_type: &str,
            initial: T,
            target: T,
            test_cases: &[(&str, Option<AnimationConfig>)],
        ) {
            for (name, config) in test_cases {
                let active = config.is_some();
                let mut samples = Vec::with_capacity(7);
                for _ in 0..7 {
                    let mut motion = Motion::new(initial).expect("finite initial value");
                    if let Some(config) = config.clone() {
                        motion
                            .animate_to(target, config.with_loop(LoopMode::Infinite))
                            .expect("valid animation configuration");
                    }
                    let start = Instant::now();
                    for _ in 0..ITERATIONS {
                        black_box(
                            motion
                                .update(black_box(DT))
                                .expect("representable animation frame"),
                        );
                        black_box(motion.get_value());
                    }
                    samples.push(start.elapsed());
                    assert_eq!(motion.is_running(), active);
                }
                samples.sort_unstable();
                println!(
                    "{value_type}/{name}: {:.2} ns/update (median of 7 samples, {ITERATIONS} updates/sample)",
                    samples[3].as_nanos() as f64 / ITERATIONS as f64,
                );
            }
        }
        measure("f32", 0.0f32, 100.0, &test_cases);
        measure(
            "Color",
            Color::new(1.0, 0.5, 0.25, 1.0),
            Color::new(0.0, 0.0, 0.0, 0.0),
            &test_cases,
        );
        measure(
            "Transform",
            Transform::identity(),
            Transform::new(100.0, 50.0, 2.0, 1.0),
            &test_cases,
        );
    }

    /// Integration test to verify the simplified motion loop remains deterministic
    #[test]
    fn test_motion_behavior_consistency() {
        use crate::Motion;
        use crate::animations::core::AnimationMode;
        use crate::prelude::{AnimationConfig, Tween};

        const DT: f32 = 1.0 / 60.0;
        const ANIMATION_STEPS: usize = 120; // 2 seconds at 60fps

        // Create two identical motions
        let mut motion1 = Motion::new(0.0f32).expect("finite initial value");
        let mut motion2 = Motion::new(0.0f32).expect("finite initial value");

        let config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));

        motion1
            .animate_to(100.0f32, config.clone())
            .expect("valid animation configuration");
        motion2
            .animate_to(100.0f32, config)
            .expect("valid animation configuration");

        // Run both animations and verify they produce identical results
        for step in 0..ANIMATION_STEPS {
            let result1 = motion1.update(DT).expect("representable animation frame");
            let result2 = motion2.update(DT).expect("representable animation frame");

            // Both should return the same continuation result
            assert_eq!(
                result1, result2,
                "Animation continuation mismatch at step {}",
                step
            );

            // Both should have the same current value (within floating point precision)
            let value_diff = (motion1.get_value() - motion2.get_value()).abs();
            assert!(
                value_diff < 0.001,
                "Animation values diverged at step {}: {} vs {}",
                step,
                motion1.get_value(),
                motion2.get_value()
            );

            // Both should have the same running state
            assert_eq!(
                motion1.is_running(),
                motion2.is_running(),
                "Running state mismatch at step {}",
                step
            );

            // If animation is complete, break
            if !result1 {
                break;
            }
        }

        // Final values should be identical
        assert_eq!(
            motion1.get_value(),
            motion2.get_value(),
            "Final animation values don't match"
        );
        assert_eq!(
            motion1.is_running(),
            motion2.is_running(),
            "Final running states don't match"
        );
    }

    /// Test motion memory usage efficiency
    #[test]
    fn test_motion_memory_efficiency() {
        use crate::Motion;
        use std::mem;

        let motion_size = mem::size_of::<Motion<f32>>();

        println!("Motion<f32> size: {} bytes", motion_size);

        // Total size should be reasonable
        assert!(
            motion_size <= 512,
            "Motion struct is too large: {} bytes",
            motion_size
        );
    }
}
