//! Platform abstraction for time-related functionality
//!
//! Provides cross-platform timing operations for animations.
//! Supports both web (WASM) and native platforms.

use instant::{Duration, Instant};
use std::future::Future;

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
                let cb = Closure::once(move || {
                    let _ = sender.send(());
                });

                window
                    .request_animation_frame(cb.as_ref().unchecked_ref())
                    .expect("Failed to request animation frame");

                cb.forget();
            } else {
                // For longer delays, use setTimeout which is more appropriate
                let cb = Closure::once(move || {
                    let _ = sender.send(());
                });

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
            let start = Instant::now();

            #[cfg(feature = "desktop")]
            tokio::time::sleep(duration).await;
            #[cfg(not(feature = "desktop"))]
            std::thread::sleep(duration);

            // High precision timing for desktop
            let remaining = duration.saturating_sub(start.elapsed());
            if remaining.subsec_micros() > 0 {
                spin_sleep::sleep(remaining);
            }
        })
    }
}

/// Type alias for the default time provider
pub type Time = MotionTime;
