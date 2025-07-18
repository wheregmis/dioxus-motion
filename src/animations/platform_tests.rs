//! Platform-specific performance tests for sleep optimization
//!
//! Tests the performance characteristics of different sleep implementations
//! to validate optimization effectiveness.

#[cfg(test)]
mod tests {
    use super::super::platform::{MotionTime, TimeProvider};
    use instant::Duration;

    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn test_desktop_sleep_threshold_optimization() {
        // Test that very short durations don't use spin sleep
        let short_duration = Duration::from_micros(500);
        let start = instant::Instant::now();
        
        MotionTime::delay(short_duration).await;
        
        let elapsed = start.elapsed();
        
        // For very short durations, we should yield instead of sleep
        // The elapsed time should be minimal (less than 1ms)
        assert!(elapsed < Duration::from_millis(2), 
                "Short duration sleep took too long: {:?}", elapsed);
    }

    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn test_desktop_sleep_longer_duration() {
        // Test that longer durations use proper sleep
        let long_duration = Duration::from_millis(10);
        let start = instant::Instant::now();
        
        MotionTime::delay(long_duration).await;
        
        let elapsed = start.elapsed();
        
        // For longer durations, we should sleep properly
        // Allow some tolerance for timing variations
        assert!(elapsed >= Duration::from_millis(8), 
                "Long duration sleep was too short: {:?}", elapsed);
        assert!(elapsed <= Duration::from_millis(15), 
                "Long duration sleep was too long: {:?}", elapsed);
    }

    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn test_desktop_sleep_threshold_boundary() {
        // Test the 1ms threshold boundary
        let threshold_duration = Duration::from_millis(1);
        let start = instant::Instant::now();
        
        MotionTime::delay(threshold_duration).await;
        
        let elapsed = start.elapsed();
        
        // At the threshold, we should still use proper sleep
        assert!(elapsed >= Duration::from_micros(800), 
                "Threshold duration sleep was too short: {:?}", elapsed);
        assert!(elapsed <= Duration::from_millis(3), 
                "Threshold duration sleep was too long: {:?}", elapsed);
    }

    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn test_desktop_sleep_performance_comparison() {
        // Performance test comparing different sleep durations
        let test_cases = vec![
            Duration::from_micros(100),   // Very short - should yield
            Duration::from_micros(500),   // Short - should yield  
            Duration::from_millis(1),     // Threshold - should sleep
            Duration::from_millis(5),     // Medium - should sleep
            Duration::from_millis(16),    // Frame time - should sleep
        ];

        for duration in test_cases {
            let start = instant::Instant::now();
            MotionTime::delay(duration).await;
            let elapsed = start.elapsed();
            
            if duration < Duration::from_millis(1) {
                // Very short durations should complete quickly
                assert!(elapsed < Duration::from_millis(2), 
                        "Duration {:?} took too long: {:?}", duration, elapsed);
            } else {
                // Longer durations should be reasonably accurate
                let tolerance = Duration::from_millis(2);
                assert!(elapsed >= duration.saturating_sub(tolerance), 
                        "Duration {:?} was too short: {:?}", duration, elapsed);
                assert!(elapsed <= duration + tolerance, 
                        "Duration {:?} was too long: {:?}", duration, elapsed);
            }
        }
    }

    #[cfg(feature = "web")]
    #[test]
    fn test_web_closure_pooling_performance() {
        use crate::animations::closure_pool::{register_pooled_callback, closure_pool_stats};
        
        // Test that closure pooling doesn't significantly impact performance
        let start = instant::Instant::now();
        
        // Register multiple callbacks to test pool performance
        let mut callback_ids = Vec::new();
        for i in 0..100 {
            let callback = Box::new(move || {
                // Simple callback that captures the loop variable
                let _result = i * 2;
            });
            let id = register_pooled_callback(callback);
            callback_ids.push(id);
        }
        
        let registration_time = start.elapsed();
        
        // Execute all callbacks
        let execution_start = instant::Instant::now();
        for id in callback_ids {
            crate::animations::closure_pool::execute_and_return_pooled_closure(id);
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

    #[test]
    fn test_time_provider_now() {
        // Test that TimeProvider::now() works consistently
        let time1 = MotionTime::now();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let time2 = MotionTime::now();
        
        assert!(time2 > time1, "Time should advance");
        assert!(time2.duration_since(time1) >= Duration::from_millis(1), 
                "Time difference should be at least 1ms");
    }
}

// Benchmark module for more detailed performance testing
#[cfg(test)]
mod benchmarks {
    use super::super::platform::{MotionTime, TimeProvider};
    use instant::{Duration, Instant};

    /// Benchmark sleep performance for different durations
    #[cfg(not(feature = "web"))]
    pub async fn benchmark_sleep_performance() {
        let durations = vec![
            Duration::from_micros(100),
            Duration::from_micros(500),
            Duration::from_millis(1),
            Duration::from_millis(5),
            Duration::from_millis(10),
            Duration::from_millis(16),
        ];

        println!("Sleep Performance Benchmark:");
        println!("Duration\t\tActual\t\tOverhead");
        
        for duration in durations {
            let mut total_elapsed = Duration::ZERO;
            let iterations = 10;
            
            for _ in 0..iterations {
                let start = Instant::now();
                MotionTime::delay(duration).await;
                total_elapsed += start.elapsed();
            }
            
            let avg_elapsed = total_elapsed / iterations as u32;
            let overhead = avg_elapsed.saturating_sub(duration);
            
            println!("{:?}\t\t{:?}\t\t{:?}", 
                     duration, avg_elapsed, overhead);
        }
    }

    /// Benchmark closure pooling performance
    #[cfg(feature = "web")]
    pub fn benchmark_closure_pooling() {
        use crate::animations::closure_pool::{register_pooled_callback, execute_and_return_pooled_closure};
        
        let iterations = 1000;
        
        // Benchmark callback registration
        let start = Instant::now();
        let mut callback_ids = Vec::with_capacity(iterations);
        
        for i in 0..iterations {
            let callback = Box::new(move || {
                let _result = i * i;
            });
            let id = register_pooled_callback(callback);
            callback_ids.push(id);
        }
        
        let registration_time = start.elapsed();
        
        // Benchmark callback execution
        let execution_start = Instant::now();
        for id in callback_ids {
            execute_and_return_pooled_closure(id);
        }
        let execution_time = execution_start.elapsed();
        
        println!("Closure Pooling Benchmark:");
        println!("Iterations: {}", iterations);
        println!("Registration time: {:?} ({:?} per callback)", 
                 registration_time, registration_time / iterations as u32);
        println!("Execution time: {:?} ({:?} per callback)", 
                 execution_time, execution_time / iterations as u32);
    }
}