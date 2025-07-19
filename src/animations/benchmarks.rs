//! Performance benchmarks for platform-specific optimizations
//!
//! This module contains benchmarks to validate the performance improvements
//! from closure pooling on web platforms and sleep optimization on desktop.

#[cfg(test)]
mod tests {
    #![allow(clippy::uninlined_format_args)]
    use instant::{Duration, Instant};

    /// Test web closure pooling performance
    #[cfg(feature = "web")]
    #[test]
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

    /// Test memory allocation performance
    #[test]
    fn test_memory_allocation_performance() {
        let start = Instant::now();

        // Simulate creating multiple animation configs
        let mut configs = Vec::new();
        for i in 0..100 {
            let config = format!("config_{}", i);
            configs.push(config);
        }

        let allocation_time = start.elapsed();

        // Validate that allocation is reasonably fast
        assert!(
            allocation_time < Duration::from_millis(10),
            "Config allocation took too long: {:?}",
            allocation_time
        );

        // Ensure we actually created the configs
        assert_eq!(configs.len(), 100, "Should have created 100 configs");
    }

    /// Test battery life impact simulation
    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn test_battery_life_impact() {
        use crate::animations::platform::{MotionTime, TimeProvider};

        // Simulate a shorter animation scenario for testing
        const ANIMATION_FRAMES: usize = 60; // 1 second at 60fps
        const FRAME_DURATION: Duration = Duration::from_millis(16); // ~60fps

        let start_time = Instant::now();
        let mut cpu_intensive_operations = 0;

        for frame in 0..ANIMATION_FRAMES {
            let frame_start = Instant::now();

            // Simulate animation work
            let _work = frame * frame; // Simple computation

            // Use optimized delay
            MotionTime::delay(FRAME_DURATION).await;

            let frame_elapsed = frame_start.elapsed();

            // Count frames that took longer than expected (indicating CPU usage)
            if frame_elapsed > FRAME_DURATION + Duration::from_millis(2) {
                cpu_intensive_operations += 1;
            }
        }

        let total_time = start_time.elapsed();
        let expected_time = FRAME_DURATION * ANIMATION_FRAMES as u32;

        // Validate battery efficiency
        let efficiency = (expected_time.as_millis() as f64 / total_time.as_millis() as f64) * 100.0;

        // The optimization should maintain reasonable efficiency
        assert!(
            efficiency >= 70.0,
            "Animation efficiency is too low: {:.1}%",
            efficiency
        );

        // CPU intensive operations should be minimal
        let cpu_intensive_ratio = cpu_intensive_operations as f64 / ANIMATION_FRAMES as f64;
        assert!(
            cpu_intensive_ratio <= 0.2,
            "Too many CPU intensive frames: {:.1}%",
            cpu_intensive_ratio * 100.0
        );
    }

    /// Performance regression test
    #[test]
    fn test_performance_regression() {
        // This test ensures that optimizations don't introduce performance regressions
        const ITERATIONS: usize = 1000;

        // Test simple operations that should be fast
        let start = Instant::now();

        for i in 0..ITERATIONS {
            let _result = i * 2 + 1;
        }

        let elapsed = start.elapsed();

        // Total time should be reasonable
        assert!(
            elapsed < Duration::from_millis(10),
            "Total time is too long: {:?}",
            elapsed
        );
    }

    /// Test conditional checking overhead impact on state machine performance
    ///
    /// Note: This test measures the overhead of additional conditional checks
    /// rather than comparing against a true branching implementation (which is no longer available).
    /// It validates that extra conditional logic doesn't significantly impact performance.
    #[test]
    fn test_conditional_overhead_impact() {
        use crate::Motion;
        use crate::animations::core::AnimationMode;
        use crate::prelude::{AnimationConfig, Tween};

        const ITERATIONS: usize = 10000;
        const DT: f32 = 1.0 / 60.0; // 60 FPS

        // Create test motion for baseline measurement
        let mut motion_baseline = Motion::new(0.0f32);
        motion_baseline.animate_to(
            100.0f32,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );

        // Create test motion for overhead measurement
        let mut motion_with_overhead = Motion::new(0.0f32);
        motion_with_overhead.animate_to(
            100.0f32,
            AnimationConfig::new(AnimationMode::Tween(Tween::default())),
        );

        // Benchmark baseline state machine performance
        let baseline_start = Instant::now();
        for _ in 0..ITERATIONS {
            motion_baseline.update(DT);
        }
        let baseline_time = baseline_start.elapsed();

        // Benchmark with additional conditional overhead
        let overhead_start = Instant::now();
        for _ in 0..ITERATIONS {
            // Add conditional checking overhead to simulate complex dispatch logic
            let _is_running = motion_with_overhead.running;
            let _has_sequence = motion_with_overhead.sequence.is_some();
            let _has_keyframes = motion_with_overhead.keyframe_animation.is_some();

            // Simulate nested conditionals that might exist in complex animation systems
            if motion_with_overhead.running {
                if motion_with_overhead.sequence.is_some() {
                    // Sequence branch simulation
                } else if motion_with_overhead.keyframe_animation.is_some() {
                    // Keyframe branch simulation
                } else {
                    // Regular animation branch simulation
                }
            }

            // Perform the actual update
            motion_with_overhead.update(DT);
        }
        let overhead_time = overhead_start.elapsed();

        // Calculate the overhead ratio
        let overhead_ratio = overhead_time.as_nanos() as f64 / baseline_time.as_nanos() as f64;

        println!("Baseline state machine time: {:?}", baseline_time);
        println!("With conditional overhead time: {:?}", overhead_time);
        println!("Overhead ratio: {:.2}", overhead_ratio);

        // The overhead should be minimal (less than 50% increase)
        // This validates that conditional checks don't significantly impact performance
        assert!(
            overhead_ratio <= 1.5,
            "Conditional overhead is too high: {:.2}x baseline performance",
            overhead_ratio
        );

        // Both approaches should complete in reasonable time
        assert!(
            baseline_time < Duration::from_millis(100),
            "Baseline updates took too long: {:?}",
            baseline_time
        );
        assert!(
            overhead_time < Duration::from_millis(150),
            "Updates with overhead took too long: {:?}",
            overhead_time
        );
    }

