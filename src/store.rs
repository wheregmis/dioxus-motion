//! Motion Store API - The primary animation interface for Dioxus Motion
//!
//! This module provides the main animation API for Dioxus Motion. It offers a
//! store-based approach with fine-grained reactivity, allowing components to
//! subscribe to specific fields of the animation state rather than the entire
//! animation object. This reduces unnecessary re-renders and provides optimal
//! performance.
//!
//! # Features
//! - Fine-grained reactivity via Dioxus Store
//! - Support for simple animations, keyframes, and sequences
//! - Event callbacks (on_complete, on_update)
//! - Unified animation loop (no duplicate spawns)
//! - Platform-specific optimizations

use crate::Duration;
use crate::animations::core::{Animatable, AnimationConfig, AnimationMode, LoopMode};
use crate::animations::platform::TimeProvider;
use crate::keyframes::KeyframeAnimation;
use crate::sequence::AnimationSequence;
use dioxus::prelude::*;
use std::sync::Arc;

/// Helper function to calculate the appropriate delay for the animation loop
fn calculate_delay(dt: f32, running_frames: u32) -> Duration {
    #[cfg(feature = "web")]
    {
        // running_frames is not used in web builds but kept for API consistency
        let _ = running_frames;
        match dt {
            x if x < 0.008 => Duration::from_millis(8),  // ~120fps
            x if x < 0.016 => Duration::from_millis(16), // ~60fps
            _ => Duration::from_millis(32),              // ~30fps
        }
    }
    #[cfg(not(feature = "web"))]
    {
        if running_frames <= 200 {
            Duration::from_micros(8333) // ~120fps
        } else {
            match dt {
                x if x < 0.005 => Duration::from_millis(8),  // ~120fps
                x if x < 0.011 => Duration::from_millis(16), // ~60fps
                _ => Duration::from_millis(33),              // ~30fps
            }
        }
    }
}

/// Animation type discriminant
#[derive(Clone, Debug, PartialEq, Default)]
enum AnimationType {
    #[default]
    Simple,
    Keyframes,
    Sequence,
}

/// Store-based Motion struct with fine-grained reactivity
///
/// Unlike the signal-based `Motion<T>`, `MotionStore<T>` allows components to subscribe
/// to specific fields, reducing unnecessary re-renders. Components can subscribe to:
/// - `current()` - The animated value (most common)
/// - `running()` - Animation state
/// - `target()` - Animation target
/// - And other fields as needed
#[derive(Store)]
pub struct MotionStore<T> {
    /// The current animated value
    pub current: T,
    /// The target value the animation is moving towards
    pub target: T,
    /// Current velocity of the animation
    pub velocity: T,

    /// The initial value when the animation was created
    pub initial: T,
    /// Whether the animation is currently running
    pub running: bool,
    /// Time elapsed since animation started
    pub elapsed: Duration,
    /// Time elapsed for delay (before animation starts)
    pub delay_elapsed: Duration,
    /// Current loop iteration (for looped animations)
    pub current_loop: u8,
    /// Whether the animation is running in reverse
    pub reverse: bool,

    /// Current keyframe index for keyframe animations
    pub current_keyframe: u8,
    /// Current sequence step for sequence animations
    pub current_sequence_step: u8,
    /// Animation configuration (spring/tween settings)
    pub config: AnimationConfig,
}

impl<T: Animatable + Copy + Default> MotionStore<T> {
    /// Create a new MotionStore with the given initial value
    pub fn new(initial: T) -> Self {
        Self {
            initial,
            current: initial,
            target: initial,
            velocity: T::default(),
            running: false,
            elapsed: Duration::default(),
            delay_elapsed: Duration::default(),
            current_loop: 0,
            reverse: false,
            current_keyframe: 0,
            current_sequence_step: 0,
            config: AnimationConfig::default(),
        }
    }
}

