# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Remove the allocation and redundant spin-sleep from native frame delays. Use Tokio virtual time in delay tests so machine load cannot create false failures.

- Reject nonfinite epsilon values in validation. Keep scalar and transform interpolation finite for opposite extreme endpoints, preserve exact translation/scale endpoints, and wrap rotations spanning multiple turns along the shortest path.

- Update documentation examples to current Dioxus component syntax and compile-check them; highlight guide examples at compile time with `dioxus-code`.

### Changed

- Breaking: presence tween `duration` fields take `Duration` instead of floating-point milliseconds. Use `Duration::from_millis(350)` or `Duration::from_micros(220_500)`; dynamic seconds can use `Duration::try_from_secs_f64`. The macro no longer performs a conversion that can panic on negative, nonfinite, or overflowing input.

- Rebuild the six documentation lessons with compiled-source previews, concrete exercises, ordered navigation, and isolated route-transition history. Remove duplicate unrouted guide examples.

- Breaking: `TimeProvider::delay` returns `Result<(), AnimationError>`; failed scheduling returns `TimerUnavailable` and stops the motion driver instead of spinning. Later playback requests on that failed hook also return `TimerUnavailable`. Presence layout falls back to immediate settlement.

- Springs reject shared CSS properties with incompatible units or complex-string shapes before replacing active playback, including sequence steps and velocity changes. Use tweens for discrete transitions. Removing numeric properties now springs toward zero before dropping them at completion.
- Style fields and typed/complex CSS numbers reuse overflow-safe scalar interpolation. Style tween frames avoid temporary vector arithmetic/property maps, and CSS number formatting widens rounding arithmetic so finite extremes remain finite strings.
- `Animatable` now requires `PartialEq` for exact reactive change detection, including CSS keywords, units, and property keys. Derive or implement it for custom animated values. Store updates avoid cloning the new value and computing an arithmetic difference/magnitude.
- Value subscribers receive every nonzero motion change, including frames smaller than spring completion epsilon. Running subscribers remain independent, and unchanged values do not trigger renders.
- Removed the public `animations::closure_pool` module and its unused benchmark. Browser timers own their JavaScript closures directly; consumers of the legacy registry must likewise own each closure and cancel its scheduled browser request before dropping it.
- Handle updates report running state after completion callbacks, so a callback that restarts motion keeps manually driven frame loops alive. Ordinary frames retain their existing update path.
- Keyframes copy exact segment endpoints and held values without calling easing or interpolation. Exact eased endpoints also bypass interpolation, matching tween behavior.
- Strengthened runtime mutation coverage for exact tween endpoints and bounded keyframe lookup work. Lookup comparison counts are checked through a predicate helper, keeping timing benchmarks free of instrumentation.
- Completion callbacks use nonblocking mutex acquisition. Reentrant/locked callbacks return `AnimationError::CompletionBusy`; poisoned callbacks return `CompletionPoisoned`. `AnimationConfig::execute_completion` now returns `Result<(), AnimationError>`, and motion updates propagate callback errors after finalizing playback and releasing the store guard.
- Removed the unused private config/resource pools and legacy RK4 integrator, along with their standalone tests and config-pool benchmark. Playback already uses exact cached spring transitions and owns browser timer callbacks directly. Updated crate docs to describe those production paths.
- Removed unused `AnimationStep.predicted_next` storage and builder interpolation. Sequence construction now only queues targets/configuration, leaving validation to setup. Capacity/reserve hints use `usize`, matching step indices and `Vec`. Clone tests verify independent progress and single-owner completion callbacks.
- Removed the ignored `Spring.velocity: f32` field and its presence-macro option. `Motion::set_velocity` and `AnimationManager::set_velocity` accept the animated type (`f32`, `Transform`, `Color`, etc.) and reject nonfinite components. Call the setter during active spring playback; idle, tween, and keyframe states return `AnimationError::VelocityRequiresSpring`. Animation setup resets velocity to zero.
- `Motion` fields are no longer publicly mutable. Use `get_value`, `get_target`, `get_velocity`, and `is_running` to inspect state, and `set_velocity` for a checked velocity change. Use animation setup methods to change targets/tracks and `stop`/`reset` for lifecycle control.
- Presence exit completion now removes stopped motion even if a runtime failure occurs before an effect observes its first running frame. Raw motion, presence motion, and presence styles share this completion effect. Measured style preparation rejects nonfinite values before entering a store.
- `Motion::update` and `AnimationManager::update` now return `Result<bool, AnimationError>`. Spring position/velocity and tween/keyframe interpolation are validated before committing a frame; nonfinite easing is rejected before interpolation. Failed playback stops, preserves the last valid value, and skips completion callbacks. The motion hook logs runtime errors.
- `Motion::new`, `AnimationManager::new`, `use_motion`, and presence motion/style hooks now return `Result` so invalid initial components or nonfinite default zero velocity cannot enter a store. Components can propagate errors with `?` to a Dioxus error boundary. Hook initialization validates once per component lifetime, matching the existing initial-value semantics.

