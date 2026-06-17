# AnimatePresence Spec

## Goal

Add a Framer Motion-inspired presence layer to `dioxus-motion` that lets keyed Dioxus children run exit animations after they are logically removed, then unmounts them when the exit work is complete.

This feature must provide a reliable retained-child lifecycle, a clear manual completion path, and a declarative helper for common value-driven exits. It must define the complete public contract for presence retention, sequencing, cancellation, nested propagation, custom data, and layout popping.

## Upstream Model

Framer Motion's `AnimatePresence` works by diffing direct child keys between renders. Children missing from the latest render are kept in an internal rendered-child list, marked as exiting, and removed only after all registered exit animations report completion.

Core upstream semantics to preserve:

- Direct children need stable, unique keys.
- Removed children remain mounted while exiting.
- `sync` mode allows entering and exiting children to coexist immediately.
- `wait` mode delays entering children until all exiting children complete, and should be treated as a single-child transition mode.
- `initial = false` suppresses initial animations for children present on the first render only.
- `custom` data from the render that detects removal remains available to exiting children after the parent state that created them is gone.
- `on_exit_complete` fires after every currently exiting child has completed.
- `use_presence` exposes `is_present` plus a `safe_to_remove` callback for manual exit orchestration.
- `use_is_present` exposes read-only presence state.
- Nested presence trees should support propagation so a parent exit can trigger descendant exits.
- Layout animation is owned by the animated element, not by `AnimatePresence`. In Framer this is `motion.div layout`; in this crate the equivalent public surface is `use_presence_style(presence_style! { layout: size, ... })`.

Framer Motion also has `popLayout`, which measures exiting DOM nodes and temporarily absolutely positions them so siblings reflow immediately. The Dioxus API must expose the same semantic mode, with target-specific implementation behavior defined below.

Important Framer implementation details to preserve conceptually:

- `AnimatePresence` stores a rendered-child list separate from the latest input children. Removed keyed children are spliced back into that rendered list at their previous index, marked not present, and retained until every registered exit has completed.
- `PresenceChild` owns a descendant completion map. Each motion descendant or manual `usePresence` user registers, marks itself incomplete for the current exit generation, and later reports completion. The child boundary only reports complete when every registered descendant has completed.
- `PopChild` does not collapse the exiting element's height. It measures the DOM node at the transition from present to exiting, then pins that same visual box with explicit position, width, and height until exit completion.
- Framer uses `getSnapshotBeforeUpdate` for the last pre-exit measurement. Dioxus does not expose the same React lifecycle hook, so the implementation must maintain a recent present-layout snapshot and use that snapshot when `is_present` flips false.
- Framer injects a temporary CSS rule targeting a unique data attribute rather than overwriting user inline styles. Dioxus can initially apply the pop style to an owned wrapper, but it must not mutate or clobber the retained child vnode's style.
- Framer supports `anchorX`/`anchorY` because absolute coordinates can drift in RTL, right-anchored, bottom-anchored, resized, or transformed containers.
- Layout projection and presence exit are separate systems. Popping an exiting element removes it from normal flow; sibling layout animation is handled by projection/snapshot math, not by clipping the exiting child with `overflow: hidden` or `height: 0`.
- Framer layout projection snapshots boxes before the DOM update, measures boxes after the DOM update, computes the delta, applies an inverse transform, then animates that transform back to identity. Dioxus should preserve this conceptual model even if the implementation uses a simpler root-level projection registry rather than Framer's full projection tree.
- Projection styles must compose with existing width/height/layout transitions. Applying a projection transform must not overwrite unrelated transitions on the same owned element.
- Framer's layout behavior is not just sibling delta animation. It is built on a projection node tree that can reason about nested parent-relative transforms, scale correction, scroll and resize changes, layout groups, `layoutId`, and shared layout promotion/relegation. A root-level Dioxus projection registry is only a limited first step and must not be documented or tested as equivalent to Framer's full `layout` system.
- Framer composes projection transforms with user-authored transforms through its visual/projection pipeline. Dioxus must not write projection `transform` styles onto the same DOM node that owns animated user transforms unless there is a deterministic transform compositor. Otherwise projection can clobber `use_presence_style` transforms, user inline transforms, or transitions.

