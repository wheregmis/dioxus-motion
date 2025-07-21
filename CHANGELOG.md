# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-v0.3.1...dioxus-motion-v0.4.0) - 2025-07-21

### Added

- update edition
- rustfmt **/*.rs --edition 2024
- cargo update

### Fixed

- fixed page transition animation from/to to root route
- fixing animation config explicit conversion

### Other

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
- Making version 0.3.1  so it wont publish things on crate.io
- Clean and Easy Animation For now
- wip nested route
- Thanks to Evan
- Using use_context_provider
- Update the changelog
- Using Outlet to Include Layout for the time being
- Dumping all the changes
- Not showing layout, now need to show it somehow
- Update workspace configuration and add utils module for transitions

## [0.1.1](https://github.com/wheregmis/dioxus-motion/compare/dioxus-motion-transitions-macro-v0.1.0...dioxus-motion-transitions-macro-v0.1.1) - 2025-07-21

### Added

- update edition
- rustfmt **/*.rs --edition 2024

### Fixed

- edition2024 -> error: binding modifiers may only be written when the default binding mode is move

### Other

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