/// Handle for controlling a motion store animation
///
/// This provides methods for starting different animation types while maintaining
/// access to the internal signals needed for keyframes and sequences.
#[derive(Clone, Copy, PartialEq)]
pub struct MotionHandle<T: Animatable + Copy + Default + Send + 'static> {
    pub store: Store<MotionStore<T>>,
    keyframes_ref: Signal<Option<Arc<KeyframeAnimation<T>>>>,
    sequence_ref: Signal<Option<Arc<AnimationSequence<T>>>>,
    animation_type: Signal<AnimationType>,
}

impl<T: Animatable + Copy + Default + Send + 'static> MotionHandle<T> {
    /// Get the underlying store for field subscriptions
    pub fn store(&self) -> &Store<MotionStore<T>> {
        &self.store
    }

    /// Animate to a target value with the given configuration
    pub fn animate_to(&mut self, target: T, config: AnimationConfig) {
        self.animation_type.set(AnimationType::Simple);
        // Capture current value as the starting point for this animation
        self.store.initial().set(self.store.current()());
        self.store.config().set(config.clone());
        self.store.target().set(target);
        // Reset velocity to prevent bleed from previous animations
        self.store.velocity().set(T::default());
        self.store.running().set(true);
        self.store.elapsed().set(Duration::default());
        self.store.delay_elapsed().set(config.delay);
        self.store.current_loop().set(0);
        self.store.current_keyframe().set(0);
        self.store.current_sequence_step().set(0);
    }

    /// Start a keyframe animation
    pub fn animate_keyframes(&mut self, animation: KeyframeAnimation<T>) {
        self.animation_type.set(AnimationType::Keyframes);
        self.keyframes_ref.set(Some(Arc::new(animation)));
        self.store.running().set(true);
        self.store.elapsed().set(Duration::default());
        self.store.current_keyframe().set(0);
    }

    /// Start a sequence animation
    pub fn animate_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.animation_type.set(AnimationType::Sequence);
        self.sequence_ref.set(Some(Arc::new(sequence.clone())));

        if let Some(first_step) = sequence.steps().first() {
            self.store.target().set(first_step.target);
            self.store.config().set((*first_step.config).clone());
            self.store.running().set(true);
            self.store.elapsed().set(Duration::default());
            self.store.current_sequence_step().set(0);
        }
    }

    /// Stop the current animation
    pub fn stop(&mut self) {
        self.store.running().set(false);
        self.store.velocity().set(T::default());
        self.animation_type.set(AnimationType::Simple);
    }

    /// Reset to initial value
    pub fn reset(&mut self) {
        let initial = self.store.initial()();
        self.store.current().set(initial);
        self.store.target().set(initial);
        self.store.running().set(false);
        self.store.velocity().set(T::default());
        self.animation_type.set(AnimationType::Simple);
    }

    /// Get the current value
    pub fn get_value(&self) -> T {
        self.store.current()()
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        self.store.running()()
    }
}

/// Custom store methods using the #[store] attribute
/// This provides computed properties and business logic for the MotionStore<T>
#[store]
impl<T: Animatable + Copy + Default> Store<MotionStore<T>> {
    /// Get the current animated value
    fn get_value(&self) -> T {
        self.current().cloned()
    }

    /// Check if animation is currently running
    fn is_running(&self) -> bool {
        self.running().cloned()
    }

    /// Check if animation has reached its target value (within epsilon)
    fn is_at_target(&self) -> bool {
        let current = self.current().cloned();
        let target = self.target().cloned();
        let diff = current - target;
        diff.magnitude() < T::epsilon()
    }

    /// Get the magnitude of the current velocity
    fn get_velocity_magnitude(&self) -> f32 {
        self.velocity().cloned().magnitude()
    }

