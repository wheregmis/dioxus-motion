//! Platform abstraction for time-related functionality
//!
//! Provides cross-platform timing operations for animations.
//! Supports both web (WASM) and native platforms.

use super::core::AnimationError;
use instant::{Duration, Instant};
use std::future::Future;

#[cfg(feature = "web")]
enum BrowserRequest {
    AnimationFrame(i32),
    Timeout(i32),
}

#[cfg(feature = "web")]
struct BrowserTimer {
    window: web_sys::Window,
    request: BrowserRequest,
    _callback: wasm_bindgen::closure::Closure<dyn FnMut()>,
}

#[cfg(feature = "web")]
impl Drop for BrowserTimer {
    fn drop(&mut self) {
        match self.request {
            BrowserRequest::AnimationFrame(id) => {
                let _ = self.window.cancel_animation_frame(id);
            }
            BrowserRequest::Timeout(id) => self.window.clear_timeout_with_handle(id),
        }
    }
}

/// Provides platform-agnostic timing operations
///
/// Abstracts timing functionality across different platforms,
/// ensuring consistent animation behavior in both web and native environments.
pub trait TimeProvider {
    /// Returns the current instant
    fn now() -> Instant;

    /// Creates a future that completes after the specified duration.
    /// Returns an error if a platform timer cannot be scheduled.
    fn delay(duration: Duration) -> impl Future<Output = Result<(), AnimationError>>;
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
    /// Uses requestAnimationFrame for delays up to 16ms
    /// Uses setTimeout for longer delays, capped at i32::MAX milliseconds
    ///
    /// # Native
    /// Uses tokio::time::sleep in a Tokio runtime with time enabled.
    #[cfg(feature = "web")]
    async fn delay(duration: Duration) -> Result<(), AnimationError> {
        use wasm_bindgen::prelude::*;
        let Some(window) = web_sys::window() else {
            return Err(AnimationError::TimerUnavailable);
        };
        let (sender, receiver) = futures_channel::oneshot::channel::<()>();
        let callback = Closure::once(move || {
            let _ = sender.send(());
        });
        let request = if duration <= Duration::from_millis(16) {
            window
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .map(BrowserRequest::AnimationFrame)
        } else {
            // Browser timers take a signed 32-bit millisecond delay.
            let milliseconds = i32::try_from(duration.as_millis()).unwrap_or(i32::MAX);
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    milliseconds,
                )
                .map(BrowserRequest::Timeout)
        };
        let request = request.map_err(|_| AnimationError::TimerUnavailable)?;
        let _timer = BrowserTimer {
            window,
            request,
            _callback: callback,
        };
        receiver.await.map_err(|_| AnimationError::TimerUnavailable)
    }

    #[cfg(not(feature = "web"))]
    async fn delay(duration: Duration) -> Result<(), AnimationError> {
        tokio::runtime::Handle::try_current().map_err(|_| AnimationError::TimerUnavailable)?;
        if duration >= Duration::from_millis(1) {
            tokio::time::sleep(duration).await;
        } else {
            tokio::task::yield_now().await;
        }
        Ok(())
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
        std::thread::sleep(Duration::from_millis(1));
        let time2 = MotionTime::now();

        assert!(time2 > time1, "Time should advance");
        assert!(
            time2.duration_since(time1) >= Duration::from_millis(1),
            "Time difference should be at least 1ms"
        );
    }

    #[cfg(not(feature = "web"))]
    #[test]
    fn missing_runtime_returns_error() {
        let mut delay = Box::pin(MotionTime::delay(Duration::from_millis(10)));
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        assert_eq!(
            delay.as_mut().poll(&mut context),
            std::task::Poll::Ready(Err(AnimationError::TimerUnavailable))
        );
    }

    #[cfg(not(feature = "web"))]
    #[tokio::test(start_paused = true)]
    async fn test_desktop_sleep_threshold_optimization() {
        let start = tokio::time::Instant::now();
        let delay = MotionTime::delay(Duration::from_micros(500));
        futures_util::pin_mut!(delay);
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(delay.as_mut().poll(&mut context).is_pending());
        delay.await.expect("available test timer");
        assert_eq!(start.elapsed(), Duration::ZERO);
    }

    #[cfg(not(feature = "web"))]
    #[tokio::test(start_paused = true)]
    async fn test_desktop_sleep_longer_duration() {
        let duration = Duration::from_millis(10);
        let start = tokio::time::Instant::now();
        MotionTime::delay(duration)
            .await
            .expect("available test timer");
        assert_eq!(start.elapsed(), duration);
    }

    #[cfg(not(feature = "web"))]
    #[tokio::test(start_paused = true)]
    async fn test_desktop_sleep_threshold_boundary() {
        let duration = Duration::from_millis(1);
        let start = tokio::time::Instant::now();
        MotionTime::delay(duration)
            .await
            .expect("available test timer");
        assert_eq!(start.elapsed(), duration);
    }
}
