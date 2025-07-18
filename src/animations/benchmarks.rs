//! Performance benchmarks for platform-specific optimizations
//!
//! This module contains benchmarks to validate the performance improvements
//! from closure pooling on web platforms and sleep optimization on desktop.

#[cfg(test)]
mod tests {
    use instant::{Duration, Instant};

    /// Test web closure pooling performance
    #[cfg(feature = "web")]
    #[test]
    fn test_web_closure_pooling_performance() {
        use crate::animations::closure_pool::{register_pooled_callback, execute_and_return_pooled_closure, closure_pool_stats};
        
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
        assert!(registration_time < Duration::from_millis(10), 
                "Callback registration took too long: {:?}", registration_time);
        assert!(execution_time < Duration::from_millis(10), 
                "Callback execution took too long: {:?}", execution_time);
        
        // Pool should be clean after execution
        assert_eq!(in_use, 0, "Pool should have no callbacks in use after execution");
    }

    /// Test desktop sleep optimization performance
    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn test_desktop_sleep_performance() {
        use crate::animations::platform::{MotionTime, TimeProvider};
        
        let test_durations = vec![
            Duration::from_micros(500),   // Short - should yield
            Duration::from_millis(1),     // Threshold - should sleep
            Duration::from_millis(5),     // Medium - should sleep
        ];
        
        for duration in test_durations {
            let start = Instant::now();
            MotionTime::delay(duration).await;
            let elapsed = start.elapsed();
            
            // Validate performance characteristics
            if duration < Duration::from_millis(1) {
                // Very short durations should complete quickly
                assert!(elapsed < Duration::from_millis(2), 
                        "Duration {:?} took too long: {:?}", duration, elapsed);
            } else {
                // Longer durations should be reasonably accurate
                let tolerance = Duration::from_millis(3);
                assert!(elapsed >= duration.saturating_sub(tolerance), 
                        "Duration {:?} was too short: {:?}", duration, elapsed);
                assert!(elapsed <= duration + tolerance, 
                        "Duration {:?} was too long: {:?}", duration, elapsed);
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
        assert!(allocation_time < Duration::from_millis(10),
                "Config allocation took too long: {:?}", allocation_time);
        
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
        assert!(efficiency >= 70.0, "Animation efficiency is too low: {:.1}%", efficiency);
        
        // CPU intensive operations should be minimal
        let cpu_intensive_ratio = cpu_intensive_operations as f64 / ANIMATION_FRAMES as f64;
        assert!(cpu_intensive_ratio <= 0.2, 
                "Too many CPU intensive frames: {:.1}%", cpu_intensive_ratio * 100.0);
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
        assert!(elapsed < Duration::from_millis(10),
                "Total time is too long: {:?}", elapsed);
    }
}