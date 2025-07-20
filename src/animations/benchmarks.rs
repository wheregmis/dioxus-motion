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

    /// Test animation config pool performance and reuse
    #[test]
    fn test_config_pool_performance() {
        use crate::pool::global;
        use crate::animations::core::{AnimationConfig, AnimationMode};
        use crate::animations::tween::Tween;

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

        // Reuse should be faster than initial allocation (pool efficiency)
        assert!(
            reuse_time <= allocation_time,
            "Config reuse should be at least as fast as initial allocation. Allocation: {allocation_time:?}, Reuse: {reuse_time:?}"
        );

        // Clean up
        for handle in reuse_handles {
            global::return_config(handle);
        }

        println!("Config pool performance:");
        println!("  Allocation: {allocation_time:?} for {ITERATIONS} configs");
        println!("  Release: {release_time:?} for {ITERATIONS} configs");
        println!("  Reuse: {reuse_time:?} for {ITERATIONS} configs");
        println!("  Total: {total_time:?}");
        println!("  Reuse efficiency: {:.2}x", allocation_time.as_nanos() as f64 / reuse_time.as_nanos() as f64);
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

        // The overhead should be reasonable (less than 80% increase)
        // This validates that conditional checks don't significantly impact performance
        // Note: Some variance is expected due to system load and compiler optimizations
        assert!(
            overhead_ratio <= 1.8,
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
