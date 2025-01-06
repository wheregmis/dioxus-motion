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
    fn delay(_duration: Duration) -> impl Future<Output = ()> {
        #[cfg(feature = "web")]
        {
            use futures_util::FutureExt;
            use wasm_bindgen::prelude::*;
            use web_sys::window;

            let (sender, receiver) = futures_channel::oneshot::channel::<()>();

            if let Some(window) = window() {
                let cb = Closure::once(move || {
                    let _ = sender.send(());
                });

                // Cache the callback reference
                let cb_ref = cb.as_ref().unchecked_ref();

                window.request_animation_frame(cb_ref).unwrap();
                cb.forget();
            }

            receiver.map(|_| ())
        }

        #[cfg(not(feature = "web"))]
        {
            use futures_util::future::BoxFuture;
            Box::pin(async move {
                tokio::time::sleep(_duration).await;
            })
        }
    }
}

/// Type alias for the default time provider
pub type Time = MotionTime;
