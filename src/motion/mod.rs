// Re-export all motion components and types
mod animation;
mod state;
mod components;
mod base;

// Re-export animation types
pub use animation::{AnimationTarget, TransitionConfig, TransitionType, Variants};

// Re-export motion components
pub use components::{Button, Div, Span};

// Lowercase aliases for compatibility with existing code
pub mod elements {
    pub use super::components::{Button as button, Div as div, Span as span};
}
