use std::{marker::PhantomData, str::FromStr};

use dioxus::prelude::*;

use crate::{
    prelude::{AnimationConfig, AnimationMode, Spring},
    use_motion, AnimationManager,
};

use super::utils::TransitionVariant;

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

#[component]
pub fn AnimatedOutlet<R: AnimatableRoute>() -> Element {
    let route = use_route::<R>();
    // Create router context only if we're the root AnimatedOutlet
    let mut prev_route = use_signal(|| AnimatedRouterContext::In(route.clone()));
    use_context_provider(move || prev_route);

    // Update route if changed
    if prev_route.peek().target_route() != &route {
        prev_route.write().set_target_route(route.clone());
    }

    let outlet: OutletContext<R> = use_outlet_context();

    println!("Route level: {}", route.get_layout_depth());
    println!(
        "Current Route: {:?}",
        prev_route().target_route().to_string()
    );
    println!("Outlet level: {}", outlet.level());

    let from_route: Option<(R, R)> = match prev_route() {
        AnimatedRouterContext::FromTo(from, to) => Some((from, to)),
        _ => None,
    };

    if let Some((from, to)) = from_route {
        println!("From: {:?} To: {:?}", from.to_string(), to.to_string());

        // Special handling for transitions from root path
        let is_from_root = from.to_string() == "/";

        // Animate if either we're at the correct level OR we're transitioning from root
        if is_from_root || outlet.level() == route.get_layout_depth() {
            return rsx! {
                FromRouteToCurrent::<R> {
                    route_type: PhantomData,
                    from: from.clone(),
                    to: to.clone(),
                }
            };
        } else {
            return rsx! {
                Outlet::<R> {}
            };
        }
    } else {
        return rsx! {
            Outlet::<R> {}
        };
    }
}

pub trait AnimatableRoute: Routable + Clone + PartialEq {
    fn get_transition(&self) -> TransitionVariant;
    fn get_component(&self) -> Element;
    fn get_layout_depth(&self) -> usize;
}

/// Shortcut to get access to the [AnimatedRouterContext].
pub fn use_animated_router<Route: Routable + PartialEq>() -> Signal<AnimatedRouterContext<Route>> {
    use_context()
}

#[component]
fn FromRouteToCurrent<R: AnimatableRoute>(route_type: PhantomData<R>, from: R, to: R) -> Element {
    let mut animated_router = use_animated_router::<R>();
    let config = to.get_transition().get_config();
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
        div { style: "position: relative; overflow: hidden;",
            div {
                class: "route-content from",
                style: "
                    transform: translate3d({from_transform.get_value().x}%, {from_transform.get_value().y}%, 0) 
                             scale({from_transform.get_value().scale});
                    opacity: {from_opacity.get_value()};
                    will-change: transform, opacity;
                    backface-visibility: hidden;
                    -webkit-backface-visibility: hidden;
                ",
                {from.get_component()}
            }
            div {
                class: "route-content to",
                style: "
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
