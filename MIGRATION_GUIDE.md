# Migration Guide: dioxus-motion 0.3.x → 0.4.0

This guide helps you migrate from the old store API to the new unified motion store API.

## Breaking Changes Summary

1. **Unified Store API**: `use_motion_store()` now handles all animation types (simple, keyframes, sequences)
2. **MotionHandle**: Returns a handle instead of separate hooks/tuples
3. **Removed APIs**: `use_motion_store_keyframes`, `use_motion_store_sequence`, `use_motion_store_with_keyframes`, `use_motion_store_with_sequences`
4. **Helper Functions**: `animate_to`, `animate_keyframes`, `animate_sequence` are now methods on `MotionHandle`
5. **AnimationConfig Helpers**: Added `.spring()` and `.tween()` convenience constructors

## Migration Examples

### Basic Animations

**Before (0.3.x)**:
```rust
let motion = use_motion_store(0.0);
let current = motion.current();

// Animate via helper function
animate_to(&motion, 100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
```

**After (0.4.0)**:
```rust
let motion = use_motion_store(0.0);
let current = motion.store().current();

// Animate via handle method
motion.animate_to(100.0, AnimationConfig::spring());
```

### Keyframe Animations

**Before (0.3.x)**:
```rust
let (motion, mut animate_keyframes) = use_motion_store_with_keyframes(0.0f32);
let current = motion.current();

let keyframes = KeyframeAnimation::new(Duration::from_secs(2))
    .add_keyframe(0.0, 0.0, None).unwrap()
    .add_keyframe(100.0, 1.0, None).unwrap();
    
animate_keyframes(keyframes);
```

**After (0.4.0)**:
```rust
let motion = use_motion_store(0.0f32);
let current = motion.store().current();

let keyframes = KeyframeAnimation::new(Duration::from_secs(2))
    .add_keyframe(0.0, 0.0, None).unwrap()
    .add_keyframe(100.0, 1.0, None).unwrap();
    
motion.animate_keyframes(keyframes);
```

### Sequence Animations

**Before (0.3.x)**:
```rust
let (motion, mut animate_sequence) = use_motion_store_with_sequences(0.0f32);
let current = motion.current();

let sequence = AnimationSequence::new()
    .then(100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())))
    .then(200.0, AnimationConfig::new(AnimationMode::Tween(Tween::default())));
    
animate_sequence(sequence);
```

**After (0.4.0)**:
```rust
let motion = use_motion_store(0.0f32);
let current = motion.store().current();

let sequence = AnimationSequence::new()
    .then(100.0, AnimationConfig::spring())
    .then(200.0, AnimationConfig::tween());
    
motion.animate_sequence(sequence);
```

### Accessing Store Fields

**Before (0.3.x)**:
```rust
let motion = use_motion_store(0.0);
let current = motion.current();  // Direct access
let running = motion.running();
```

**After (0.4.0)**:
```rust
let motion = use_motion_store(0.0);
let current = motion.store().current();  // Access via .store()
let running = motion.store().running();
```

### Control Methods

**Before (0.3.x)**:
```rust
motion.target().set(100.0);
motion.running().set(false);
```

**After (0.4.0)**:
```rust
// Use handle methods instead of direct field access
motion.animate_to(100.0, AnimationConfig::spring());
motion.stop();
motion.reset();
```

## AnimationConfig Convenience Methods

New convenience constructors make creating configs easier:

```rust
// Before
AnimationConfig::new(AnimationMode::Spring(Spring::default()))
AnimationConfig::new(AnimationMode::Tween(Tween::default()))

// After
AnimationConfig::spring()
AnimationConfig::tween()

// Custom parameters
AnimationConfig::custom_spring(200.0, 20.0, 1.0)
AnimationConfig::custom_tween(Duration::from_secs(1), easing_fn)
```

## Component Prop Changes

If you're passing motion stores to components:

**Before (0.3.x)**:
```rust
#[component]
fn MyComponent(motion: Store<MotionStore<f32>>) -> Element {
    let current = motion.current();
    // ...
}
```

**After (0.4.0)**:
```rust
// Option 1: Pass the handle
#[component]
fn MyComponent(motion_handle: MotionHandle<f32>) -> Element {
    let current = motion_handle.store().current();
    // ...
}

// Option 2: Pass just the store (for read-only components)
#[component]
fn MyComponent(motion: Store<MotionStore<f32>>) -> Element {
    let current = motion.current();
    // ...
}
```

## Key Takeaways

1. **One Hook**: `use_motion_store()` replaces all previous store hooks
2. **Handle Pattern**: The handle provides methods for animation control
3. **Store Access**: Use `.store()` to access reactive fields for subscriptions
4. **Cleaner API**: Convenience methods on `AnimationConfig` reduce boilerplate
5. **Type Safety**: The unified approach maintains full type safety with better ergonomics

## Performance Benefits

The unified API brings:
- **Single animation loop** per motion store (no duplicate spawns)
- **Unified dispatch** for all animation types
- **Better resource management** with shared signals
- **Same fine-grained reactivity** as before

## Need Help?

- Check the updated examples in `examples/` directory
- See the inline documentation in `src/store.rs`
- Open an issue on GitHub for migration questions
