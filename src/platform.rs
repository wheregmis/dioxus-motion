use instant::{Duration, Instant};
use std::future::Future;

pub trait TimeProvider {
    fn now() -> Instant;
    fn delay(duration: Duration) -> impl Future<Output = ()>;
}

#[derive(Debug, Clone, Copy)]
pub struct WebTime;

impl TimeProvider for WebTime {
    fn now() -> Instant {
        Instant::now()
    }

    fn delay(duration: Duration) -> impl Future<Output = ()> {
        let (sender, receiver) = futures_channel::oneshot::channel::<()>();

        // Use web-sys for wasm-bindgen compatible setTimeout
        #[cfg(feature = "web")]
        {
            use futures_util::FutureExt;
            use wasm_bindgen::prelude::*;
            use web_sys::window;

            if let Some(window) = window() {
                let cb = Closure::once(move || {
                    let _ = sender.send(());
                });

                // Use requestAnimationFrame for smoother animations
                if duration.as_millis() < 10 {
                    window
                        .request_animation_frame(cb.as_ref().unchecked_ref())
                        .unwrap();
                    cb.forget();
                } else {
                    // Use setTimeout for longer delays
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.as_ref().unchecked_ref(),
                            duration.as_millis() as i32,
                        )
                        .unwrap();
                    cb.forget();
                }
            }

            receiver.map(|_| ())
        }

        // Fallback for non-wasm or in case of window lookup failure
        #[cfg(not(feature = "web"))]
        {
            use futures_util::future::ready;
            std::thread::sleep(duration);
            ready(())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DesktopTime;

impl TimeProvider for DesktopTime {
    fn now() -> Instant {
        Instant::now()
    }

    fn delay(duration: Duration) -> impl Future<Output = ()> {
        #[cfg(not(feature = "web"))]
        {
            async move {
                tokio::time::sleep(duration).await;
            }
        }

        #[cfg(feature = "web")]
        {
            WebTime::delay(duration)
        }
    }
}
