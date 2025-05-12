# TODO List for Dioxus-Motion Updates

## Critical Errors to Fix
- [x] Fix `LoopMode` import in `docs/src/components/animations.rs` (private enum)
- [x] Fix missing `set_timeout` function in components (replaced with toggle)
- [x] Fix missing `path` element in motion primitives (used regular SVG path)
- [x] Fix missing `stroke_dashoffset` method in `AnimationTarget` (used CSS transition)
- [x] Fix missing `shadow` method in `AnimationTarget` (removed shadow animation)
- [x] Fix missing `repeat` method in `TransitionConfig` (used CSS transition)
- [x] Fix string interpolation in `value_animation.rs` conic gradient
- [x] Fix missing `Easing` trait import for `ease_in_out` and `ease_out` functions

## Remaining Issues
- [ ] Fix deprecated `TransitionConfig::new()` usage in example_project components
- [ ] Add proper support for SVG elements in motion primitives
- [ ] Add missing methods to `AnimationTarget` (stroke_dashoffset, shadow)
- [ ] Add missing methods to `TransitionConfig` (repeat)
- [x] Fix duration not being properly respected in animations

## Documentation Updates
- [x] Update library documentation in `src/lib.rs` to use motion primitives
- [x] Update `AnimatedButton` component in docs landing page
- [x] Update basic guide to use motion primitives
- [x] Update README.md with motion primitives examples
- [x] Update `TransformAnimationShowcase` component
- [x] Fix issues with `ValueAnimationShowcase` component
- [x] Fix issues with `AnimatedCounter` component
- [x] Fix issues with `PathAnimation` component
- [x] Fix issues with `BasicValueAnimation` component
- [x] Fix issues with `TransformAnimation` component
- [x] Fix issues with `SequenceAnimation` component
- [x] Update `NavBar` component to use motion primitives
- [ ] Update `CustomColorAnimation` component to use motion primitives
- [ ] Update `AdvancedFeaturesAnimation` component to use motion primitives
- [ ] Update code examples in `AnimationStep` components to show motion primitives

## Library Improvements
- [ ] Fix deprecated `TransitionConfig::new()` usage in example_project components
- [ ] Add missing methods to `AnimationTarget` (stroke_dashoffset, shadow)
- [ ] Add missing SVG elements to motion primitives (path)
- [ ] Add missing methods to `TransitionConfig` (repeat)
- [ ] Implement a proper `set_timeout` function or alternative

## Testing
- [x] Test docs with `dx serve --platform web`
- [x] Test all animation examples
- [ ] Ensure animations work properly in Safari
- [ ] Test with reduced motion preferences

## Future Enhancements
- [ ] Add more comprehensive examples of motion primitives
- [ ] Create a dedicated motion primitives guide
- [ ] Add examples of layout animations
- [ ] Add examples of gesture-based animations