    /// Update the animation by the given delta time (for simple animations)
    ///
    /// Returns true if the animation is still running, false if it completed
    fn update_simple(&mut self, dt: f32) -> bool {
        if !self.running().cloned() {
            return false;
        }

        // Check if we're still in delay phase
        let delay_elapsed = self.delay_elapsed().cloned();
        if delay_elapsed > Duration::default() {
            let dt_duration = Duration::from_secs_f32(dt);
            if dt_duration >= delay_elapsed {
                self.delay_elapsed().set(Duration::default());
            } else {
                self.delay_elapsed().set(delay_elapsed - dt_duration);
                return true;
            }
        }

        // Update elapsed time
        let new_elapsed = self.elapsed().cloned() + Duration::from_secs_f32(dt);
        self.elapsed().set(new_elapsed);

        // Get current animation state
        let current = self.current().cloned();
        let target = self.target().cloned();
        let current_velocity = self.velocity().cloned();
        let diff = target - current;
        let config = self.config().cloned();
        let epsilon = T::epsilon();

        if diff.magnitude() < epsilon {
            // Close enough to target, snap to target
            self.current().set(target);
            self.velocity().set(T::default());
            self.handle_loop_completion()
        } else {
            match config.mode {
                AnimationMode::Spring(spring) => {
                    // Spring physics: F = -kx - cv
                    let stiffness = spring.stiffness;
                    let damping_ratio = spring.damping;
                    let mass = spring.mass;

                    let spring_force = diff * stiffness;
                    let damping_force = current_velocity * damping_ratio;
                    let net_force = spring_force - damping_force;

                    let acceleration = net_force * (1.0 / mass);
                    let new_velocity = current_velocity + acceleration * dt;
                    let new_current = current + new_velocity * dt;

                    self.current().set(new_current);
                    self.velocity().set(new_velocity);
                    true
                }
                AnimationMode::Tween(tween) => {
                    // Tween interpolation based on elapsed time and duration
                    let progress =
                        (new_elapsed.as_secs_f32() / tween.duration.as_secs_f32()).clamp(0.0, 1.0);

                    // Apply easing function
                    let eased_progress = (tween.easing)(progress, 0.0, 1.0, 1.0);

                    // Interpolate from initial value to target
                    let initial = self.initial().cloned();
                    let new_current = initial.interpolate(&target, eased_progress);

                    // Guard against dt == 0.0 when computing velocity
                    let new_velocity = if dt > 0.0 {
                        let delta = new_current - current;
                        delta * (1.0 / dt)
                    } else {
                        T::default()
                    };

                    self.current().set(new_current);
                    self.velocity().set(new_velocity);

                    // Check if tween is complete
                    if progress >= 1.0 {
                        self.current().set(target);
                        self.velocity().set(T::default());
                        self.handle_loop_completion()
                    } else {
                        true
                    }
                }
            }
        }
    }

    /// Handle loop completion logic
    fn handle_loop_completion(&mut self) -> bool {
        let config = self.config().cloned();
        let target = self.target().cloned();

        if let Some(loop_mode) = config.loop_mode {
            match loop_mode {
                LoopMode::Infinite => {
                    self.current().set(self.initial().cloned());
                    self.elapsed().set(Duration::default());
                    self.current_loop().set(self.current_loop().cloned() + 1);
                    true
                }
                LoopMode::Times(count) => {
                    let current_loop = self.current_loop().cloned();
                    if current_loop < count - 1 {
                        self.current().set(self.initial().cloned());
                        self.elapsed().set(Duration::default());
                        self.current_loop().set(current_loop + 1);
                        true
                    } else {
                        self.execute_on_complete();
                        false
                    }
                }
                LoopMode::Alternate => {
                    let new_target = if self.reverse().cloned() {
                        self.initial().cloned()
                    } else {
                        target
                    };
                    let new_initial = if self.reverse().cloned() {
                        target
                    } else {
                        self.initial().cloned()
                    };

                    self.target().set(new_target);
                    self.initial().set(new_initial);
                    self.reverse().set(!self.reverse().cloned());
                    self.elapsed().set(Duration::default());
                    self.current_loop().set(self.current_loop().cloned() + 1);
                    true
                }
                LoopMode::AlternateTimes(count) => {
                    let current_loop = self.current_loop().cloned();
                    if current_loop < (count * 2) - 1 {
                        let new_target = if self.reverse().cloned() {
                            self.initial().cloned()
                        } else {
                            target
                        };
                        let new_initial = if self.reverse().cloned() {
                            target
                        } else {
                            self.initial().cloned()
                        };

                        self.target().set(new_target);
                        self.initial().set(new_initial);
                        self.reverse().set(!self.reverse().cloned());
                        self.elapsed().set(Duration::default());
                        self.current_loop().set(current_loop + 1);
                        true
                    } else {
                        self.execute_on_complete();
                        false
                    }
                }
                LoopMode::None => {
                    self.execute_on_complete();
                    false
                }
            }
        } else {
            self.execute_on_complete();
            false
        }
    }