## Local Fit

`dioxus-motion` currently has a per-value animation model centered on `use_motion` and `MotionHandle<T>`. That API drives individual animated values and does not own a keyed child registry or component-tree retention.

The new feature should therefore be a Dioxus-gated presence subsystem, not a change to the core animation math types:

- Add a generic child-presence layer in a new `presence` module.
- Reuse `MotionHandle` and existing animation configs for value animation inside retained children.
- Keep route transitions as a separate higher-level feature. Existing route transition code already keeps old routes alive until animation settlement, but it is route/context-specific and should not become the generic presence implementation.
- Export the stable public API through `prelude`.

## Proposed Public API

Names are Rust-style, while keeping the Framer Motion concepts recognizable.

```rust
use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
fn TodoList(items: Vec<Todo>) -> Element {
    rsx! {
        AnimatePresence {
            mode: PresenceMode::Sync,
            on_exit_complete: move |_| tracing::debug!("all exits complete"),
            for item in items {
                TodoRow {
                    key: "{item.id}",
                    item
                }
            }
        }
    }
}
```

### Components and Hooks

```rust
#[component]
pub fn AnimatePresence(
    children: Element,
    mode: Option<PresenceMode>,
    anchor_x: Option<PresenceAnchorX>,
    anchor_y: Option<PresenceAnchorY>,
    initial: Option<bool>,
    propagate: Option<bool>,
    custom: Option<PresenceCustom>,
    on_exit_complete: Option<EventHandler<()>>,
) -> Element;

pub enum PresenceMode {
    Sync,
    Wait,
    PopLayout,
}

pub fn use_is_present() -> bool;

pub fn use_presence() -> PresenceHandle;

pub struct PresenceHandle {
    pub is_present: bool,
    pub safe_to_remove: Callback<()>,
}

pub fn use_presence_data<T: Clone + 'static>() -> Option<T>;
```

`AnimatePresence` also exposes pop-layout anchoring controls:

```rust
pub enum PresenceAnchorX {
    Left,
    Right,
}

pub enum PresenceAnchorY {
    Top,
    Bottom,
}
```

These controls apply to `PresenceMode::PopLayout`. They should not be used as a replacement for element-owned layout configuration.

`children: Element` is the required public contract. `AnimatePresence` must normalize the rendered `VNode` into a flat ordered list of direct presence children by reading public Dioxus core structures:

- `Element` is `Result<VNode, RenderError>`.
- `VNodeInner.key` exposes the key for a root template. In fragments, it is the key of the first child.
- `VNodeInner.dynamic_nodes` exposes dynamic roots.
- `DynamicNode::Fragment(Vec<VNode>)` exposes RSX iterator output as child `VNode`s.
- `DynamicNode::Component(VComponent)` identifies a component child; its root `VNode` key is the presence key.

The supported RSX shape is direct keyed roots under `AnimatePresence`, including children produced by `for` loops:

```rust
rsx! {
    AnimatePresence {
        for item in items {
            div {
                key: "{item.id}",
                "{item.title}"
            }
        }
    }
}
```

If a direct child has no key, `AnimatePresence` must emit a clear development diagnostic and treat it as invalid input. If any sibling is keyed, all siblings must be keyed, matching Dioxus's own keyed sibling invariant.

### Declarative Exit Helper

Because dioxus-motion does not currently expose Framer-style `motion.div` components, the presence API must include a thin helper for binding presence state to `use_motion`.

```rust
#[component]
pub fn PresenceMotion<T: Animatable + Clone + 'static>(
    initial: Option<T>,
    animate: T,
    exit: T,
    config: AnimationConfig,
    render: Callback<T, Element>,
) -> Element;
```

`PresenceMotion` should:

