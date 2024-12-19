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
                let ms = duration.as_millis() as i32;
                let closure = Closure::once_into_js(move || {
                    let _ = sender.send(());
                });
                let function = closure.as_ref().unchecked_ref();
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(function, ms);
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