    /// Execute the on_complete callback if present
    fn execute_on_complete(&self) {
        let config = self.config().cloned();
        #[allow(clippy::collapsible_if)]
        if let Some(on_complete) = &config.on_complete {
            if let Ok(mut callback) = on_complete.lock() {
                callback();
            }
        }
    }
}

/// Hook that creates a unified motion store for any animatable type
///
/// This is the primary hook for store-based animations. It supports:
/// - Simple spring/tween animations
/// - Keyframe animations
/// - Sequence animations
///
/// All animation types use the same unified loop for efficiency.
///
/// # Benefits
/// - **Fine-grained reactivity**: Components only re-render when subscribed data changes
/// - **Better performance**: Eliminates unnecessary re-renders in complex UIs
/// - **Unified API**: One hook for all animation types
/// - **Type safety**: Works with any type implementing `Animatable`
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_motion::prelude::*;
///
/// #[component]
/// fn AnimatedComponent() -> Element {
///     let mut motion = use_motion_store(0.0f32);
///     let current = motion.store().current();
///     
///     rsx! {
///         div {
///             style: "transform: translateX({current()}px)",
///             onclick: move |_| {
///                 motion.animate_to(100.0, AnimationConfig::spring());
///             },
///             "Animated element"
///         }
///     }
/// }
/// ```
pub fn use_motion_store<T: Animatable + Copy + Default + Send + 'static>(
    initial: T,
) -> MotionHandle<T> {
    let store = use_store(|| MotionStore::new(initial));

    // Refs for keyframe and sequence data (shared across the loop)
    let keyframes_ref = use_signal(|| None::<Arc<KeyframeAnimation<T>>>);
    let sequence_ref = use_signal(|| None::<Arc<AnimationSequence<T>>>);
    let animation_type = use_signal(|| AnimationType::Simple);

    // Unified animation loop with smoother frame rates
    #[cfg(feature = "web")]
    let idle_poll_rate = Duration::from_millis(32); // ~30 FPS when idle

    #[cfg(not(feature = "web"))]
    let idle_poll_rate = Duration::from_millis(16); // ~60 FPS when idle

    use_effect({
        move || {
            spawn(async move {
                let mut last_frame = crate::MotionTime::now();
                let mut running_frames = 0u32;

                loop {
                    let now = crate::MotionTime::now();
                    let dt = (now.duration_since(last_frame).as_secs_f32()).min(0.1);
                    last_frame = now;

                    if store.is_running() {
                        running_frames += 1;
                        let prev_value = store.get_value();

                        // Dispatch to appropriate update based on animation type
                        let updated = match *animation_type.read() {
                            AnimationType::Simple => store.clone().update_simple(dt),
                            AnimationType::Keyframes => keyframes_ref.read().as_ref().map_or_else(
                                || store.clone().update_simple(dt),
                                |keyframes| update_keyframes(&store, keyframes, dt),
                            ),
                            AnimationType::Sequence => sequence_ref.read().as_ref().map_or_else(
                                || store.clone().update_simple(dt),
                                |sequence| update_sequence(&store, sequence, dt),
                            ),
                        };

                        let new_value = store.get_value();
                        let epsilon = T::epsilon();

                        // Only continue if the value changed significantly
                        if (new_value - prev_value).magnitude() > epsilon || updated {
                            // Continue with normal frame timing
                        } else {
                            let delay = calculate_delay(dt, running_frames);
                            crate::MotionTime::delay(delay).await;
                            continue;
                        }

                        // Maintain minimum frame time
                        let delay =
                            calculate_delay(dt, running_frames).max(Duration::from_millis(8)); // Max ~120 FPS
                        crate::MotionTime::delay(delay).await;
                    } else {
                        running_frames = 0;
                        crate::MotionTime::delay(idle_poll_rate).await;
                    }
                }
            });
        }
    });

    MotionHandle {
        store,
        keyframes_ref,
        sequence_ref,
        animation_type,
    }
}

