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
        prelude::{AnimationConfig, AnimationError, CssValue, MotionStyle, Spring},
    };
    let error = |message: &str| JsValue::from_str(message);
    for initial in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        if Motion::new(initial).is_ok() {
            return Err(error("nonfinite initial value entered motion state"));
        }
    }
    let mut completion_config = AnimationConfig::tween_ms(0).with_on_complete(|| {});
    let callback = completion_config
        .on_complete
        .as_ref()
        .expect("test callback")
        .clone();
    let guard = callback
        .lock()
        .map_err(|_| error("callback mutex poisoned"))?;
    let mut completed = Motion::new(0.0f32).map_err(|e| error(&e.to_string()))?;
    completed
        .animate_to(1.0, completion_config.clone())
        .map_err(|e| error(&e.to_string()))?;
    if completed.update(0.1) != Err(AnimationError::CompletionBusy)
        || completed.get_value() != 1.0
        || completed.is_running()
    {
        return Err(error(
            "busy completion must return an error after finishing playback",
        ));
    }
    drop(guard);
    completion_config
        .execute_completion()
        .map_err(|e| error(&e.to_string()))?;
    for (initial, role) in [(-f32::MAX, "spring value"), (0.0, "spring velocity")] {
        let mut motion = Motion::new(initial).map_err(|e| error(&e.to_string()))?;
        motion
            .animate_to(f32::MAX, AnimationConfig::spring(Spring::default()))
            .map_err(|e| error(&e.to_string()))?;
        if motion.update(1.0 / 60.0) != Err(AnimationError::NonFiniteValue(role))
            || motion.get_value() != initial
            || !motion.get_velocity().is_finite()
            || motion.is_running()
        {
            return Err(error(
                "overflowing spring frame was not rejected atomically",
            ));
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
                }),
            )
            .map_err(|e| error(&e.to_string()))?;
        for frame in 0..1000 {
            motion
                .update([1.0 / 60.0, 0.001, 0.1][frame % 3])
                .map_err(|e| error(&e.to_string()))?;
            if !motion.get_value().is_finite() || !motion.get_velocity().is_finite() {
                return Err(error("spring state became nonfinite"));
            }
            if frame == 0
                && mass == 1e-30
                && ((motion.get_value() - 0.153_518_27).abs() > 1e-6
                    || (motion.get_velocity() - 8.464_818).abs() > 1e-5)
            {
                return Err(error("low-mass spring differs from its first-order limit"));
            }
            if frame == 0 && damping == 1e30 && motion.get_value() <= 0.0 {
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
            }),
        )
        .map_err(|e| error(&e.to_string()))?;
    reference
        .set_velocity(2.0)
        .map_err(|e| error(&e.to_string()))?;
    reference.update(0.02).map_err(|e| error(&e.to_string()))?;
    if (reference.get_value() - 4.929_104_5).abs() > 1e-5
        || (reference.get_velocity() + 8.963_278).abs() > 1e-5
    {
        return Err(error("spring differs from closed-form reference"));
    }
    let saved = reference.get_value();
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        if reference
            .animate_to(bad, AnimationConfig::tween_ms(1))
            .is_ok()
            || reference.get_value() != saved
        {
            return Err(error("nonfinite target replaced the current animation"));
        }
    }
    let initial = MotionStyle::default().property("width", CssValue::Px(100.0));
    let mut removed = Motion::new(initial).map_err(|e| error(&e.to_string()))?;
    removed
        .animate_to(
            MotionStyle::default(),
            AnimationConfig::spring(Spring::default()),
        )
        .map_err(|e| error(&e.to_string()))?;
    removed
        .update(1.0 / 60.0)
        .map_err(|e| error(&e.to_string()))?;
    if !matches!(removed.get_value().properties.get("width"), Some(CssValue::Px(width)) if (0.0..100.0).contains(width))
    {
        return Err(error("removed CSS width sprang away from zero"));
    }
    let initial = MotionStyle::default().property("width", CssValue::Px(100.0));
    let target = MotionStyle::default().property("width", CssValue::Percent(10.0));
    let mut incompatible = Motion::new(initial.clone()).map_err(|e| error(&e.to_string()))?;
    if incompatible.animate_to(target, AnimationConfig::spring(Spring::default()))
        != Err(AnimationError::IncompatibleSpringValues)
        || incompatible.get_value() != initial
    {
        return Err(error(
            "incompatible CSS spring was accepted or changed its value",
        ));
    }
    let initial = MotionStyle {
        x: -f32::MAX,
        ..MotionStyle::default()
    }
    .property("width", CssValue::Px(-f32::MAX));
    let target = MotionStyle {
        x: f32::MAX,
        ..MotionStyle::default()
    }
    .property("width", CssValue::Px(f32::MAX));
    if target.to_css().contains("inf") {
        return Err(error("finite style formatted as infinite CSS"));
    }
    let mut extreme_style = Motion::new(initial).map_err(|e| error(&e.to_string()))?;
    extreme_style
        .animate_to(target, AnimationConfig::tween_ms(1000))
        .map_err(|e| error(&e.to_string()))?;
    extreme_style
        .update(0.5)
        .map_err(|e| error(&e.to_string()))?;
    let midpoint = extreme_style.get_value();
    if midpoint.x != 0.0 || midpoint.properties.get("width") != Some(&CssValue::Px(0.0)) {
        return Err(error("extreme style interpolation overflowed"));
    }
    let mut initial = MotionStyle::default();
    initial.add_css_property("color", "rgba(0, 0, 0, 0)");
    let mut target = initial.clone();
    target.add_css_property("color", "rgba(0, 0, 0, 1)");
    let mut style = Motion::new(initial).expect("finite initial value");
    style
        .animate_to(target, AnimationConfig::spring(Spring::default()))
        .map_err(|e| error(&e.to_string()))?;
    if !style
        .update(1.0 / 60.0)
        .map_err(|e| error(&e.to_string()))?
    {
        return Err(error("alpha-only spring snapped instead of animating"));
    }
    Ok(())
}
