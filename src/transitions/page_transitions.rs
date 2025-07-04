use std::marker::PhantomData;

use dioxus::prelude::*;

use crate::{
    AnimationManager,
    prelude::{AnimationConfig, AnimationMode, Spring},
    use_motion,
};

use super::config::TransitionVariant;
use crate::animations::core::Animatable;
use crate::prelude::Transform;
use wide::f32x4;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageTransitionAnimation {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotation: f32,
    pub opacity: f32,
}

impl PageTransitionAnimation {
    pub fn from_transform_and_opacity(transform: &Transform, opacity: f32) -> Self {
        Self {
            x: transform.x,
            y: transform.y,
            scale: transform.scale,
            rotation: transform.rotation,
            opacity,
        }
    }
    pub fn from_exit_start(config: &super::config::TransitionConfig) -> Self {
        Self::from_transform_and_opacity(&config.exit_start, 1.0)
    }
    pub fn from_exit_end(config: &super::config::TransitionConfig) -> Self {
        Self::from_transform_and_opacity(&config.exit_end, 0.0)
    }
    pub fn from_enter_start(config: &super::config::TransitionConfig) -> Self {
        Self::from_transform_and_opacity(&config.enter_start, 0.0)
    }
    pub fn from_enter_end(config: &super::config::TransitionConfig) -> Self {
        Self::from_transform_and_opacity(&config.enter_end, 1.0)
    }
}

impl Animatable for PageTransitionAnimation {
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 0.0,
            rotation: 0.0,
            opacity: 0.0,
        }
    }
    fn epsilon() -> f32 {
        0.005
    }
    fn magnitude(&self) -> f32 {
        (self.x * self.x
            + self.y * self.y
            + self.scale * self.scale
            + self.rotation * self.rotation
            + self.opacity * self.opacity)
            .sqrt()
    }
    fn scale(&self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            scale: self.scale * factor,
            rotation: self.rotation * factor,
            opacity: self.opacity * factor,
        }
    }
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            scale: self.scale + other.scale,
            rotation: self.rotation + other.rotation,
            opacity: self.opacity + other.opacity,
        }
    }
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            scale: self.scale - other.scale,
            rotation: self.rotation - other.rotation,
            opacity: self.opacity - other.opacity,
        }
    }
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let a = [self.x, self.y, self.scale, self.opacity];
        let b = [target.x, target.y, target.scale, target.opacity];
        let va = f32x4::new([a[0], a[1], a[2], a[3]]);
        let vb = f32x4::new([b[0], b[1], b[2], b[3]]);
        let vt = f32x4::splat(t.clamp(0.0, 1.0));
        let result = va + (vb - va) * vt;
        let out = result.to_array();
        // Rotation: shortest path
        let mut rotation_diff = target.rotation - self.rotation;
        if rotation_diff > std::f32::consts::PI {
            rotation_diff -= 2.0 * std::f32::consts::PI;
        } else if rotation_diff < -std::f32::consts::PI {
            rotation_diff += 2.0 * std::f32::consts::PI;
        }
        let rotation = self.rotation + rotation_diff * t;
        Self {
            x: out[0],
            y: out[1],
            scale: out[2],
            rotation,
            opacity: out[3],
        }
    }
}

#[component]
/// Renders an outlet that supports animated transitions between routes.
///
/// This function sets up a routing context and monitors changes in the current route to
/// determine when an animated transition should occur. When a transition is detected and
/// the layout depth or route conditions are met, it renders a transition component; otherwise,
/// it renders a standard outlet.
/// ```
pub fn AnimatedOutlet<R: AnimatableRoute>() -> Element {
    let route = use_route::<R>();
    // Create router context only if we're the root AnimatedOutlet
    let mut prev_route = use_signal(|| AnimatedRouterContext::In(route.clone()));
    use_context_provider(move || prev_route);

    use_effect(move || {
        if prev_route.peek().target_route() != &use_route::<R>() {
            prev_route
                .write()
                .set_target_route(use_route::<R>().clone());
        }
    });

    let outlet: OutletContext<R> = use_outlet_context();

    let from_route: Option<(R, R)> = match prev_route() {
        AnimatedRouterContext::FromTo(from, to) => Some((from, to)),
        _ => None,
    };

    if let Some((from, to)) = from_route {
        // Get the layout depth of both the previous and current routes
        let from_depth = from.get_layout_depth();
        let to_depth = to.get_layout_depth();

        // Get the current level of nesting in the outlet
        let current_level = outlet.level();

        // Determine if the transition involves the root route (depth 1)
        let involves_root = from_depth == 1 || to_depth == 1;

        // Check if the depth hasn't changed and the outlet level matches
        let is_same_depth_and_matching_level = from_depth == to_depth && current_level == to_depth;

        // If we're transitioning from/to root, or the outlet is at the same depth,
        // render the animated transition between routes
        if involves_root || is_same_depth_and_matching_level {
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
    let mut from_anim = use_motion(PageTransitionAnimation::from_exit_start(&config));
    let mut to_anim = use_motion(PageTransitionAnimation::from_enter_start(&config));

    let spring = try_use_context::<Signal<Spring>>().unwrap_or_else(|| {
        use_signal(|| Spring {
            stiffness: 160.0,
            damping: 25.0,
            mass: 1.0,
            velocity: 0.0,
        })
    });

    use_effect(move || {
        from_anim.animate_to(
            PageTransitionAnimation::from_exit_end(&config),
            AnimationConfig::new(AnimationMode::Spring(spring())).with_epsilon(0.01),
        );
        to_anim.animate_to(
            PageTransitionAnimation::from_enter_end(&config),
            AnimationConfig::new(AnimationMode::Spring(spring())).with_epsilon(0.01),
        );
    });

    use_effect(move || {
        if !from_anim.is_running() && !to_anim.is_running() {
            animated_router.write().settle();
        }
    });

    let from_val = from_anim.get_value();
    let to_val = to_anim.get_value();

    rsx! {
        div {
            class: "route-container",
            style: "position: relative; overflow-visible; perspective: 1000px;",
            div {
                class: "route-content from",
                style: format!(
                    "transform: translate3d({}% , {}%, 0) scale({}); opacity: {}; will-change: transform, opacity; backface-visibility: hidden; -webkit-backface-visibility: hidden; contain: layout style;",
                    from_val.x, from_val.y, from_val.scale, from_val.opacity
                ),
                {from.render(from.get_layout_depth() + 1)}
            }
            div {
                class: "route-content to",
                style: format!(
                    "transform: translate3d({}% , {}%, 0) scale({}); opacity: {}; will-change: transform, opacity; backface-visibility: hidden; -webkit-backface-visibility: hidden;",
                    to_val.x, to_val.y, to_val.scale, to_val.opacity
                ),
                Outlet::<R> {}
            }
        }
    }
}
