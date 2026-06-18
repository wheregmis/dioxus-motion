//! Presence child normalization and lifecycle state.

use std::{
    any::Any,
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashSet},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::{
    Duration, Time,
    animations::{
        core::{Animatable, AnimationConfig},
        css::CssValue,
        platform::TimeProvider,
    },
    manager::{AnimationManager, MotionHandle},
    use_motion,
};
use dioxus::prelude::*;
use dioxus_core::{
    DynamicNode, Element, RenderError, ScopeId, VNode, current_scope_id, needs_update_any,
};
use thiserror::Error;

/// A normalized direct child managed by presence.
#[derive(Debug, Clone)]
pub(crate) struct PresenceChild {
    pub(crate) key: String,
    pub(crate) vnode: VNode,
}

/// Custom presence data captured for exiting children.
#[derive(Clone)]
pub struct PresenceCustom {
    value: Arc<dyn Any + Send + Sync>,
}

impl PresenceCustom {
    /// Stores a cloneable custom value for presence descendants.
    pub fn new<T: Clone + Send + Sync + 'static>(value: T) -> Self {
        Self {
            value: Arc::new(value),
        }
    }

    /// Attempts to read the custom value as `T`.
    pub fn get<T: Clone + 'static>(&self) -> Option<T> {
        self.value.downcast_ref::<T>().cloned()
    }
}

impl std::fmt::Debug for PresenceCustom {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PresenceCustom")
            .finish_non_exhaustive()
    }
}

impl PartialEq for PresenceCustom {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
    }
}

#[derive(Debug, Clone)]
struct RenderedPresenceChild {
    key: String,
    vnode: VNode,
    is_present: bool,
    initial_allowed: bool,
    layout_enter_allowed: bool,
    generation: u64,
    custom: Option<PresenceCustom>,
    mode: PresenceMode,
}

#[derive(Debug, Clone)]
struct PresenceRender {
    children: Vec<RenderedPresenceChild>,
    exit_completed: bool,
    notify_scopes: Vec<ScopeId>,
}

/// Errors produced while normalizing or reconciling presence children.
#[derive(Debug, Error)]
pub(crate) enum PresenceError {
    /// The child render returned an error.
    #[error("child render error: {0:?}")]
    Render(RenderError),

    /// Presence children must be keyed.
    #[error("AnimatePresence children must each have a stable key")]
    MissingKey,

    /// Presence sibling keys must be unique.
    #[error("AnimatePresence children must have unique keys; duplicate key `{0}`")]
    DuplicateKey(String),

    /// The RSX shape cannot be split into independent retained children.
    #[error(
        "AnimatePresence cannot split multiple static RSX roots; use a keyed iterator or a single keyed root"
    )]
    UnsupportedMultipleStaticRoots,

    /// The requested key is not currently tracked.
    #[error("presence key `{0}` is not tracked")]
    UnknownKey(String),
}

impl From<RenderError> for PresenceError {
    fn from(error: RenderError) -> Self {
        Self::Render(error)
    }
}

/// Normalize RSX children into keyed presence children.
///
/// This accepts the idiomatic Dioxus shapes used for conditional children and
/// keyed `for` loops. A keyed `for` loop is represented by Dioxus as a dynamic
/// fragment containing one `VNode` per child, which gives presence a stable
/// public key and an intact subtree to retain.
pub(crate) fn normalize_presence_children(
    children: Element,
) -> Result<Vec<PresenceChild>, PresenceError> {
    let vnode = children?;
    let mut normalized = Vec::new();
    collect_presence_children(vnode, &mut normalized)?;
    validate_keys(&normalized)?;
    Ok(normalized)
}

fn collect_presence_children(
    vnode: VNode,
    normalized: &mut Vec<PresenceChild>,
) -> Result<(), PresenceError> {
    if is_empty_presence_placeholder(&vnode) {
        return Ok(());
    }

    if vnode.template.roots.len() == 1 {
        if let Some(DynamicNode::Fragment(children)) = vnode.dynamic_root(0) {
            for child in children {
                if !is_empty_presence_placeholder(child) {
                    normalized.push(normalize_single_child(child.clone())?);
                }
            }
            return Ok(());
        }
    } else {
        return Err(PresenceError::UnsupportedMultipleStaticRoots);
    }

    normalized.push(normalize_single_child(vnode)?);
    Ok(())
}

fn is_empty_presence_placeholder(vnode: &VNode) -> bool {
    vnode.key.is_none()
        && vnode.template.roots.len() == 1
        && matches!(vnode.dynamic_root(0), Some(DynamicNode::Placeholder(_)))
}

fn normalize_single_child(vnode: VNode) -> Result<PresenceChild, PresenceError> {
    let Some(key) = vnode.key.clone() else {
        return Err(PresenceError::MissingKey);
    };

    Ok(PresenceChild { key, vnode })
}

fn validate_keys(children: &[PresenceChild]) -> Result<(), PresenceError> {
    let mut keys = HashSet::new();
    for child in children {
        if !keys.insert(child.key.clone()) {
            return Err(PresenceError::DuplicateKey(child.key.clone()));
        }
    }
    Ok(())
}

/// Controls how entering and exiting presence children are sequenced.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PresenceMode {
    /// Entering and exiting children render at the same time.
    #[default]
    Sync,
    /// Entering children wait until exiting children have completed.
    Wait,
    /// Exiting children are popped from layout where platform APIs support it.
    PopLayout,
}

/// Horizontal anchor used when popping exiting children out of layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PresenceAnchorX {
    /// Keep the measured left edge stable.
    #[default]
    Left,
    /// Keep the measured right edge stable.
    Right,
}

/// Vertical anchor used when popping exiting children out of layout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PresenceAnchorY {
    /// Keep the measured top edge stable.
    #[default]
    Top,
    /// Keep the measured bottom edge stable.
    Bottom,
}

/// Controls optional layout animation for a presence style hook.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PresenceLayout {
    /// Do not wrap or animate layout size.
    #[default]
    None,
    /// Animate measured content width and height.
    Size,
}

pub use crate::animations::style::MotionStyle;

/// Initial, animate, exit, and transition config for a presence style animation.
#[derive(Clone)]
pub struct PresenceConfig {
    /// Style used when a child first enters and initial animations are enabled.
    pub initial: MotionStyle,
    /// Style animated to while the child is present.
    pub animate: MotionStyle,
    /// Style animated to when the child exits.
    pub exit: MotionStyle,
    /// Animation configuration used while entering or present.
    pub enter_transition: AnimationConfig,
    /// Animation configuration used while exiting.
    pub exit_transition: AnimationConfig,
    /// Optional layout animation owned by this presence style hook.
    pub layout: PresenceLayout,
    /// Optional animation configuration used specifically for layout changes.
    pub layout_transition: Option<AnimationConfig>,
}

impl PresenceConfig {
    /// Creates a presence style animation config with one transition for enter and exit.
    pub fn new(
        initial: MotionStyle,
        animate: MotionStyle,
        exit: MotionStyle,
        transition: AnimationConfig,
    ) -> Self {
        Self::with_transitions(initial, animate, exit, transition.clone(), transition)
    }

    /// Creates a presence style animation config with separate enter and exit transitions.
    pub fn with_transitions(
        initial: MotionStyle,
        animate: MotionStyle,
        exit: MotionStyle,
        enter_transition: AnimationConfig,
        exit_transition: AnimationConfig,
    ) -> Self {
        Self {
            initial,
            animate,
            exit,
            enter_transition,
            exit_transition,
            layout: PresenceLayout::None,
            layout_transition: None,
        }
    }

