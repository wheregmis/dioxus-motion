use crate::core::transform::Transform;
use crate::core::transition::TransitionVariant;

impl TransitionVariant {
    pub fn get_config(&self) -> PageTransitionConfig {
        let identity = Transform::identity();

        match self {
            TransitionVariant::SlideLeft => {
                PageTransitionConfig {
                    exit_start: identity,                              // Start in place
                    exit_end: Transform::new(-100.0, 0.0, 1.0, 0.0),   // Exit left
                    enter_start: Transform::new(100.0, 0.0, 1.0, 0.0), // Enter from right
                    enter_end: identity,                               // End in place
                }
            }

            TransitionVariant::SlideRight => {
                PageTransitionConfig {
                    exit_start: identity,                               // Start in place
                    exit_end: Transform::new(100.0, 0.0, 1.0, 0.0),     // Exit right
                    enter_start: Transform::new(-100.0, 0.0, 1.0, 0.0), // Enter from left
                    enter_end: identity,                                // End in place
                }
            }

            TransitionVariant::SlideUp => {
                PageTransitionConfig {
                    exit_start: identity,                              // Start in place
                    exit_end: Transform::new(0.0, -100.0, 1.0, 0.0),   // Exit up
                    enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Enter from bottom
                    enter_end: identity,                               // End in place
                }
            }

            TransitionVariant::SlideDown => {
                PageTransitionConfig {
                    exit_start: identity,                               // Start in place
                    exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),     // Exit down
                    enter_start: Transform::new(0.0, -100.0, 1.0, 0.0), // Enter from top
                    enter_end: identity,                                // End in place
                }
            }

            TransitionVariant::Fade => PageTransitionConfig {
                exit_start: identity,                            // Start fully visible
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // Fade out completely
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0), // Start invisible
                enter_end: identity,                             // Fade in completely
            },
            TransitionVariant::ScaleUp => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 0.0, 0.0),    // Shrink to nothing
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::ScaleDown => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 2.0, 0.0),    // Grow to twice size
                enter_start: Transform::new(0.0, 0.0, 2.0, 0.0), // Start twice size
                enter_end: identity,                             // Shrink to full size
            },
            TransitionVariant::FlipHorizontal => PageTransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 180.0),     // Flip 180 degrees horizontally
                enter_start: Transform::new(0.0, 0.0, 1.0, -180.0), // Start flipped 180 degrees horizontally
                enter_end: identity,                                // End in place
            },
            TransitionVariant::FlipVertical => PageTransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 180.0),     // Flip 180 degrees vertically
                enter_start: Transform::new(0.0, 0.0, 1.0, -180.0), // Start flipped 180 degrees vertically
                enter_end: identity,                                // End in place
            },
            TransitionVariant::RotateLeft => PageTransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 90.0),     // Rotate 90 degrees to the left
                enter_start: Transform::new(0.0, 0.0, 1.0, -90.0), // Start rotated 90 degrees to the right
                enter_end: identity,                               // End in place
            },
            TransitionVariant::RotateRight => PageTransitionConfig {
                exit_start: identity,                             // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, -90.0),   // Rotate 90 degrees to the right
                enter_start: Transform::new(0.0, 0.0, 1.0, 90.0), // Start rotated 90 degrees to the left
                enter_end: identity,                              // End in place
            },
            TransitionVariant::SlideUpFade => PageTransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, -100.0, 1.0, 0.0),   // Exit up
                enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Enter from bottom
                enter_end: identity,                               // End in place
            },
            TransitionVariant::SlideDownFade => PageTransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),     // Exit down
                enter_start: Transform::new(0.0, -100.0, 1.0, 0.0), // Enter from top
                enter_end: identity,                                // End in place
            },
            TransitionVariant::ScaleUpFade => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 0.0, 0.0),    // Shrink to nothing
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::BounceIn => PageTransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),      // No change
                enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Start from bottom
                enter_end: identity,                               // End in place
            },
            TransitionVariant::BounceOut => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),  // Exit to bottom
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0), // Start in place
                enter_end: identity,                             // No change
            },
            TransitionVariant::ScaleDownFade => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 2.0, 0.0),    // Grow to twice size
                enter_start: Transform::new(0.0, 0.0, 2.0, 0.0), // Start twice size
                enter_end: identity,                             // Shrink to full size
            },
            TransitionVariant::RotateLeftFade => PageTransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 90.0),     // Rotate 90 degrees to the left
                enter_start: Transform::new(0.0, 0.0, 1.0, -90.0), // Start rotated 90 degrees to the right
                enter_end: identity,                               // End in place
            },
            TransitionVariant::RotateRightFade => PageTransitionConfig {
                exit_start: identity,                             // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, -90.0),   // Rotate 90 degrees to the right
                enter_start: Transform::new(0.0, 0.0, 1.0, 90.0), // Start rotated 90 degrees to the left
                enter_end: identity,                              // End in place
            },
            TransitionVariant::FlipHorizontalFade => PageTransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 180.0),     // Flip 180 degrees horizontally
                enter_start: Transform::new(0.0, 0.0, 1.0, -180.0), // Start flipped 180 degrees horizontally
                enter_end: identity,                                // End in place
            },
            TransitionVariant::FlipVerticalFade => PageTransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 180.0),     // Flip 180 degrees vertically
                enter_start: Transform::new(0.0, 0.0, 1.0, -180.0), // Start flipped 180 degrees vertically
                enter_end: identity,                                // End in place
            },
            TransitionVariant::ZoomIn => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::ZoomOut => PageTransitionConfig {
                exit_start: identity,                          // Start in place
                exit_end: Transform::new(0.0, 0.0, 2.0, 0.0),  // Grow to twice size
                enter_start: identity,                         // Start in place
                enter_end: Transform::new(0.0, 0.0, 0.0, 0.0), // Shrink to full size
            },
            TransitionVariant::SlideDiagonalUpLeft => PageTransitionConfig {
                exit_start: identity,                                // Start in place
                exit_end: Transform::new(-100.0, -100.0, 1.0, 0.0),  // Exit up and left
                enter_start: Transform::new(100.0, 100.0, 1.0, 0.0), // Enter from bottom right
                enter_end: identity,                                 // End in place
            },
            TransitionVariant::SlideDiagonalUpRight => PageTransitionConfig {
                exit_start: identity,                                 // Start in place
                exit_end: Transform::new(100.0, -100.0, 1.0, 0.0),    // Exit up and right
                enter_start: Transform::new(-100.0, 100.0, 1.0, 0.0), // Enter from bottom left
                enter_end: identity,                                  // End in place
            },
            TransitionVariant::SlideDiagonalDownLeft => PageTransitionConfig {
                exit_start: identity,                                 // Start in place
                exit_end: Transform::new(-100.0, 100.0, 1.0, 0.0),    // Exit down and left
                enter_start: Transform::new(100.0, -100.0, 1.0, 0.0), // Enter from top right
                enter_end: identity,                                  // End in place
            },
            TransitionVariant::SlideDiagonalDownRight => PageTransitionConfig {
                exit_start: identity,                                  // Start in place
                exit_end: Transform::new(100.0, 100.0, 1.0, 0.0),      // Exit down and right
                enter_start: Transform::new(-100.0, -100.0, 1.0, 0.0), // Enter from top left
                enter_end: identity,                                   // End in place
            },
            TransitionVariant::SpiralIn => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::SpiralOut => PageTransitionConfig {
                exit_start: identity,                          // Start in place
                exit_end: Transform::new(0.0, 0.0, 2.0, 0.0),  // Grow to twice size
                enter_start: identity,                         // Start in place
                enter_end: Transform::new(0.0, 0.0, 0.0, 0.0), // Shrink to full size
            },
            TransitionVariant::ElasticIn => PageTransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),      // No change
                enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Start from bottom
                enter_end: identity,                               // End in place
            },
            TransitionVariant::ElasticOut => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),  // Exit to bottom
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0), // Start in place
                enter_end: identity,                             // No change
            },
            TransitionVariant::SwingIn => PageTransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),      // No change
                enter_start: Transform::new(0.0, 100.0, 1.0, 0.0), // Start from bottom
                enter_end: identity,                               // End in place
            },
            TransitionVariant::SwingOut => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 100.0, 1.0, 0.0),  // Exit to bottom
                enter_start: Transform::new(0.0, 0.0, 1.0, 0.0), // Start in place
                enter_end: identity,                             // No change
            },
            TransitionVariant::SlideLeftFade => PageTransitionConfig {
                exit_start: identity,                              // Start in place
                exit_end: Transform::new(-100.0, 0.0, 1.0, 0.0),   // Exit left
                enter_start: Transform::new(100.0, 0.0, 1.0, 0.0), // Enter from right
                enter_end: identity,                               // End in place
            },
            TransitionVariant::SlideRightFade => PageTransitionConfig {
                exit_start: identity,                               // Start in place
                exit_end: Transform::new(100.0, 0.0, 1.0, 0.0),     // Exit right
                enter_start: Transform::new(-100.0, 0.0, 1.0, 0.0), // Enter from left
                enter_end: identity,                                // End in place
            },
            TransitionVariant::ScaleRotateFade => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::SlideFadeRotate => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::ScaleFadeFlip => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
            TransitionVariant::RotateScaleSlide => PageTransitionConfig {
                exit_start: identity,                            // Start in place
                exit_end: Transform::new(0.0, 0.0, 1.0, 0.0),    // No change
                enter_start: Transform::new(0.0, 0.0, 0.0, 0.0), // Start as nothing
                enter_end: identity,                             // Grow to full size
            },
        }
    }
}
