# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### BREAKING CHANGES:
- **Major Simplification: Simplified Animatable Trait**
  - Reduced from 7 required methods to just 2: `interpolate()` and `magnitude()`
  - Now leverages standard Rust operator traits (`Add`, `Sub`, `Mul<f32>`, `Default`)
  - Eliminates custom `zero()`, `epsilon()`, `scale()`, `add()`, `sub()` methods
  - Single default epsilon (0.01) for consistent behavior
  - ~70% less boilerplate when implementing custom animatable types
  
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