- read `use_is_present`;
- animate from `initial` to `animate` on mount unless the nearest presence context suppresses initial animation;
- animate to `exit` when `is_present` becomes false;
- call `safe_to_remove` exactly once when the exit animation completes;
- remain a thin helper over `use_motion`, not a new animation engine.

If generic `render: Callback<T, Element>` is not ergonomic in Dioxus, expose `use_presence_motion(initial, animate, exit, config) -> MotionHandle<T>` instead. The chosen API must preserve the same completion semantics.

## Configuration

`PresenceMode::Sync`:

- Default mode.
- New children render immediately.
- Exiting children remain rendered until completion.
- Multiple children may enter and exit at the same time.

`PresenceMode::Wait`:

- Intended for one-at-a-time view transitions.
- If one child exits, new children are held until all exiting children complete.
- Development builds should warn or panic clearly when multiple present children are supplied in wait mode. Prefer a debug assertion or structured warning over silent odd behavior.

`initial`:

- Defaults to `true`.
- When `false`, children present on the first render should be marked as present without running initial enter behavior.
- Later inserted children should still be eligible for enter animation.
- Later inserted children with `layout: size` should also be eligible for layout enter animation. They must not take their final layout slot immediately unless the user has opted out of layout animation.

`custom`:

- Stores parent-provided data in presence context.
- When a key is removed, the exiting record captures the nearest `AnimatePresence custom` value from that same render, then keeps it stable for the retained exit lifetime.
- This is not necessarily the old child prop value from when the child was last present.
- `PresenceCustom` must be an owned, cloneable value that can survive after the logical source child has been removed. The implementation may use typed context or type-erased storage, but the public API must keep `use_presence_data<T>()` type-safe and must return `None` on type mismatch.

`propagate`:

- Defaults to `false`.
- When `false`, a nested `AnimatePresence` removed by a parent should be removed as a unit; its descendants do not get an extra exit lifecycle from the nested boundary.
- When `true`, a nested `AnimatePresence` should register itself as a child presence subscriber with the parent. When the parent marks it not present, the nested boundary marks its retained children as exiting. The parent must wait for the nested boundary to call its own `safe_to_remove`.
- Nested completion must flow upward exactly once per boundary/generation.

## State Machine

Each tracked child should have a stable record:

```rust
struct PresenceRecord {
    key: PresenceKey,
    vnode: Element,
    state: PresenceState,
    custom: PresenceCustom,
    generation: u64,
    last_order: usize,
    initial_allowed: bool,
    layout_enter_allowed: bool,
    subscribers: SubscriberRegistry,
}

enum PresenceState {
    Entering,
    Present,
    Exiting,
}
```

Parent render flow:

1. Receive `children: Element` and normalize it into an ordered list of direct keyed `VNode` children.
2. Reject or clearly warn on duplicate keys before mutating retained state.
3. Diff incoming keys against retained records.
4. For new keys, create `Entering` records unless this is the first render with `initial = false`, in which case create `Present` records.
5. For existing keys, update the rendered vnode, update `last_order`, mark as `Present`, and cancel any stale exit for that generation.
6. For removed keys, keep the previous vnode, mark as `Exiting`, increment or preserve a generation token for that exit, and capture the nearest `custom` value from the removal render.
7. Render records according to mode.
8. Remove an exiting record only after every registered child has called `safe_to_remove` or the automatic exit completion path reports complete.
9. When the last exiting record is removed, call `on_exit_complete`.

The state machine must distinguish first-render children from later inserted children for layout purposes. `initial = false` suppresses first-render value animation, but it must not permanently disable enter behavior. Later inserted records should have `layout_enter_allowed = true`; first-render records should have `layout_enter_allowed = false` so initial page layout is not collapsed or measured into overlapping zero-height slots.

Ordering rules:

- Present records render in the order supplied by the normalized RSX children.
- Exiting records keep their last known order index from the render before removal.
- In `sync`, merge present and exiting records by last known order so list exits do not jump.
- In `wait`, render only exiting records while exits are pending; once none remain, render the latest pending present entries.
- In `popLayout`, use the same retention and completion rules as `sync`, but remove exiting records from normal layout flow where the target platform supports layout measurement and absolute positioning.

