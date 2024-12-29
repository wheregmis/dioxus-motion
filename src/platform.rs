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
        // Use web-sys for wasm-bindgen compatible setTimeout
        #[cfg(feature = "web")]
        {
            use futures_util::FutureExt;
            use wasm_bindgen::prelude::*;
            use web_sys::{window, Window};

            let (sender, receiver) = futures_channel::oneshot::channel::<()>();

            // Calculate total frames needed for the duration
            // Using 16.67ms as frame time (60 FPS)
            // TODO: Make this frame rate configured as per the device FPS
            let total_frames = (duration.as_millis() as f64 / 16.67).ceil() as i32;

            fn request_next_frame(
                window: &Window,
                frames_left: i32,
                sender: futures_channel::oneshot::Sender<()>,
            ) {
                if frames_left <= 0 {
                    let _ = sender.send(());
                    return;
                }

                let window_clone = window.clone();
                let cb = Closure::once(move || {
                    request_next_frame(&window_clone, frames_left - 1, sender);
                });

                window
                    .request_animation_frame(cb.as_ref().unchecked_ref())
                    .unwrap();
                cb.forget();
            }

            if let Some(window) = window() {
                request_next_frame(&window, total_frames, sender);
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
