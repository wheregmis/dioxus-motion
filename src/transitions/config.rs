//! Transition configuration and variant definitions for Dioxus Motion
//!
//! This module defines the configuration structures and enums for page transitions.

use crate::prelude::Transform;

#[derive(Clone)]
pub struct TransitionConfig {
    // For the page that's leaving (FROM)
    pub exit_start: Transform, // Starting position of exiting page
    pub exit_end: Transform,   // Final position of exiting page

    // For the page that's entering (TO)
    pub enter_start: Transform, // Starting position of entering page
    pub enter_end: Transform,   // Final position of entering page
}

#[derive(PartialEq, Clone)]
pub enum TransitionVariant {
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
    Fade,
    // Scale transitions
    ScaleUp,
    ScaleDown,
    // Flip transitions
    FlipHorizontal,
    FlipVertical,
    // Rotate transitions
    RotateLeft,
    RotateRight,
    // Combinations
    SlideUpFade,
    SlideDownFade,
    ScaleUpFade,
    // Bounce effects
    BounceIn,
    BounceOut,

    // Additional combined transitions
    ScaleDownFade,
    RotateLeftFade,
    RotateRightFade,
    FlipHorizontalFade,
    FlipVerticalFade,

    // Zoom transitions
    ZoomIn,
    ZoomOut,

    // Diagonal slides
    SlideDiagonalUpLeft,
    SlideDiagonalUpRight,
    SlideDiagonalDownLeft,
    SlideDiagonalDownRight,

    // Spiral transitions
    SpiralIn,
    SpiralOut,

    // Elastic transitions
    ElasticIn,
    ElasticOut,

    // Swing transitions
    SwingIn,
    SwingOut,

    SlideLeftFade,
    SlideRightFade,

    ScaleRotateFade,
    SlideFadeRotate,
    ScaleFadeFlip,
    RotateScaleSlide,
}

// Helper functions to reduce repetition
fn slide_horizontal(exit_x: f32, enter_x: f32) -> TransitionConfig {
    let identity = Transform::identity();
    TransitionConfig {
        exit_start: identity,
        exit_end: Transform::new(exit_x, 0.0, 1.0, 0.0),
        enter_start: Transform::new(enter_x, 0.0, 1.0, 0.0),
        enter_end: identity,
    }
}

fn slide_vertical(exit_y: f32, enter_y: f32) -> TransitionConfig {
    let identity = Transform::identity();
    TransitionConfig {
        exit_start: identity,
        exit_end: Transform::new(0.0, exit_y, 1.0, 0.0),
        enter_start: Transform::new(0.0, enter_y, 1.0, 0.0),
        enter_end: identity,
    }
}

fn scale_transition(exit_scale: f32, enter_scale: f32) -> TransitionConfig {
    let identity = Transform::identity();
    TransitionConfig {
        exit_start: identity,
        exit_end: Transform::new(0.0, 0.0, exit_scale, 0.0),
        enter_start: Transform::new(0.0, 0.0, enter_scale, 0.0),
        enter_end: identity,
    }
}

fn rotate_transition(exit_rotation: f32, enter_rotation: f32) -> TransitionConfig {
    let identity = Transform::identity();
    TransitionConfig {
        exit_start: identity,
        exit_end: Transform::new(0.0, 0.0, 1.0, exit_rotation),
        enter_start: Transform::new(0.0, 0.0, 1.0, enter_rotation),
        enter_end: identity,
    }
}

fn slide_diagonal(exit_x: f32, exit_y: f32, enter_x: f32, enter_y: f32) -> TransitionConfig {
    let identity = Transform::identity();
    TransitionConfig {
        exit_start: identity,
        exit_end: Transform::new(exit_x, exit_y, 1.0, 0.0),
        enter_start: Transform::new(enter_x, enter_y, 1.0, 0.0),
        enter_end: identity,
    }
}