- `Animatable` now requires `is_finite()`, checking every numerical component directly. Built-in values validate components independently of magnitude. Motion targets, sequence targets, and keyframe values reject NaN/infinity with typed errors; invalid setup preserves active motion. Add `is_finite` to custom implementations, for example `[self.x, self.y].into_iter().all(f32::is_finite)`.

- Motion springs use the closed-form damped oscillator on native and web, with coefficients cached for repeated frame deltas. Stiff, low-mass, and heavily damped springs no longer diverge through frame integration. Spring trajectories change slightly from the previous RK4/Euler approximation; zero-force springs remain supported.

- `Motion` and `AnimationManager` setup methods `animate_to`, `animate_sequence`, and `animate_keyframes` now return `Result<(), AnimationError>`. Handle the error or propagate it with `?`; fixed, known-valid configurations in examples use `.expect("valid animation configuration")`. Invalid epsilon or spring parameters leave the existing animation unchanged, and sequences validate every step before playback. `AnimationConfig::validate` and `AnimationSequence::validate` expose the checks.

- Color arithmetic now preserves signed, unbounded components so spring displacement, forces, and velocity are valid. `Color::new`, interpolation, and `to_rgba` continue to clamp display values.

- Update Dioxus to 0.7.10 and declare Rust 1.89 as the minimum toolchain.
- Sequence step indices now return `usize`; `Motion::current_loop` is `u16` to support all alternate loop counts.
- `KeyframeAnimation::keyframes` is private. Read frames with `keyframes()` and add frames with `add_keyframe()` to preserve validated, sorted offsets.
- Keyframe insertion preserves ordering without re-sorting the track. Large tracks use binary lookup; small tracks retain linear lookup.
- Remove redundant value clones and comparisons from the hook frame loop; store updates already notify changed values.

### Fixed

- Layout projection owns and cancels its pending frame instead of forgetting RAF closures. Replacing or removing a projection releases its task, and timer failures settle the temporary transform immediately.

- Include alpha in CSS color convergence and combine embedded color/number components as a Euclidean magnitude. Alpha-only style springs now animate instead of snapping at setup. Compound magnitudes use a wider fallback for squared-component overflow/underflow, and spring completion compares magnitudes directly to avoid squaring extreme epsilon values.

- Browser delays own and release their closures, cancel pending RAF/timeout requests when dropped, and finish safely if scheduling fails. Large timeout values saturate instead of wrapping negative.
- Run browser registry callbacks after releasing the registry borrow; remove duplicate in-use tracking and correct the registry documentation.

- Initialize and reset spring velocity to the additive zero value, including transform scale and color alpha; identity defaults no longer introduce motion at rest.

- Ignore invalid frame deltas, preserve time left after delays, and bound spring work after stalls.
- Execute completion callbacks after finalizing the animation and releasing the Dioxus store write, so callbacks can read and restart their handle. Support sequences longer than 255 steps and clear previous animations when starting a sequence.

### Tests

- Check closed-form spring trajectories, coefficient extremes, and alpha-only springs in the real browser/WASM harness. Cache regression tests count coefficient calculations without clock-based assertions.

- Add `just check-browser-delay` for browser completion, cancellation, scheduling failure, timeout bounds, and JS reference retention checks. Requires Chrome, matching ChromeDriver, and a wasm-bindgen CLI matching Cargo.lock (`CHROME_BIN`, `CHROMEDRIVER`, and `WASM_BINDGEN` can override tool paths).

- Compare color and transform spring trajectories with scalar motion in both directions; check zero velocity across lifecycle operations and benchmark all three value types.

- Add deterministic frame-delta fuzzing, exhaustive loop-count checks, sequence boundary checks, and interpolation/physics regression checks informed by mutation testing.
- Keep host-dependent timing measurements as manual benchmarks and remove misleading idle-heavy and battery-simulation checks.

## [0.3.6](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-v0.3.5...dioxus-motion-v0.3.6) - 2026-05-22

### <!-- 2 -->Fixes

- *(ci)* separate release-plz release from release-pr trigger

## [0.3.5](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-v0.3.4...dioxus-motion-v0.3.5) - 2026-04-07

