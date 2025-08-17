//! Store-based Motion API for fine-grained reactivity
//!
//! This module provides a store-based alternative to the signal-based Motion API,
//! enabling fine-grained reactivity where components can subscribe to specific
//! fields of the animation state rather than the entire Motion struct.

use crate::Duration;
use crate::animations::core::{Animatable, AnimationConfig, AnimationMode, LoopMode};
use crate::animations::platform::TimeProvider;
use crate::keyframes::KeyframeAnimation;
use crate::sequence::AnimationSequence;
use dioxus::prelude::*;
use std::sync::Arc;

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

    /// Animation type: "simple", "keyframes", or "sequence"
    pub animation_type: String,
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
            animation_type: "simple".to_string(),
            current_keyframe: 0,
            current_sequence_step: 0,
            config: AnimationConfig::default(),
        }
    }
}

/// Custom store methods using the #[store] attribute
/// This provides computed properties and business logic for the MotionStore<T>
#[store]
impl<T: Animatable + Copy + Default> Store<MotionStore<T>> {
    /// Get the current animated value
    ///
    /// This is the most commonly used method. Components that only need
    /// the animated value should subscribe to this to avoid unnecessary
    /// re-renders when internal animation state changes.
    ///
    /// # Example
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_motion::prelude::*;
    ///
    /// #[component]
    /// fn Example() -> Element {
    ///     let motion = use_motion_store(0.0f32);
    ///     let value = motion.current(); // Reactive to current value changes only
    ///     rsx! { div { "Value: {value()}" } }
    /// }
    /// ```
    fn get_value(&self) -> T {
        self.current().cloned()
    }

    /// Check if animation is currently running
    ///
    /// # Example
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_motion::prelude::*;
    ///
    /// #[component]
    /// fn Example() -> Element {
    ///     let motion = use_motion_store(0.0f32);
    ///     let is_running = motion.running(); // Reactive to running state changes
    ///     rsx! { div { "Running: {is_running()}" } }
    /// }
    /// ```
    fn is_running(&self) -> bool {
        self.running().cloned()
    }

    /// Check if animation has reached its target value (within epsilon)
    ///
    /// This is a computed property that automatically updates when
    /// the current value or target value changes.
    fn is_at_target(&self) -> bool {
        let current = self.current().cloned();
        let target = self.target().cloned();
        let diff = current - target;
        diff.magnitude() < T::epsilon() // Use type-specific epsilon
    }

    /// Get the magnitude of the current velocity
    ///
    /// Useful for determining how fast the animation is moving.
    fn get_velocity_magnitude(&self) -> f32 {
        self.velocity().cloned().magnitude()
    }

    /// Animate to a target value with the given configuration
    ///
    /// # Example
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_motion::prelude::*;
    ///
    /// #[component]
    /// fn Example() -> Element {
    ///     let motion = use_motion_store(0.0f32);
    ///     let current = motion.current();
    ///     
    ///     rsx! {
    ///         div {
    ///             style: "transform: translateX({current()}px)",
    ///             onclick: move |_| {
    ///                 animate_to(&motion, 100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
    ///             },
    ///             "Click to animate"
    ///         }
    ///     }
    /// }
    /// ```
    fn animate_to(&mut self, target: T, config: AnimationConfig) {
        // Store the animation configuration for use in update loop
        self.config().set(config);
        self.target().set(target);
        self.running().set(true);
        self.elapsed().set(Duration::default());
        self.delay_elapsed().set(Duration::default());
        self.current_loop().set(0);
    }

    /// Stop the animation
    fn stop(&mut self) {
        self.running().set(false);
        self.velocity().set(T::default());
        self.current_loop().set(0);
    }

    /// Reset animation to initial state
    fn reset(&mut self) {
        let initial = self.initial().cloned();
        self.current().set(initial);
        self.target().set(initial);
        self.animation_type().set("simple".to_string());
        self.current_keyframe().set(0);
        self.current_sequence_step().set(0);
        self.config().set(AnimationConfig::default());
        self.stop();
    }

    /// Add a delay before the animation starts
    fn delay(&mut self, duration: Duration) {
        self.delay_elapsed().set(duration);
    }

