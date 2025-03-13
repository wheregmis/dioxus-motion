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
    // // Scale transitions
    // ScaleUp,
    // ScaleDown,
    // // Flip transitions
    // FlipHorizontal,
    // FlipVertical,
    // // Rotate transitions
    // RotateLeft,
    // RotateRight,
    // // Combinations
    // SlideUpFade,
    // SlideDownFade,
    // ScaleUpFade,
    // // Bounce effects
    // BounceIn,
    // BounceOut,

    // // Additional combined transitions
    // ScaleDownFade,
    // RotateLeftFade,
    // RotateRightFade,
    // FlipHorizontalFade,
    // FlipVerticalFade,

    // // Zoom transitions
    // ZoomIn,
    // ZoomOut,

    // // Diagonal slides
    // SlideDiagonalUpLeft,
    // SlideDiagonalUpRight,
    // SlideDiagonalDownLeft,
    // SlideDiagonalDownRight,

    // // Spiral transitions
    // SpiralIn,
    // SpiralOut,

    // // Elastic transitions
    // ElasticIn,
    // ElasticOut,

    // // Swing transitions
    // SwingIn,
    // SwingOut,

    // SlideLeftFade,
    // SlideRightFade,

    // ScaleRotateFade,
    // SlideFadeRotate,
    // ScaleFadeFlip,
    // RotateScaleSlide,
}

impl TransitionVariant {
    pub fn get_config(&self) -> TransitionConfig {
        let identity = Transform::identity();

        match self {
            TransitionVariant::SlideLeft => {
                TransitionConfig {
                    exit_start: identity,                              // Start in place
                    exit_end: Transform::new(-100.0, 0.0, 1.0, 0.0),   // Exit left
                    enter_start: Transform::new(100.0, 0.0, 1.0, 0.0), // Enter from right
                    enter_end: identity,                               // End in place
                }
            }

            TransitionVariant::SlideRight => {
                TransitionConfig {
                    exit_start: identity,                               // Start in place
                    exit_end: Transform::new(100.0, 0.0, 1.0, 0.0),     // Exit right
                    enter_start: Transform::new(-100.0, 0.0, 1.0, 0.0), // Enter from left
                    enter_end: identity,                                // End in place
                }
            }

            TransitionVariant::SlideUp => {
                TransitionConfig {
                    exit_start: identity,                              // Start in place
                    exit_end: Transform::new(0.0, -100.0, 1.0, 0.0),   // Exit up
                    enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Enter from bottom
                    enter_end: identity,                               // End in place
                }
            }

            TransitionVariant::SlideDown => {
                TransitionConfig {
                    exit_start: identity,                               // Start in place
                    exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),     // Exit down
                    enter_start: Transform::new(0.0, -100.0, 1.0, 0.0), // Enter from top
                    enter_end: identity,                                // End in place
                }
            }

            TransitionVariant::Fade => TransitionConfig {
                exit_start: identity,                            // Start fully visible
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // Fade out completely
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0), // Start invisible
                enter_end: identity,                             // Fade in completely
            },
        }
    }
}