### <!-- 3 -->Other

- release

## [0.1.2](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-transitions-macro-v0.1.1...dioxus-motion-transitions-macro-v0.1.2) - 2026-04-07

### <!-- 3 -->Other

- release
- more dep update

## [0.3.4](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-v0.3.3...dioxus-motion-v0.3.4) - 2026-04-07

### <!-- 3 -->Other

- Release 0.3.4: bump version and docs
- Adjust features, thresholds, and dioxus gating
- Remove resource_pools init call from docs
- Make Store derives conditional & refine animation updates
- Keep docs verification aligned with the final transition defaults
- *(team)* auto-checkpoint worker-1 [unknown]
- *(team)* auto-checkpoint worker-1 [unknown]
- *(team)* auto-checkpoint worker-2 [unknown]
- *(team)* auto-checkpoint worker-3 [unknown]
- *(team)* auto-checkpoint worker-2 [unknown]
- *(team)* auto-checkpoint worker-2 [unknown]
- *(team)* auto-checkpoint worker-3 [unknown]
- Simplify the motion runtime around a direct store-backed engine
- Derive Default and simplify animation logic
- *(team)* checkpoint worker-1 shutdown changes
- *(team)* auto-checkpoint worker-1 [unknown]
- *(team)* auto-checkpoint worker-1 [unknown]
- more dep update
- Update dependency versions and regenerate lockfile

## [0.1.2](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-transitions-macro-v0.1.1...dioxus-motion-transitions-macro-v0.1.2) - 2026-04-07

### <!-- 3 -->Other

- more dep update

## [0.3.4](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-v0.3.3...dioxus-motion-v0.3.4) - 2026-04-07

### <!-- 1 -->New features

- add a clean `default-features = false` core build by gating the Dioxus-only hook/store surface behind the `dioxus` feature
- make the `transitions` feature enable `dioxus` automatically so transition users do not need to wire that dependency surface manually

### <!-- 2 -->Fixes

- refresh release notes and docs around the current published Dioxus `0.7.4` line while keeping the workspace ready for a future `0.7.5` bump once it is available on crates.io
- declare MSRV `1.85.0` for the `Tween` `fn_addr_eq` `PartialEq` implementation
- fix page transition store tests to run inside a real Dioxus runtime instead of constructing stores outside hooks
- remove stale doctest and installation/version drift so release docs match the published crate surface

### <!-- 3 -->Other

- refresh migration guidance for the upcoming release
- update docs site installation snippets and version badges to match the current crate version

## [0.3.3](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-v0.3.2...dioxus-motion-v0.3.3) - 2026-01-17

### <!-- 3 -->Other

- bump

## [0.3.2](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-v0.3.1...dioxus-motion-v0.3.2) - 2025-11-03

### <!-- 1 -->New features