    /// Update the animation by the given delta time
    ///
    /// Returns true if the animation is still running, false if it completed
    ///
    /// This implementation uses proper spring physics or tween interpolation based on the stored AnimationConfig.
    fn update(&mut self, dt: f32) -> bool {
        if !self.running().cloned() {
            return false;
        }

        // Check if we're still in delay phase
        let delay_elapsed = self.delay_elapsed().cloned();
        if delay_elapsed > Duration::default() {
            // Still delaying, reduce delay time
            let dt_duration = Duration::from_secs_f32(dt);
            if dt_duration >= delay_elapsed {
                // Delay complete, start animation
                self.delay_elapsed().set(Duration::default());
            } else {
                // Still delaying, reduce delay time safely
                let new_delay = delay_elapsed - dt_duration;
                self.delay_elapsed().set(new_delay);
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

        // Use type-specific epsilon for stopping condition
        let epsilon = T::epsilon();

        if diff.magnitude() < epsilon {
            // Close enough to target, snap to target
            self.current().set(target);
            self.velocity().set(T::default());

            // Handle loop modes
            if let Some(loop_mode) = config.loop_mode {
                match loop_mode {
                    LoopMode::Infinite => {
                        // Restart animation from initial to target
                        self.current().set(self.initial().cloned());
                        self.elapsed().set(Duration::default());
                        self.current_loop().set(self.current_loop().cloned() + 1);
                        true
                    }
                    LoopMode::Times(count) => {
                        let current_loop = self.current_loop().cloned();
                        if current_loop < count - 1 {
                            // Restart animation for next loop
                            self.current().set(self.initial().cloned());
                            self.elapsed().set(Duration::default());
                            self.current_loop().set(current_loop + 1);
                            true
                        } else {
                            // All loops complete, stop animation and execute completion callback
                            if let Some(on_complete) = &config.on_complete
                                && let Ok(mut callback) = on_complete.lock()
                            {
                                callback();
                            }
                            self.stop();
                            false
                        }
                    }
                    LoopMode::Alternate => {
                        // Toggle between initial and target values
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

                        // Update target and initial for next iteration
                        self.target().set(new_target);
                        self.initial().set(new_initial);
                        // Don't jump current position - let it animate smoothly to new target
                        self.reverse().set(!self.reverse().cloned());
                        self.elapsed().set(Duration::default());
                        self.current_loop().set(self.current_loop().cloned() + 1);
                        true
                    }
                    LoopMode::AlternateTimes(count) => {
                        let current_loop = self.current_loop().cloned();
                        if current_loop < (count * 2) - 1 {
                            // Toggle between initial and target values
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

                            // Update target and initial for next iteration
                            self.target().set(new_target);
                            self.initial().set(new_initial);
                            // Don't jump current position - let it animate smoothly to new target
                            self.reverse().set(!self.reverse().cloned());
                            self.elapsed().set(Duration::default());
                            self.current_loop().set(current_loop + 1);
                            true
                        } else {
                            // All loops complete, stop animation and execute completion callback
                            if let Some(on_complete) = &config.on_complete
                                && let Ok(mut callback) = on_complete.lock()
                            {
                                callback();
                            }
                            self.stop();
                            false
                        }
                    }
                    LoopMode::None => {
                        // No loop, stop animation and execute completion callback
                        if let Some(on_complete) = &config.on_complete
                            && let Ok(mut callback) = on_complete.lock()
                        {
                            callback();
                        }
                        self.stop();
                        false
                    }
                }
            } else {
                // No loop mode specified, stop animation and execute completion callback
                if let Some(on_complete) = &config.on_complete
                    && let Ok(mut callback) = on_complete.lock()
                {
                    callback();
                }
                self.stop();
                false
            }
        } else {
            match config.mode {
                AnimationMode::Spring(spring) => {
                    // Spring physics: F = -kx - cv
                    let stiffness = spring.stiffness;
                    let damping_ratio = spring.damping;
                    let mass = spring.mass;

                    // Calculate spring force and damping force
                    let spring_force = diff * stiffness;
                    let damping_force = current_velocity * damping_ratio;
                    let net_force = spring_force - damping_force;

                    // Calculate acceleration and new velocity
                    let acceleration = net_force * (1.0 / mass);
                    let new_velocity = current_velocity + acceleration * dt;

                    // Update position
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

                    // Simple tween: just move towards target based on time-based progress
                    let step = diff * eased_progress;
                    let new_current = current + step;

                    // Calculate velocity for smooth transitions
                    let new_velocity = step * (1.0 / dt);

                    self.current().set(new_current);
                    self.velocity().set(new_velocity);

                    // Check if tween is complete
                    if progress >= 1.0 {
                        self.current().set(target);
                        self.velocity().set(T::default());

                        // Handle loop modes for tween animations
                        if let Some(loop_mode) = config.loop_mode {
                            match loop_mode {
                                LoopMode::Infinite => {
                                    // Restart animation from initial to target
                                    self.current().set(self.initial().cloned());
                                    self.elapsed().set(Duration::default());
                                    self.current_loop().set(self.current_loop().cloned() + 1);
                                    true
                                }
                                LoopMode::Times(count) => {
                                    let current_loop = self.current_loop().cloned();
                                    if current_loop < count - 1 {
                                        // Restart animation for next loop
                                        self.current().set(self.initial().cloned());
                                        self.elapsed().set(Duration::default());
                                        self.current_loop().set(current_loop + 1);
                                        true
                                    } else {
                                        // All loops complete, stop animation and execute completion callback
                                        if let Some(on_complete) = &config.on_complete
                                            && let Ok(mut callback) = on_complete.lock()
                                        {
                                            callback();
                                        }
                                        self.stop();
                                        false
                                    }
                                }
                                LoopMode::Alternate => {
                                    // Toggle between initial and target values
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

                                    // Update target and initial for next iteration
                                    self.target().set(new_target);
                                    self.initial().set(new_initial);
                                    // Don't jump current position - let it animate smoothly to new target
                                    self.reverse().set(!self.reverse().cloned());
                                    self.elapsed().set(Duration::default());
                                    self.current_loop().set(self.current_loop().cloned() + 1);
                                    true
                                }
                                LoopMode::AlternateTimes(count) => {
                                    let current_loop = self.current_loop().cloned();
                                    if current_loop < (count * 2) - 1 {
                                        // Toggle between initial and target values
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

                                        // Update target and initial for next iteration
                                        self.target().set(new_target);
                                        self.initial().set(new_initial);
                                        // Don't jump current position - let it animate smoothly to new target
                                        self.reverse().set(!self.reverse().cloned());
                                        self.elapsed().set(Duration::default());
                                        self.current_loop().set(current_loop + 1);
                                        true
                                    } else {
                                        // All loops complete, stop animation and execute completion callback
                                        #[allow(clippy::collapsible_if)]
                                        if let Some(on_complete) = &config.on_complete {
                                            if let Ok(mut callback) = on_complete.lock() {
                                                callback();
                                            }
                                        }
                                        self.stop();
                                        false
                                    }
                                }
                                LoopMode::None => {
                                    // No loop, stop animation and execute completion callback
                                    #[allow(clippy::collapsible_if)]
                                    if let Some(on_complete) = &config.on_complete {
                                        if let Ok(mut callback) = on_complete.lock() {
                                            callback();
                                        }
                                    }
                                    self.stop();
                                    false
                                }
                            }
                        } else {
                            // No loop mode specified, stop animation and execute completion callback
                            #[allow(clippy::collapsible_if)]
                            if let Some(on_complete) = &config.on_complete {
                                if let Ok(mut callback) = on_complete.lock() {
                                    callback();
                                }
                            }
                            self.stop();
                            false
                        }
                    } else {
                        true
                    }
                }
            }
        }
    }

    /// Start a keyframe animation
    ///
    /// # Example
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_motion::prelude::*;
    ///
    /// #[component]
    /// fn Example() -> Element {
    ///     let motion = use_motion_store(0.0f32);
    ///     let current = motion.current();
    ///     
    ///     rsx! {
    ///         div {
    ///             style: "transform: translateX({current()}px)",
    ///             onclick: move |_| {
    ///                 // For keyframe animations, use the dedicated hook
    ///                 // let keyframes_motion = use_motion_store_keyframes(0.0f32);
    ///                 // This method is available on the store trait, not the Store type
    ///             },
    ///             "Click for keyframes"
    ///         }
    ///     }
    /// }
    /// ```
    fn animate_keyframes(&mut self, _animation: KeyframeAnimation<T>) {
        self.animation_type().set("keyframes".to_string());
        self.running().set(true);
        self.elapsed().set(Duration::default());
        self.current_keyframe().set(0);
        // Store keyframes data would go here (see use_motion_store for full implementation)
    }

    /// Start a sequence animation
    ///
    /// # Example
    /// ```rust
    /// use dioxus::prelude::*;
    /// use dioxus_motion::prelude::*;
    ///
    /// #[component]
    /// fn Example() -> Element {
    ///     let motion = use_motion_store(0.0f32);
    ///     let current = motion.current();
    ///     
    ///     rsx! {
    ///         div {
    ///             style: "transform: translateX({current()}px)",
    ///             onclick: move |_| {
    ///                 // For sequence animations, use the dedicated hook
    ///                 // let sequence_motion = use_motion_store_sequence(0.0f32);
    ///                 // This method is available on the store trait, not the Store type
    ///             },
    ///             "Click for sequence"
    ///         }
    ///     }
    /// }
    /// ```
    fn animate_sequence(&mut self, _sequence: AnimationSequence<T>) {
        self.animation_type().set("sequence".to_string());
        self.running().set(true);
        self.elapsed().set(Duration::default());
        self.current_sequence_step().set(0);
        // Store sequence data would go here (see use_motion_store for full implementation)
    }
}

/// Hook that creates a motion store with fine-grained reactivity for any animatable type
///
/// This is the store-based alternative to `use_motion()` that provides
/// better performance for complex UIs where multiple components need
/// different aspects of the animation state.
///
/// # Benefits
/// - **Fine-grained reactivity**: Components only re-render when their subscribed data changes
/// - **Better performance**: Eliminates unnecessary re-renders in complex UIs
/// - **Direct manipulation**: Set values directly without going through animation API
/// - **Type safety**: Works with any type implementing `Animatable`
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_motion::prelude::*;
///
/// #[component]
/// fn AnimatedComponent() -> Element {
///     let motion = use_motion_store(0.0f32);
///     let current = motion.current(); // Fine-grained subscription to position only
///     
///     rsx! {
///         div {
///             style: "transform: translateX({current()}px)",
///             onclick: move |_| {
///                 animate_to(&motion, 100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
///             },
///             "Animated element"
///         }
///     }
/// }
/// ```
pub fn use_motion_store<T: Animatable + Copy + Default + Send + 'static>(
    initial: T,
) -> Store<MotionStore<T>> {
    let store = use_store(|| MotionStore::new(initial));

    // Set up the animation loop with smoother frame rates
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
                        let updated = store.clone().update(dt);
                        let new_value = store.get_value();
                        let epsilon = T::epsilon();

                        // Only continue if the value changed significantly
                        if (new_value - prev_value).magnitude() > epsilon || updated {
                            // Continue with normal frame timing
                        } else {
                            // Skip this frame to avoid unnecessary updates
                            let delay = crate::calculate_delay(dt, running_frames);
                            crate::MotionTime::delay(delay).await;
                            continue;
                        }

                        // Maintain minimum frame time to prevent excessive updates
                        let delay = crate::calculate_delay(dt, running_frames)
                            .max(Duration::from_millis(8)); // Max ~120 FPS
                        crate::MotionTime::delay(delay).await;
                    } else {
                        running_frames = 0;
                        crate::MotionTime::delay(idle_poll_rate).await;
                    }
                }
            });
        }
    });

    store
}