/// Update function for keyframe animations
fn update_keyframes<T: Animatable + Copy + Default>(
    store: &Store<MotionStore<T>>,
    animation: &KeyframeAnimation<T>,
    dt: f32,
) -> bool {
    let elapsed = store.elapsed()();
    let progress = (elapsed.as_secs_f32() / animation.duration.as_secs_f32()).clamp(0.0, 1.0);

    if animation.keyframes.is_empty() {
        return false;
    }

    // Find the current keyframe segment
    let (start, end) = animation
        .keyframes
        .windows(2)
        .find(|w| progress >= w[0].offset && progress <= w[1].offset)
        .map(|w| (&w[0], &w[1]))
        .unwrap_or_else(|| {
            if progress <= animation.keyframes[0].offset {
                let first = &animation.keyframes[0];
                (first, first)
            } else {
                let last = animation
                    .keyframes
                    .last()
                    .expect("Keyframes should not be empty");
                (last, last)
            }
        });

    // Calculate local progress within the keyframe segment
    let local_progress = if start.offset == end.offset {
        1.0
    } else {
        (progress - start.offset) / (end.offset - start.offset)
    };

    // Apply easing if present
    let eased_progress = end.easing.map_or(local_progress, |easing| {
        easing(local_progress, 0.0, 1.0, 1.0)
    });

    // Interpolate between keyframes
    let new_value = start.value.interpolate(&end.value, eased_progress);
    store.current().set(new_value);

    // Update elapsed time
    let new_elapsed = elapsed + Duration::from_secs_f32(dt);
    store.elapsed().set(new_elapsed);

    // Check if animation is complete
    if progress >= 1.0 {
        store.execute_on_complete();
        store.running().set(false);
        false
    } else {
        true
    }
}

/// Update function for sequence animations
fn update_sequence<T: Animatable + Copy + Default>(
    store: &Store<MotionStore<T>>,
    sequence: &AnimationSequence<T>,
    dt: f32,
) -> bool {
    let current_step_index = store.current_sequence_step()() as usize;

    sequence.steps().get(current_step_index).map_or_else(
        || {
            // No valid step, stop animation
            store.running().set(false);
            false
        },
        |current_step| {
            // Update current animation towards the step target
            let current = store.current()();
            let target = current_step.target;
            let diff = target - current;
            let epsilon = T::epsilon();

            if diff.magnitude() < epsilon {
                // Reached current step target, advance to next step
                let next_index = current_step_index + 1;
                if next_index < sequence.total_steps() {
                    sequence.steps().get(next_index).map_or_else(
                        || {
                            // Should not happen, but handle gracefully
                            store.running().set(false);
                            false
                        },
                        |next_step| {
                            store.current_sequence_step().set(next_index as u8);
                            store.target().set(next_step.target);
                            store.config().set((*next_step.config).clone());
                            store.elapsed().set(Duration::default());
                            true
                        },
                    )
                } else {
                    // Sequence complete
                    sequence.execute_completion();
                    store.running().set(false);
                    false
                }
            } else {
                // Continue animating towards current step target
                store.target().set(target);
                store.config().set((*current_step.config).clone());
                store.clone().update_simple(dt)
            }
        },
    )
}

// Legacy compatibility: keep old names for a migration period
pub use MotionHandle as MotionStoreHandle;

/// Legacy helper (deprecated - use MotionHandle.animate_to instead)
#[deprecated(since = "0.6.0", note = "Use MotionHandle.animate_to() instead")]
pub fn animate_to<T: Animatable + Copy + Default>(
    motion: &Store<MotionStore<T>>,
    target: T,
    config: AnimationConfig,
) {
    motion.config().set(config.clone());
    motion.target().set(target);
    motion.running().set(true);
    motion.elapsed().set(Duration::default());
    motion.delay_elapsed().set(config.delay);
    motion.current_loop().set(0);
    motion.current_keyframe().set(0);
    motion.current_sequence_step().set(0);
}
