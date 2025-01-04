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

            let (sender, receiver) = futures_channel::oneshot::channel::<()>();

            if let Some(window) = window() {
                let cb = Closure::once(move || {
                    let _ = sender.send(());
                });

                window
                    .request_animation_frame(cb.as_ref().unchecked_ref())
                    .unwrap();
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
