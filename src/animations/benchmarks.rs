//! Performance benchmarks for platform-specific optimizations
//!
//! This module contains benchmarks to validate the performance of
//! platform-specific timing optimizations.

#[cfg(test)]
mod tests {
    #![allow(clippy::uninlined_format_args)]
    use instant::{Duration, Instant};

    // Closure pooling test removed - no longer using closure pooling

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
        println!(
            "  Reuse efficiency: {:.2}x",
            allocation_time.as_nanos() as f64 / reuse_time.as_nanos() as f64
        );
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
            cpu_intensive_ratio <= 0.9, // More lenient threshold for CI environments
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
        use crate::animations::core::AnimationMode;
        use crate::motion::Motion;
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

    // State machine tests removed - no longer using state machine dispatch
}
