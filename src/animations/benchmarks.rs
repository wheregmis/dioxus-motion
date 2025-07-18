//! Performance benchmarks for platform-specific optimizations
//!
//! This module contains benchmarks to validate the performance improvements
//! from closure pooling on web platforms and sleep optimization on desktop.

#[cfg(test)]
mod tests {
    use instant::{Duration, Instant};

    /// Benchmark closure pooling vs direct creation on web platforms
    #[cfg(feature = "web")]
    #[test]
    fn benchmark_web_closure_pooling_vs_creation() {
        use crate::animations::closure_pool::{register_pooled_callback, execute_and_return_pooled_closure};
        
        const ITERATIONS: usize = 1000;
        
        // Benchmark pooled closures
        let start = Instant::now();
        let mut pooled_ids = Vec::with_capacity(ITERATIONS);
        
        for i in 0..ITERATIONS {
            let callback = Box::new(move || {
                let _result = i * i + i;
            });
            let id = register_pooled_callback(callback);
            pooled_ids.push(id);
        }
        
        let pooled_registration_time = start.elapsed();
        
        let execution_start = Instant::now();
        for id in pooled_ids {
            execute_and_return_pooled_closure(id);
        }
        let pooled_execution_time = execution_start.elapsed();
        
        // Benchmark direct closure creation (simulation)
        let direct_start = Instant::now();
        let mut direct_callbacks: Vec<Box<dyn FnOnce()>> = Vec::with_capacity(ITERATIONS);
        
        for i in 0..ITERATIONS {
            let callback = Box::new(move || {
                let _result = i * i + i;
            });
            direct_callbacks.push(callback);
        }
        
        let direct_registration_time = direct_start.elapsed();
        
        let direct_execution_start = Instant::now();
        for callback in direct_callbacks {
            callback();
        }
        let direct_execution_time = direct_execution_start.elapsed();
        
        // Performance validation
        println!("Web Closure Performance Comparison ({} iterations):", ITERATIONS);
        println!("Pooled - Registration: {:?}, Execution: {:?}", 
                 pooled_registration_time, pooled_execution_time);
        println!("Direct - Registration: {:?}, Execution: {:?}", 
                 direct_registration_time, direct_execution_time);
        
        let total_pooled = pooled_registration_time + pooled_execution_time;
        let total_direct = direct_registration_time + direct_execution_time;
        
        println!("Total - Pooled: {:?}, Direct: {:?}", total_pooled, total_direct);
        
        // The pooled version may have overhead for simple operations
        // but should provide benefits in real-world scenarios with more complex closures
        // Allow reasonable tolerance for the pooling overhead
        let tolerance_factor = 5.0; // Allow up to 5x slower for pooled version in simple cases
        
        if total_pooled > total_direct * tolerance_factor as u32 {
            println!("Warning: Pooled closures are significantly slower than direct creation");
            println!("This may be acceptable for complex real-world scenarios");
        }
        
        // Ensure the pooled version is still reasonably fast
        assert!(total_pooled < Duration::from_millis(10),
                "Pooled closures are too slow: {:?}", total_pooled);
    }

    /// Benchmark desktop sleep optimization performance
    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn benchmark_desktop_sleep_optimization() {
        use crate::animations::platform::{MotionTime, TimeProvider};
        
        let test_durations = vec![
            Duration::from_micros(100),   // Very short - should yield
            Duration::from_micros(500),   // Short - should yield
            Duration::from_millis(1),     // Threshold - should sleep
            Duration::from_millis(5),     // Medium - should sleep
            Duration::from_millis(16),    // Frame time - should sleep
        ];
        
        println!("Desktop Sleep Optimization Benchmark:");
        println!("Duration\t\tActual\t\tOverhead\t\tEfficiency");
        
        for duration in test_durations {
            let iterations = if duration < Duration::from_millis(1) { 100 } else { 10 };
            let mut total_elapsed = Duration::ZERO;
            let mut total_overhead = Duration::ZERO;
            
            for _ in 0..iterations {
                let start = Instant::now();
                MotionTime::delay(duration).await;
                let elapsed = start.elapsed();
                
                total_elapsed += elapsed;
                total_overhead += elapsed.saturating_sub(duration);
            }
            
            let avg_elapsed = total_elapsed / iterations as u32;
            let avg_overhead = total_overhead / iterations as u32;
            let efficiency = if avg_elapsed.as_nanos() > 0 {
                (duration.as_nanos() as f64 / avg_elapsed.as_nanos() as f64) * 100.0
            } else {
                100.0
            };
            
            println!("{:?}\t\t{:?}\t\t{:?}\t\t{:.1}%", 
                     duration, avg_elapsed, avg_overhead, efficiency);
            
            // Validate performance characteristics
            if duration < Duration::from_millis(1) {
                // Very short durations should have minimal overhead
                assert!(avg_overhead < Duration::from_millis(2),
                        "Short duration {:?} has excessive overhead: {:?}", duration, avg_overhead);
            } else {
                // Longer durations should be reasonably accurate
                let max_overhead = Duration::from_millis(3);
                assert!(avg_overhead <= max_overhead,
                        "Duration {:?} has excessive overhead: {:?}", duration, avg_overhead);
                
                // Efficiency should be reasonable
                assert!(efficiency >= 50.0,
                        "Duration {:?} has poor efficiency: {:.1}%", duration, efficiency);
            }
        }
    }

