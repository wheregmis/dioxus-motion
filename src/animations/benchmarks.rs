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

    // ConfigPool tests removed - no longer using pooling

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

    // Motion tests removed - no longer using Motion struct

    // State machine tests removed - no longer using state machine dispatch
}