/// Hook that creates a motion store specifically for keyframe animations
///
/// This provides the same fine-grained reactivity as `use_motion_store` but with
/// support for complex keyframe animations with easing and multiple waypoints.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_motion::prelude::*;
///
/// #[component]
/// fn KeyframeComponent() -> Element {
///     let motion = use_motion_store_keyframes(0.0f32);
///     let current = motion.current();
///     
///     let start_animation = move |_| {
///         // The keyframes hook handles keyframe animations internally
///         // You can set values directly or use the store's methods
///         motion.target().set(100.0);
///         motion.running().set(true);
///     };
///     
///     rsx! {
///         div {
///             style: "transform: translateX({current()}px)",
///             onclick: start_animation,
///             "Animated element"
///         }
///     }
/// }
/// ```
pub fn use_motion_store_keyframes<T: Animatable + Copy + Default + Send + 'static>(
    initial: T,
) -> Store<MotionStore<T>> {
    let store = use_store(|| MotionStore::new(initial));
    let keyframes_ref = use_signal(|| None::<Arc<KeyframeAnimation<T>>>);

    // Set up the animation loop with keyframe support
    #[cfg(feature = "web")]
    let idle_poll_rate = Duration::from_millis(32);

    #[cfg(not(feature = "web"))]
    let idle_poll_rate = Duration::from_millis(16);

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
                        let animation_type = store.animation_type()();
                        let prev_value = store.get_value();

                        let updated = if animation_type == "keyframes" {
                            keyframes_ref.read().as_ref().map_or_else(
                                || store.clone().update(dt),
                                |keyframes| update_keyframes(&store, keyframes, dt),
                            )
                        } else {
                            store.clone().update(dt)
                        };

                        let new_value = store.get_value();
                        let epsilon = T::epsilon();

                        if (new_value - prev_value).magnitude() > epsilon || updated {
                            // Continue with normal frame timing
                        } else {
                            let delay = crate::calculate_delay(dt, running_frames);
                            crate::MotionTime::delay(delay).await;
                            continue;
                        }

                        let delay = crate::calculate_delay(dt, running_frames)
                            .max(Duration::from_millis(8));
                        crate::MotionTime::delay(delay).await;
                    } else {
                        running_frames = 0;
                        crate::MotionTime::delay(idle_poll_rate).await;
                    }
                }
            });
        }
    });

    store
}

