//! Platform abstraction for time-related functionality
//!
//! Provides cross-platform timing operations for animations.
//! Supports both web (WASM) and native platforms.

use instant::{Duration, Instant};
use std::future::Future;

#[cfg(feature = "web")]
use crate::animations::closure_pool::{register_pooled_callback, create_pooled_closure};

/// Provides platform-agnostic timing operations
///
/// Abstracts timing functionality across different platforms,
/// ensuring consistent animation behavior in both web and native environments.
pub trait TimeProvider {
    /// Returns the current instant
    fn now() -> Instant;

    /// Creates a future that completes after the specified duration
    fn delay(duration: Duration) -> impl Future<Output = ()>;
}

/// Default time provider implementation for motion animations
///
/// Implements platform-specific timing operations:
/// - For web: Uses requestAnimationFrame or setTimeout
/// - For native: Uses tokio's sleep
#[derive(Debug, Clone, Copy)]
pub struct MotionTime;

impl TimeProvider for MotionTime {
    fn now() -> Instant {
        Instant::now()
    }

    /// Creates a delay future using platform-specific implementations
    ///
    /// # Web
    /// Uses requestAnimationFrame for short delays (<16ms)
    /// Uses setTimeout for longer delays
    ///
    /// # Native
    /// Uses tokio::time::sleep
    #[cfg(feature = "web")]
    fn delay(_duration: Duration) -> impl Future<Output = ()> {
        use futures_util::FutureExt;
        use wasm_bindgen::prelude::*;
        use web_sys::window;

        const RAF_THRESHOLD_MS: u8 = 16;

        let (sender, receiver) = futures_channel::oneshot::channel::<()>();

        if let Some(window) = window() {
            // Choose timing method based on duration
            if _duration.as_millis() <= RAF_THRESHOLD_MS as u128 {
                // For frame-based timing, use requestAnimationFrame
                // This is ideal for animation frames (typically 16ms at 60fps)
                
                // Use pooled closure for better performance
                let callback_id = register_pooled_callback(Box::new(move || {
                    let _ = sender.send(());
                }));
                let cb = create_pooled_closure(callback_id);

                window
                    .request_animation_frame(cb.as_ref().unchecked_ref())
                    .expect("Failed to request animation frame");

                cb.forget();
            } else {
                // For longer delays, use setTimeout which is more appropriate
                
                // Use pooled closure for better performance
                let callback_id = register_pooled_callback(Box::new(move || {
                    let _ = sender.send(());
                }));
                let cb = create_pooled_closure(callback_id);

                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        _duration.as_millis() as i32,
                    )
                    .expect("Failed to set timeout");

                cb.forget();
            }
        } else {
            // Fallback: complete immediately if no window
            let _ = sender.send(());
        }

        receiver.map(|_| ())
    }

    #[cfg(not(feature = "web"))]
    fn delay(duration: Duration) -> impl Future<Output = ()> {
        Box::pin(async move {
            // Threshold-based sleep optimization
            const MIN_SPIN_THRESHOLD: Duration = Duration::from_millis(1);
            
            if duration > MIN_SPIN_THRESHOLD {
                let start = std::time::Instant::now();
                
                // Use tokio sleep for longer durations
                tokio::time::sleep(duration).await;
                
                // High precision timing for desktop - only for remaining time
                let remaining = duration.saturating_sub(start.elapsed());
                if remaining > Duration::from_micros(100) {
                    spin_sleep::sleep(remaining);
                }
            } else {
                // For very short durations, skip sleep entirely to avoid CPU waste
                // This prevents unnecessary context switching for sub-millisecond delays
                tokio::task::yield_now().await;
            }
        })
    }
}

/// Type alias for the default time provider
pub type Time = MotionTime;

#[cfg(test)]
mod tests {
    use super::*;

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

    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn test_desktop_sleep_threshold_optimization() {
        // Test that very short durations don't use spin sleep
        let short_duration = Duration::from_micros(500);
        let start = Instant::now();
        
        MotionTime::delay(short_duration).await;
        
        let elapsed = start.elapsed();
        
        // For very short durations, we should yield instead of sleep
        // The elapsed time should be minimal (less than 2ms)
        assert!(elapsed < Duration::from_millis(2), 
                "Short duration sleep took too long: {:?}", elapsed);
    }

    #[cfg(not(feature = "web"))]
    #[tokio::test]
    async fn test_desktop_sleep_longer_duration() {
        // Test that longer durations use proper sleep
        let long_duration = Duration::from_millis(10);
        let start = Instant::now();
        
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
        let start = Instant::now();
        
        MotionTime::delay(threshold_duration).await;
        
        let elapsed = start.elapsed();
        
        // At the threshold, we should still use proper sleep
        assert!(elapsed >= Duration::from_micros(800), 
                "Threshold duration sleep was too short: {:?}", elapsed);
        assert!(elapsed <= Duration::from_millis(3), 
                "Threshold duration sleep was too long: {:?}", elapsed);
    }
}