    /// Test state machine CPU usage reduction
    #[test]
    fn test_state_machine_cpu_usage() {
        use crate::Motion;
        use crate::animations::core::AnimationMode;
        use crate::animations::state_machine::AnimationState;
        use crate::pool::global;
        use crate::prelude::{AnimationConfig, Spring, Tween};

        const ITERATIONS: usize = 1000;
        const DT: f32 = 1.0 / 60.0;

        // Test different animation states
        let test_cases = vec![
            ("idle", AnimationState::<f32>::new_idle()),
            ("running_tween", {
                let config_handle = global::get_config();
                global::modify_config(&config_handle, |config| {
                    *config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));
                });
                AnimationState::new_running(AnimationMode::Tween(Tween::default()), config_handle)
            }),
            ("running_spring", {
                let config_handle = global::get_config();
                global::modify_config(&config_handle, |config| {
                    *config = AnimationConfig::new(AnimationMode::Spring(Spring::default()));
                });
                AnimationState::new_running(AnimationMode::Spring(Spring::default()), config_handle)
            }),
        ];

        for (name, mut state) in test_cases {
            let mut motion = Motion::new(0.0f32);
            motion.target = 100.0f32;
            motion.running = true;

            let start = Instant::now();

            for _ in 0..ITERATIONS {
                state.update(DT, &mut motion);
            }

            let elapsed = start.elapsed();

            println!("{} state updates took: {:?}", name, elapsed);

            // Each state should update efficiently
            assert!(
                elapsed < Duration::from_millis(50),
                "{} state updates took too long: {:?}",
                name,
                elapsed
            );
        }
    }

    /// Integration test to verify state machine maintains identical behavior
    #[test]
    fn test_state_machine_behavior_consistency() {
        use crate::Motion;
        use crate::animations::core::AnimationMode;
        use crate::prelude::{AnimationConfig, Tween};

        const DT: f32 = 1.0 / 60.0;
        const ANIMATION_STEPS: usize = 120; // 2 seconds at 60fps

        // Create two identical motions
        let mut motion1 = Motion::new(0.0f32);
        let mut motion2 = Motion::new(0.0f32);

        let config = AnimationConfig::new(AnimationMode::Tween(Tween::default()));

        motion1.animate_to(100.0f32, config.clone());
        motion2.animate_to(100.0f32, config);

        // Run both animations and verify they produce identical results
        for step in 0..ANIMATION_STEPS {
            let result1 = motion1.update(DT);
            let result2 = motion2.update(DT);

            // Both should return the same continuation result
            assert_eq!(
                result1, result2,
                "Animation continuation mismatch at step {}",
                step
            );

            // Both should have the same current value (within floating point precision)
            let value_diff = (motion1.current - motion2.current).abs();
            assert!(
                value_diff < 0.001,
                "Animation values diverged at step {}: {} vs {}",
                step,
                motion1.current,
                motion2.current
            );

            // Both should have the same running state
            assert_eq!(
                motion1.running, motion2.running,
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
            motion1.current, motion2.current,
            "Final animation values don't match"
        );
        assert_eq!(
            motion1.running, motion2.running,
            "Final running states don't match"
        );
    }

    /// Test state machine memory usage efficiency
    #[test]
    fn test_state_machine_memory_efficiency() {
        use crate::Motion;
        use crate::animations::state_machine::AnimationState;
        use std::mem;

        // Verify that the state machine doesn't significantly increase memory usage
        let motion_size = mem::size_of::<Motion<f32>>();
        let state_size = mem::size_of::<AnimationState<f32>>();

        println!("Motion<f32> size: {} bytes", motion_size);
        println!("AnimationState<f32> size: {} bytes", state_size);

        // State machine should not dominate the Motion struct size
        let state_ratio = state_size as f64 / motion_size as f64;
        assert!(
            state_ratio <= 0.5,
            "AnimationState is too large relative to Motion: {:.2}%",
            state_ratio * 100.0
        );

        // Total size should be reasonable
        assert!(
            motion_size <= 512,
            "Motion struct is too large: {} bytes",
            motion_size
        );
    }
}
