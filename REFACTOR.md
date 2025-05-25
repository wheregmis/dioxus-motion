# Dioxus Motion Refactor Plan

This document outlines a step-by-step plan to refactor the `dioxus-motion` library for improved maintainability, extensibility, and contributor-friendliness.

---

## 1. Restructure the Codebase into Modules

**Goal:** Split `lib.rs` into smaller, focused files.

**Proposed Structure:**
```
src/
  lib.rs
  motion.rs
  sequence.rs
  keyframes.rs
  manager.rs
  prelude.rs
  animations/
    mod.rs
    utils.rs
    spring.rs
    tween.rs
    colors.rs
    transform.rs
  transitions/
    mod.rs
    page_transitions.rs
    utils.rs
```

**Steps:**
- Move `Motion<T>` and its impls to `motion.rs`.
- Move `AnimationSequence<T>` and related types to `sequence.rs`.
- Move `Keyframe` and `KeyframeAnimation` to `keyframes.rs`.
- Move the `AnimationManager` trait and its impls to `manager.rs`.
- Keep only re-exports, feature flags, and the `prelude` in `lib.rs`.

---

## 2. Trait Improvements & Extensibility

**Goal:** Make the animation system more extensible and trait-based.

**Steps:**
- Implement `AnimationManager` for both `Motion<T>` and `Signal<Motion<T>>`.
- Consider defining a trait for easing functions and animation modes:
  ```rust
  pub trait Easing: Send + Sync {
      fn ease(&self, t: f32) -> f32;
  }
  ```
- Allow users to provide custom animation modes by trait objects or enums with boxed trait impls.

---

## 3. Error Handling & Diagnostics

**Goal:** Avoid panics in public APIs and improve debuggability.

**Steps:**
- Replace panics (e.g., in keyframe sorting) with `Result`-based errors.
- Add logging (e.g., with the `tracing` crate) for state changes and errors.

---

## 4. Documentation & Examples

**Goal:** Make the library easy to use and understand.

**Steps:**
- Add or expand doc comments for all public types and methods.
- Add more usage examples, especially for advanced features.
- Create an `/examples` directory with real Dioxus apps using the library.
- Document feature flags and platform-specific behavior in the README.

---

## 5. Testing & CI

**Goal:** Ensure reliability and prevent regressions.

**Steps:**
- Add unit tests for core logic (spring/tween math, sequence progression, keyframe interpolation).
- Add integration tests for Dioxus integration.
- Set up CI (e.g., GitHub Actions) to run tests and lints on PRs.

---

## 6. API Ergonomics

**Goal:** Make the API intuitive and consistent.

**Steps:**
- Use builder patterns for configuration.
- Change methods like `then` and `on_complete` to take `&mut self` instead of consuming `self`.
- Ensure consistent naming conventions across the API.

---

## 7. Miscellaneous Cleanups

**Goal:** Polish and future-proof the codebase.

**Steps:**
- Remove duplicate lints.
- Use `#[non_exhaustive]` on public enums.
- Profile and optimize performance as needed.

---

## Implementation Notes
- Tackle the refactor incrementally, starting with module splits.
- After each major step, run tests and update documentation.
- Encourage code review and discussion for each PR.

---

**Let's use this plan as our guide for the refactor!** 