Cancellation rules:

- If a key is removed and then re-added before exit completes, cancel the exit and make the record present again.
- If a key is replaced with a different key, treat it as one exit and one enter.
- If the parent `AnimatePresence` unmounts, drop retained records and clear completion callbacks. Do not leave async tasks or callbacks holding stale scopes.
- If a child never registers any presence-aware animation or manual `use_presence` subscriber, it should be safe to remove immediately.

## Registration Protocol

Presence completion should be context-owned and generation-bound.

```rust
struct PresenceContext {
    key: PresenceKey,
    generation: u64,
    is_present: bool,
    initial_allowed: bool,
    custom: PresenceCustom,
    register: Callback<RegisterPresenceSubscriber, PresenceSubscriberToken>,
    complete: Callback<PresenceSubscriberToken>,
}

struct PresenceSubscriberToken {
    key: PresenceKey,
    generation: u64,
    subscriber_id: SubscriberId,
}
```

Rules:

- `use_presence` registers a subscriber with the nearest presence record and returns a `safe_to_remove` callback bound to the returned token.
- `PresenceMotion` also registers a subscriber while it owns an exit animation.
- Registration is idempotent per hook instance. Re-renders must not create unbounded subscriber entries.
- Completion is idempotent per token.
- Completion with a stale generation is ignored.
- Dropping/unmounting a subscriber before exit starts unregisters it.
- Dropping/unmounting a subscriber during exit should either mark that subscriber complete or unregister it and re-evaluate whether the record can now be removed. The implementation must choose one behavior and test it.
- A record that enters `Exiting` with zero subscribers is removed on the next presence reconciliation.
- A nested `AnimatePresence` with `propagate = true` registers one subscriber token with its parent and completes it only after its own exiting descendants have completed.

## Completion Integration

The presence layer needs one automatic path and one manual path.

Automatic path:

- `PresenceMotion` or `use_presence_motion` must report completion automatically.
- If users bypass that helper, use `use_presence_motion_completion` to bind a raw `MotionHandle` to presence removal without capturing Dioxus callbacks inside `AnimationConfig::with_on_complete`.
- The implementation should avoid global mutable registries. Completion should flow through Dioxus context owned by the nearest `AnimatePresence`.

Manual path:

```rust
#[component]
fn Toast(message: String) -> Element {
    let presence = use_presence();
    let mut opacity = use_motion(1.0);
    let mut exit_started = use_signal(|| false);

    use_effect(move || {
        if !presence.is_present {
            exit_started.set(true);
            opacity.animate_to(
                0.0,
                AnimationConfig::new(AnimationMode::Tween(Tween::default())),
            );
        }
    });
    use_presence_motion_completion(opacity, exit_started());

    rsx! {
        div {
            style: "opacity: {opacity.get_value()}",
            "{message}"
        }
    }
}
```

The callback must satisfy the current `AnimationConfig` bounds. Direct capture of a Dioxus callback is not `Send + 'static`, so `use_presence_motion_completion` is the presence-safe adapter for raw `use_motion` exits.

## Presence Style API Direction

Presence style ergonomics should follow the Motion/Framer model rather than exposing one hook per CSS property.

The desired docs-facing shape is:

```rust
let style = use_presence_style(presence_style! {
    initial: { opacity: 0.0, y: 20.0, scale: 0.92 },
    animate: { opacity: 1.0 },
    exit: { opacity: 0.0, y: -16.0, scale: 0.96 },
    transition: spring { stiffness: 180.0, damping: 22.0 },
});
```

Design notes from Motion's implementation:

