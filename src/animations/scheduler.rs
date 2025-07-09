use crate::animations::core::Animatable;
use crate::motion::Motion;
use dioxus::prelude::*;

/// Enhanced use_motion that leverages optimized scheduling
/// This is a simplified version that creates a Signal<Motion<T>> directly
pub fn use_motion_scheduled<T: Animatable + Send + Sync + 'static>(
    initial: T,
) -> Signal<Motion<T>> {
    use_signal(|| Motion::new(initial))
}
