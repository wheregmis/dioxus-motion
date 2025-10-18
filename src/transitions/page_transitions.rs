use std::marker::PhantomData;

use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use dioxus_router::{Outlet, OutletContext, Routable, use_outlet_context, use_route};
use std::rc::Rc;

use crate::{
    prelude::{AnimationConfig, AnimationMode, MotionStoreStoreExt, Spring, Tween},
    store::use_motion_store,
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

/// Store-based router context for fine-grained reactivity
///
/// Unlike the signal-based `AnimatedRouterContext`, components can subscribe
/// to specific fields to avoid unnecessary re-renders:
/// - `to_route()` - Subscribe only to destination route changes
/// - `from_route()` - Subscribe only to source route changes  
/// - `is_transitioning()` - Subscribe only to transition state changes
#[derive(Store)]
pub struct AnimatedRouterStore<R> {
    /// The route we're transitioning from (None if settled)
    pub from_route: Option<R>,
    /// The route we're transitioning to (current destination)
    pub to_route: R,
    /// Whether we're currently in a transition
    pub is_transitioning: bool,
}

impl<R: Routable + PartialEq + Clone + 'static> AnimatedRouterStore<R> {
    /// Create a new router store in settled state
    pub fn new(route: R) -> Self {
        Self {
            from_route: None,
            to_route: route,
            is_transitioning: false,
        }
    }
}

/// Store methods for AnimatedRouterStore
#[store]
impl<R: Routable + PartialEq + Clone + 'static> Store<AnimatedRouterStore<R>> {
    /// Get the current destination route
    ///
    /// Components can subscribe to this to react only to destination changes
    fn target_route(&self) -> R {
        self.to_route().cloned()
    }

    /// Check if we're currently transitioning between routes
    ///
    /// Components can subscribe to this to react only to transition state changes
    fn is_in_transition(&self) -> bool {
        self.is_transitioning().cloned()
    }

    /// Get the route pair if transitioning, None if settled
    ///
    /// Returns (from, to) tuple if transitioning, None if settled
    fn get_transition_routes(&self) -> Option<(R, R)> {
        if self.is_transitioning().cloned() {
            self.from_route()
                .cloned()
                .map(|from| (from, self.to_route().cloned()))
        } else {
            None
        }
    }

    /// Update the destination route and manage transition state
    fn set_target_route(&mut self, new_route: R) {
        let current_to = self.to_route().cloned();

        if current_to != new_route {
            if self.is_transitioning().cloned() {
                // Already transitioning, update the chain: old_from -> old_to -> new_to
                self.from_route().set(Some(current_to));
            } else {
                // Starting new transition: current -> new
                self.from_route().set(Some(current_to));
                self.is_transitioning().set(true);
            }

            self.to_route().set(new_route);
        }
    }

    /// Settle the transition - move to non-transitioning state
    fn settle(&mut self) {
        self.from_route().set(None);
        self.is_transitioning().set(false);
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

impl Default for PageTransitionAnimation {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0, // Default scale to 1.0 for identity
            rotation: 0.0,
            opacity: 1.0, // Default to fully opaque
        }
    }
}

impl std::ops::Add for PageTransitionAnimation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            scale: self.scale + other.scale,
            rotation: self.rotation + other.rotation,
            opacity: self.opacity + other.opacity,
        }
    }
}

impl std::ops::Sub for PageTransitionAnimation {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            scale: self.scale - other.scale,
            rotation: self.rotation - other.rotation,
            opacity: self.opacity - other.opacity,
        }
    }
}

impl std::ops::Mul<f32> for PageTransitionAnimation {
    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            scale: self.scale * factor,
            rotation: self.rotation * factor,
            opacity: self.opacity * factor,
        }
    }
}

impl Animatable for PageTransitionAnimation {
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

    fn magnitude(&self) -> f32 {
        (self.x * self.x
            + self.y * self.y
            + self.scale * self.scale
            + self.rotation * self.rotation
            + self.opacity * self.opacity)
            .sqrt()
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
    // Create router store for fine-grained reactivity
    let mut router_store = use_store(|| AnimatedRouterStore::new(route.clone()));
    use_context_provider(move || router_store);

    use_effect(move || {
        let current_route = use_route::<R>();
        if router_store.target_route() != current_route {
            router_store.set_target_route(current_route.clone());
        }
    });

    let outlet: OutletContext<R> = use_outlet_context();

    // Get transition routes using store method for fine-grained reactivity
    let from_route: Option<(R, R)> = router_store.get_transition_routes();

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

/// Shortcut to get access to the AnimatedRouterStore for fine-grained reactivity.
pub fn use_animated_router<Route: Routable + PartialEq + Clone + 'static>()
-> Store<AnimatedRouterStore<Route>> {
    use_context()
}

// Add a type alias for the resolver
pub type TransitionVariantResolver<R> = Rc<dyn Fn(&R, &R) -> TransitionVariant>;

#[component]
fn FromRouteToCurrent<R: AnimatableRoute>(route_type: PhantomData<R>, from: R, to: R) -> Element {
    let mut animated_router = use_animated_router::<R>();
    // Try to get a dynamic transition resolver from context
    let resolver = try_use_context::<TransitionVariantResolver<R>>();
    // Use the resolver if present, otherwise use the static transition
    let transition_variant =
        resolver.map_or_else(|| to.get_transition(), |resolver| resolver(&from, &to));
    let config = transition_variant.get_config();
    let mut from_anim = use_motion_store(PageTransitionAnimation::from_exit_start(&config));
    let mut to_anim = use_motion_store(PageTransitionAnimation::from_enter_start(&config));

    // Try to get a Tween from context, otherwise use Spring
    let tween = try_use_context::<Signal<Tween>>();
    let spring = try_use_context::<Signal<Spring>>();

    use_effect(move || {
        let (from_config, to_config) = tween.map_or_else(
            || {
                let spring = spring.unwrap_or_else(|| {
                    use_signal(|| Spring {
                        stiffness: 160.0,
                        damping: 25.0,
                        mass: 1.0,
                        velocity: 0.0,
                    })
                });
                (
                    AnimationConfig::new(AnimationMode::Spring(spring())),
                    AnimationConfig::new(AnimationMode::Spring(spring())),
                )
            },
            |tween| {
                (
                    AnimationConfig::new(AnimationMode::Tween(tween())),
                    AnimationConfig::new(AnimationMode::Tween(tween())),
                )
            },
        );
        // Use the new MotionHandle API
        from_anim.animate_to(PageTransitionAnimation::from_exit_end(&config), from_config);
        to_anim.animate_to(PageTransitionAnimation::from_enter_end(&config), to_config);
    });

    use_effect(move || {
        if !from_anim.store().running()() && !to_anim.store().running()() {
            animated_router.settle();
        }
    });

    let from_val = from_anim.store().current()();
    let to_val = to_anim.store().current()();

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
