//! Browser timing and physics harness; run scripts/check_browser_delay.py.
#![cfg(target_arch = "wasm32")]

use dioxus_motion::animations::platform::{MotionTime, TimeProvider};
use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    task::{Context, Waker},
};
use wasm_bindgen::prelude::*;

thread_local! {
    static DELAY: RefCell<Option<Pin<Box<dyn Future<Output = ()>>>>> = const { RefCell::new(None) };
}

#[wasm_bindgen]
pub fn start_delay(milliseconds: u32) -> bool {
    DELAY.with(|delay| {
        *delay.borrow_mut() = Some(Box::pin(MotionTime::delay(
            std::time::Duration::from_millis(u64::from(milliseconds)),
        )));
    });
    poll_delay()
}

#[wasm_bindgen]
pub fn poll_delay() -> bool {
    DELAY.with(|delay| {
        let mut delay = delay.borrow_mut();
        let Some(future) = delay.as_mut() else {
            return true;
        };
        let mut context = Context::from_waker(Waker::noop());
        if future.as_mut().poll(&mut context).is_ready() {
            *delay = None;
            true
        } else {
            false
        }
    })
}

#[wasm_bindgen]
pub fn cancel_delay() {
    DELAY.with(|delay| *delay.borrow_mut() = None);
}

#[wasm_bindgen]
pub fn retained_js_values() -> u32 {
    wasm_bindgen::externref_heap_live_count()
}

#[wasm_bindgen]
pub fn check_spring_numerics() -> Result<(), JsValue> {
    use dioxus_motion::{
        motion::Motion,
        prelude::{AnimationConfig, MotionStyle, Spring},
    };
    let error = |message: &str| JsValue::from_str(message);
    for initial in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        if Motion::new(initial).is_ok() {
            return Err(error("nonfinite initial value entered motion state"));
        }
    }
    for (stiffness, damping, mass) in [
        (100.0, 0.0, 1.0),
        (100.0, 20.0, 1.0),
        (100.0, 40.0, 1.0),
        (1e6, 10.0, 1.0),
        (100.0, 10.0, 1e-30),
        (100.0, 1e30, 1.0),
    ] {
        let mut motion = Motion::new(0.0f32).expect("finite initial value");
        motion
            .animate_to(
                1.0,
                AnimationConfig::spring(Spring {
                    stiffness,
                    damping,
                    mass,
                    velocity: 0.0,
                }),
            )
            .map_err(|e| error(&e.to_string()))?;
        for frame in 0..1000 {
            motion.update([1.0 / 60.0, 0.001, 0.1][frame % 3]);
            if !motion.current.is_finite() || !motion.velocity.is_finite() {
                return Err(error("spring state became nonfinite"));
            }
            if frame == 0
                && mass == 1e-30
                && ((motion.current - 0.153_518_27).abs() > 1e-6
                    || (motion.velocity - 8.464_818).abs() > 1e-5)
            {
                return Err(error("low-mass spring differs from its first-order limit"));
            }
            if frame == 0 && damping == 1e30 && motion.current <= 0.0 {
                return Err(error(
                    "overdamped spring lost its representable displacement",
                ));
            }
        }
    }
    let mut reference = Motion::new(5.0f32).expect("finite initial value");
    reference
        .animate_to(
            -3.0,
            AnimationConfig::spring(Spring {
                stiffness: 140.0,
                damping: 6.0,
                mass: 2.0,
                velocity: 0.0,
            }),
        )
        .map_err(|e| error(&e.to_string()))?;
    reference.velocity = 2.0;
    reference.update(0.02);
    if (reference.current - 4.929_104_5).abs() > 1e-5
        || (reference.velocity + 8.963_278).abs() > 1e-5
    {
        return Err(error("spring differs from closed-form reference"));
    }
    let saved = reference.current;
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        if reference
            .animate_to(bad, AnimationConfig::tween_ms(1))
            .is_ok()
            || reference.current != saved
        {
            return Err(error("nonfinite target replaced the current animation"));
        }
    }
    let mut initial = MotionStyle::default();
    initial.add_css_property("color", "rgba(0, 0, 0, 0)");
    let mut target = initial.clone();
    target.add_css_property("color", "rgba(0, 0, 0, 1)");
    let mut style = Motion::new(initial).expect("finite initial value");
    style
        .animate_to(target, AnimationConfig::spring(Spring::default()))
        .map_err(|e| error(&e.to_string()))?;
    if !style.update(1.0 / 60.0) {
        return Err(error("alpha-only spring snapped instead of animating"));
    }
    Ok(())
}