- Motion's public style type accepts broad React CSS properties, then treats transforms and known value types specially at render time.
- Transform shorthands are a curated API. They are not applied as literal CSS properties; they are composed into a single `transform` string in a stable order.
- Numeric CSS properties use a property-to-value-type map for default units, for example width-like properties use pixels while opacity stays unitless.
- Animatability is value-based as well as property-based. Numbers, compatible dimensions, colors, and compatible complex strings can animate; values such as `display: "none"` or `url(...)` should not be treated as interpolable.
- CSS property-name validation and animatable-value support are separate concerns. Dioxus has a generated list of style attribute names in `dioxus-html`, but that list does not tell us how to interpolate each value.
- Framer's broad CSS support comes from value archetypes, not from hardcoding every property. A property is accepted when its value can be represented as an animatable number, dimension, color, or compatible complex string. Transform shorthands are handled separately because they compose into one `transform` declaration.

Near-term presence style API requirements:

- Keep `presence_style!` focused on a known animatable subset rather than accepting every CSS property.
- Support Framer-style transform shorthands first: `x`, `y`, `z`, `translateX`, `translateY`, `translateZ`, `scale`, `scaleX`, `scaleY`, `scaleZ`, `rotate`, `rotateX`, `rotateY`, `rotateZ`, `skew`, `skewX`, `skewY`, and `transformPerspective`. Rust-friendly snake-case aliases such as `scale_y`, `rotate_x`, and `transform_perspective` should map to the same fields.
- Use CSS-friendly units in the macro surface: translations and perspective are pixels, rotations and skews are degrees, and opacity/scales are unitless.
- Compose transform output in a stable order instead of relying on users to write raw `transform` strings.
- Support symmetric transition syntax:

```rust
transition: tween { duration: 300.0 }
transition: tween { duration: 300.0, easing: easer::functions::Cubic::ease_in_out }
transition: spring { stiffness: 180.0, damping: 22.0 }
```

- Preserve fractional millisecond durations instead of truncating them.
- `transition` applies to both enter and exit. Users can override each side explicitly:

```rust
enter_transition: spring { stiffness: 180.0, damping: 22.0 },
exit_transition: tween { duration: 140.0 },
```

- Exit completion is based on the actual motion handle settling or stopping, not only a wall-clock sleep equal to the nominal transition duration. Otherwise frame scheduling can remove exiting children before the final exit frame is rendered.
- `layout: size` enables element-owned layout slot animation. It belongs in the presence style config for the element/hook that should animate layout, not on `AnimatePresence`.
- `layout: size` is a pragmatic slot-size animation surface, not full Framer layout projection by itself. It may use measured wrapper `width`/`height` for enter and exit slot expansion/collapse, but it must be documented separately from Framer-equivalent projection until a real projection tree exists.
- Layout wrappers must not clip or collapse retained exiting children before their exit transition completes. If layout size animation is enabled, an exiting child wrapper preserves its measured size for the first exit frame, then animates its layout slot to `0px` while the retained child performs its own exit transform/opacity animation.
- Later inserted `layout: size` children must animate their layout slot from `0px` to the measured final height/width after mount. The inserted child vnode may mount immediately so its own initial transform/opacity animation can run, but the wrapper slot must start collapsed and expand smoothly.
- First-render `layout: size` children must render at natural size. They must not be collapsed or overlaid during initial page load.
- After a `layout: size` enter expansion completes, the wrapper should settle back to natural layout (`auto`/visible) rather than keeping stale measured dimensions indefinitely.
- Unknown macro fields should be accepted as CSS properties when their values convert into a supported animated value type. Field names are normalized from Rust/JS style to CSS names, for example `borderRadius` and `border_radius` both become `border-radius`.

Future style API requirements:

- Numeric CSS values should be value-typed, not property-hardcoded. Unitless CSS properties such as `opacity`, `z-index`, and flex factors remain unitless; other numeric properties default to pixels.
- Numeric layout properties that often start as `auto`, especially `height` and `width`, need a measured start value when they only appear in the exit target. Before starting exit, seed the current animated style from the latest measured presence box so `exit: { height: 0.0 }` animates from the rendered height to `0px` instead of jumping immediately.
- Add explicit value constructors such as `px(...)`, `percent(...)`, `deg(...)`, and color constructors or parsers as the value archetype surface grows.
- Model animated style as a map of style properties to typed animated values. Do not grow `PresenceStyle` into an unbounded CSS struct by hand.