- update edition
- rustfmt **/*.rs --edition 2024
- cargo update

### <!-- 2 -->Fixes

- fixed page transition animation from/to to root route
- fixing animation config explicit conversion

### <!-- 3 -->Other

- update release-me workflow
- Bump Dioxus Version
- Update release-plz.toml
- Update gh_pages.yml
- enable basepath and also commit just file
- Enable gh pages
- Fix formatting
- remove --lib on tests
- small changes
- CI?
- Some fixes for CI
- alternative loop fix and tests added
- more tests
- loop mode fix
- clippy fixes and ci
- Enhance README.md with new SharedValue struct and Animatable trait implementation. Update Animatable trait definition to include operator traits and default methods. Refactor Position struct to implement standard Rust operators and simplify Animatable methods for better usability.
- Refactor AnimationSequence to use Arc<Mutex> for thread-safe completion callbacks, enhancing concurrency support and ensuring safe execution without ownership. Update related methods and tests accordingly.
- Add PoolStatsProvider trait and integrate statistics tracking for SpringIntegratorPool
- Enhance ConfigHandle to track validity and ensure safe automatic cleanup upon drop, preventing double-return issues in the config pool.
- Implement config pool trimming functionality and clean up code formatting in pool.rs
- Refactor motion.rs for improved readability by cleaning up whitespace and formatting in animation methods
- Update .gitignore and refactor AnimationState for improved readability and structure
- simpler tracking
- small benchmarks tests
- Some readme and changelog updates
- proper time support
- more cleanup
- more cleanup
- state machine way
- basic test cleanups
- basic pooling
- dioxus bump to alpha 3
- Export TransitionVariantResolver in prelude module
- Add dynamic transition resolver for page transitions
- Add support for Tween-based page transitions
- simplifying animatable trait
- Clippy Fixes
- Refactor animation frame counter variable naming
- Standardize animation epsilon values
- Refactor timing logic in MotionTime for web platform
- Add new feature to process user input
- Add value caching to Motion for frame optimization
- Optimize animation interpolation with SIMD via wide
- Add perspective and contain to route transition styles
- Refactor Motion<T> update logic into helper functions
- Refactor animation and transition modules structure
- Refactor animation delay logic into helper function
- Create FUNDING.yml
- providing animation context
- Adjust spring parameters and styles in page transitions
- remove more unused stuffs
- small cleanups
- small tweak for keyframe
- Update src/keyframes.rs
- Improve keyframe animation handling by adding a check for empty keyframes, ensuring proper behavior when no keyframes are present.
- Clippy Fixes
- more refactor
- more refactor
- update cube_animation.rs source link in README.md
- remove use_drop
- ai suggestions
- More Guide
- Update readme
- Using dioxus main branch
- Clippy FIxes
- Merge branch 'main' of https://github.com/wheregmis/dioxus-motion
- Modified and Added some tests
- Few Timing optimization for desktop
- Clippy Fixes
- Fix Nested Page transition
- Setting basepath for github deployment
- Remove example project and embed it into docs
- step calculation fix
- optimize animation state updates and route handling in effects
- remove AnimationSignal implementation from AnimationManager trait
- More cleanup
- few cleanup
- animation step stack allocation
- more spring optimizations
- optimized spring
- using arc for shared config, memory opt
- adaptive delay calculation
- 📝 Add docstrings to `ft_docs`
- Fix Clippy
- Revert page transitions and platform.rs
- Few opts
- Wrap first Draft on Dioxus Motion Docs
- Making docs cross platform
- Basic Landing Page
- Update changelog to use consistent versioning format
- Update changelog to include project documentation and semantic versioning details
- Update changelog
- Refactor dioxus-motion-transitions-macro package structure and remove obsolete route_transitions crate
- Update Cargo.toml to use resolver version 3
- Add release-plz configuration for dioxus-motion packages
- Update dependencies: syn to 2.0.100, quote to 1.0.40, and proc-macro2 to 1.0.94
- Disable publish.yml
- remove router_test from workspace
- Lets try release plz action
- Lock tokio to 1.43.0
- Now we fully support nested routing
- Comment Github Pages Action
- Bringing back all the Page Transitions- AI Generated
- Making version 0.3.1 so it wont publish things on crate.io
- Clean and Easy Animation For now
- wip nested route
- Thanks to Evan
- Using use_context_provider
- Update the changelog
- Using Outlet to Include Layout for the time being
- Dumping all the changes
- Not showing layout, now need to show it somehow
- Update workspace configuration and add utils module for transitions

## [0.1.1](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-transitions-macro-v0.1.0...dioxus-motion-transitions-macro-v0.1.1) - 2025-11-03

### <!-- 1 -->New features

- update edition
- rustfmt **/*.rs --edition 2024

### <!-- 2 -->Fixes

- edition2024 -> error: binding modifiers may only be written when the default binding mode is move

### <!-- 3 -->Other

- Bump Dioxus Version
- Refactor dioxus-motion-transitions-macro package structure and remove obsolete route_transitions crate
### BREAKING CHANGES:
- **Major Simplification: Simplified Animatable Trait**
  - Reduced from 7 required methods to just 2: `interpolate()` and `magnitude()`
  - Now leverages standard Rust operator traits (`Add`, `Sub`, `Mul<f32>`, `Default`)
  - Eliminates custom `zero()`, `epsilon()`, `scale()`, `add()`, `sub()` methods
  - Single default epsilon (0.01) for consistent behavior
  - ~70% less boilerplate when implementing custom animatable types
  
- **`use_motion<T>` now requires `T: Send + 'static`**
  - The `use_motion<T>` function now requires types to implement `Send + 'static` in addition to `Animatable`
  - This enables better thread safety and resource management for animations
  - Types that don't satisfy these bounds will no longer compile with `use_motion`
  
- `KeyframeAnimation::add_keyframe` now returns a `Result`, not `Self`. Chaining requires `.and_then(...).unwrap()` or error handling. All documentation and guides updated to reflect this.