impl TransitionVariant {
    pub fn get_config(&self) -> TransitionConfig {
        let identity = Transform::identity();

        match self {
            // Basic slides
            TransitionVariant::SlideLeft => slide_horizontal(-100.0, 100.0),
            TransitionVariant::SlideRight => slide_horizontal(100.0, -100.0),
            TransitionVariant::SlideUp => slide_vertical(-100.0, 100.0),
            TransitionVariant::SlideDown => slide_vertical(100.0, -100.0),

            // Fade transitions
            TransitionVariant::Fade => TransitionConfig {
                exit_start: identity,
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0),
                enter_end: identity,
            },

            // Scale transitions
            TransitionVariant::ScaleUp => scale_transition(0.0, 0.0),
            TransitionVariant::ScaleDown => scale_transition(2.0, 2.0),
            TransitionVariant::ZoomIn => scale_transition(1.0, 0.0),
            TransitionVariant::ZoomOut => scale_transition(2.0, 0.0),

            // Flip transitions
            TransitionVariant::FlipHorizontal => rotate_transition(180.0, -180.0),
            TransitionVariant::FlipVertical => rotate_transition(180.0, -180.0),

            // Rotation transitions
            TransitionVariant::RotateLeft => rotate_transition(90.0, -90.0),
            TransitionVariant::RotateRight => rotate_transition(-90.0, 90.0),

            // Combined slide transitions (reuse basic slides)
            TransitionVariant::SlideUpFade => slide_vertical(-100.0, 100.0),
            TransitionVariant::SlideDownFade => slide_vertical(100.0, -100.0),
            TransitionVariant::SlideLeftFade => slide_horizontal(-100.0, 100.0),
            TransitionVariant::SlideRightFade => slide_horizontal(100.0, -100.0),

            // Combined scale transitions
            TransitionVariant::ScaleUpFade => scale_transition(0.0, 0.0),
            TransitionVariant::ScaleDownFade => scale_transition(2.0, 2.0),

            // Combined rotation transitions
            TransitionVariant::RotateLeftFade => rotate_transition(90.0, -90.0),
            TransitionVariant::RotateRightFade => rotate_transition(-90.0, 90.0),

            // Combined flip transitions
            TransitionVariant::FlipHorizontalFade => rotate_transition(180.0, -180.0),
            TransitionVariant::FlipVerticalFade => rotate_transition(180.0, -180.0),

            // Diagonal slides
            TransitionVariant::SlideDiagonalUpLeft => slide_diagonal(-100.0, -100.0, 100.0, 100.0),
            TransitionVariant::SlideDiagonalUpRight => slide_diagonal(100.0, -100.0, -100.0, 100.0),
            TransitionVariant::SlideDiagonalDownLeft => {
                slide_diagonal(-100.0, 100.0, 100.0, -100.0)
            }
            TransitionVariant::SlideDiagonalDownRight => {
                slide_diagonal(100.0, 100.0, -100.0, -100.0)
            }

            // Bounce/Elastic/Swing transitions (reuse vertical slides)
            TransitionVariant::BounceIn => slide_vertical(0.0, 100.0),
            TransitionVariant::BounceOut => slide_vertical(100.0, 0.0),
            TransitionVariant::ElasticIn => slide_vertical(0.0, 100.0),
            TransitionVariant::ElasticOut => slide_vertical(100.0, 0.0),
            TransitionVariant::SwingIn => slide_vertical(0.0, 100.0),
            TransitionVariant::SwingOut => slide_vertical(100.0, 0.0),

            // Spiral transitions (reuse scale transitions)
            TransitionVariant::SpiralIn => scale_transition(1.0, 0.0),
            TransitionVariant::SpiralOut => scale_transition(2.0, 0.0),

            // Complex combined transitions (simplified for now)
            TransitionVariant::ScaleRotateFade => scale_transition(1.0, 0.0),
            TransitionVariant::SlideFadeRotate => slide_vertical(0.0, 0.0),
            TransitionVariant::ScaleFadeFlip => scale_transition(1.0, 0.0),
            TransitionVariant::RotateScaleSlide => rotate_transition(0.0, 0.0),
        }
    }
}