## Architecture Requirements

- Keep the presence subsystem Dioxus-gated. Non-Dioxus core animation types should not depend on VNodes, scopes, contexts, or DOM APIs.
- Do not couple generic presence to router-specific transition internals.
- Prefer parent-owned state with explicit generations over scattered child-local state. This makes re-add cancellation, stale completion filtering, and `wait` sequencing tractable.
- Prefer Framer's ownership model for layout: `AnimatePresence` owns key diffing, retention, sequencing, custom data, and pop-layout anchoring; the animated element/hook owns `layout: size`, layout transition timing, projection registration, and layout slot animation.
- Keep the Framer ownership boundary explicit in code: retained presence state and descendant completion belong to the presence boundary, while projection measurement, transform composition, and scale/layout correction belong to a layout-projection subsystem. Do not let `AnimatePresence` become a layout engine.
- Keep completion idempotent. Multiple `safe_to_remove` calls for the same child/generation should not double-remove or double-fire `on_exit_complete`.
- Preserve child order. Exiting records must appear at their previous positions in `sync` mode.
- Public APIs need Rust doc comments with examples.
- Avoid broad feature flags. Presence depends on Dioxus and should be gated with the existing `dioxus` feature. DOM measurement for `popLayout` must be gated behind platform capabilities such as `web`.
- Projection coordination must avoid panicking runtime borrows. If a projection pass is already active, the implementation may skip that best-effort pass, but it must not crash the app.
- Projection must have a clear ownership target. Either projection writes to a dedicated wrapper that never owns user transforms, or projection and user transforms are combined through a compositor before writing `transform`. Removing/restoring `transform` directly on a user-owned element is not acceptable for Framer-aligned behavior.
- Wrapper size animation must not be mistaken for projection. It is acceptable as a near-term size-collapse/expand feature, but full Framer parity requires snapshot-before/measure-after projection with nested parent-relative correction.

## Feature Requirements

- `AnimatePresence`
- `PresenceMode::Sync`
- `PresenceMode::Wait`
- `PresenceMode::PopLayout`
- `initial`
- `on_exit_complete`
- `use_is_present`
- `use_presence`
- `custom` / `use_presence_data`
- `propagate`
- `PresenceMotion` or `use_presence_motion`
- immediate removal for children with no registered exit/manual subscriber
- re-add cancellation by key
- robust nested completion
- DOM measurement and style injection for `popLayout` where platform APIs support it
- graceful, documented non-DOM behavior for `popLayout`
- anchor controls for popped layout when absolute positioning is supported
- element-owned `layout: size` configuration through `presence_style!`
- inserted-child layout slot expansion from zero to measured size
- exiting-child layout slot collapse from measured size to zero
- snapshot-before/measure-after projection for present siblings with transform-to-identity animation
- transition composition between projection transforms and width/height layout transitions
- deterministic transform composition between projection transforms and user-authored/animated transforms
- documented limitation for root-level projection until nested projection nodes, scale correction, scroll/resize coordination, layout groups, `layoutId`, and shared layout are implemented
- integration helper for raw `use_motion` exit completion
- examples for list item exits, single-view wait transitions, declarative motion exits, manual `safe_to_remove`, custom data, propagation, and pop layout
- focused state-machine and VirtualDom tests
- docs guide and examples

## Layout Size and Projection Design Notes

`layout: size` is the crate's near-term slot-size animation API. It should be configured on `use_presence_style`, not on `AnimatePresence`.

Do not describe the initial `layout: size` implementation as full Framer `motion.div layout` parity unless the projection subsystem has the same architectural guarantees. Framer's `layout` is based on projection nodes, not only wrapper height/width transitions. The Dioxus API may expose a familiar user-facing shape, but the docs must distinguish:

- slot-size animation: measured wrapper expansion/collapse for ordinary enter and exit layout space;
- sibling projection: snapshot-before/measure-after transform animation for present siblings;
- full layout projection: nested projection nodes with parent-relative deltas, scale correction, scroll/resize handling, layout groups, `layoutId`, and shared layout transitions.

