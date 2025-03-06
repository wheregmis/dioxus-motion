use crate::prelude::Transform;

#[derive(Clone)]
pub struct TransitionConfig {
    pub initial_from: Transform,
    pub final_from: Transform,
    pub initial_to: Transform,
    pub final_to: Transform,
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
        match self {
            TransitionVariant::SlideLeft => TransitionConfig {
                initial_from: Transform::identity(),
                final_from: Transform::new(-100.0, 0.0, 1.0, 1.0),
                initial_to: Transform::new(100.0, 0.0, 1.0, 1.0),
                final_to: Transform::identity(),
            },
            TransitionVariant::SlideRight => TransitionConfig {
                initial_from: Transform::identity(),
                final_from: Transform::new(100.0, 0.0, 1.0, 1.0),
                initial_to: Transform::new(-100.0, 0.0, 1.0, 1.0),
                final_to: Transform::identity(),
            },
            TransitionVariant::SlideUp => TransitionConfig {
                initial_from: Transform::identity(),
                final_from: Transform::new(0.0, -100.0, 1.0, 1.0),
                initial_to: Transform::new(0.0, 100.0, 1.0, 1.0),
                final_to: Transform::identity(),
            },
            TransitionVariant::SlideDown => TransitionConfig {
                initial_from: Transform::identity(),
                final_from: Transform::new(0.0, 100.0, 1.0, 1.0),
                initial_to: Transform::new(0.0, -100.0, 1.0, 1.0),
                final_to: Transform::identity(),
            },
            TransitionVariant::Fade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.0, 0.0),
                initial_to: Transform::new(0.0, 0.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            // // Combined Transitions
            // TransitionVariant::SlideUpFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, -50.0, 1.0, 0.0),
            //     initial_to: Transform::new(0.0, 50.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SlideDownFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 50.0, 1.0, 0.0),
            //     initial_to: Transform::new(0.0, -50.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SlideLeftFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(-50.0, 0.0, 1.0, 0.0),
            //     initial_to: Transform::new(50.0, 0.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SlideRightFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(50.0, 0.0, 1.0, 0.0),
            //     initial_to: Transform::new(-50.0, 0.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ScaleUpFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.2, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, 0.8, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ScaleDownFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 0.8, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, 1.2, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::RotateLeftFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.0, -45.0),
            //     initial_to: Transform::new(0.0, 0.0, 1.0, 45.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::RotateRightFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.0, 45.0),
            //     initial_to: Transform::new(0.0, 0.0, 1.0, -45.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::FlipHorizontalFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, -1.0, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, -1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::FlipVerticalFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 180.0, 1.0, 0.0),
            //     initial_to: Transform::new(0.0, -180.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ScaleRotateFade => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.5, -45.0),
            //     initial_to: Transform::new(0.0, 0.0, 0.5, 45.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SlideFadeRotate => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(-50.0, -50.0, 1.0, -45.0),
            //     initial_to: Transform::new(50.0, 50.0, 1.0, 45.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ScaleFadeFlip => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 180.0, 1.5, 0.0),
            //     initial_to: Transform::new(0.0, -180.0, 0.5, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::RotateScaleSlide => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(-50.0, 0.0, 1.5, -90.0),
            //     initial_to: Transform::new(50.0, 0.0, 0.5, 90.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ScaleUp => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.5, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, 0.5, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ScaleDown => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 0.5, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, 1.5, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::FlipHorizontal => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, -1.0, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, -1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::FlipVertical => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 180.0, 1.0, 0.0),
            //     initial_to: Transform::new(0.0, -180.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::RotateLeft => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.0, -90.0),
            //     initial_to: Transform::new(0.0, 0.0, 1.0, 90.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::RotateRight => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.0, 90.0),
            //     initial_to: Transform::new(0.0, 0.0, 1.0, -90.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::BounceIn => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 50.0, 0.3, 0.0),
            //     initial_to: Transform::new(0.0, -50.0, 1.3, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::BounceOut => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, -50.0, 1.3, 0.0),
            //     initial_to: Transform::new(0.0, 50.0, 0.3, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ZoomIn => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 2.0, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, 0.1, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ZoomOut => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 0.1, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, 2.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SlideDiagonalUpLeft => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(-100.0, -100.0, 1.0, 0.0),
            //     initial_to: Transform::new(100.0, 100.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SlideDiagonalUpRight => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(100.0, -100.0, 1.0, 0.0),
            //     initial_to: Transform::new(-100.0, 100.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SlideDiagonalDownLeft => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(-100.0, 100.0, 1.0, 0.0),
            //     initial_to: Transform::new(100.0, -100.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SlideDiagonalDownRight => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(100.0, 100.0, 1.0, 0.0),
            //     initial_to: Transform::new(-100.0, -100.0, 1.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SpiralIn => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 0.0, -720.0),
            //     initial_to: Transform::new(0.0, 0.0, 2.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SpiralOut => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 2.0, 720.0),
            //     initial_to: Transform::new(0.0, 0.0, 0.0, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ElasticIn => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 0.3, 0.0),
            //     initial_to: Transform::new(0.0, 30.0, 1.5, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::ElasticOut => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 30.0, 1.5, 0.0),
            //     initial_to: Transform::new(0.0, 0.0, 0.3, 0.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SwingIn => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.0, -20.0),
            //     initial_to: Transform::new(0.0, 0.0, 1.0, 20.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
            // TransitionVariant::SwingOut => TransitionConfig {
            //     initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
            //     final_from: Transform::new(0.0, 0.0, 1.0, 20.0),
            //     initial_to: Transform::new(0.0, 0.0, 1.0, -20.0),
            //     final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            // },
        }
    }
}
