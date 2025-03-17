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
            TransitionVariant::ScaleUp => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 0.0, 0.0),    // Shrink to nothing
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::ScaleDown => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 2.0, 0.0),    // Grow to twice size
                enter_start: Transform::new(0.0, 0.0, 2.0, 0.0), // Start twice size
                enter_end: identity,                             // Shrink to full size
            },
            TransitionVariant::FlipHorizontal => TransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 180.0),     // Flip 180 degrees horizontally
                enter_start: Transform::new(0.0, 0.0, 1.0, -180.0), // Start flipped 180 degrees horizontally
                enter_end: identity,                                // End in place
            },
            TransitionVariant::FlipVertical => TransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 180.0),     // Flip 180 degrees vertically
                enter_start: Transform::new(0.0, 0.0, 1.0, -180.0), // Start flipped 180 degrees vertically
                enter_end: identity,                                // End in place
            },
            TransitionVariant::RotateLeft => TransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 90.0),     // Rotate 90 degrees to the left
                enter_start: Transform::new(0.0, 0.0, 1.0, -90.0), // Start rotated 90 degrees to the right
                enter_end: identity,                               // End in place
            },
            TransitionVariant::RotateRight => TransitionConfig {
                exit_start: identity,                             // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, -90.0),   // Rotate 90 degrees to the right
                enter_start: Transform::new(0.0, 0.0, 1.0, 90.0), // Start rotated 90 degrees to the left
                enter_end: identity,                              // End in place
            },
            TransitionVariant::SlideUpFade => TransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, -100.0, 1.0, 0.0),   // Exit up
                enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Enter from bottom
                enter_end: identity,                               // End in place
            },
            TransitionVariant::SlideDownFade => TransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),     // Exit down
                enter_start: Transform::new(0.0, -100.0, 1.0, 0.0), // Enter from top
                enter_end: identity,                                // End in place
            },
            TransitionVariant::ScaleUpFade => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 0.0, 0.0),    // Shrink to nothing
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::BounceIn => TransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),      // No change
                enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Start from bottom
                enter_end: identity,                               // End in place
            },
            TransitionVariant::BounceOut => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),  // Exit to bottom
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0), // Start in place
                enter_end: identity,                             // No change
            },
            TransitionVariant::ScaleDownFade => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 2.0, 0.0),    // Grow to twice size
                enter_start: Transform::new(0.0, 0.0, 2.0, 0.0), // Start twice size
                enter_end: identity,                             // Shrink to full size
            },
            TransitionVariant::RotateLeftFade => TransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 90.0),     // Rotate 90 degrees to the left
                enter_start: Transform::new(0.0, 0.0, 1.0, -90.0), // Start rotated 90 degrees to the right
                enter_end: identity,                               // End in place
            },
            TransitionVariant::RotateRightFade => TransitionConfig {
                exit_start: identity,                             // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, -90.0),   // Rotate 90 degrees to the right
                enter_start: Transform::new(0.0, 0.0, 1.0, 90.0), // Start rotated 90 degrees to the left
                enter_end: identity,                              // End in place
            },
            TransitionVariant::FlipHorizontalFade => TransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 180.0),     // Flip 180 degrees horizontally
                enter_start: Transform::new(0.0, 0.0, 1.0, -180.0), // Start flipped 180 degrees horizontally
                enter_end: identity,                                // End in place
            },
            TransitionVariant::FlipVerticalFade => TransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 180.0),     // Flip 180 degrees vertically
                enter_start: Transform::new(0.0, 0.0, 1.0, -180.0), // Start flipped 180 degrees vertically
                enter_end: identity,                                // End in place
            },
            TransitionVariant::ZoomIn => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::ZoomOut => TransitionConfig {
                exit_start: identity,                          // Start in place
                exit_end: Transform::new(0.0, 0.0, 2.0, 0.0),  // Grow to twice size
                enter_start: identity,                         // Start in place
                enter_end: Transform::new(0.0, 0.0, 0.0, 0.0), // Shrink to full size
            },
            TransitionVariant::SlideDiagonalUpLeft => TransitionConfig {
                exit_start: identity,                                // Start in place
                exit_end: Transform::new(-100.0, -100.0, 1.0, 0.0),  // Exit up and left
                enter_start: Transform::new(100.0, 100.0, 1.0, 0.0), // Enter from bottom right
                enter_end: identity,                                 // End in place
            },
            TransitionVariant::SlideDiagonalUpRight => TransitionConfig {
                exit_start: identity,                                 // Start in place
                exit_end: Transform::new(100.0, -100.0, 1.0, 0.0),    // Exit up and right
                enter_start: Transform::new(-100.0, 100.0, 1.0, 0.0), // Enter from bottom left
                enter_end: identity,                                  // End in place
            },
            TransitionVariant::SlideDiagonalDownLeft => TransitionConfig {
                exit_start: identity,                                 // Start in place
                exit_end: Transform::new(-100.0, 100.0, 1.0, 0.0),    // Exit down and left
                enter_start: Transform::new(100.0, -100.0, 1.0, 0.0), // Enter from top right
                enter_end: identity,                                  // End in place
            },
            TransitionVariant::SlideDiagonalDownRight => TransitionConfig {
                exit_start: identity,                                  // Start in place
                exit_end: Transform::new(100.0, 100.0, 1.0, 0.0),      // Exit down and right
                enter_start: Transform::new(-100.0, -100.0, 1.0, 0.0), // Enter from top left
                enter_end: identity,                                   // End in place
            },
            TransitionVariant::SpiralIn => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::SpiralOut => TransitionConfig {
                exit_start: identity,                          // Start in place
                exit_end: Transform::new(0.0, 0.0, 2.0, 0.0),  // Grow to twice size
                enter_start: identity,                         // Start in place
                enter_end: Transform::new(0.0, 0.0, 0.0, 0.0), // Shrink to full size
            },
            TransitionVariant::ElasticIn => TransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),      // No change
                enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Start from bottom
                enter_end: identity,                               // End in place
            },
            TransitionVariant::ElasticOut => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),  // Exit to bottom
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0), // Start in place
                enter_end: identity,                             // No change
            },
            TransitionVariant::SwingIn => TransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),      // No change
                enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Start from bottom
                enter_end: identity,                               // End in place
            },
            TransitionVariant::SwingOut => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),  // Exit to bottom
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0), // Start in place
                enter_end: identity,                             // No change
            },
            TransitionVariant::SlideLeftFade => TransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(-100.0, 0.0, 1.0, 0.0),   // Exit left
                enter_start: Transform::new(100.0, 0.0, 1.0, 0.0), // Enter from right
                enter_end: identity,                               // End in place
            },
            TransitionVariant::SlideRightFade => TransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(100.0, 0.0, 1.0, 0.0),     // Exit right
                enter_start: Transform::new(-100.0, 0.0, 1.0, 0.0), // Enter from left
                enter_end: identity,                                // End in place
            },
            TransitionVariant::ScaleRotateFade => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::SlideFadeRotate => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::ScaleFadeFlip => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::RotateScaleSlide => TransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
        }
    }
}