Required behavior:

- Present first-render children render naturally. Initial page layout must not require a measurement pass to avoid overlapping or zero-height rows.
- Later inserted children render their child content immediately, but the owned layout wrapper starts with a collapsed slot (`height: 0px` for vertical flow). After measurement, the wrapper animates to the measured size using the layout transition.
- Exiting non-pop children keep their measured slot for the first exit render, then animate that slot to zero. This lets the child run its own exit style while surrounding layout closes smoothly.
- `popLayout` children do not use this collapse path. A popped exiting child is removed from normal flow immediately and visually pinned from its measured position.
- Present sibling movement should use projection: snapshot registered layout nodes before reconciliation/render, measure after the update commits, apply the inverse transform, then animate to identity.
- Projection should write only owned projection styles (`transform`, `transform-origin`, `will-change`, and composed transition data). It must preserve and restore unrelated transition state, especially `width` and `height` transitions.
- Projection must compose transform output with any transform produced by `presence_style!`, raw inline styles, or other motion hooks. If composition is not implemented, projection must target a wrapper whose transform is reserved exclusively for projection.
- Projection timing uses the element's layout transition when supplied; otherwise it falls back to the normal enter transition duration.
- The current implementation may use a root-level projection registry for simple sibling movement only. A future full Framer-style projection tree would add nested parent-relative projection, scale correction, scroll/resize handling, layout groups, `layoutId`, and shared layout promotion/relegation. The root-level implementation must not claim to support those features until they are implemented and tested.
- A projection pass must not remove a user-owned `transform` or `transition` property as a reset strategy. If reset is required, store projection-owned state separately or write projection to an isolated wrapper.

## Child Normalization Requirements

`AnimatePresence` must support idiomatic RSX children. The normalizer must:

- propagate `RenderError` from `children` instead of swallowing it;
- treat a single keyed root `VNode` as one presence child;
- flatten a root `DynamicNode::Fragment(Vec<VNode>)` into ordered direct children;
- preserve each child `VNode` unchanged so Dioxus can render the retained subtree normally;
- read each child key from `VNodeInner.key`;
- reject duplicate keys;
- reject missing keys for presence-managed siblings;
- preserve caller order across static roots, dynamic fragments, and component children;
- avoid inspecting component props or relying on private `VComponent` internals.

The normalizer must not introduce a separate builder DSL as the primary API. A helper macro may exist only as syntactic sugar around ordinary RSX children if it remains fully compatible with this normalization model.

## Pop Layout Design Notes

`PresenceMode::PopLayout` must:

- Measure the exiting node before layout changes. If the platform lacks a pre-update measurement lifecycle, keep a last-known present snapshot and update it while the child is present.
- Treat last-known present snapshots as an approximation of Framer's `getSnapshotBeforeUpdate` behavior. They must be refreshed while present and tested against rapid remove/re-add, resize-before-removal, transformed ancestors, and right/bottom anchored layouts.
- Temporarily position the exiting node at its measured coordinates with explicit width and height.
- Let siblings reflow immediately.
- Preserve the exiting child vnode's own inline styles; apply pop positioning to an owned wrapper or a temporary selector.
- Remove injected styles or wrapper positioning when exit completes.
- Never implement pop layout by animating the exiting child's wrapper to `height: 0` or clipping with `overflow: hidden`. That visually truncates slower exit animations and couples layout timing to exit timing.
- Keep pop layout separate from layout projection. Sibling layout animation should be a separate transform/snapshot feature rather than a side effect of presence retention.
- Be web-gated where DOM APIs are required.
- Gracefully degrade to retained `sync` semantics on targets without DOM layout measurement.
- Inject positioning with a generated selector or equivalent owned wrapper rule rather than overwriting the retained child's inline style.
- On web, measure the exiting element and its offset parent, temporarily ensure a static offset parent is positioned, and calculate absolute coordinates relative to that parent.
- Expose `root` or equivalent style-injection target configuration on web if the implementation needs to inject outside the component subtree.
- Expose horizontal and vertical anchor controls for absolute positioning.
- Document that transformed ancestors affect layout positioning.
- Prefer a temporary generated selector or data attribute for pop positioning, following Framer's non-destructive style injection model. Applying pop positioning to an owned wrapper is acceptable only if it never clobbers the retained child vnode's inline styles or transform animation.