    /// Enables layout animation for this presence style hook.
    pub fn with_layout(mut self, layout: PresenceLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets a layout-specific transition, matching Framer's transition.layout model.
    pub fn with_layout_transition(mut self, transition: AnimationConfig) -> Self {
        self.layout_transition = Some(transition);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PresenceRecordState {
    Present,
    Exiting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PresenceToken {
    subscriber_id: u64,
    generation: u64,
}

#[derive(Debug, Clone)]
struct PresenceRecord {
    child: PresenceChild,
    state: PresenceRecordState,
    generation: u64,
    last_order: usize,
    initial_allowed: bool,
    layout_enter_allowed: bool,
    enter_on_next_reconcile: bool,
    custom: Option<PresenceCustom>,
    subscribers: BTreeMap<u64, PresenceSubscriber>,
}

#[derive(Debug, Clone, Copy)]
struct PresenceSubscriber {
    completed: bool,
    scope_id: Option<ScopeId>,
}

impl PresenceRecord {
    fn new(
        child: PresenceChild,
        last_order: usize,
        initial_allowed: bool,
        layout_enter_allowed: bool,
        custom: Option<PresenceCustom>,
    ) -> Self {
        Self {
            child,
            state: PresenceRecordState::Present,
            generation: 0,
            last_order,
            initial_allowed,
            layout_enter_allowed,
            enter_on_next_reconcile: false,
            custom,
            subscribers: BTreeMap::new(),
        }
    }

    fn can_remove(&self) -> bool {
        self.state == PresenceRecordState::Exiting
            && self
                .subscribers
                .values()
                .all(|subscriber| subscriber.completed)
    }

    fn subscriber_scopes(&self) -> Vec<ScopeId> {
        self.subscribers
            .values()
            .filter_map(|subscriber| subscriber.scope_id)
            .collect()
    }
}

/// Parent-owned lifecycle state for retained presence children.
#[derive(Debug, Default)]
struct PresenceStateMachine {
    records: BTreeMap<String, PresenceRecord>,
    next_subscriber_id: u64,
    pending_wait_children: Option<PendingWaitChildren>,
    exit_completed_since_last_reconcile: bool,
    has_rendered: bool,
}

#[derive(Debug, Clone)]
struct PendingWaitChildren {
    children: Vec<PresenceChild>,
    custom: Option<PresenceCustom>,
}

impl PresenceStateMachine {
    fn reconcile(
        &mut self,
        children: Vec<PresenceChild>,
        mode: PresenceMode,
        initial: bool,
        custom: Option<PresenceCustom>,
    ) -> Result<PresenceRender, PresenceError> {
        validate_keys(&children)?;
        let layout_enter_allowed = self.has_rendered;
        let initial_allowed = self.has_rendered || initial;
        self.has_rendered = true;

        let incoming_keys = children
            .iter()
            .map(|child| child.key.clone())
            .collect::<BTreeSet<_>>();
        let mut notify_scopes = Vec::new();
        let should_wait = mode == PresenceMode::Wait
            && (self.has_exiting_children()
                || self.records.iter().any(|(key, record)| {
                    record.state == PresenceRecordState::Present && !incoming_keys.contains(key)
                }));

        if should_wait {
            for (order, child) in children.iter().cloned().enumerate() {
                if let Some(record) = self.records.get_mut(&child.key) {
                    if record.state == PresenceRecordState::Exiting {
                        record.generation = record.generation.saturating_add(1);
                        record.subscribers.clear();
                    }
                    record.child = child;
                    record.state = PresenceRecordState::Present;
                    record.last_order = order;
                    record.initial_allowed = false;
                    record.layout_enter_allowed = false;
                    record.custom = custom.clone();
                }
            }

            let tracked_keys = self.records.keys().cloned().collect::<Vec<_>>();
            for key in tracked_keys {
                if !incoming_keys.contains(&key) {
                    if let Some(record) = self.records.get_mut(&key) {
                        if record.state == PresenceRecordState::Present {
                            notify_scopes.extend(record.subscriber_scopes());
                        }
                        record.state = PresenceRecordState::Exiting;
                        record.custom = custom.clone();
                    }
                }
            }

            self.pending_wait_children = Some(PendingWaitChildren {
                children,
                custom: custom.clone(),
            });
            let removed_during_reconcile = self.remove_completed_exits();
            let exit_completed =
                removed_during_reconcile || self.exit_completed_since_last_reconcile;
            self.exit_completed_since_last_reconcile = false;

            return Ok(PresenceRender {
                children: self.render_children(false, mode),
                exit_completed,
                notify_scopes,
            });
        }

        for (order, child) in children.iter().cloned().enumerate() {
            if let Some(record) = self.records.get_mut(&child.key) {
                let preserve_enter = record.enter_on_next_reconcile;
                if record.state == PresenceRecordState::Exiting {
                    record.generation = record.generation.saturating_add(1);
                    record.subscribers.clear();
                }
                record.child = child;
                record.state = PresenceRecordState::Present;
                record.last_order = order;
                record.initial_allowed = preserve_enter && record.initial_allowed;
                record.layout_enter_allowed = preserve_enter && record.layout_enter_allowed;
                record.enter_on_next_reconcile = false;
                record.custom = custom.clone();
            } else {
                self.records.insert(
                    child.key.clone(),
                    PresenceRecord::new(
                        child,
                        order,
                        initial_allowed,
                        layout_enter_allowed,
                        custom.clone(),
                    ),
                );
            }
        }

        let tracked_keys = self.records.keys().cloned().collect::<Vec<_>>();
        for key in tracked_keys {
            if !incoming_keys.contains(&key) {
                if let Some(record) = self.records.get_mut(&key) {
                    if record.state == PresenceRecordState::Present {
                        notify_scopes.extend(record.subscriber_scopes());
                    }
                    record.state = PresenceRecordState::Exiting;
                    record.custom = custom.clone();
                }
            }
        }

        let removed_during_reconcile = self.remove_completed_exits();
        let exit_completed = removed_during_reconcile || self.exit_completed_since_last_reconcile;
        self.exit_completed_since_last_reconcile = false;

        self.pending_wait_children = None;
        Ok(PresenceRender {
            children: self.render_children(false, mode),
            exit_completed,
            notify_scopes,
        })
    }

    fn register(
        &mut self,
        key: &str,
        scope_id: Option<ScopeId>,
    ) -> Result<PresenceToken, PresenceError> {
        let Some(record) = self.records.get_mut(key) else {
            return Err(PresenceError::UnknownKey(key.to_string()));
        };

        let subscriber_id = self.next_subscriber_id;
        self.next_subscriber_id = self.next_subscriber_id.saturating_add(1);
        record.subscribers.insert(
            subscriber_id,
            PresenceSubscriber {
                completed: false,
                scope_id,
            },
        );

        Ok(PresenceToken {
            subscriber_id,
            generation: record.generation,
        })
    }

    fn complete(&mut self, key: &str, token: PresenceToken) {
        if let Some(record) = self.records.get_mut(key) {
            if record.generation == token.generation {
                if let Some(subscriber) = record.subscribers.get_mut(&token.subscriber_id) {
                    subscriber.completed = true;
                }
            }
        }
        if self.remove_completed_exits() {
            self.exit_completed_since_last_reconcile = true;
        }
    }

    fn unregister(&mut self, key: &str, token: PresenceToken) {
        if let Some(record) = self.records.get_mut(key) {
            if record.generation == token.generation {
                record.subscribers.remove(&token.subscriber_id);
            }
        }
        if self.remove_completed_exits() {
            self.exit_completed_since_last_reconcile = true;
        }
    }

    fn has_exiting_children(&self) -> bool {
        self.records
            .values()
            .any(|record| record.state == PresenceRecordState::Exiting)
    }

    fn remove_completed_exits(&mut self) -> bool {
        let removable = self
            .records
            .iter()
            .filter(|(_, record)| record.can_remove())
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>();
        let removed_any = !removable.is_empty();

        for key in removable {
            self.records.remove(&key);
        }

        let completed_all_exits = removed_any && !self.has_exiting_children();

        if !self.has_exiting_children() {
            if let Some(pending) = self.pending_wait_children.take() {
                for (order, child) in pending.children.into_iter().enumerate() {
                    if let Some(record) = self.records.get_mut(&child.key) {
                        record.child = child;
                        record.state = PresenceRecordState::Present;
                        record.last_order = order;
                        record.initial_allowed = true;
                        record.layout_enter_allowed = true;
                        record.enter_on_next_reconcile = true;
                        record.custom = pending.custom.clone();
                    } else {
                        let mut record =
                            PresenceRecord::new(child, order, true, true, pending.custom.clone());
                        record.enter_on_next_reconcile = true;
                        self.records.insert(record.child.key.clone(), record);
                    }
                }
            }
        }

        completed_all_exits
    }

    fn render_children(
        &self,
        exiting_only: bool,
        mode: PresenceMode,
    ) -> Vec<RenderedPresenceChild> {
        let mut records = self.records.values().collect::<Vec<_>>();
        records.sort_by(|left, right| {
            left.last_order.cmp(&right.last_order).then_with(|| {
                presence_render_state_order(left.state)
                    .cmp(&presence_render_state_order(right.state))
            })
        });
        records
            .into_iter()
            .filter(|record| !exiting_only || record.state == PresenceRecordState::Exiting)
            .map(|record| RenderedPresenceChild {
                key: record.child.key.clone(),
                vnode: record.child.vnode.clone(),
                is_present: record.state == PresenceRecordState::Present,
                initial_allowed: record.initial_allowed,
                layout_enter_allowed: record.layout_enter_allowed,
                generation: record.generation,
                custom: record.custom.clone(),
                mode,
            })
            .collect()
    }
}

fn presence_render_state_order(state: PresenceRecordState) -> u8 {
    match state {
        PresenceRecordState::Exiting => 0,
        PresenceRecordState::Present => 1,
    }
}

#[derive(Clone, Copy, PartialEq)]
struct PresenceRoot {
    state: Signal<PresenceStateMachine>,
    scope_id: ScopeId,
}

#[derive(Clone, PartialEq)]
struct PresenceContext {
    root: PresenceRoot,
    key: String,
    status: Signal<PresenceChildStatus>,
    measured_size: Signal<Option<PresenceMeasuredSize>>,
    layouts: Signal<BTreeMap<u64, PresenceLayoutConfig>>,
    projection_root: PresenceProjectionRootHandle,
}

#[derive(Clone, PartialEq)]
struct PresenceChildStatus {
    is_present: bool,
    initial_allowed: bool,
    generation: u64,
    custom: Option<PresenceCustom>,
}

#[derive(Clone, Default)]
struct PresenceLayoutConfig {
    layout: PresenceLayout,
    enter_transition: Option<AnimationConfig>,
    exit_transition: Option<AnimationConfig>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PresenceProjectionSnapshot {
    left: f64,
    top: f64,
    width: f64,
    height: f64,
}

#[derive(Clone)]
struct PresenceProjectionRootHandle(Rc<RefCell<PresenceProjectionRoot>>);

impl PartialEq for PresenceProjectionRootHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl PresenceProjectionRootHandle {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(PresenceProjectionRoot::default())))
    }

    fn register(&self, key: String, mounted: Rc<MountedData>, transition: Duration) {
        let Ok(mut root) = self.0.try_borrow_mut() else {
            debug_presence_projection_skip("register", &key);
            return;
        };
        root.register(key, mounted, transition);
    }

    fn unregister(&self, key: &str) {
        let Ok(mut root) = self.0.try_borrow_mut() else {
            debug_presence_projection_skip("unregister", key);
            return;
        };
        root.unregister(key);
    }

    fn will_update(&self) {
        let Ok(mut root) = self.0.try_borrow_mut() else {
            debug_presence_projection_skip("will_update", "*");
            return;
        };
        root.will_update();
    }

    fn did_update(&self) {
        let Ok(mut root) = self.0.try_borrow_mut() else {
            debug_presence_projection_skip("did_update", "*");
            return;
        };
        root.did_update();
    }
}

#[derive(Default)]
struct PresenceProjectionRoot {
    nodes: BTreeMap<String, PresenceProjectionNode>,
}

struct PresenceProjectionNode {
    mounted: Rc<MountedData>,
    transition: Duration,
    snapshot: Option<PresenceProjectionSnapshot>,
}

#[cfg(feature = "web")]
const PROJECTION_ACTIVE_ATTR: &str = "data-dioxus-motion-projection-active";
#[cfg(feature = "web")]
const PROJECTION_PREVIOUS_TRANSFORM_ATTR: &str = "data-dioxus-motion-projection-previous-transform";
#[cfg(feature = "web")]
const PROJECTION_PREVIOUS_TRANSITION_ATTR: &str =
    "data-dioxus-motion-projection-previous-transition";

impl PresenceProjectionRoot {
    fn register(&mut self, key: String, mounted: Rc<MountedData>, transition: Duration) {
        if let Some(node) = self.nodes.get_mut(&key) {
            node.mounted = mounted;
            node.transition = transition;
        } else {
            self.nodes.insert(
                key,
                PresenceProjectionNode {
                    mounted,
                    transition,
                    snapshot: None,
                },
            );
        }
    }

    fn unregister(&mut self, key: &str) {
        self.nodes.remove(key);
    }

    #[cfg(feature = "web")]
    fn will_update(&mut self) {
        for (key, node) in self.nodes.iter_mut() {
            reset_projection_styles(&node.mounted);
            node.snapshot = measure_projection_snapshot(&node.mounted);
            if let Some(snapshot) = node.snapshot {
                debug_presence_projection(
                    "will_update",
                    key,
                    Some(snapshot),
                    None,
                    None,
                    "snapshot",
                );
            } else {
                debug_presence_projection("will_update", key, None, None, None, "unmounted");
            }
        }
    }

    #[cfg(not(feature = "web"))]
    fn will_update(&mut self) {}

    #[cfg(feature = "web")]
    fn did_update(&mut self) {
        for (key, node) in self.nodes.iter_mut() {
            let Some(previous) = node.snapshot.take() else {
                debug_presence_projection("did_update", key, None, None, None, "missing snapshot");
                continue;
            };
            let Some(next) = measure_projection_snapshot(&node.mounted) else {
                debug_presence_projection(
                    "did_update",
                    key,
                    Some(previous),
                    None,
                    None,
                    "unmounted",
                );
                continue;
            };
            apply_projection_animation(key, &node.mounted, previous, next, node.transition);
        }
    }

    #[cfg(not(feature = "web"))]
    fn did_update(&mut self) {}
}

#[cfg(feature = "web")]
fn projection_element(mounted: &Rc<MountedData>) -> Option<web_sys::HtmlElement> {
    use dioxus::web::WebEventExt;
    use wasm_bindgen::JsCast;

    mounted.as_ref().as_web_event().dyn_into().ok()
}

#[cfg(feature = "web")]
fn measure_projection_snapshot(mounted: &Rc<MountedData>) -> Option<PresenceProjectionSnapshot> {
    projection_element(mounted).map(|element| {
        let rect = element.get_bounding_client_rect();
        PresenceProjectionSnapshot {
            left: rect.left(),
            top: rect.top(),
            width: rect.width(),
            height: rect.height(),
        }
    })
}

#[cfg(feature = "web")]
fn reset_projection_styles(mounted: &Rc<MountedData>) {
    if let Some(element) = projection_element(mounted) {
        if element.get_attribute(PROJECTION_ACTIVE_ATTR).is_none() {
            return;
        }

        let style = element.style();
        match element.get_attribute(PROJECTION_PREVIOUS_TRANSFORM_ATTR) {
            Some(transform) if !transform.is_empty() => {
                let _ = style.set_property("transform", &transform);
            }
            _ => {
                let _ = style.remove_property("transform");
            }
        }
        let _ = style.remove_property("transform-origin");
        let _ = style.remove_property("will-change");
        if let Some(transition) = element.get_attribute(PROJECTION_PREVIOUS_TRANSITION_ATTR) {
            let _ = style.set_property("transition", &transition);
        }
        let _ = element.remove_attribute(PROJECTION_ACTIVE_ATTR);
        let _ = element.remove_attribute(PROJECTION_PREVIOUS_TRANSFORM_ATTR);
        let _ = element.remove_attribute(PROJECTION_PREVIOUS_TRANSITION_ATTR);
    }
}

#[cfg(feature = "web")]
fn apply_projection_animation(
    key: &str,
    mounted: &Rc<MountedData>,
    previous: PresenceProjectionSnapshot,
    next: PresenceProjectionSnapshot,
    transition: Duration,
) {
    let translate_x = previous.left - next.left;
    let translate_y = previous.top - next.top;
    let scale_x = if next.width.abs() > f64::EPSILON {
        previous.width / next.width
    } else {
        1.0
    };
    let scale_y = if next.height.abs() > f64::EPSILON {
        previous.height / next.height
    } else {
        1.0
    };
    let position_changed = translate_x.abs() > 0.5 || translate_y.abs() > 0.5;
    let size_changed = (scale_x - 1.0).abs() > 0.01 || (scale_y - 1.0).abs() > 0.01;
    if !position_changed && !size_changed {
        debug_presence_projection(
            "did_update",
            key,
            Some(previous),
            Some(next),
            None,
            "unchanged",
        );
        return;
    }

    let Some(element) = projection_element(mounted) else {
        debug_presence_projection(
            "did_update",
            key,
            Some(previous),
            Some(next),
            None,
            "unmounted",
        );
        return;
    };
    let style = element.style();
    let previous_transform = style.get_property_value("transform").unwrap_or_default();
    let previous_transition = style.get_property_value("transition").unwrap_or_default();
    let base_transform = normalize_base_transform(previous_transform.trim());
    let _ = element.set_attribute(PROJECTION_ACTIVE_ATTR, "true");
    if let Some(base_transform) = base_transform {
        let _ = element.set_attribute(PROJECTION_PREVIOUS_TRANSFORM_ATTR, base_transform);
    } else {
        let _ = element.remove_attribute(PROJECTION_PREVIOUS_TRANSFORM_ATTR);
    }
    if !previous_transition.trim().is_empty() {
        let _ = element.set_attribute(
            PROJECTION_PREVIOUS_TRANSITION_ATTR,
            previous_transition.trim(),
        );
    } else {
        let _ = element.remove_attribute(PROJECTION_PREVIOUS_TRANSITION_ATTR);
    }
    let _ = style.set_property("transform-origin", "0 0");
    let _ = style.set_property("will-change", "transform");
    let _ = style.set_property("transition", "none");
    let _ = style.set_property(
        "transform",
        &compose_projection_transform(translate_x, translate_y, scale_x, scale_y, base_transform),
    );

    debug_presence_projection(
        "did_update",
        key,
        Some(previous),
        Some(next),
        Some((translate_x, translate_y, scale_x, scale_y)),
        "animate",
    );
    schedule_projection_finish(element, transition);
}

#[cfg(feature = "web")]
fn schedule_projection_finish(element: web_sys::HtmlElement, transition: Duration) {
    use wasm_bindgen::{JsCast, closure::Closure};

    let callback = Closure::once(move || {
        let style = element.style();
        let previous_transition = element
            .get_attribute(PROJECTION_PREVIOUS_TRANSITION_ATTR)
            .unwrap_or_default();
        let previous_transform = element.get_attribute(PROJECTION_PREVIOUS_TRANSFORM_ATTR);
        let transition_value =
            compose_projection_transition(previous_transition.trim(), transition);
        let _ = style.set_property("transition", &transition_value);
        let _ = style.set_property(
            "transform",
            &compose_projection_identity_transform(previous_transform.as_deref()),
        );
    });

    if let Some(window) = web_sys::window() {
        let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
        callback.forget();
    }
}

#[cfg(feature = "web")]
fn normalize_base_transform(transform: &str) -> Option<&str> {
    if transform.is_empty() || transform == "none" {
        None
    } else {
        Some(transform)
    }
}

#[cfg(feature = "web")]
fn compose_projection_transform(
    translate_x: f64,
    translate_y: f64,
    scale_x: f64,
    scale_y: f64,
    base_transform: Option<&str>,
) -> String {
    let projection =
        format!("translate({translate_x}px, {translate_y}px) scale({scale_x}, {scale_y})");
    match base_transform {
        Some(base_transform) => format!("{projection} {base_transform}"),
        None => projection,
    }
}

#[cfg(feature = "web")]
fn compose_projection_identity_transform(base_transform: Option<&str>) -> String {
    base_transform
        .and_then(normalize_base_transform)
        .map_or_else(
            || "translate(0px, 0px) scale(1, 1)".to_string(),
            |base_transform| format!("translate(0px, 0px) scale(1, 1) {base_transform}"),
        )
}

#[cfg(feature = "web")]
fn compose_projection_transition(previous_transition: &str, transition: Duration) -> String {
    let transform_transition = format!("transform {}ms ease", transition.as_millis());
    if previous_transition.is_empty() || previous_transition == "none" {
        return transform_transition;
    }

    let retained = previous_transition
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .filter(|part| !part.starts_with("transform "))
        .collect::<Vec<_>>();

    if retained.is_empty() {
        transform_transition
    } else {
        format!("{}, {transform_transition}", retained.join(", "))
    }
}

#[cfg(feature = "web")]
fn debug_presence_projection(
    phase: &str,
    key: &str,
    previous: Option<PresenceProjectionSnapshot>,
    next: Option<PresenceProjectionSnapshot>,
    delta: Option<(f64, f64, f64, f64)>,
    message: &str,
) {
    if !cfg!(debug_assertions) {
        return;
    }

    use wasm_bindgen::JsValue;

    web_sys::console::debug_1(&JsValue::from_str(&format!(
        "[dioxus-motion presence layout] phase={phase} key={key} message={message} previous={} next={} delta={}",
        format_projection_snapshot(previous),
        format_projection_snapshot(next),
        format_projection_delta(delta),
    )));
}

#[cfg(feature = "web")]
fn debug_presence_projection_skip(phase: &str, key: &str) {
    if !cfg!(debug_assertions) {
        return;
    }

    use wasm_bindgen::JsValue;

    web_sys::console::debug_1(&JsValue::from_str(&format!(
        "[dioxus-motion presence layout] phase={phase} key={key} message=busy"
    )));
}

#[cfg(not(feature = "web"))]
fn debug_presence_projection_skip(_phase: &str, _key: &str) {}

#[cfg(feature = "web")]
fn format_projection_snapshot(snapshot: Option<PresenceProjectionSnapshot>) -> String {
    snapshot.map_or_else(
        || "none".to_string(),
        |snapshot| {
            format!(
                "left:{:.2},top:{:.2},width:{:.2},height:{:.2}",
                snapshot.left, snapshot.top, snapshot.width, snapshot.height
            )
        },
    )
}

#[cfg(feature = "web")]
fn format_projection_delta(delta: Option<(f64, f64, f64, f64)>) -> String {
    delta.map_or_else(
        || "none".to_string(),
        |(x, y, scale_x, scale_y)| {
            format!("x:{x:.2},y:{y:.2},scaleX:{scale_x:.4},scaleY:{scale_y:.4}")
        },
    )
}

/// A handle returned by [`use_presence`] for manual exit orchestration.
#[derive(Clone)]
pub struct PresenceHandle {
    /// Whether the nearest presence boundary still considers this child present.
    pub is_present: bool,
    /// Marks the child as safe to remove after its exit work has completed.
    pub safe_to_remove: Callback<()>,
}

/// Render keyed children while allowing removed children to finish exit work.
#[component]
pub fn AnimatePresence(
    children: Element,
    #[props(default)] mode: PresenceMode,
    #[props(default)] anchor_x: PresenceAnchorX,
    #[props(default)] anchor_y: PresenceAnchorY,
    #[props(default = true)] initial: bool,
    #[props(default)] propagate: bool,
    #[props(default)] custom: Option<PresenceCustom>,
    #[props(default)] on_exit_complete: Option<EventHandler<()>>,
) -> Element {
    let parent_presence = use_presence_subscription(propagate);
    let mut state = use_signal(PresenceStateMachine::default);
    let projection_root = use_hook(PresenceProjectionRootHandle::new);
    projection_root.will_update();

    let root = PresenceRoot {
        state,
        scope_id: current_scope_id(),
    };
    let mut normalized = match normalize_presence_children(children) {
        Ok(children) => children,
        Err(error) => {
            tracing::error!("{error}");
            return VNode::empty();
        }
    };
    let propagated_exit = propagate && !parent_presence.is_present;
    if propagated_exit {
        normalized.clear();
    }
    if mode == PresenceMode::Wait && normalized.len() > 1 {
        tracing::warn!(
            "AnimatePresence wait mode is intended for a single child; received {} keyed children",
            normalized.len()
        );
    }

    let rendered = match state.write().reconcile(normalized, mode, initial, custom) {
        Ok(rendered) => rendered,
        Err(error) => {
            tracing::error!("{error}");
            return VNode::empty();
        }
    };

    for scope_id in rendered.notify_scopes.iter().copied() {
        needs_update_any(scope_id);
    }

    if rendered.exit_completed {
        if let Some(handler) = on_exit_complete {
            handler.call(());
        }
        if propagated_exit {
            parent_presence.safe_to_remove.call(());
        }
    } else if propagated_exit && rendered.children.is_empty() {
        parent_presence.safe_to_remove.call(());
    }

    let children = rsx! {
        for child in rendered.children {
                PresenceChildBoundary {
                    key: "{child.key}",
                    root,
                    presence_key: child.key.clone(),
                    is_present: child.is_present,
                    initial_allowed: child.initial_allowed,
                    layout_enter_allowed: child.layout_enter_allowed,
                    generation: child.generation,
                    custom: child.custom.clone(),
                    mode: child.mode,
                    projection_root: projection_root.clone(),
                    anchor_x,
                    anchor_y,
                    child: child.vnode
                }
        }
    };

    use_effect(move || {
        projection_root.did_update();
    });

    children
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PresenceMeasuredSize {
    width: f64,
    height: f64,
}

async fn measure_presence_layout_size(
    mounted: Rc<MountedData>,
) -> Result<PresenceMeasuredSize, MountedError> {
    let rect = mounted.get_client_rect().await?;
    Ok(PresenceMeasuredSize {
        width: rect.size.width,
        height: rect.size.height,
    })
}

async fn measure_presence_layout_size_stable(
    mounted: Rc<MountedData>,
) -> Result<PresenceMeasuredSize, MountedError> {
    let _ = measure_presence_layout_size(mounted.clone()).await?;
    Time::delay(Duration::from_millis(16)).await;
    measure_presence_layout_size(mounted).await
}

fn schedule_presence_layout_measurement(
    mounted: Rc<MountedData>,
    layout_enter_allowed: bool,
    layouts: Signal<BTreeMap<u64, PresenceLayoutConfig>>,
    mut measured_size: Signal<Option<PresenceMeasuredSize>>,
    mut expand_enter: Signal<bool>,
    mut settle_enter: Signal<bool>,
) {
    spawn(async move {
        if let Ok(size) = measure_presence_layout_size_stable(mounted).await {
            measured_size.set(Some(size));
            if layout_enter_allowed {
                let duration = resolve_presence_layout_config(&layouts.read(), true)
                    .enter_transition
                    .as_ref()
                    .map(AnimationConfig::get_duration)
                    .unwrap_or_default();
                Time::delay(Duration::from_millis(16)).await;
                expand_enter.set(true);
                Time::delay(duration).await;
                settle_enter.set(true);
            }
        }
    });
}

#[component]
fn PresenceChildBoundary(
    root: PresenceRoot,
    presence_key: String,
    is_present: bool,
    initial_allowed: bool,
    layout_enter_allowed: bool,
    generation: u64,
    custom: Option<PresenceCustom>,
    mode: PresenceMode,
    projection_root: PresenceProjectionRootHandle,
    anchor_x: PresenceAnchorX,
    anchor_y: PresenceAnchorY,
    child: VNode,
) -> Element {
    let mut status = use_signal(|| PresenceChildStatus {
        is_present,
        initial_allowed,
        generation,
        custom: custom.clone(),
    });
    let next_status = PresenceChildStatus {
        is_present,
        initial_allowed,
        generation,
        custom,
    };
    if *status.read() != next_status {
        status.set(next_status);
    }
    let measured_size = use_signal(|| None::<PresenceMeasuredSize>);
    let layouts = use_signal(BTreeMap::<u64, PresenceLayoutConfig>::new);
    use_context_provider(|| PresenceContext {
        root,
        key: presence_key.clone(),
        status,
        measured_size,
        layouts,
        projection_root: projection_root.clone(),
    });

    let child = if mode == PresenceMode::PopLayout {
        rsx! { PopLayoutBoundary { is_present, anchor_x, anchor_y, measured_size, child } }
    } else {
        rsx! { {child} }
    };

    rsx! {
        PresenceLayoutChildBoundary {
            layouts,
            is_present,
            initial_allowed,
            layout_enter_allowed,
            pop_layout: mode == PresenceMode::PopLayout,
            presence_key: presence_key.clone(),
            projection_root,
            measured_size,
            child
        }
    }
}

#[component]
fn PresenceLayoutChildBoundary(
    layouts: Signal<BTreeMap<u64, PresenceLayoutConfig>>,
    is_present: bool,
    initial_allowed: bool,
    layout_enter_allowed: bool,
    pop_layout: bool,
    presence_key: String,
    projection_root: PresenceProjectionRootHandle,
    measured_size: Signal<Option<PresenceMeasuredSize>>,
    child: Element,
) -> Element {
    let mut content_node = use_signal(|| None::<Rc<MountedData>>);
    let mut wrapper_node = use_signal(|| None::<Rc<MountedData>>);
    let mut collapse_exit = use_signal(|| true);
    let mut expand_enter = use_signal(|| !layout_enter_allowed);
    let mut settle_enter = use_signal(|| !layout_enter_allowed);
    let mut last_layout_present = use_signal(|| is_present);
    let mut collapse_exit_value = *collapse_exit.read();
    let expand_enter_value = *expand_enter.read();
    let settle_enter_value = *settle_enter.read();

    if *last_layout_present.read() != is_present {
        last_layout_present.set(is_present);
        if is_present {
            collapse_exit.set(true);
            collapse_exit_value = true;
            expand_enter.set(!layout_enter_allowed);
            settle_enter.set(!layout_enter_allowed);
        } else {
            collapse_exit.set(false);
            collapse_exit_value = false;
        }
    }

    use_effect(use_reactive((&is_present,), move |_| {
        if !is_present
            || resolve_presence_layout_config(&layouts.read(), true).layout != PresenceLayout::Size
        {
            return;
        }
        if let Some(mounted) = content_node.read().clone() {
            schedule_presence_layout_measurement(
                mounted,
                layout_enter_allowed,
                layouts,
                measured_size,
                expand_enter,
                settle_enter,
            );
        }
    }));

    use_effect(use_reactive((&is_present,), move |_| {
        if !is_present {
            spawn(async move {
                Time::delay(Duration::from_millis(16)).await;
                collapse_exit.set(true);
            });
        }
    }));

    let measured_size_value = measured_size.read().as_ref().copied();
    let pop_layout_ready = pop_layout && measured_size_value.is_some();
    let style = presence_layout_child_style(
        &resolve_presence_layout_config(&layouts.read(), is_present),
        measured_size_value,
        PresenceLayoutChildStyleState {
            is_present,
            initial_allowed,
            layout_enter_allowed,
            expand_enter: expand_enter_value,
            settle_enter: settle_enter_value,
            pop_layout_ready,
            collapse_exit: collapse_exit_value && (!pop_layout || pop_layout_ready),
        },
    );
    use_effect({
        let presence_key = presence_key.clone();
        let projection_root = projection_root.clone();
        move || {
            let Some(mounted) = wrapper_node.read().clone() else {
                return;
            };

            let layout_config = resolve_presence_layout_config(&layouts.read(), is_present);
            let projection_transition = layout_config
                .enter_transition
                .as_ref()
                .map(AnimationConfig::get_duration)
                .unwrap_or_default();

            if layout_config.layout == PresenceLayout::Size && is_present {
                projection_root.register(presence_key.clone(), mounted, projection_transition);
            } else {
                projection_root.unregister(&presence_key);
            }
        }
    });

    use_drop({
        let presence_key = presence_key.clone();
        let projection_root = projection_root.clone();
        move || {
            projection_root.unregister(&presence_key);
        }
    });

    rsx! {
        div {
            "data-dioxus-motion-layout-slot": "true",
            style,
            onmounted: move |event| {
                wrapper_node.set(Some(event.data.clone()));
            },
            div {
                "data-dioxus-motion-layout-content": "true",
                onmounted: move |event| {
                    let mounted = event.data.clone();
                    content_node.set(Some(mounted.clone()));
                    schedule_presence_layout_measurement(
                        mounted,
                        layout_enter_allowed,
                        layouts,
                        measured_size,
                        expand_enter,
                        settle_enter,
                    );
                },
                {child}
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
struct PresenceLayoutChildStyleState {
    is_present: bool,
    initial_allowed: bool,
    layout_enter_allowed: bool,
    expand_enter: bool,
    settle_enter: bool,
    pop_layout_ready: bool,
    collapse_exit: bool,
}

fn presence_layout_child_style(
    config: &PresenceLayoutConfig,
    measured_size: Option<PresenceMeasuredSize>,
    state: PresenceLayoutChildStyleState,
) -> String {
    match config.layout {
        PresenceLayout::None => String::new(),
        PresenceLayout::Size => {
            // This is slot-size animation, not full Framer-style layout projection.
            // Projection transforms are applied to the owned slot wrapper separately.
            let transition_config = if state.is_present {
                config.enter_transition.as_ref()
            } else {
                config.exit_transition.as_ref()
            };
            let duration = transition_config
                .map(AnimationConfig::get_duration)
                .unwrap_or_default();
            let transition = format!(
                "transition: width {}ms ease, height {}ms ease;",
                duration.as_millis(),
                duration.as_millis()
            );
            let base =
                "box-sizing: border-box; flex: 0 0 auto; max-width: 100%; overflow: visible;";
            let clipped_base =
                "box-sizing: border-box; flex: 0 0 auto; max-width: 100%; overflow: hidden;";
            match (measured_size, state.is_present) {
                (Some(_), true) if state.layout_enter_allowed && state.settle_enter => {
                    format!("{base} width: auto; height: auto; {transition}")
                }
                (Some(size), true) if state.layout_enter_allowed && state.expand_enter => format!(
                    "{clipped_base} width: min({}px, 100%); height: {}px; {}",
                    size.width, size.height, transition
                ),
                (Some(_), true) if state.layout_enter_allowed => {
                    format!("{clipped_base} height: 0px; {transition}")
                }
                (Some(_), true) => format!("{base} {transition}"),
                (Some(_), false) if state.pop_layout_ready && state.collapse_exit => {
                    format!("{base} width: 0px; height: 0px; {transition}")
                }
                (Some(size), false) if state.collapse_exit => format!(
                    "{base} width: min({}px, 100%); height: 0px; {}",
                    size.width, transition
                ),
                (Some(size), false) => format!(
                    "{base} width: min({}px, 100%); height: {}px; {}",
                    size.width, size.height, transition
                ),
                (None, true) if state.layout_enter_allowed => {
                    format!("{clipped_base} height: 0px; {transition}")
                }
                (None, true) if state.initial_allowed => format!("{base} {transition}"),
                (None, true) => format!("{base} {transition}"),
                (None, false) if state.collapse_exit => {
                    format!("{base} width: fit-content; height: 0px; {}", transition)
                }
                (None, false) => format!("{base} width: fit-content; {transition}"),
            }
        }
    }
}

fn resolve_presence_layout_config(
    configs: &BTreeMap<u64, PresenceLayoutConfig>,
    is_present: bool,
) -> PresenceLayoutConfig {
    let mut resolved = PresenceLayoutConfig::default();

    for config in configs.values() {
        if config.layout != PresenceLayout::Size {
            continue;
        }

        resolved.layout = PresenceLayout::Size;
        let candidate = if is_present {
            config.enter_transition.clone()
        } else {
            config.exit_transition.clone()
        };
        let target = if is_present {
            &mut resolved.enter_transition
        } else {
            &mut resolved.exit_transition
        };

        if target
            .as_ref()
            .is_none_or(|current| candidate_duration(&candidate) > current.get_duration())
        {
            *target = candidate;
        }
    }

    resolved
}

fn candidate_duration(config: &Option<AnimationConfig>) -> Duration {
    config
        .as_ref()
        .map(AnimationConfig::get_duration)
        .unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LayoutSnapshot {
    offset_left: f64,
    offset_top: f64,
    width: f64,
    height: f64,
    parent_width: f64,
    parent_height: f64,
    parent_positioned: bool,
}

#[cfg(feature = "web")]
#[derive(Clone)]
struct ParentStyleMutation {
    element: web_sys::HtmlElement,
    previous_position: String,
}

impl LayoutSnapshot {
    #[cfg(not(feature = "web"))]
    fn viewport(offset_left: f64, offset_top: f64, width: f64, height: f64) -> Self {
        Self {
            offset_left,
            offset_top,
            width,
            height,
            parent_width: web_viewport_width(),
            parent_height: web_viewport_height(),
            parent_positioned: true,
        }
    }
}

#[cfg(feature = "web")]
fn measure_pop_layout_snapshot(
    mounted: Rc<MountedData>,
    parent_style_mutation: Rc<RefCell<Option<ParentStyleMutation>>>,
) -> Option<LayoutSnapshot> {
    use dioxus::web::WebEventExt;
    use wasm_bindgen::JsCast;

    let element = mounted.as_ref().as_web_event();
    let rect = element.get_bounding_client_rect();
    element.parent_element().map_or(
        Some(LayoutSnapshot {
            offset_left: rect.left(),
            offset_top: rect.top(),
            width: rect.width(),
            height: rect.height(),
            parent_width: web_viewport_width(),
            parent_height: web_viewport_height(),
            parent_positioned: true,
        }),
        |parent_element| {
            let parent_rect = parent_element.get_bounding_client_rect();
            let parent_positioned = web_sys::window()
                .and_then(|window| window.get_computed_style(&parent_element).ok().flatten())
                .map(|style| {
                    let position = style
                        .get_property_value("position")
                        .unwrap_or_else(|_| "static".to_string());
                    position != "static"
                })
                .unwrap_or(true);
            if !parent_positioned {
                if let Some(parent_html) = parent_element.dyn_ref::<web_sys::HtmlElement>() {
                    let previous_position = parent_html
                        .style()
                        .get_property_value("position")
                        .unwrap_or_default();
                    let _ = parent_html.style().set_property("position", "relative");
                    if parent_style_mutation.borrow().is_none() {
                        parent_style_mutation
                            .borrow_mut()
                            .replace(ParentStyleMutation {
                                element: parent_html.clone(),
                                previous_position,
                            });
                    }
                }
            }

            Some(LayoutSnapshot {
                offset_left: rect.left() - parent_rect.left(),
                offset_top: rect.top() - parent_rect.top(),
                width: rect.width(),
                height: rect.height(),
                parent_width: parent_rect.width(),
                parent_height: parent_rect.height(),
                parent_positioned,
            })
        },
    )
}

#[cfg(not(feature = "web"))]
async fn measure_pop_layout_snapshot(mounted: Rc<MountedData>) -> Option<LayoutSnapshot> {
    mounted.get_client_rect().await.ok().map(|rect| {
        LayoutSnapshot::viewport(
            rect.origin.x,
            rect.origin.y,
            rect.size.width,
            rect.size.height,
        )
    })
}

#[cfg(feature = "web")]
fn web_viewport_width() -> f64 {
    web_sys::window()
        .and_then(|window| window.inner_width().ok())
        .and_then(|width| width.as_f64())
        .unwrap_or(0.0)
}

#[cfg(not(feature = "web"))]
fn web_viewport_width() -> f64 {
    0.0
}

#[cfg(feature = "web")]
fn web_viewport_height() -> f64 {
    web_sys::window()
        .and_then(|window| window.inner_height().ok())
        .and_then(|height| height.as_f64())
        .unwrap_or(0.0)
}

#[cfg(not(feature = "web"))]
fn web_viewport_height() -> f64 {
    0.0
}

#[component]
fn PopLayoutBoundary(
    is_present: bool,
    anchor_x: PresenceAnchorX,
    anchor_y: PresenceAnchorY,
    measured_size: Signal<Option<PresenceMeasuredSize>>,
    child: VNode,
) -> Element {
    static NEXT_POP_LAYOUT_ID: AtomicU64 = AtomicU64::new(1);

    let pop_id = use_hook(|| {
        format!(
            "dioxus-motion-pop-{}",
            NEXT_POP_LAYOUT_ID.fetch_add(1, Ordering::Relaxed)
        )
    });
    let mut content_node = use_signal(|| None::<Rc<MountedData>>);
    let mut snapshot = use_signal(|| None::<LayoutSnapshot>);
    let mut last_present = use_signal(|| is_present);
    #[cfg(feature = "web")]
    let parent_style_mutation = use_hook(|| Rc::new(RefCell::new(None::<ParentStyleMutation>)));
    #[cfg(feature = "web")]
    use_drop({
        let parent_style_mutation = parent_style_mutation.clone();
        move || {
            if let Some(mutation) = parent_style_mutation.borrow_mut().take() {
                if mutation.previous_position.is_empty() {
                    let _ = mutation.element.style().remove_property("position");
                } else {
                    let _ = mutation
                        .element
                        .style()
                        .set_property("position", &mutation.previous_position);
                }
            }
        }
    });
    #[cfg(feature = "web")]
    let effect_parent_style_mutation = parent_style_mutation.clone();
    #[cfg(feature = "web")]
    let mounted_parent_style_mutation = parent_style_mutation.clone();

    if *last_present.read() != is_present {
        last_present.set(is_present);
        if is_present {
            snapshot.set(None);
        }
    }

    use_effect(move || {
        if !is_present {
            return;
        }
        if let Some(mounted) = content_node.read().clone() {
            #[cfg(feature = "web")]
            {
                if let Some(measured) =
                    measure_pop_layout_snapshot(mounted, effect_parent_style_mutation.clone())
                {
                    snapshot.set(Some(measured));
                }
            }
            #[cfg(not(feature = "web"))]
            spawn(async move {
                if let Some(measured) = measure_pop_layout_snapshot(mounted).await {
                    snapshot.set(Some(measured));
                }
            });
        }
    });

    let pop_style = pop_layout_wrapper_style(
        snapshot.read().as_ref().copied(),
        measured_size.read().as_ref().copied(),
        is_present,
        anchor_x,
        anchor_y,
    );

    rsx! {
        div {
            id: "{pop_id}",
            style: "{pop_style}",
            onmounted: move |event| {
                let mounted = event.data.clone();
                content_node.set(Some(mounted.clone()));
                #[cfg(feature = "web")]
                {
                    if let Some(measured) =
                        measure_pop_layout_snapshot(mounted, mounted_parent_style_mutation.clone())
                    {
                        snapshot.set(Some(measured));
                    }
                }
                #[cfg(not(feature = "web"))]
                spawn(async move {
                    if let Some(measured) = measure_pop_layout_snapshot(mounted).await {
                        snapshot.set(Some(measured));
                    }
                });
            },
            {child}
        }
    }
}

fn pop_layout_wrapper_style(
    snapshot: Option<LayoutSnapshot>,
    measured_size: Option<PresenceMeasuredSize>,
    is_present: bool,
    anchor_x: PresenceAnchorX,
    anchor_y: PresenceAnchorY,
) -> String {
    let declarations = pop_layout_style(snapshot, measured_size, is_present, anchor_x, anchor_y);
    if declarations.is_empty() {
        "display: inline-block;".to_string()
    } else {
        format!("display: inline-block; {declarations}")
    }
}

fn pop_layout_style(
    snapshot: Option<LayoutSnapshot>,
    measured_size: Option<PresenceMeasuredSize>,
    is_present: bool,
    anchor_x: PresenceAnchorX,
    anchor_y: PresenceAnchorY,
) -> String {
    if is_present {
        return String::new();
    }

    let Some(rect) = snapshot else {
        return String::new();
    };

    let width = measured_size.map_or(rect.width, |size| size.width);
    let height = measured_size.map_or(rect.height, |size| size.height);

    let x = match anchor_x {
        PresenceAnchorX::Left => format!("left: {}px;", rect.offset_left),
        PresenceAnchorX::Right => {
            format!("right: {}px;", rect.parent_width - rect.offset_left - width)
        }
    };
    let y = match anchor_y {
        PresenceAnchorY::Top => format!("top: {}px;", rect.offset_top),
        PresenceAnchorY::Bottom => format!(
            "bottom: {}px;",
            rect.parent_height - rect.offset_top - height
        ),
    };
    format!(
        "position: absolute; box-sizing: border-box; width: {}px; height: {}px; {} {}",
        width, height, x, y
    )
}

/// Returns true while the nearest presence child is still present.
pub fn use_is_present() -> bool {
    try_consume_context::<PresenceContext>()
        .map(|context| context.status.read().is_present)
        .unwrap_or(true)
}

/// Returns custom data captured from the nearest [`AnimatePresence`] boundary.
pub fn use_presence_data<T: Clone + 'static>() -> Option<T> {
    try_consume_context::<PresenceContext>()
        .and_then(|context| context.status.read().custom.clone())
        .and_then(|custom| custom.get::<T>())
}

/// Register the current component for manual presence removal.
pub fn use_presence() -> PresenceHandle {
    use_presence_subscription(true)
}

fn use_presence_subscription(enabled: bool) -> PresenceHandle {
    let context = try_consume_context::<PresenceContext>();
    let token = use_hook({
        let context = context.clone();
        move || {
            if !enabled {
                return None;
            }
            context.and_then(|context| {
                let mut state = context.root.state;
                let token = state
                    .write()
                    .register(&context.key, Some(current_scope_id()))
                    .ok();
                token.map(|token| (context, token))
            })
        }
    });

    use_drop({
        let token = token.clone();
        move || {
            if let Some((context, token)) = token {
                let mut state = context.root.state;
                state.write().unregister(&context.key, token);
                needs_update_any(context.root.scope_id);
            }
        }
    });

    let is_present = context
        .as_ref()
        .map(|context| context.status.read().is_present)
        .unwrap_or(true);
    let safe_to_remove = Callback::new(move |_| {
        if let Some((context, token)) = token.clone() {
            let mut state = context.root.state;
            state.write().complete(&context.key, token);
            needs_update_any(context.root.scope_id);
        }
    });

    PresenceHandle {
        is_present,
        safe_to_remove,
    }
}

/// Creates a motion handle that follows presence enter and exit state.
///
/// The handle animates to `animate` while present and to `exit` when the nearest
/// presence boundary starts exiting. Once the exit animation settles, it calls
/// `safe_to_remove`.
pub fn use_presence_motion<T>(
    initial: T,
    animate: T,
    exit: T,
    config: AnimationConfig,
) -> MotionHandle<T>
where
    T: Animatable + Send + 'static,
{
    use_presence_motion_with_transitions(initial, animate, exit, config.clone(), config)
}

/// Creates a motion handle with separate enter and exit transitions.
pub fn use_presence_motion_with_transitions<T>(
    initial: T,
    animate: T,
    exit: T,
    enter_config: AnimationConfig,
    exit_config: AnimationConfig,
) -> MotionHandle<T>
where
    T: Animatable + Send + 'static,
{
    let presence = use_presence();
    let context = try_consume_context::<PresenceContext>();
    let status = context.as_ref().map(|context| context.status);
    let start = context
        .as_ref()
        .map(|context| {
            if context.status.read().initial_allowed {
                initial.clone()
            } else {
                animate.clone()
            }
        })
        .unwrap_or_else(|| animate.clone());
    let mut motion = use_motion(start);
    let mut last_target_present = use_signal(|| None::<bool>);
    let mut awaiting_exit_completion = use_signal(|| false);
    let mut exit_observed_running = use_signal(|| false);
    let exit_duration = exit_config.get_duration();
    use_effect(move || {
        let is_present = status
            .map(|status| status.read().is_present)
            .unwrap_or(true);
        if *last_target_present.read() != Some(is_present) {
            *last_target_present.write() = Some(is_present);
            if is_present {
                awaiting_exit_completion.set(false);
                exit_observed_running.set(false);
                motion.animate_to(animate.clone(), enter_config.clone());
            } else {
                awaiting_exit_completion.set(true);
                exit_observed_running.set(false);
                motion.animate_to(exit.clone(), exit_config.clone());
            }
        }
    });
    use_effect(move || {
        if *awaiting_exit_completion.read() {
            if motion.is_running() {
                exit_observed_running.set(true);
            } else if *exit_observed_running.read() || exit_duration == Duration::default() {
                awaiting_exit_completion.set(false);
                exit_observed_running.set(false);
                presence.safe_to_remove.call(());
            }
        }
    });

    motion
}

/// Completes presence removal when a custom motion handle finishes its exit work.
///
/// Use this when raw [`use_motion`](crate::use_motion) logic controls the exit
/// animation instead of [`use_presence_motion`] or [`use_presence_style`].
/// Pass `true` for `exit_started` after starting the exit animation; the hook
/// calls `safe_to_remove` once the nearest presence child is exiting and the
/// motion handle is no longer running.
pub fn use_presence_motion_completion<T>(
    motion: MotionHandle<T>,
    exit_started: bool,
) -> PresenceHandle
where
    T: Animatable + Send + 'static,
{
    let presence = use_presence();
    let mut awaiting_exit_completion = use_signal(|| false);

    use_effect(move || {
        if presence.is_present {
            awaiting_exit_completion.set(false);
        } else if exit_started {
            awaiting_exit_completion.set(true);
        }
    });

    use_effect(move || {
        if *awaiting_exit_completion.read() && !motion.is_running() {
            awaiting_exit_completion.set(false);
            presence.safe_to_remove.call(());
        }
    });

    presence
}

/// Creates a CSS-ready presence style handle for opacity and transform animations.
pub fn use_presence_style(config: PresenceConfig) -> MotionHandle<MotionStyle> {
    let presence = use_presence();
    let context = try_consume_context::<PresenceContext>();
    let status = context.as_ref().map(|context| context.status);
    let measured_size = context.as_ref().map(|context| context.measured_size);
    static NEXT_PRESENCE_LAYOUT_ID: AtomicU64 = AtomicU64::new(1);
    let layout_id = use_hook(|| NEXT_PRESENCE_LAYOUT_ID.fetch_add(1, Ordering::Relaxed));
    let layout_context = context.clone();
    let layout_drop_context = context.clone();
    let layout_transition = config.layout_transition.clone();
    let layout_config = PresenceLayoutConfig {
        layout: config.layout,
        enter_transition: Some(
            layout_transition
                .clone()
                .unwrap_or_else(|| config.enter_transition.clone()),
        ),
        exit_transition: Some(layout_transition.unwrap_or_else(|| config.exit_transition.clone())),
    };
    use_effect(move || {
        if let Some(mut context) = layout_context.clone() {
            context
                .layouts
                .write()
                .insert(layout_id, layout_config.clone());
        }
    });
    use_drop(move || {
        if let Some(mut context) = layout_drop_context.clone() {
            context.layouts.write().remove(&layout_id);
        }
    });
    let start = context
        .as_ref()
        .map(|context| {
            if context.status.read().initial_allowed {
                config.initial.clone()
            } else {
                config.animate.clone()
            }
        })
        .unwrap_or_else(|| config.animate.clone());
    let mut motion = use_motion(start);
    let mut last_target_present = use_signal(|| None::<bool>);
    let mut awaiting_exit_completion = use_signal(|| false);
    let mut exit_observed_running = use_signal(|| false);
    let exit_duration = config.exit_transition.get_duration();

    use_effect(move || {
        let is_present = status
            .map(|status| status.read().is_present)
            .unwrap_or(true);
        if *last_target_present.read() != Some(is_present) {
            *last_target_present.write() = Some(is_present);
            if is_present {
                awaiting_exit_completion.set(false);
                exit_observed_running.set(false);
                if let Some(size) = measured_size.and_then(|size| *size.read()) {
                    let enter_start = presence_style_enter_start(
                        motion.get_value(),
                        &config.animate,
                        &config.exit,
                        Some(size),
                    );
                    motion.set_current(enter_start);
                }
                motion.animate_to(config.animate.clone(), config.enter_transition.clone());
            } else {
                awaiting_exit_completion.set(true);
                exit_observed_running.set(false);
                if let Some(size) = measured_size.and_then(|size| *size.read()) {
                    let exit_start =
                        presence_style_exit_start(motion.get_value(), &config.exit, Some(size));
                    motion.set_current(exit_start);
                }

                motion.animate_to(config.exit.clone(), config.exit_transition.clone());
            }
        }
    });
    use_effect(move || {
        if *awaiting_exit_completion.read() {
            if motion.is_running() {
                exit_observed_running.set(true);
            } else if *exit_observed_running.read() || exit_duration == Duration::default() {
                awaiting_exit_completion.set(false);
                exit_observed_running.set(false);
                presence.safe_to_remove.call(());
            }
        }
    });

    motion
}

fn presence_style_enter_start(
    mut current: MotionStyle,
    animate: &MotionStyle,
    exit: &MotionStyle,
    measured_size: Option<PresenceMeasuredSize>,
) -> MotionStyle {
    let Some(size) = measured_size else {
        return current;
    };

    if should_seed_enter_dimension(&current, animate, exit, "height") {
        current
            .properties
            .insert("height".to_string(), CssValue::Px(size.height as f32));
    }

    if should_seed_enter_dimension(&current, animate, exit, "width") {
        current
            .properties
            .insert("width".to_string(), CssValue::Px(size.width as f32));
    }

    current
}

fn should_seed_enter_dimension(
    current: &MotionStyle,
    animate: &MotionStyle,
    exit: &MotionStyle,
    property: &str,
) -> bool {
    animate.properties.contains_key(property)
        && current
            .properties
            .get(property)
            .is_none_or(|current_value| exit.properties.get(property) == Some(current_value))
}

fn presence_style_exit_start(
    mut current: MotionStyle,
    exit: &MotionStyle,
    measured_size: Option<PresenceMeasuredSize>,
) -> MotionStyle {
    let Some(size) = measured_size else {
        return current;
    };

    if exit.properties.contains_key("height") && !current.properties.contains_key("height") {
        current
            .properties
            .insert("height".to_string(), CssValue::Px(size.height as f32));
    }

    if exit.properties.contains_key("width") && !current.properties.contains_key("width") {
        current
            .properties
            .insert("width".to_string(), CssValue::Px(size.width as f32));
    }

    current
}

#[cfg(test)]
mod tests {
    use super::{
        MotionStyle, PresenceChild, PresenceCustom, PresenceError, PresenceMode,
        PresenceStateMachine, normalize_presence_children,
    };
    use crate::animations::core::Animatable;
    use crate::animations::css::{CssValue, IntoCssValue};
    use dioxus::prelude::*;
    use dioxus_core::ScopeId;
    use std::collections::BTreeMap;

    fn child(key: &str) -> Result<PresenceChild, PresenceError> {
        let children = rsx! {
            div {
                key: "{key}",
                "{key}"
            }
        };

        let mut normalized = normalize_presence_children(children)?;
        if normalized.is_empty() {
            return Err(PresenceError::MissingKey);
        }
        Ok(normalized.remove(0))
    }

    #[test]
    fn presence_style_formats_opacity_and_transform_css() {
        let style = MotionStyle::new(0.5)
            .x(12.0)
            .y(-8.0)
            .z(3.0)
            .scale(0.96)
            .scale_y(0.8)
            .rotate(15.0)
            .rotate_x(8.0)
            .skew_y(-4.0)
            .perspective(600.0);

        assert_eq!(
            style.to_css(),
            "opacity: 0.5; transform: perspective(600px) translateX(12px) translateY(-8px) translateZ(3px) scale(0.96) scaleX(1) scaleY(0.8) scaleZ(1) rotate(15deg) rotateX(8deg) rotateY(0deg) rotateZ(0deg) skew(0deg) skewX(0deg) skewY(-4deg)"
        );
    }

    #[test]
    fn presence_style_macro_builds_config() {
        let config = crate::presence_style! {
            initial: { opacity: 0.0, y: 20.0, scale: 0.92, rotate: -6.0 },
            animate: { opacity: 1.0 },
            exit: { opacity: 0.0, y: -16.0, scale: 0.96 },
            transition: tween { duration: 220.5 },
        };

        assert_eq!(
            config.initial,
            MotionStyle::new(0.0).y(20.0).scale(0.92).rotate(-6.0)
        );
        assert_eq!(config.animate, MotionStyle::new(1.0));
        assert_eq!(config.exit, MotionStyle::new(0.0).y(-16.0).scale(0.96));
        assert_eq!(
            config.enter_transition.get_duration(),
            crate::Duration::from_secs_f64(0.2205)
        );
        assert_eq!(
            config.exit_transition.get_duration(),
            crate::Duration::from_secs_f64(0.2205)
        );
    }

    #[test]
    fn presence_style_macro_accepts_transform_shorthands_and_aliases() {
        let config = crate::presence_style! {
            initial: {
                opacity: 0,
                translateX: 12,
                translate_y: -8,
                z: 3,
                scaleX: 0.9,
                scale_y: 0,
                scaleZ: 1.2,
                rotateX: 10,
                rotate_y: -12,
                rotateZ: 4,
                skew: 3,
                skewX: -2,
                skew_y: 6,
                transformPerspective: 800,
            },
            animate: { opacity: 1.0 },
            exit: { opacity: 0.0, scaleY: 0 },
            transition: tween { duration: 220.0 },
        };

        assert_eq!(
            config.initial,
            MotionStyle::new(0.0)
                .x(12.0)
                .y(-8.0)
                .z(3.0)
                .scale_x(0.9)
                .scale_y(0.0)
                .scale_z(1.2)
                .rotate_x(10.0)
                .rotate_y(-12.0)
                .rotate_z(4.0)
                .skew(3.0)
                .skew_x(-2.0)
                .skew_y(6.0)
                .perspective(800.0)
        );
        assert_eq!(config.exit, MotionStyle::new(0.0).scale_y(0.0));
    }

    #[test]
    fn presence_style_macro_accepts_value_typed_css_properties() {
        let config = crate::presence_style! {
            initial: { opacity: 0.0, height: 48.0, borderRadius: 8 },
            animate: { opacity: 1.0, height: 96.0, zIndex: 2 },
            exit: { opacity: 0.0, height: 0.0 },
            transition: tween { duration: 420.0 },
        };

        assert_eq!(
            config.initial.properties.get("height"),
            Some(&CssValue::Px(48.0))
        );
        assert_eq!(
            config.initial.properties.get("border-radius"),
            Some(&CssValue::Px(8.0))
        );
        assert_eq!(
            config.animate.properties.get("z-index"),
            Some(&CssValue::Number(2.0))
        );
        assert!(config.exit.to_css().contains("; height: 0px"));
    }

    #[test]
    fn presence_style_accepts_framer_style_string_values() {
        let config = crate::presence_style! {
            initial: {
                opacity: 0.0,
                width: "25%",
                height: "10vh",
                left: "5vw",
                rotate: 0,
                backgroundColor: "#000",
                boxShadow: "0px 4px 8px rgba(0, 0, 0, 0.2)",
                color: "var(--text-color)",
            },
            animate: {
                opacity: 1.0,
                width: "75%",
                height: "20vh",
                left: "15vw",
                backgroundColor: "rgb(255, 128, 0)",
                boxShadow: "10px 12px 20px rgba(255, 128, 0, 0.6)",
                color: "var(--accent-color)",
            },
            exit: { opacity: 0.0, width: "0%" },
            transition: tween { duration: 200.0 },
        };

        assert_eq!(
            config.initial.properties.get("width"),
            Some(&CssValue::Percent(25.0))
        );
        assert_eq!(
            config.initial.properties.get("height"),
            Some(&CssValue::Vh(10.0))
        );
        assert_eq!(
            config.initial.properties.get("left"),
            Some(&CssValue::Vw(5.0))
        );
        assert!(matches!(
            config.initial.properties.get("background-color"),
            Some(CssValue::Color(_))
        ));
        assert!(matches!(
            config.initial.properties.get("box-shadow"),
            Some(CssValue::Complex(_))
        ));
        assert_eq!(
            config.initial.properties.get("color"),
            Some(&CssValue::Keyword("var(--text-color)".to_string()))
        );
    }

    #[test]
    fn presence_style_interpolates_compatible_string_values() {
        let start = MotionStyle::new(1.0)
            .property("width", CssValue::Percent(25.0))
            .property(
                "background-color",
                "#000".into_css_value("background-color"),
            )
            .property(
                "box-shadow",
                "0px 4px 8px rgba(0, 0, 0, 0.2)".into_css_value("box-shadow"),
            );
        let end = MotionStyle::new(1.0)
            .property("width", CssValue::Percent(75.0))
            .property(
                "background-color",
                "#ffffff".into_css_value("background-color"),
            )
            .property(
                "box-shadow",
                "10px 12px 20px rgba(255, 128, 0, 0.6)".into_css_value("box-shadow"),
            );

        let style = start.interpolate(&end, 0.5);
        assert_eq!(
            style.properties.get("width"),
            Some(&CssValue::Percent(50.0))
        );
        assert_eq!(
            style
                .properties
                .get("background-color")
                .map(|value| value.to_css()),
            Some("rgba(128, 128, 128, 1)".to_string())
        );
        assert_eq!(
            style
                .properties
                .get("box-shadow")
                .map(|value| value.to_css()),
            Some("5px 8px 14px rgba(128, 64, 0, 0.4)".to_string())
        );
    }

    #[test]
    fn presence_style_exit_start_uses_measured_size_for_missing_height_and_width() {
        let current = MotionStyle::new(1.0);
        let exit = MotionStyle::new(0.0)
            .property("height", CssValue::Px(0.0))
            .property("width", CssValue::Px(0.0));
        let start = super::presence_style_exit_start(
            current,
            &exit,
            Some(super::PresenceMeasuredSize {
                width: 240.0,
                height: 96.0,
            }),
        );

        assert_eq!(start.properties.get("height"), Some(&CssValue::Px(96.0)));
        assert_eq!(start.properties.get("width"), Some(&CssValue::Px(240.0)));
    }

    #[test]
    fn presence_style_enter_start_does_not_inject_height_without_enter_target() {
        let current = MotionStyle::new(0.0);
        let animate = MotionStyle::new(1.0);
        let start = super::presence_style_enter_start(
            current,
            &animate,
            &MotionStyle::new(0.0),
            Some(super::PresenceMeasuredSize {
                width: 240.0,
                height: 96.0,
            }),
        );

        assert_eq!(start.properties.get("height"), None);
        assert_eq!(start.properties.get("width"), None);
    }

    #[test]
    fn presence_style_enter_start_uses_measured_size_for_missing_height_and_width() {
        let current = MotionStyle::new(0.0);
        let animate = MotionStyle::new(1.0)
            .property("height", CssValue::Px(48.0))
            .property("width", CssValue::Px(120.0));
        let start = super::presence_style_enter_start(
            current,
            &animate,
            &MotionStyle::new(0.0),
            Some(super::PresenceMeasuredSize {
                width: 240.0,
                height: 96.0,
            }),
        );

        assert_eq!(start.properties.get("height"), Some(&CssValue::Px(96.0)));
        assert_eq!(start.properties.get("width"), Some(&CssValue::Px(240.0)));
    }

    #[test]
    fn presence_style_enter_start_reseeds_collapsed_exit_dimensions() {
        let current = MotionStyle::new(0.0)
            .property("height", CssValue::Px(0.0))
            .property("width", CssValue::Px(0.0));
        let animate = MotionStyle::new(1.0)
            .property("height", CssValue::Px(48.0))
            .property("width", CssValue::Px(120.0));
        let exit = MotionStyle::new(0.0)
            .property("height", CssValue::Px(0.0))
            .property("width", CssValue::Px(0.0));
        let start = super::presence_style_enter_start(
            current,
            &animate,
            &exit,
            Some(super::PresenceMeasuredSize {
                width: 240.0,
                height: 96.0,
            }),
        );

        assert_eq!(start.properties.get("height"), Some(&CssValue::Px(96.0)));
        assert_eq!(start.properties.get("width"), Some(&CssValue::Px(240.0)));
    }

    #[test]
    fn presence_style_macro_applies_transition_to_enter_and_exit() {
        let config = crate::presence_style! {
            initial: { opacity: 0.0, y: 20.0, scale: 0.92 },
            animate: { opacity: 1.0 },
            exit: { opacity: 0.0, y: -16.0, scale: 0.96 },
            transition: tween { duration: 420.0 },
        };

        assert_eq!(
            config.enter_transition.get_duration(),
            crate::Duration::from_millis(420)
        );
        assert_eq!(
            config.exit_transition.get_duration(),
            crate::Duration::from_millis(420)
        );
    }

    #[test]
    fn presence_style_macro_builds_layout_transition() {
        let config = crate::presence_style! {
            initial: { opacity: 0.0, x: 20.0 },
            animate: { opacity: 1.0, x: 0.0 },
            exit: { opacity: 0.0, x: -20.0 },
            layout: size,
            transition: tween { duration: 440.0 },
            layout_transition: tween { duration: 444.0 },
        };

        assert_eq!(config.layout, super::PresenceLayout::Size);
        assert_eq!(
            config.enter_transition.get_duration(),
            crate::Duration::from_millis(440)
        );
        assert_eq!(
            config
                .layout_transition
                .as_ref()
                .map(crate::animations::core::AnimationConfig::get_duration),
            Some(crate::Duration::from_millis(444))
        );
    }

    #[test]
    fn presence_style_macro_accepts_nested_layout_transition() {
        let config = crate::presence_style! {
            initial: { opacity: 0.0, x: 20.0 },
            animate: { opacity: 1.0, x: 0.0 },
            exit: { opacity: 0.0, x: -20.0 },
            layout: size,
            transition: tween {
                layout: tween { duration: 444.0 },
            duration: 440.0,
            },
        };

        assert_eq!(config.layout, super::PresenceLayout::Size);
        assert_eq!(
            config.enter_transition.get_duration(),
            crate::Duration::from_millis(440)
        );
        assert_eq!(
            config
                .layout_transition
                .as_ref()
                .map(crate::animations::core::AnimationConfig::get_duration),
            Some(crate::Duration::from_millis(444))
        );
    }

    #[test]
    fn presence_style_macro_builds_spring_transition() {
        let config = crate::presence_style! {
            initial: { opacity: 0.0 },
            animate: { opacity: 1.0 },
            exit: { opacity: 0.0 },
            transition: spring { stiffness: 180.0, damping: 22.0 },
        };

        if let crate::animations::core::AnimationMode::Spring(spring) = config.enter_transition.mode
        {
            assert_eq!(spring.stiffness, 180.0);
            assert_eq!(spring.damping, 22.0);
            assert_eq!(spring.mass, 1.0);
        } else {
            assert!(matches!(
                config.enter_transition.mode,
                crate::animations::core::AnimationMode::Spring(_)
            ));
        }
    }

    #[test]
    fn presence_style_macro_builds_separate_enter_exit_transitions() {
        let config = crate::presence_style! {
            initial: { opacity: 0.0, y: 12.0 },
            animate: { opacity: 1.0, y: 0.0 },
            exit: { opacity: 0.0, y: -12.0 },
            enter_transition: spring { stiffness: 180.0, damping: 22.0 },
            exit_transition: tween { duration: 140.0 },
        };

        assert!(matches!(
            config.enter_transition.mode,
            crate::animations::core::AnimationMode::Spring(_)
        ));
        assert_eq!(
            config.exit_transition.get_duration(),
            crate::Duration::from_millis(140)
        );
    }

    #[test]
    fn exiting_layout_child_animates_height_to_zero_without_clipping() {
        let style = super::presence_layout_child_style(
            &super::PresenceLayoutConfig {
                layout: super::PresenceLayout::Size,
                enter_transition: None,
                exit_transition: Some(crate::animations::core::AnimationConfig::tween(
                    crate::Duration::from_millis(220),
                )),
            },
            Some(super::PresenceMeasuredSize {
                width: 320.0,
                height: 64.0,
            }),
            super::PresenceLayoutChildStyleState {
                collapse_exit: true,
                ..Default::default()
            },
        );

        assert!(style.contains("height: 0px"));
        assert!(style.contains("overflow: visible"));
    }

    #[test]
    fn present_layout_child_does_not_clip_before_or_after_measurement() {
        let layout = super::PresenceLayoutConfig {
            layout: super::PresenceLayout::Size,
            enter_transition: Some(crate::animations::core::AnimationConfig::tween(
                crate::Duration::from_millis(220),
            )),
            exit_transition: None,
        };
        let unmeasured_style = super::presence_layout_child_style(
            &layout,
            None,
            super::PresenceLayoutChildStyleState {
                is_present: true,
                collapse_exit: true,
                ..Default::default()
            },
        );
        let initial_unmeasured_style = super::presence_layout_child_style(
            &layout,
            None,
            super::PresenceLayoutChildStyleState {
                is_present: true,
                initial_allowed: true,
                collapse_exit: true,
                ..Default::default()
            },
        );
        let measured_style = super::presence_layout_child_style(
            &layout,
            Some(super::PresenceMeasuredSize {
                width: 320.0,
                height: 64.0,
            }),
            super::PresenceLayoutChildStyleState {
                is_present: true,
                collapse_exit: true,
                ..Default::default()
            },
        );

        assert!(unmeasured_style.contains("overflow: visible"));
        assert!(measured_style.contains("overflow: visible"));
        assert!(!unmeasured_style.contains("height: 0"));
        assert!(!initial_unmeasured_style.contains("height: 0"));
        assert!(!measured_style.contains("height: 64px"));
    }

    #[test]
    fn inserted_layout_child_expands_from_zero_height_after_measurement() {
        let layout = super::PresenceLayoutConfig {
            layout: super::PresenceLayout::Size,
            enter_transition: Some(crate::animations::core::AnimationConfig::tween(
                crate::Duration::from_millis(444),
            )),
            exit_transition: None,
        };

        let unmeasured_style = super::presence_layout_child_style(
            &layout,
            None,
            super::PresenceLayoutChildStyleState {
                is_present: true,
                initial_allowed: true,
                layout_enter_allowed: true,
                collapse_exit: true,
                ..Default::default()
            },
        );
        let measured_collapsed_style = super::presence_layout_child_style(
            &layout,
            Some(super::PresenceMeasuredSize {
                width: 320.0,
                height: 64.0,
            }),
            super::PresenceLayoutChildStyleState {
                is_present: true,
                initial_allowed: true,
                layout_enter_allowed: true,
                collapse_exit: true,
                ..Default::default()
            },
        );
        let measured_expanded_style = super::presence_layout_child_style(
            &layout,
            Some(super::PresenceMeasuredSize {
                width: 320.0,
                height: 64.0,
            }),
            super::PresenceLayoutChildStyleState {
                is_present: true,
                initial_allowed: true,
                layout_enter_allowed: true,
                expand_enter: true,
                collapse_exit: true,
                ..Default::default()
            },
        );
        let measured_settled_style = super::presence_layout_child_style(
            &layout,
            Some(super::PresenceMeasuredSize {
                width: 320.0,
                height: 64.0,
            }),
            super::PresenceLayoutChildStyleState {
                is_present: true,
                initial_allowed: true,
                layout_enter_allowed: true,
                expand_enter: true,
                settle_enter: true,
                collapse_exit: true,
                ..Default::default()
            },
        );

        assert!(unmeasured_style.contains("height: 0px"));
        assert!(measured_collapsed_style.contains("height: 0px"));
        assert!(measured_expanded_style.contains("height: 64px"));
        assert!(
            measured_expanded_style.contains("transition: width 444ms ease, height 444ms ease")
        );
        assert!(measured_settled_style.contains("overflow: visible"));
        assert!(measured_settled_style.contains("height: auto"));
        assert!(!measured_settled_style.contains("height: 64px"));
    }

    #[cfg(feature = "web")]
    #[test]
    fn projection_transition_composition_preserves_width_and_height_entries() {
        let transition = super::compose_projection_transition(
            "width 220ms ease, height 220ms ease, transform 100ms linear",
            crate::Duration::from_millis(444),
        );

        assert!(transition.contains("width 220ms ease"));
        assert!(transition.contains("height 220ms ease"));
        assert!(!transition.contains("transform 100ms linear"));
        assert!(transition.contains("transform 444ms ease"));
    }

    #[cfg(feature = "web")]
    #[test]
    fn projection_transform_composes_with_existing_transform() {
        let transform = super::compose_projection_transform(
            12.0,
            -8.0,
            1.25,
            0.75,
            Some("rotate(15deg) scale(0.9)"),
        );

        assert_eq!(
            transform,
            "translate(12px, -8px) scale(1.25, 0.75) rotate(15deg) scale(0.9)"
        );
    }

    #[cfg(feature = "web")]
    #[test]
    fn projection_identity_transform_preserves_existing_transform() {
        let transform =
            super::compose_projection_identity_transform(Some("rotate(15deg) scale(0.9)"));

        assert_eq!(
            transform,
            "translate(0px, 0px) scale(1, 1) rotate(15deg) scale(0.9)"
        );
    }

    #[cfg(feature = "web")]
    #[test]
    fn projection_identity_transform_ignores_empty_or_none_base_transform() {
        assert_eq!(
            super::compose_projection_identity_transform(Some("none")),
            "translate(0px, 0px) scale(1, 1)"
        );
        assert_eq!(
            super::compose_projection_identity_transform(None),
            "translate(0px, 0px) scale(1, 1)"
        );
    }

    #[test]
    fn exiting_pop_layout_child_contributes_no_layout_space() {
        let style = super::presence_layout_child_style(
            &super::PresenceLayoutConfig {
                layout: super::PresenceLayout::Size,
                enter_transition: None,
                exit_transition: Some(crate::animations::core::AnimationConfig::tween(
                    crate::Duration::from_millis(220),
                )),
            },
            Some(super::PresenceMeasuredSize {
                width: 320.0,
                height: 64.0,
            }),
            super::PresenceLayoutChildStyleState {
                pop_layout_ready: true,
                collapse_exit: true,
                ..Default::default()
            },
        );

        assert!(style.contains("width: 0px"));
        assert!(style.contains("height: 0px"));
        assert!(style.contains("transition: width 220ms ease, height 220ms ease"));
    }

    #[test]
    fn exiting_pop_layout_child_without_measurement_stays_in_flow() {
        let style = super::presence_layout_child_style(
            &super::PresenceLayoutConfig {
                layout: super::PresenceLayout::Size,
                enter_transition: None,
                exit_transition: Some(crate::animations::core::AnimationConfig::tween(
                    crate::Duration::from_millis(220),
                )),
            },
            None,
            super::PresenceLayoutChildStyleState {
                collapse_exit: false,
                ..Default::default()
            },
        );

        assert!(!style.contains("width: 0px"));
        assert!(!style.contains("height: 0px"));
    }

    #[test]
    fn exiting_layout_child_starts_from_measured_height_before_collapse() {
        let style = super::presence_layout_child_style(
            &super::PresenceLayoutConfig {
                layout: super::PresenceLayout::Size,
                enter_transition: None,
                exit_transition: Some(crate::animations::core::AnimationConfig::tween(
                    crate::Duration::from_millis(220),
                )),
            },
            Some(super::PresenceMeasuredSize {
                width: 320.0,
                height: 64.0,
            }),
            super::PresenceLayoutChildStyleState::default(),
        );

        assert!(style.contains("height: 64px"));
    }

    #[test]
    fn layout_config_resolver_uses_longest_registered_layout_transition() {
        let mut layouts = BTreeMap::new();
        layouts.insert(
            1,
            super::PresenceLayoutConfig {
                layout: super::PresenceLayout::Size,
                enter_transition: Some(crate::animations::core::AnimationConfig::tween(
                    crate::Duration::from_millis(180),
                )),
                exit_transition: Some(crate::animations::core::AnimationConfig::tween(
                    crate::Duration::from_millis(220),
                )),
            },
        );
        layouts.insert(
            2,
            super::PresenceLayoutConfig {
                layout: super::PresenceLayout::Size,
                enter_transition: Some(crate::animations::core::AnimationConfig::tween(
                    crate::Duration::from_millis(260),
                )),
                exit_transition: Some(crate::animations::core::AnimationConfig::tween(
                    crate::Duration::from_millis(140),
                )),
            },
        );

        let enter = super::resolve_presence_layout_config(&layouts, true);
        let exit = super::resolve_presence_layout_config(&layouts, false);

        assert_eq!(
            enter
                .enter_transition
                .as_ref()
                .map(crate::animations::core::AnimationConfig::get_duration),
            Some(crate::Duration::from_millis(260))
        );
        assert_eq!(
            exit.exit_transition
                .as_ref()
                .map(crate::animations::core::AnimationConfig::get_duration),
            Some(crate::Duration::from_millis(220))
        );
    }

    #[test]
    fn pop_layout_present_child_stays_in_normal_flow() {
        let style = super::pop_layout_style(
            Some(super::LayoutSnapshot {
                offset_left: 12.0,
                offset_top: 24.0,
                width: 320.0,
                height: 64.0,
                parent_width: 640.0,
                parent_height: 480.0,
                parent_positioned: true,
            }),
            None,
            true,
            super::PresenceAnchorX::Left,
            super::PresenceAnchorY::Top,
        );

        assert_eq!(style, "");
    }

    #[test]
    fn pop_layout_without_snapshot_degrades_to_normal_flow() {
        let style = super::pop_layout_style(
            None,
            None,
            false,
            super::PresenceAnchorX::Left,
            super::PresenceAnchorY::Top,
        );

        assert_eq!(style, "");
    }

    #[test]
    fn pop_layout_present_child_ignores_stale_snapshot() {
        let style = super::pop_layout_style(
            Some(super::LayoutSnapshot {
                offset_left: 12.0,
                offset_top: 24.0,
                width: 320.0,
                height: 64.0,
                parent_width: 640.0,
                parent_height: 480.0,
                parent_positioned: true,
            }),
            None,
            true,
            super::PresenceAnchorX::Right,
            super::PresenceAnchorY::Bottom,
        );

        assert_eq!(style, "");
    }

    #[test]
    fn pop_layout_exiting_child_uses_preserved_snapshot_box() {
        let style = super::pop_layout_style(
            Some(super::LayoutSnapshot {
                offset_left: 12.0,
                offset_top: 24.0,
                width: 320.0,
                height: 64.0,
                parent_width: 640.0,
                parent_height: 480.0,
                parent_positioned: true,
            }),
            None,
            false,
            super::PresenceAnchorX::Left,
            super::PresenceAnchorY::Top,
        );

        assert!(style.contains("position: absolute"));
        assert!(style.contains("left: 12px"));
        assert!(style.contains("top: 24px"));
        assert!(style.contains("width: 320px"));
        assert!(style.contains("height: 64px"));
    }

    #[test]
    fn pop_layout_wrapper_style_pins_owned_wrapper() {
        let style = super::pop_layout_wrapper_style(
            Some(super::LayoutSnapshot {
                offset_left: 12.0,
                offset_top: 24.0,
                width: 320.0,
                height: 64.0,
                parent_width: 640.0,
                parent_height: 480.0,
                parent_positioned: true,
            }),
            None,
            false,
            super::PresenceAnchorX::Left,
            super::PresenceAnchorY::Top,
        );

        assert!(style.starts_with("display: inline-block; "));
        assert!(style.contains("position: absolute"));
        assert!(style.contains("width: 320px"));
    }

    #[test]
    fn pop_layout_uses_measured_slot_size_for_pinned_box() {
        let style = super::pop_layout_style(
            Some(super::LayoutSnapshot {
                offset_left: 12.0,
                offset_top: 24.0,
                width: 39.0,
                height: 18.0,
                parent_width: 640.0,
                parent_height: 480.0,
                parent_positioned: true,
            }),
            Some(super::PresenceMeasuredSize {
                width: 75.0,
                height: 50.0,
            }),
            false,
            super::PresenceAnchorX::Left,
            super::PresenceAnchorY::Top,
        );

        assert!(style.contains("width: 75px"));
        assert!(style.contains("height: 50px"));
    }

    #[test]
    fn pop_layout_exiting_child_supports_right_bottom_anchors() {
        let style = super::pop_layout_style(
            Some(super::LayoutSnapshot {
                offset_left: 12.0,
                offset_top: 24.0,
                width: 320.0,
                height: 64.0,
                parent_width: 640.0,
                parent_height: 480.0,
                parent_positioned: true,
            }),
            None,
            false,
            super::PresenceAnchorX::Right,
            super::PresenceAnchorY::Bottom,
        );

        assert!(style.contains("right: 308px"));
        assert!(style.contains("bottom: 392px"));
    }

    fn children(keys: &[&str]) -> Result<Vec<PresenceChild>, PresenceError> {
        keys.iter().map(|key| child(key)).collect()
    }

    fn reconcile(
        state: &mut PresenceStateMachine,
        children: Vec<PresenceChild>,
        mode: PresenceMode,
    ) -> Result<super::PresenceRender, PresenceError> {
        state.reconcile(children, mode, true, None)
    }

    #[test]
    fn normalizes_single_keyed_root() -> Result<(), PresenceError> {
        let key = "alpha";
        let children = rsx! {
            div {
                key: "{key}",
                "Alpha"
            }
        };

        let normalized = normalize_presence_children(children)?;

        assert_eq!(normalized.len(), 1);
        assert_eq!(normalized[0].key, "alpha");
        assert!(normalized[0].vnode.key.is_some());
        Ok(())
    }

    #[test]
    fn normalizes_keyed_for_loop_fragment() -> Result<(), PresenceError> {
        let items = ["alpha", "beta", "gamma"];
        let children = rsx! {
            for item in items {
                div {
                    key: "{item}",
                    "{item}"
                }
            }
        };

        let normalized = normalize_presence_children(children)?;
        let keys = normalized
            .iter()
            .map(|child| child.key.as_str())
            .collect::<Vec<_>>();

        assert_eq!(keys, ["alpha", "beta", "gamma"]);
        assert!(normalized.iter().all(|child| child.vnode.key.is_some()));
        Ok(())
    }

    #[component]
    fn Row(id: String) -> Element {
        rsx! {
            div { "{id}" }
        }
    }

    #[test]
    fn normalizes_keyed_component_for_loop_fragment() -> Result<(), PresenceError> {
        let items = ["alpha", "beta", "gamma"];
        let children = rsx! {
            for item in items {
                Row {
                    key: "{item}",
                    id: item.to_string(),
                }
            }
        };

        let normalized = normalize_presence_children(children)?;
        let keys = normalized
            .iter()
            .map(|child| child.key.as_str())
            .collect::<Vec<_>>();

        assert_eq!(keys, ["alpha", "beta", "gamma"]);
        assert!(normalized.iter().all(|child| child.vnode.key.is_some()));
        Ok(())
    }

    #[test]
    fn rejects_missing_key() {
        let children = rsx! {
            div { "Alpha" }
        };

        assert!(matches!(
            normalize_presence_children(children),
            Err(PresenceError::MissingKey)
        ));
    }

    #[test]
    fn rejects_duplicate_keys() {
        let first = "same";
        let second = "same";
        let children = rsx! {
            for item in [first, second] {
                div {
                    key: "{item}",
                    "{item}"
                }
            }
        };

        assert!(matches!(
            normalize_presence_children(children),
            Err(PresenceError::DuplicateKey(key)) if key == "same"
        ));
    }

    #[test]
    fn normalizes_empty_conditional_as_no_children() -> Result<(), PresenceError> {
        let visible = false;
        let key = "panel";
        let children = rsx! {
            if visible {
                div {
                    key: "{key}",
                    "Panel"
                }
            }
        };

        let normalized = normalize_presence_children(children)?;

        assert!(normalized.is_empty());
        Ok(())
    }

    #[test]
    fn sync_retains_exiting_child_until_completion() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        let render = reconcile(&mut state, children(&["a", "b"])?, PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), ["a", "b"]);
        assert!(!render.exit_completed);
        let token = state.register("b", None)?;

        let render = reconcile(&mut state, children(&["a"])?, PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), ["a", "b"]);
        assert!(!render.exit_completed);

        state.complete("b", token);
        let render = reconcile(&mut state, children(&["a"])?, PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), ["a"]);
        assert!(render.exit_completed);
        Ok(())
    }

    #[test]
    fn child_without_subscribers_is_removed_immediately() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        reconcile(&mut state, children(&["a", "b"])?, PresenceMode::Sync)?;

        let render = reconcile(&mut state, children(&["a"])?, PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), ["a"]);
        assert!(render.exit_completed);
        Ok(())
    }

    #[test]
    fn exiting_registered_child_notifies_subscriber_scope() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        reconcile(&mut state, children(&["a"])?, PresenceMode::Sync)?;
        let _token = state.register("a", Some(ScopeId(42)))?;

        let render = reconcile(&mut state, Vec::new(), PresenceMode::Sync)?;

        assert_eq!(rendered_keys(render.clone()), ["a"]);
        assert_eq!(render.notify_scopes, [ScopeId(42)]);
        Ok(())
    }

