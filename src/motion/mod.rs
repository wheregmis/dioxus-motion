//!
//! Dioxus Motion module
//!
//! Contains Dioxus-specific motion components, props, state, and glue code.
//! Re-exports motion-enabled HTML elements and ergonomic aliases.

mod base;
mod components;
mod state;

// Re-export animation types
// pub use core::{AnimationTarget, TransitionConfig, TransitionType, Variants};

// Re-export motion components
pub use components::{
    A, Article, Aside, Button, Div, Footer, H1, H2, H3, H4, H5, H6, Header, Img, Li, Main, Nav, Ol,
    P, Section, Span, Ul,
};

// Grid layout components have been removed

// Lowercase aliases for compatibility with existing code
pub mod elements {
    pub use super::components::{
        A as a, Article as article, Aside as aside, Button as button, Div as div, Footer as footer,
        H1 as h1, H2 as h2, H3 as h3, H4 as h4, H5 as h5, H6 as h6, Header as header, Img as img,
        Li as li, Main as main, Nav as nav, Ol as ol, P as p, Section as section, Span as span,
        Ul as ul,
    };
}