## Testing Requirements

Minimum tests:

- A removed keyed child remains in the rendered records until `safe_to_remove`.
- `on_exit_complete` fires once after all current exits complete.
- Re-adding the same key before completion cancels the exit and prevents removal.
- `sync` renders entering and exiting children together.
- `wait` suppresses entering children until exits complete.
- `initial = false` only affects first-render children.
- Calling `safe_to_remove` twice is harmless.
- A child without registered exit subscribers is removed immediately.
- `custom` captured on the removal render remains visible through `use_presence_data` during exit.
- A stale `safe_to_remove` from an older generation cannot remove a re-added child.
- Subscriber unregister during present state does not block later removal.
- Subscriber unregister during exit follows the documented behavior and cannot leak records.
- Nested propagation marks nested children exiting when enabled and does not when disabled.
- Duplicate keys are rejected or warned with a clear diagnostic.
- `popLayout` measures an exiting node, removes it from normal flow, cleans up injected styles, and degrades predictably where layout APIs are unavailable.
- `layout: size` inserted children animate their layout slot from zero to measured size without breaking first-render natural layout.
- `layout: size` exiting children preserve measured size for the first exit frame before collapsing.
- Projection transform transitions compose with existing width/height transitions.
- Projection transform output composes with user-authored transforms and `use_presence_style` transforms, or uses a dedicated projection wrapper so transforms cannot clobber each other.
- Projection coordination does not panic on re-entrant render/effect scheduling.
- Root-level projection is covered by limitation tests for nested layouts, transformed parents, scroll/resize, and reordering. Unsupported scenarios should degrade predictably or be documented as unsupported.
- `popLayout` uses a fresh enough snapshot when the element is resized or reflowed immediately before removal.
- `popLayout` handles rapid remove/re-add without applying a stale absolute-position rule to a present child.

Recommended verification commands:

```bash
cargo test --all-features
cargo clippy --workspace --all-features -- -D warnings
cargo test --features transitions
cargo check -p docs --features web
```

## Documentation Requirements

Add user-facing docs in the existing guide style:

- What presence solves.
- Keyed child requirement.
- Basic conditional rendering example.
- List item exit example.
- `wait` mode example.
- `PresenceMotion` or `use_presence_motion` example.
- Manual `use_presence` example.
- Nested presence and propagation example.
- `popLayout` example and platform notes.
- Troubleshooting:
  - missing/stable keys,
  - duplicate keys,
  - `wait` with multiple children,
  - child removed immediately because no exit completion subscriber was registered,
  - stale custom data expectations,
  - platform limitations for `popLayout`.

Update README only after the API is implemented. Avoid hardcoding crate version numbers in examples unless release automation requires it.

## Code Quality Bar

- The implementation should be small enough to reason about as a state machine.
- Presence state transitions should be covered by focused tests before polishing examples.
- Public APIs must have doc comments and examples.
- The state model must make stale callbacks impossible to apply to a new generation of the same key.
- The implementation should not require users to manually coordinate unmounting in the common case.
- Error messages or debug assertions should point to the exact contract violation: missing key, duplicate key, or invalid `wait` usage.
- Keep source changes focused. Do not refactor core animation math while adding presence.

## API Decisions

- `AnimatePresence` accepts RSX `children: Element` and normalizes public Dioxus `VNode`/`DynamicNode` structures into keyed presence records.
- Presence lives behind the existing `dioxus` feature, not the route-specific `transitions` feature.
- `PresenceCustom` must support type-safe retrieval through `use_presence_data<T>()`.
- The declarative helper may be a component or a hook, but it must bind `use_motion` to presence state and call `safe_to_remove` on exit completion.
