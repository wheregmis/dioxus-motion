use instant::{Duration, Instant};
use std::future::Future;

pub trait TimeProvider {
    fn now() -> Instant;
    fn delay(duration: Duration) -> impl Future<Output = ()>;
}

#[derive(Debug, Clone, Copy)]
pub struct MotionTime;

impl TimeProvider for MotionTime {
    fn now() -> Instant {
        Instant::now()
    }

    fn delay(_duration: Duration) -> impl Future<Output = ()> {
        #[cfg(feature = "web")]
        {
            use futures_util::FutureExt;
            use wasm_bindgen::prelude::*;
            use web_sys::window;

            const RAF_THRESHOLD_MS: u64 = 16;

            let (sender, receiver) = futures_channel::oneshot::channel::<()>();

            if let Some(window) = window() {
                let cb = Closure::once(move || {
                    let _ = sender.send(());
                });

                // Cache the callback reference
                let cb_ref = cb.as_ref().unchecked_ref();

                // Choose timing method based on duration
                if _duration.as_millis() < RAF_THRESHOLD_MS as u128 {
                    window.request_animation_frame(cb_ref).unwrap();
                } else {
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb_ref,
                            _duration.as_millis() as i32,
                        )
                        .unwrap();
                }

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

pub type Time = MotionTime;
