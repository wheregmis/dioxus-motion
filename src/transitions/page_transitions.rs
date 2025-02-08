use std::marker::PhantomData;

use dioxus_lib::prelude::*;
use dioxus_router::{
    hooks::use_route,
    prelude::{Outlet, Routable},
};

use crate::{
    prelude::{AnimationConfig, AnimationMode, Spring, Transform},
    use_motion, AnimationManager,
};

/// Ask for MARC Permission and Give him Credit for his work on this code

#[derive(Clone)]
pub enum AnimatedRouterContext<R: Routable + PartialEq> {
    /// Transition from one route to another.
    FromTo(R, R),
    /// Settled in a route.
    In(R),
}

impl<R: Routable + PartialEq> AnimatedRouterContext<R> {
    /// Get the current destination route.
    pub fn target_route(&self) -> &R {
        match self {
            Self::FromTo(_, to) => to,
            Self::In(to) => to,
        }
    }

    /// Update the destination route.
    pub fn set_target_route(&mut self, to: R) {
        match self {
            Self::FromTo(old_from, old_to) => {
                *old_from = old_to.clone();
                *old_to = to
            }
            Self::In(old_to) => *self = Self::FromTo(old_to.clone(), to),
        }
    }

    /// After the transition animation has finished, make the outlet only render the destination route.
    pub fn settle(&mut self) {
        if let Self::FromTo(_, to) = self {
            *self = Self::In(to.clone())
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AnimatedRouterProps {}

/// Provide a mechanism for outlets to animate between route transitions.
///
/// See the `animated_sidebar.rs` or `animated_tabs.rs` for an example on how to use it.
#[allow(non_snake_case)]
pub fn AnimatedRouter<R: AnimatableRoute>(AnimatedRouterProps {}: AnimatedRouterProps) -> Element {
    let route = use_route::<R>();
    let mut prev_route = use_signal(|| AnimatedRouterContext::In(route.clone()));
    use_context_provider(move || prev_route);

    if prev_route.peek().target_route() != &route {
        prev_route.write().set_target_route(route);
    }

    rsx!(AnimatedOutlet::<R> {})
}

pub trait AnimatableRoute: Routable + Clone + PartialEq {
    fn get_transition(&self) -> TransitionVariant;
    fn get_component(&self) -> Element;
}

#[allow(non_snake_case)]
pub fn AnimatedOutlet<R: AnimatableRoute>() -> Element {
    let animated_router = use_context::<Signal<AnimatedRouterContext<R>>>();
    let from_route: Option<(Element, TransitionVariant)> = match animated_router() {
        AnimatedRouterContext::FromTo(from, to) => {
            Some((from.get_component(), to.get_transition()))
        }
        _ => None,
    };

    rsx! {
        div {
            if let Some((from, transition)) = from_route {
                FromRouteToCurrent { route_type: PhantomData::<R>, from, transition }
            } else {
                Outlet::<R> {}
            }
        }
    }
}

/// Shortcut to get access to the [AnimatedRouterContext].
pub fn use_animated_router<Route: Routable + PartialEq>() -> Signal<AnimatedRouterContext<Route>> {
    use_context()
}

#[derive(Clone)]
pub struct TransitionConfig {
    initial_from: Transform,
    final_from: Transform,
    initial_to: Transform,
    final_to: Transform,
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
    fn get_config(&self) -> TransitionConfig {
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
            // Combined Transitions
            TransitionVariant::SlideUpFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, -50.0, 1.0, 0.0),
                initial_to: Transform::new(0.0, 50.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SlideDownFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 50.0, 1.0, 0.0),
                initial_to: Transform::new(0.0, -50.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SlideLeftFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(-50.0, 0.0, 1.0, 0.0),
                initial_to: Transform::new(50.0, 0.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SlideRightFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(50.0, 0.0, 1.0, 0.0),
                initial_to: Transform::new(-50.0, 0.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ScaleUpFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.2, 0.0),
                initial_to: Transform::new(0.0, 0.0, 0.8, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ScaleDownFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 0.8, 0.0),
                initial_to: Transform::new(0.0, 0.0, 1.2, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::RotateLeftFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.0, -45.0),
                initial_to: Transform::new(0.0, 0.0, 1.0, 45.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::RotateRightFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.0, 45.0),
                initial_to: Transform::new(0.0, 0.0, 1.0, -45.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::FlipHorizontalFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, -1.0, 0.0),
                initial_to: Transform::new(0.0, 0.0, -1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::FlipVerticalFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 180.0, 1.0, 0.0),
                initial_to: Transform::new(0.0, -180.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ScaleRotateFade => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.5, -45.0),
                initial_to: Transform::new(0.0, 0.0, 0.5, 45.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SlideFadeRotate => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(-50.0, -50.0, 1.0, -45.0),
                initial_to: Transform::new(50.0, 50.0, 1.0, 45.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ScaleFadeFlip => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 180.0, 1.5, 0.0),
                initial_to: Transform::new(0.0, -180.0, 0.5, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::RotateScaleSlide => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(-50.0, 0.0, 1.5, -90.0),
                initial_to: Transform::new(50.0, 0.0, 0.5, 90.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ScaleUp => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.5, 0.0),
                initial_to: Transform::new(0.0, 0.0, 0.5, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ScaleDown => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 0.5, 0.0),
                initial_to: Transform::new(0.0, 0.0, 1.5, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::FlipHorizontal => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, -1.0, 0.0),
                initial_to: Transform::new(0.0, 0.0, -1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::FlipVertical => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 180.0, 1.0, 0.0),
                initial_to: Transform::new(0.0, -180.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::RotateLeft => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.0, -90.0),
                initial_to: Transform::new(0.0, 0.0, 1.0, 90.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::RotateRight => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.0, 90.0),
                initial_to: Transform::new(0.0, 0.0, 1.0, -90.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::BounceIn => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 50.0, 0.3, 0.0),
                initial_to: Transform::new(0.0, -50.0, 1.3, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::BounceOut => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, -50.0, 1.3, 0.0),
                initial_to: Transform::new(0.0, 50.0, 0.3, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ZoomIn => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 2.0, 0.0),
                initial_to: Transform::new(0.0, 0.0, 0.1, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ZoomOut => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 0.1, 0.0),
                initial_to: Transform::new(0.0, 0.0, 2.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SlideDiagonalUpLeft => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(-100.0, -100.0, 1.0, 0.0),
                initial_to: Transform::new(100.0, 100.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SlideDiagonalUpRight => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(100.0, -100.0, 1.0, 0.0),
                initial_to: Transform::new(-100.0, 100.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SlideDiagonalDownLeft => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(-100.0, 100.0, 1.0, 0.0),
                initial_to: Transform::new(100.0, -100.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SlideDiagonalDownRight => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(100.0, 100.0, 1.0, 0.0),
                initial_to: Transform::new(-100.0, -100.0, 1.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SpiralIn => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 0.0, -720.0),
                initial_to: Transform::new(0.0, 0.0, 2.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SpiralOut => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 2.0, 720.0),
                initial_to: Transform::new(0.0, 0.0, 0.0, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ElasticIn => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 0.3, 0.0),
                initial_to: Transform::new(0.0, 30.0, 1.5, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::ElasticOut => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 30.0, 1.5, 0.0),
                initial_to: Transform::new(0.0, 0.0, 0.3, 0.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SwingIn => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.0, -20.0),
                initial_to: Transform::new(0.0, 0.0, 1.0, 20.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
            TransitionVariant::SwingOut => TransitionConfig {
                initial_from: Transform::new(0.0, 0.0, 1.0, 1.0),
                final_from: Transform::new(0.0, 0.0, 1.0, 20.0),
                initial_to: Transform::new(0.0, 0.0, 1.0, -20.0),
                final_to: Transform::new(0.0, 0.0, 1.0, 1.0),
            },
        }
    }
}

#[component]
fn FromRouteToCurrent<R: AnimatableRoute>(
    route_type: PhantomData<R>,
    from: Element,
    transition: TransitionVariant,
) -> Element {
    let mut animated_router = use_animated_router::<R>();
    let config = transition.get_config();
    let mut from_transform = use_motion(config.initial_from);
    let mut to_transform = use_motion(config.initial_to);
    let mut from_opacity = use_motion(1.0f32);
    let mut to_opacity = use_motion(0.0f32);

    use_effect(move || {
        let spring = Spring {
            stiffness: 160.0, // Reduced from 180.0 for less aggressive movement
            damping: 20.0,    // Increased from 12.0 for faster settling
            mass: 1.5,        // Slightly increased for more "weight"
            velocity: 10.0,   // Keep at 0 for predictable start
        };

        // Animate FROM route
        from_transform.animate_to(
            config.final_from,
            AnimationConfig::new(AnimationMode::Spring(spring)),
        );

        // Animate TO route
        to_transform.animate_to(
            config.final_to,
            AnimationConfig::new(AnimationMode::Spring(spring)),
        );

        // Fade out old route
        from_opacity.animate_to(0.0, AnimationConfig::new(AnimationMode::Spring(spring)));
        to_opacity.animate_to(1.0, AnimationConfig::new(AnimationMode::Spring(spring)));
    });

    use_effect(move || {
        if !from_transform.is_running() && !to_transform.is_running() {
            animated_router.write().settle();
        }
    });

    rsx! {
        div {
            class: "route-container",
            style: "
                position: relative; 
                width: 100%; 
                height: 100vh; 
                overflow: hidden;
                transform-style: preserve-3d;
                -webkit-transform-style: preserve-3d;
                -webkit-tap-highlight-color: transparent;
            ",
            div {
                class: "route-content from",
                style: "
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    transform: translate3d({from_transform.get_value().x}%, {from_transform.get_value().y}%, 0) 
                             scale({from_transform.get_value().scale});
                    opacity: {from_opacity.get_value()};
                    will-change: transform, opacity;
                    backface-visibility: hidden;
                    -webkit-backface-visibility: hidden;
                ",
                {from}
            }
            div {
                class: "route-content to",
                style: "
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    transform: translate3d({to_transform.get_value().x}%, {to_transform.get_value().y}%, 0) 
                             scale({to_transform.get_value().scale});
                    opacity: {to_opacity.get_value()};
                    will-change: transform, opacity;
                    backface-visibility: hidden;
                    -webkit-backface-visibility: hidden;
                ",
                Outlet::<R> {}
            }
        }
    }
}