### Migration Guide:
For custom `Animatable` implementations:
```rust
// Before (Old trait):
impl Animatable for MyType {
    fn zero() -> Self { /* implementation */ }
    fn epsilon() -> f32 { /* implementation */ }  
    fn magnitude(&self) -> f32 { /* implementation */ }
    fn scale(&self, factor: f32) -> Self { /* implementation */ }
    fn add(&self, other: &Self) -> Self { /* implementation */ }
    fn sub(&self, other: &Self) -> Self { /* implementation */ }
    fn interpolate(&self, target: &Self, t: f32) -> Self { /* implementation */ }
}

// After (New simplified trait):
#[derive(Default)] // Add Default derive
impl Animatable for MyType {
    fn interpolate(&self, target: &Self, t: f32) -> Self { /* implementation */ }
    fn magnitude(&self) -> f32 { /* implementation */ }
}

// Also implement standard operators:
impl Add for MyType { /* standard addition */ }
impl Sub for MyType { /* standard subtraction */ }
impl Mul<f32> for MyType { /* scalar multiplication */ }
```

Replace `MyType::zero()` calls with `MyType::default()`.

**For `use_motion<T>` trait bound requirements:**
- Ensure your custom types implement `Send + 'static` in addition to `Animatable`
- Most types automatically satisfy these bounds, but types with non-Send fields (like `Rc<T>`) will need to be refactored
- Use `Arc<T>` instead of `Rc<T>` for shared ownership in animatable types

### Fixes:
- Layout not being shown when animating in the case of nested Layouts
- Nested Layout fully fixed
### Changes:
- Few code refactoring
- Simplified epsilon system with single default value (0.01)
- Updated all built-in types (f32, Transform, Color, PageTransitionAnimation) to use new trait
- Enhanced documentation with simplified examples

## [0.3.1] - 2024-02-08
- Rerelease

## [0.3.0] - 2024-02-08
### New Features
- Added initial support for page transitions (Special thanks to Marc and Evan)
### Bug Fixes or Enhancements
- Support dioxus 0.6.3
### Changes
- Most of the things should be on the prelude, so if you face any erros while migrating, just import prelude::*.

## [0.2.3] - 2024-01-23
### Dioxus Version Bump
- updated to dioxus v0.6.2
- minor fixes

## [0.2.2] - 2024-01-17
### Performance Improvements
- Resource optimization for web

## [0.2.1] - 2024-01-11
### Performance Improvements
- Smoothness Optimization
### New Features
- Animation Sequence

## [0.2.0] - 2024-01-05
### Breaking Changes
- Replaced `use_value_animation` and `use_transform_animation` with `use_motion` hook
- Removed old animation configuration system
- Updated Transform property names for consistency
- Changed spring physics default parameters
- Removed deprecated animation methods

### New Features
- Added Color animation support
- Introduced new `AnimationConfig` API
- Added support for animation delays
- Implemented loop modes (Infinite, Times)
- Added new spring physics configuration
- Improved cross-platform performance
- Added new examples and documentation

### Performance Improvements
- Optimized animation frame handling
- Reduced CPU usage on desktop platforms
- Improved interpolation calculations
- Better memory management
- Enhanced cleanup on unmount

### Bug Fixes
- Fixed color interpolation for decreasing values
- Corrected spring physics calculations
- Fixed desktop platform timing issues
- Resolved memory leaks in animation loops
- Fixed transform rotation interpolation

## 🆕 What's New in v0.2.0

### New Animation API
- Unified animation hook `use_animation`
- Simplified configuration
- Enhanced type safety
- Better performance

### Color Animations
```rust
let color = use_motion(Color::from_rgba(59, 130, 246, 255));
color.animate_to(
    Color::from_rgba(168, 85, 247, 255),
    AnimationConfig::new(AnimationMode::Spring(Spring::default()))
);
```
### Animation Delays & Loops
```rust
AnimationConfig::new(mode)
    .with_delay(Duration::from_secs(1))
    .with_loop(LoopMode::Times(3))
```

## [0.1.4] - 2024-12-28
### Changes
- Update dependencies and remove unused UUID references
- Stop animations on component drop for improved resource management
- Refactor delay function to improve animation frame handling
- Optimize animation frame handling for smoother performance
- Add Screen feature to web-sys and improve frame time calculation
- Force target 90 FPS hardcoding for consistent performance

### Fixes
- Remove Tailwind CDN dependency from Index.html
- Remove Particle Effect temporarily for stability
- Revert to initial implementation of delay function
- Code cleanup and optimization

## [0.1.3] - 2024-12-27
### Changes
- Adjust animation frame threshold for smoother performance

### Fixes
- Fixed Desktop Platform (Seemed to be broken previously)

## [0.1.2] - 2024-12-27
### Changes
- Example Overhaul

### Fixes
- Fixed Desktop Platform (Seemed to be broken previously)

## [0.1.1] - 2024-12-27
### Changes
- Update Readme

## [0.1.0] - 2024-12-27
### Changes
- Initial Release
