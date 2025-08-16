//! Store-based Motion API for fine-grained reactivity
//!
//! This module provides a store-based alternative to the signal-based Motion API,
//! enabling fine-grained reactivity where components can subscribe to specific
//! fields of the animation state rather than the entire Motion struct.

use crate::Duration;
use crate::animations::core::{Animatable, AnimationConfig};
use crate::animations::platform::TimeProvider;
use dioxus::prelude::*;

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
    /// let motion = use_motion_store(0.0f32);
    /// let value = motion.get_value(); // Reactive to current value changes only
    /// ```
    fn get_value(&self) -> T {
        self.current().cloned()
    }

    /// Check if animation is currently running
    ///
    /// # Example
    /// ```rust
    /// let motion = use_motion_store(0.0f32);
    /// let is_running = motion.is_running(); // Reactive to running state changes
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
    /// let mut motion = use_motion_store(0.0f32);
    /// motion.animate_to(100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
    /// ```
    fn animate_to(&mut self, target: T, _config: AnimationConfig) {
        // Simplified implementation for store-based motion
        // Full configuration handling will be added in later phases
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
    /// Note: This is a simplified implementation for the store-based API.
    /// Full spring physics and configuration handling will be added in future versions.
    fn update(&mut self, dt: f32) -> bool {
        if !self.running().cloned() {
            return false;
        }

        // Update elapsed time
        let new_elapsed = self.elapsed().cloned() + Duration::from_secs_f32(dt);
        self.elapsed().set(new_elapsed);

        // Get current animation state
        let current = self.current().cloned();
        let target = self.target().cloned();
        let current_velocity = self.velocity().cloned();
        let diff = target - current;

        // Use type-specific epsilon for stopping condition
        let epsilon = T::epsilon();
        
        if diff.magnitude() < epsilon {
            // Close enough to target, snap to target and stop
            self.current().set(target);
            self.velocity().set(T::default());
            self.stop();
            false
        } else {
            // Smooth interpolation with damping to prevent oscillation
            let speed_factor = 4.0; // Reduced speed for smoother animation
            let damping = 0.85; // Increased damping to reduce oscillation
            
            // Calculate desired velocity towards target
            let desired_velocity = diff * speed_factor;
            
            // Apply damping to current velocity and blend with desired velocity
            let damped_velocity = current_velocity * damping;
            let blend_factor = (dt * 6.0).min(0.8); // Smoother velocity transitions
            let new_velocity = damped_velocity + (desired_velocity - damped_velocity) * blend_factor;
            
            // Update position based on velocity
            let step = new_velocity * dt;
            let new_current = current + step;

            self.current().set(new_current);
            self.velocity().set(new_velocity);
            true
        }
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
///                 motion.animate_to(100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
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
        let store = store.clone();
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
                        let delay = crate::calculate_delay(dt, running_frames).max(Duration::from_millis(8)); // Max ~120 FPS
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