    #[test]
    fn readding_key_cancels_stale_exit_completion() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        reconcile(&mut state, children(&["a"])?, PresenceMode::Sync)?;
        let token = state.register("a", None)?;
        reconcile(&mut state, Vec::new(), PresenceMode::Sync)?;

        let render = reconcile(&mut state, children(&["a"])?, PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), ["a"]);
        assert!(!render.exit_completed);
        state.complete("a", token);
        let render = reconcile(&mut state, children(&["a"])?, PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), ["a"]);
        assert!(!render.exit_completed);
        Ok(())
    }

    #[test]
    fn wait_mode_renders_only_exiting_children_until_completion() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        reconcile(&mut state, children(&["a"])?, PresenceMode::Wait)?;
        let token = state.register("a", None)?;

        let render = reconcile(&mut state, children(&["b"])?, PresenceMode::Wait)?;
        assert_eq!(rendered_keys(render.clone()), ["a"]);
        assert!(!render.exit_completed);

        state.complete("a", token);
        let render = reconcile(&mut state, children(&["b"])?, PresenceMode::Wait)?;
        assert_eq!(rendered_keys(render.clone()), ["b"]);
        assert!(render.exit_completed);
        assert!(render.children[0].initial_allowed);
        assert!(render.children[0].layout_enter_allowed);
        Ok(())
    }

    #[test]
    fn wait_mode_differs_from_sync_during_exit() -> Result<(), PresenceError> {
        let mut sync = PresenceStateMachine::default();
        reconcile(&mut sync, children(&["a"])?, PresenceMode::Sync)?;
        sync.register("a", None)?;

        let sync_render = reconcile(&mut sync, children(&["b"])?, PresenceMode::Sync)?;
        assert_eq!(rendered_keys(sync_render), ["a", "b"]);

        let mut wait = PresenceStateMachine::default();
        reconcile(&mut wait, children(&["a"])?, PresenceMode::Wait)?;
        wait.register("a", None)?;

        let wait_render = reconcile(&mut wait, children(&["b"])?, PresenceMode::Wait)?;
        assert_eq!(rendered_keys(wait_render), ["a"]);
        Ok(())
    }

    #[test]
    fn wait_mode_replaces_pending_children_instead_of_exiting_them() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        reconcile(&mut state, children(&["a"])?, PresenceMode::Wait)?;
        let token = state.register("a", None)?;

        let render = reconcile(&mut state, children(&["b"])?, PresenceMode::Wait)?;
        assert_eq!(rendered_keys(render), ["a"]);

        let render = reconcile(&mut state, children(&["c"])?, PresenceMode::Wait)?;
        assert_eq!(rendered_keys(render), ["a"]);

        state.complete("a", token);
        let render = reconcile(&mut state, children(&["c"])?, PresenceMode::Wait)?;
        assert_eq!(rendered_keys(render.clone()), ["c"]);
        assert!(render.exit_completed);
        Ok(())
    }

    #[test]
    fn wait_mode_preserves_custom_for_deferred_child() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        state.reconcile(
            children(&["a"])?,
            PresenceMode::Wait,
            true,
            Some(PresenceCustom::new("present")),
        )?;
        let token = state.register("a", None)?;

        let render = state.reconcile(
            children(&["b"])?,
            PresenceMode::Wait,
            true,
            Some(PresenceCustom::new("deferred")),
        )?;
        assert_eq!(rendered_keys(render), ["a"]);

        state.complete("a", token);
        let rendered = state.render_children(false, PresenceMode::Wait);
        assert_eq!(
            rendered
                .iter()
                .map(|child| child.key.as_str())
                .collect::<Vec<_>>(),
            ["b"]
        );
        assert_eq!(
            rendered[0]
                .custom
                .as_ref()
                .and_then(PresenceCustom::get::<&'static str>),
            Some("deferred")
        );

        let render = state.reconcile(
            children(&["b"])?,
            PresenceMode::Wait,
            true,
            Some(PresenceCustom::new("released")),
        )?;
        assert_eq!(rendered_keys(render.clone()), ["b"]);
        assert!(render.exit_completed);
        Ok(())
    }

    #[test]
    fn wait_mode_uses_latest_pending_custom_after_repeated_reconcile() -> Result<(), PresenceError>
    {
        let mut state = PresenceStateMachine::default();
        state.reconcile(
            children(&["a"])?,
            PresenceMode::Wait,
            true,
            Some(PresenceCustom::new("present")),
        )?;
        let token = state.register("a", None)?;

        state.reconcile(
            children(&["b"])?,
            PresenceMode::Wait,
            true,
            Some(PresenceCustom::new("first")),
        )?;
        let render = state.reconcile(
            children(&["c"])?,
            PresenceMode::Wait,
            true,
            Some(PresenceCustom::new("latest")),
        )?;
        assert_eq!(rendered_keys(render), ["a"]);

        state.complete("a", token);
        let rendered = state.render_children(false, PresenceMode::Wait);
        assert_eq!(
            rendered
                .iter()
                .map(|child| child.key.as_str())
                .collect::<Vec<_>>(),
            ["c"]
        );
        assert_eq!(
            rendered[0]
                .custom
                .as_ref()
                .and_then(PresenceCustom::get::<&'static str>),
            Some("latest")
        );

        let render = state.reconcile(
            children(&["c"])?,
            PresenceMode::Wait,
            true,
            Some(PresenceCustom::new("final")),
        )?;
        assert_eq!(rendered_keys(render.clone()), ["c"]);
        assert!(render.exit_completed);
        assert!(render.children[0].initial_allowed);
        Ok(())
    }

    #[test]
    fn unregister_during_exit_allows_removal() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        reconcile(&mut state, children(&["a"])?, PresenceMode::Sync)?;
        let token = state.register("a", None)?;
        reconcile(&mut state, Vec::new(), PresenceMode::Sync)?;

        state.unregister("a", token);
        let render = reconcile(&mut state, Vec::new(), PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), Vec::<String>::new());
        assert!(render.exit_completed);
        Ok(())
    }

    #[test]
    fn exit_complete_waits_for_all_exiting_children() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        reconcile(&mut state, children(&["a", "b"])?, PresenceMode::Sync)?;
        let first = state.register("a", None)?;
        let second = state.register("b", None)?;
        reconcile(&mut state, Vec::new(), PresenceMode::Sync)?;

        state.complete("a", first);
        let render = reconcile(&mut state, Vec::new(), PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), ["b"]);
        assert!(!render.exit_completed);

        state.complete("b", second);
        let render = reconcile(&mut state, Vec::new(), PresenceMode::Sync)?;
        assert_eq!(rendered_keys(render.clone()), Vec::<String>::new());
        assert!(render.exit_completed);
        Ok(())
    }

    #[test]
    fn initial_false_suppresses_only_first_render_initial() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        let first = state.reconcile(children(&["a"])?, PresenceMode::Sync, false, None)?;
        assert!(!first.children[0].initial_allowed);

        let second = state.reconcile(children(&["a", "b"])?, PresenceMode::Sync, false, None)?;
        let existing = second
            .children
            .iter()
            .find(|child| child.key == "a")
            .ok_or(PresenceError::MissingKey)?;
        let inserted = second
            .children
            .iter()
            .find(|child| child.key == "b")
            .ok_or(PresenceError::MissingKey)?;
        assert!(!existing.initial_allowed);
        assert!(inserted.initial_allowed);
        Ok(())
    }

    #[test]
    fn wait_mode_deferred_replacement_reenables_enter_after_initial_false()
    -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        let first = state.reconcile(children(&["a"])?, PresenceMode::Wait, false, None)?;
        assert!(!first.children[0].initial_allowed);
        let token = state.register("a", None)?;

        let render = state.reconcile(children(&["b"])?, PresenceMode::Wait, false, None)?;
        assert_eq!(rendered_keys(render), ["a"]);

        state.complete("a", token);
        let render = state.reconcile(children(&["b"])?, PresenceMode::Wait, false, None)?;
        assert_eq!(
            render
                .children
                .iter()
                .map(|child| child.key.as_str())
                .collect::<Vec<_>>(),
            ["b"]
        );
        assert!(render.children[0].initial_allowed);
        assert!(render.children[0].layout_enter_allowed);
        Ok(())
    }

    #[test]
    fn custom_is_captured_from_removal_render() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        state.reconcile(
            children(&["a"])?,
            PresenceMode::Sync,
            true,
            Some(PresenceCustom::new("present")),
        )?;
        let token = state.register("a", None)?;

        let render = state.reconcile(
            Vec::new(),
            PresenceMode::Sync,
            true,
            Some(PresenceCustom::new("exit")),
        )?;
        assert_eq!(
            render.children[0]
                .custom
                .as_ref()
                .and_then(PresenceCustom::get::<&'static str>),
            Some("exit")
        );

        state.complete("a", token);
        Ok(())
    }

    #[test]
    fn pop_layout_uses_sync_retention_semantics() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        state.reconcile(children(&["a", "b"])?, PresenceMode::PopLayout, true, None)?;
        let token = state.register("b", None)?;

        let render = state.reconcile(children(&["a"])?, PresenceMode::PopLayout, true, None)?;
        assert_eq!(rendered_keys(render), ["a", "b"]);

        state.complete("b", token);
        let render = state.reconcile(children(&["a"])?, PresenceMode::PopLayout, true, None)?;
        assert_eq!(rendered_keys(render), ["a"]);
        Ok(())
    }

    #[test]
    fn sync_front_removal_keeps_exiting_child_in_removed_slot() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        state.reconcile(children(&["a", "b", "c"])?, PresenceMode::Sync, true, None)?;
        let _token = state.register("a", None)?;

        let render = state.reconcile(children(&["b", "c"])?, PresenceMode::Sync, true, None)?;

        assert_eq!(rendered_keys(render), ["a", "b", "c"]);
        Ok(())
    }

    #[test]
    fn pop_layout_front_removal_keeps_exit_in_removed_slot() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        state.reconcile(
            children(&["a", "b", "c"])?,
            PresenceMode::PopLayout,
            true,
            None,
        )?;
        let _token = state.register("a", None)?;

        let render =
            state.reconcile(children(&["b", "c"])?, PresenceMode::PopLayout, true, None)?;

        assert_eq!(rendered_keys(render), ["a", "b", "c"]);
        Ok(())
    }

    #[test]
    fn pop_layout_shifted_children_do_not_replay_initial_animation() -> Result<(), PresenceError> {
        let mut state = PresenceStateMachine::default();
        state.reconcile(
            children(&["a", "b", "c"])?,
            PresenceMode::PopLayout,
            true,
            None,
        )?;
        let _token = state.register("a", None)?;

        let render =
            state.reconcile(children(&["b", "c"])?, PresenceMode::PopLayout, true, None)?;

        let shifted = render
            .children
            .iter()
            .find(|child| child.key == "b")
            .ok_or(PresenceError::MissingKey)?;
        assert!(!shifted.initial_allowed);
        Ok(())
    }

    fn rendered_keys(render: super::PresenceRender) -> Vec<String> {
        render.children.into_iter().map(|child| child.key).collect()
    }
}