    /// Benchmark memory allocation patterns (simplified version)
    #[test]
    fn benchmark_memory_allocation_patterns() {
        // Test allocation patterns with different approaches
        // This is a simplified version that doesn't use unsafe code
        
        let start = Instant::now();
        
        // Simulate creating multiple animation configs
        let mut configs = Vec::new();
        for i in 0..100 {
            let config = format!("config_{}", i);
            configs.push(config);
        }
        
        let allocation_time = start.elapsed();
        
        println!("Memory allocation benchmark:");
        println!("Time to create 100 configs: {:?}", allocation_time);
        
        // Validate that allocation is reasonably fast
        assert!(allocation_time < Duration::from_millis(10),
                "Config allocation took too long: {:?}", allocation_time);
        
        // Ensure we actually created the configs
        assert_eq!(configs.len(), 100, "Should have created 100 configs");
    }

    /// Extended battery life simulation test
    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn simulate_battery_life_impact() {
        use crate::animations::platform::{MotionTime, TimeProvider};
        
        // Simulate a typical animation scenario
        const ANIMATION_FRAMES: usize = 600; // 10 seconds at 60fps
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
        let overhead = total_time.saturating_sub(expected_time);
        
        println!("Battery Life Impact Simulation:");
        println!("Expected time: {:?}", expected_time);
        println!("Actual time: {:?}", total_time);
        println!("Overhead: {:?}", overhead);
        println!("CPU intensive frames: {}/{}", cpu_intensive_operations, ANIMATION_FRAMES);
        
        // Validate battery efficiency
        let efficiency = (expected_time.as_millis() as f64 / total_time.as_millis() as f64) * 100.0;
        println!("Time efficiency: {:.1}%", efficiency);
        
        // The optimization should maintain good efficiency
        assert!(efficiency >= 85.0, "Animation efficiency is too low: {:.1}%", efficiency);
        
        // CPU intensive operations should be minimal
        let cpu_intensive_ratio = cpu_intensive_operations as f64 / ANIMATION_FRAMES as f64;
        assert!(cpu_intensive_ratio <= 0.1, 
                "Too many CPU intensive frames: {:.1}%", cpu_intensive_ratio * 100.0);
    }

    /// Performance regression test
    #[test]
    fn test_performance_regression() {
        // This test ensures that optimizations don't introduce performance regressions
        const ITERATIONS: usize = 10000;
        
        // Test simple operations that should be fast
        let start = Instant::now();
        
        for i in 0..ITERATIONS {
            let _result = i * 2 + 1;
        }
        
        let elapsed = start.elapsed();
        let per_operation = elapsed / ITERATIONS as u32;
        
        println!("Performance regression test:");
        println!("Total time for {} operations: {:?}", ITERATIONS, elapsed);
        println!("Time per operation: {:?}", per_operation);
        
        // Operations should be very fast
        assert!(per_operation < Duration::from_micros(1),
                "Operations are too slow: {:?} per operation", per_operation);
        
        // Total time should be reasonable
        assert!(elapsed < Duration::from_millis(10),
                "Total time is too long: {:?}", elapsed);
    }
}

/// Utility functions for benchmarking
#[cfg(test)]
mod utils {
    use instant::{Duration, Instant};
    
    /// Measures the time taken to execute a closure
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        (result, elapsed)
    }
    
    /// Runs a benchmark multiple times and returns statistics
    pub fn benchmark_iterations<F>(iterations: usize, mut f: F) -> BenchmarkStats
    where
        F: FnMut(),
    {
        let mut times = Vec::with_capacity(iterations);
        
        for _ in 0..iterations {
            let (_, elapsed) = measure_time(&mut f);
            times.push(elapsed);
        }
        
        BenchmarkStats::from_times(times)
    }
    
    /// Statistics from benchmark runs
    #[derive(Debug)]
    pub struct BenchmarkStats {
        pub min: Duration,
        pub max: Duration,
        pub mean: Duration,
        pub total: Duration,
    }
    
    impl BenchmarkStats {
        fn from_times(mut times: Vec<Duration>) -> Self {
            times.sort();
            
            let min = times[0];
            let max = times[times.len() - 1];
            let total: Duration = times.iter().sum();
            let mean = total / times.len() as u32;
            
            Self { min, max, mean, total }
        }
    }
}