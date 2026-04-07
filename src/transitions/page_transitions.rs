use std::marker::PhantomData;

use dioxus::{
    prelude::*,
    router::{OutletContext, use_outlet_context},
};
use std::rc::Rc;

use crate::{
    AnimationManager,
    prelude::{AnimationConfig, AnimationMode, Spring, Tween},
    use_motion,
};

use super::config::TransitionVariant;
use crate::animations::core::Animatable;
use crate::prelude::Transform;
use wide::f32x4;

#[cfg_attr(feature = "dioxus", derive(Store))]
#[derive(Clone)]
pub enum AnimatedRouterContext<R: Routable + PartialEq> {
    /// Transition from one route to another.
    FromTo(R, R),
    /// Settled in a route.
    Settled(R),
}

impl<R: Routable + PartialEq> AnimatedRouterContext<R> {
    /// Get the current destination route.
    pub fn target_route(&self) -> &R {
        match self {
            Self::FromTo(_, to) => to,
            Self::Settled(to) => to,
        }
    }

    /// Update the destination route.
    pub fn set_target_route(&mut self, to: R) {
        match self {
            Self::FromTo(old_from, old_to) => {
                *old_from = old_to.clone();
                *old_to = to
            }
            Self::Settled(old_to) => *self = Self::FromTo(old_to.clone(), to),
        }
    }

    /// After the transition animation has finished, make the outlet only render the destination route.
    pub fn settle(&mut self) {
        if let Self::FromTo(_, to) = self {
            *self = Self::Settled(to.clone())
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
    // Create router context only if we're the root AnimatedOutlet
    let mut prev_route = use_store(|| AnimatedRouterContext::Settled(route.clone()));
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
pub fn use_animated_router<Route: Routable + PartialEq>() -> Store<AnimatedRouterContext<Route>> {
    use_context()
}

// Add a type alias for the resolver
pub type TransitionVariantResolver<R> = Rc<dyn Fn(&R, &R) -> TransitionVariant>;

fn default_transition_spring() -> Spring {
    Spring {
        stiffness: 160.0,
        damping: 25.0,
        mass: 1.0,
        velocity: 0.0,
    }
}

fn resolve_transition_mode(
    tween_store: Option<Store<Tween>>,
    spring_store: Option<Store<Spring>>,
    default_spring: Store<Spring>,
) -> AnimationMode {
    tween_store
        .map(|tween| AnimationMode::Tween(tween()))
        .unwrap_or_else(|| AnimationMode::Spring(spring_store.unwrap_or(default_spring)()))
}

#[component]
fn FromRouteToCurrent<R: AnimatableRoute>(route_type: PhantomData<R>, from: R, to: R) -> Element {
    let mut animated_router = use_animated_router::<R>();
    // Try to get a dynamic transition resolver from context
    let resolver = try_use_context::<TransitionVariantResolver<R>>();
    // Use the resolver if present, otherwise use the static transition
    let transition_variant =
        resolver.map_or_else(|| to.get_transition(), |resolver| resolver(&from, &to));
    let config = transition_variant.get_config();
    let mut from_anim = use_motion(PageTransitionAnimation::from_exit_start(&config));
    let mut to_anim = use_motion(PageTransitionAnimation::from_enter_start(&config));
    let default_spring = use_store(default_transition_spring);

    // Try to get a store-backed animation mode from context, otherwise use the default spring.
    let tween_store = try_use_context::<Store<Tween>>();
    let spring_store = try_use_context::<Store<Spring>>();

    use_effect(move || {
        let mode = resolve_transition_mode(tween_store, spring_store, default_spring);
        let animation_config = AnimationConfig::new(mode);

        from_anim.animate_to(
            PageTransitionAnimation::from_exit_end(&config),
            animation_config.clone(),
        );
        to_anim.animate_to(
            PageTransitionAnimation::from_enter_end(&config),
            animation_config,
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use dioxus::prelude::{Element, Store, VNode, VirtualDom, use_hook, use_store};
    use instant::Duration;

    use super::{AnimationMode, Spring, Tween, default_transition_spring, resolve_transition_mode};

    #[derive(Clone)]
    struct ResolveModeProps {
        tween: Option<Tween>,
        spring: Option<Spring>,
        default_spring: Spring,
        result: Rc<RefCell<Option<AnimationMode>>>,
    }

    #[allow(non_snake_case)]
    fn ResolveModeHost(props: ResolveModeProps) -> Element {
        let tween_store = use_hook(move || props.tween.map(Store::new));
        let spring_store = use_hook(move || props.spring.map(Store::new));
        let default_spring = use_store(move || props.default_spring);

        *props.result.borrow_mut() = Some(resolve_transition_mode(
            tween_store,
            spring_store,
            default_spring,
        ));

        VNode::empty()
    }

    fn resolve_mode_in_runtime(
        tween: Option<Tween>,
        spring: Option<Spring>,
        default_spring: Spring,
    ) -> AnimationMode {
        let resolved_mode = Rc::new(RefCell::new(None));
        let mut dom = VirtualDom::new_with_props(
            ResolveModeHost,
            ResolveModeProps {
                tween,
                spring,
                default_spring,
                result: Rc::clone(&resolved_mode),
            },
        );

        dom.rebuild_in_place();

        resolved_mode
            .borrow()
            .as_ref()
            .copied()
            .expect("test component should resolve an animation mode")
    }

    #[test]
    fn transition_mode_prefers_tween_store() {
        let tween = Tween::new(Duration::from_millis(450));
        let spring = Spring {
            stiffness: 320.0,
            damping: 40.0,
            mass: 2.0,
            velocity: 3.0,
        };

        let mode = resolve_mode_in_runtime(Some(tween), Some(spring), default_transition_spring());

        assert_eq!(mode, AnimationMode::Tween(tween));
    }

    #[test]
    fn transition_mode_uses_context_spring_before_default() {
        let spring = Spring {
            stiffness: 220.0,
            damping: 32.0,
            mass: 1.5,
            velocity: 2.5,
        };

        let mode = resolve_mode_in_runtime(None, Some(spring), default_transition_spring());

        assert_eq!(mode, AnimationMode::Spring(spring));
    }

    #[test]
    fn transition_mode_falls_back_to_default_spring_store() {
        let default_spring = default_transition_spring();

        let mode = resolve_mode_in_runtime(None, None, default_spring);

        assert_eq!(mode, AnimationMode::Spring(default_spring));
    }
}