/// Hook that creates a motion store specifically for sequence animations
///
/// This provides the same fine-grained reactivity as `use_motion_store` but with
/// support for chaining multiple animation steps together.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_motion::prelude::*;
///
/// #[component]
/// fn SequenceComponent() -> Element {
///     let motion = use_motion_store_sequence(0.0f32);
///     let current = motion.current();
///     
///     let start_sequence = move |_| {
///         // The sequence hook handles sequence animations internally
///         // You can set values directly or use the store's methods
///         motion.target().set(100.0);
///         motion.running().set(true);
///     };
///     
///     rsx! {
///         div {
///             style: "transform: translateX({current()}px)",
///             onclick: start_sequence,
///             "Animated element"
///         }
///     }
/// }
/// ```
pub fn use_motion_store_sequence<T: Animatable + Copy + Default + Send + 'static>(
    initial: T,
) -> Store<MotionStore<T>> {
    let store = use_store(|| MotionStore::new(initial));
    let sequence_ref = use_signal(|| None::<Arc<AnimationSequence<T>>>);

    // Set up the animation loop with sequence support
    #[cfg(feature = "web")]
    let idle_poll_rate = Duration::from_millis(32);

    #[cfg(not(feature = "web"))]
    let idle_poll_rate = Duration::from_millis(16);

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
                        let animation_type = store.animation_type()();
                        let prev_value = store.get_value();

                        let updated = if animation_type == "sequence" {
                            sequence_ref.read().as_ref().map_or_else(
                                || store.clone().update(dt),
                                |sequence| update_sequence(&store, sequence, dt),
                            )
                        } else {
                            store.clone().update(dt)
                        };

                        let new_value = store.get_value();
                        let epsilon = T::epsilon();

                        if (new_value - prev_value).magnitude() > epsilon || updated {
                            // Continue with normal frame timing
                        } else {
                            let delay = crate::calculate_delay(dt, running_frames);
                            crate::MotionTime::delay(delay).await;
                            continue;
                        }

                        let delay = crate::calculate_delay(dt, running_frames)
                            .max(Duration::from_millis(8));
                        crate::MotionTime::delay(delay).await;
                    } else {
                        running_frames = 0;
                        crate::MotionTime::delay(idle_poll_rate).await;
                    }
                }
            });
        }
    });

    store
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
                if current_step_index + 1 < sequence.total_steps() {
                    store
                        .current_sequence_step()
                        .set((current_step_index + 1) as u8);
                    store
                        .target()
                        .set(sequence.steps()[current_step_index + 1].target);
                    true
                } else {
                    // Sequence complete
                    store.running().set(false);
                    false
                }
            } else {
                // Continue animating towards current step target
                store.target().set(target);
                store.clone().update(dt)
            }
        },
    )
}

/// Helper function to easily animate a motion store to a target value with configuration
///
/// This is a convenience function that handles setting the target, config, and starting the animation.
/// It's easier than manually calling `motion.target().set()`, `motion.config().set()`, and `motion.running().set()`.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_motion::prelude::*;
///
/// #[component]
/// fn AnimatedComponent() -> Element {
///     let motion = use_motion_store(0.0f32);
///     let current = motion.current();
///
///     let start_animation = move |_| {
///         // Easy way to animate with spring physics
///         animate_to(&motion, 100.0, AnimationConfig::new(
///             AnimationMode::Spring(Spring::default())
///         ));
///     };
///
///     rsx! {
///         div {
///             style: "transform: translateX({current()}px)",
///             onclick: start_animation,
///             "Click to animate"
///         }
///     }
/// }
/// ```
pub fn animate_to<T: Animatable + Copy + Default>(
    motion: &Store<MotionStore<T>>,
    target: T,
    config: AnimationConfig,
) {
    motion.config().set(config.clone());
    motion.target().set(target);
    motion.running().set(true);
    motion.elapsed().set(Duration::default());
    motion.delay_elapsed().set(config.delay); // Set the delay from config
    motion.current_loop().set(0);
}
