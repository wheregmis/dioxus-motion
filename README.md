# Dioxus Motion

[![Crates.io](https://img.shields.io/crates/v/dioxus-motion.svg)](https://crates.io/crates/dioxus-motion)
[![API](https://docs.rs/dioxus-motion/badge.svg)](https://docs.rs/dioxus-motion)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Typed, interruptible animation for Dioxus. Animate a value, read it in your component, and let springs or tweens carry it to the next destination.

[Documentation & playground](https://wheregmis.github.io/dioxus-motion/) · [Examples](docs/src) · [Release notes](CHANGELOG.md)

## Development API

This branch targets **Dioxus 0.7.10** and **Rust 1.89+**. It contains breaking changes for the next release. Examples below describe this branch; use the published [API documentation](https://docs.rs/dioxus-motion) when depending on the crates.io release.

For a web app:

```toml
[dependencies]
dioxus = { version = "0.7.10", features = ["web"] }
dioxus-motion = { git = "https://github.com/wheregmis/dioxus-motion", branch = "new_release", default-features = false, features = ["web"] }
```

Commit `Cargo.lock` to pin the Git revision. For desktop apps, use the `desktop` feature. Add `transitions` for animated router outlets. The animation core is also available without the default features.

## Your first spring

```rust
use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
fn MovingButton() -> Element {
    let mut x = use_motion(0.0_f32)?;
    let mut forward = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    rsx! {
        button {
            onclick: move |_| {
                let target = if forward() { 0.0 } else { 160.0 };
                match x.animate_to(target, AnimationConfig::spring(Spring::default())) {
                    Ok(()) => { forward.toggle(); error.set(None); }
                    Err(problem) => error.set(Some(problem.to_string())),
                }
            },
            style: "transform: translateX({x.get_value()}px)",
            "Move me"
        }
        if let Some(problem) = error() { p { role: "alert", "{problem}" } }
    }
}
```

Retargeting a running spring starts from its current value and momentum. Reading `get_value()` subscribes the component to value changes; reading `is_running()` subscribes to playback changes.

The [live quick start](docs/src/components/quick_start.rs) is compiled as part of the documentation app, and the docs display that same file.

## Pick the right animation

| Need | API | Guide |
| --- | --- | --- |
| Natural motion with momentum | `AnimationConfig::spring(Spring { stiffness, damping, mass })` | [Values & springs](docs/src/pages/basic_guide.rs) |
| A fixed duration | `AnimationConfig::tween(Duration)` | [Loops & sequences](docs/src/pages/intermediate_guide.rs) |
| Ordered steps or explicit keyframes | `AnimationSequence`, `KeyframeAnimation` | [Sequences](docs/src/pages/intermediate_guide.rs) |
| Typed CSS properties | `MotionStyle`, `motion_style!` | [Animating CSS](docs/src/pages/motion_style_guide.rs) |
| Enter, exit, and layout coordination | `AnimatePresence`, `use_presence_motion`, `use_presence_style` | [Presence](docs/src/pages/presence_guide.rs) |
| Route changes | `MotionTransitions`, `AnimatedOutlet` | [Transitions](docs/src/components/page_transition.rs) |
| Your own data | `Animatable` | [Custom types](docs/src/pages/complex_guide.rs) |

## Predictable values and errors

Construction, hooks, animation setup, velocity changes, and frame updates return typed errors. Handle `AnimationError` at the boundary where you accept user input or configure motion.

Presence hooks validate initial, animate, and exit values and their transitions before registering work. `PresenceConfig::validate()` also checks the optional layout transition. Spring entry must support re-entry from the exit state; use a tween when those states use incompatible CSS units.

- Initial values, targets, velocities, and frame results must be finite. `Animatable::is_finite` checks every numeric component of a custom type.
- Invalid setup preserves existing playback. A failed frame stops playback and retains the last valid value.
- CSS springs require compatible units and complex shapes for shared properties. Use a tween for changing units or discrete values. Invalid spring transitions return `IncompatibleSpringValues` before playback changes.
- `set_velocity` takes the animated value’s type and requires an active spring. A scalar spring accepts `f32`; a transform spring accepts a `Transform` velocity.
- Custom values implement `Clone + PartialEq + Animatable`. Exact equality controls reactive updates; epsilon controls spring convergence.
- Read-only motion selectors do not expose mutable animation state. Use the checked setters to change values or playback.

See [CHANGELOG.md](CHANGELOG.md) for the migration details, including the removed closure pool API.

## Performance and validation

Springs use a closed-form step with cached coefficients. Keyframe lookup uses a short scan for small tracks and binary search for larger tracks. Motion signals notify only when their observed value changes.

The repository includes numerical stress tests, frame-error regressions, mutation tests, and a real-browser timing/cancellation harness. Benchmarks are measurements for a particular build and machine; they are not a guarantee of application frame rate.

```sh
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --release test_motion_update_cpu_usage -- --ignored --nocapture
```

For documentation development and the GitHub Pages deployment, see [docs/README.md](docs/README.md).

## Contributing

Keep examples aligned with the checked API, test changes at their observable boundary, and prefer a small implementation that is easy to maintain. Open an issue or pull request with a reproducible case.

## License

[MIT](LICENSE).
