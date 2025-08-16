//! Enhanced store-based Motion API with keyframes and sequences
//!
//! This provides working keyframe and sequence animation functionality
//! for the store-based Motion API.

use crate::Duration;
use crate::animations::core::Animatable;
use crate::animations::platform::TimeProvider;
use crate::keyframes::KeyframeAnimation;
use crate::sequence::AnimationSequence;
use crate::store::{MotionStore, MotionStoreStoreExt};
use dioxus::prelude::*;
use std::sync::Arc;

/// Enhanced motion store hook with keyframe animation support
///
/// Returns (motion_store, animate_keyframes_function)
pub fn use_motion_store_with_keyframes<T: Animatable + Copy + Default + Send + 'static>(
    initial: T,
) -> (Store<MotionStore<T>>, impl FnMut(KeyframeAnimation<T>)) {
    let store = use_store(|| MotionStore::new(initial));
    let keyframes_ref = use_signal(|| None::<Arc<KeyframeAnimation<T>>>);

    // Animation loop for keyframes
    use_effect({
        let store = store.clone();
        let keyframes_ref = keyframes_ref.clone();
        move || {
            spawn(async move {
                let mut last_frame = crate::MotionTime::now();

                loop {
                    let now = crate::MotionTime::now();
                    let dt = (now.duration_since(last_frame).as_secs_f32()).min(0.1);
                    last_frame = now;

                    if store.running()() && store.animation_type()() == "keyframes" {
                        if let Some(keyframes) = keyframes_ref.read().as_ref() {
                            let elapsed = store.elapsed()();
                            let progress = (elapsed.as_secs_f32()
                                / keyframes.duration.as_secs_f32())
                            .clamp(0.0, 1.0);

                            if !keyframes.keyframes.is_empty() {
                                // Find current keyframe segment
                                let (start, end) =
                                    find_keyframe_segment(&keyframes.keyframes, progress);

                                // Calculate local progress within segment
                                let local_progress = if start.offset == end.offset {
                                    1.0
                                } else {
                                    (progress - start.offset) / (end.offset - start.offset)
                                };

                                // Apply easing if present
                                let eased_progress = end.easing.map_or(local_progress, |easing| {
                                    easing(local_progress, 0.0, 1.0, 1.0)
                                });

                                // Interpolate and update current value
                                let new_value = start.value.interpolate(&end.value, eased_progress);
                                store.current().set(new_value);

                                // Update elapsed time
                                let new_elapsed = elapsed + Duration::from_secs_f32(dt);
                                store.elapsed().set(new_elapsed);

                                // Check if animation is complete
                                if progress >= 1.0 {
                                    store.running().set(false);
                                    store.animation_type().set("simple".to_string());
                                }
                            }
                        }
                    }

                    // Frame timing
                    crate::MotionTime::delay(Duration::from_millis(16)).await; // ~60 FPS
                }
            });
        }
    });

    // Function to start keyframe animation
    let animate_keyframes = {
        let store = store.clone();
        let mut keyframes_ref = keyframes_ref.clone();
        move |animation: KeyframeAnimation<T>| {
            keyframes_ref.set(Some(Arc::new(animation)));
            store.animation_type().set("keyframes".to_string());
            store.running().set(true);
            store.elapsed().set(Duration::default());
            store.current_keyframe().set(0);
        }
    };

    (store, animate_keyframes)
}

/// Enhanced motion store hook with sequence animation support
///
/// Returns (motion_store, animate_sequence_function)
pub fn use_motion_store_with_sequences<T: Animatable + Copy + Default + Send + 'static>(
    initial: T,
) -> (Store<MotionStore<T>>, impl FnMut(AnimationSequence<T>)) {
    let store = use_store(|| MotionStore::new(initial));
    let sequence_ref = use_signal(|| None::<Arc<AnimationSequence<T>>>);

    // Animation loop for sequences
    use_effect({
        let store = store.clone();
        let sequence_ref = sequence_ref.clone();
        move || {
            spawn(async move {
                let mut last_frame = crate::MotionTime::now();

                loop {
                    let now = crate::MotionTime::now();
                    let dt = (now.duration_since(last_frame).as_secs_f32()).min(0.1);
                    last_frame = now;

                    if store.running()() && store.animation_type()() == "sequence" {
                        if let Some(sequence) = sequence_ref.read().as_ref() {
                            let current_step_index = store.current_sequence_step()() as usize;

                            if let Some(current_step) = sequence.steps().get(current_step_index) {
                                // Animate towards current step target
                                let current = store.current()();
                                let target = current_step.target;
                                let diff = target - current;
                                let epsilon = T::epsilon();

                                if diff.magnitude() < epsilon {
                                    // Reached current step target, advance to next step
                                    if current_step_index + 1 < sequence.total_steps() {
                                        store
                                            .current_sequence_step()
                                            .set((current_step_index + 1) as u8);
                                        store
                                            .target()
                                            .set(sequence.steps()[current_step_index + 1].target);
                                    } else {
                                        // Sequence complete
                                        store.running().set(false);
                                        store.animation_type().set("simple".to_string());
                                    }
                                } else {
                                    // Continue animating towards current step target using simple interpolation
                                    let speed_factor = 6.0;
                                    let step = diff * (dt * speed_factor);
                                    let new_current = current + step;
                                    store.current().set(new_current);
                                }
                            }
                        }
                    }

                    // Frame timing
                    crate::MotionTime::delay(Duration::from_millis(16)).await; // ~60 FPS
                }
            });
        }
    });

    // Function to start sequence animation
    let animate_sequence = {
        let store = store.clone();
        let mut sequence_ref = sequence_ref.clone();
        move |sequence: AnimationSequence<T>| {
            sequence_ref.set(Some(Arc::new(sequence.clone())));
            store.animation_type().set("sequence".to_string());
            store.running().set(true);
            store.elapsed().set(Duration::default());
            store.current_sequence_step().set(0);

            // Set initial target to first step
            if let Some(first_step) = sequence.steps().first() {
                store.target().set(first_step.target);
            }
        }
    };

    (store, animate_sequence)
}

/// Helper function to find the current keyframe segment
fn find_keyframe_segment<T: Animatable>(
    keyframes: &[crate::keyframes::Keyframe<T>],
    progress: f32,
) -> (
    &crate::keyframes::Keyframe<T>,
    &crate::keyframes::Keyframe<T>,
) {
    keyframes
        .windows(2)
        .find(|w| progress >= w[0].offset && progress <= w[1].offset)
        .map(|w| (&w[0], &w[1]))
        .unwrap_or_else(|| {
            if progress <= keyframes[0].offset {
                let first = &keyframes[0];
                (first, first)
            } else {
                let last = keyframes.last().expect("Keyframes should not be empty");
                (last, last)
            }
        })
}
