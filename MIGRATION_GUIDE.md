# Migration Guide: Signal-based to Store-based Motion

This guide shows how to migrate from `use_motion()` to `use_motion_store()` for better performance through fine-grained reactivity. The store-based API supports all `Animatable` types: `f32`, `Transform`, `Color`, and custom types.

## 🚀 Quick Migration

### Before (Signal-based)
```rust
use dioxus_motion::prelude::*;

let mut motion = use_motion(0.0f32);
motion.animate_to(100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
let value = motion.get_value();
let is_running = motion.is_running();
```

### After (Store-based with fine-grained reactivity)
```rust
use dioxus_motion::prelude::*;

let motion = use_motion_store_f32(0.0);

// Method 1: Direct animation control (similar to old API)
motion.animate_to(100.0, AnimationConfig::new(AnimationMode::Spring(Spring::default())));
let value = motion.get_value();
let is_running = motion.is_running();

// Method 2: Fine-grained access (NEW - better performance)
let current = motion.current();  // Subscribe only to animated value
let running = motion.running();  // Subscribe only to running state
motion.target().set(100.0);     // Direct manipulation
motion.running().set(true);
```

## 📊 Performance Benefits

### Signal-based (Old) - Coarse Reactivity
```rust
let motion = use_motion(0.0f32);

#[component]
fn AnimationDisplay(motion: impl AnimationManager<f32>) -> Element {
    // ❌ Re-renders on ALL motion changes (value, velocity, state, etc.)
    rsx! { 
        div { "Value: {motion.get_value():.1}" }
    }
}

#[component] 
fn AnimationControls(motion: impl AnimationManager<f32>) -> Element {
    // ❌ ALSO re-renders on ALL motion changes, even just position updates
    rsx! { 
        button { 
            disabled: motion.is_running(),
            onclick: move |_| motion.animate_to(100.0, config),
            "Animate" 
        } 
    }
}
```

### Store-based (New) - Fine-grained Reactivity
```rust
let motion = use_motion_store_f32(0.0);

#[component]
fn AnimationDisplay(motion: Store<MotionStoreF32>) -> Element {
    let current = motion.current(); // ✅ Only re-renders when animated value changes
    rsx! { 
        div { "Value: {current():.1}" }
    }
}

#[component]
fn AnimationControls(motion: Store<MotionStoreF32>) -> Element {
    let is_running = motion.running(); // ✅ Only re-renders when running state changes
    rsx! { 
        button { 
            disabled: is_running(),
            onclick: move |_| {
                motion.target().set(100.0);
                motion.running().set(true);
            },
            "Animate" 
        } 
    }
}
```

## 🔧 Store Field Access

The store provides direct access to all animation properties:

```rust
let motion = use_motion_store_f32(0.0);

// Reactive subscriptions (only re-render when these specific values change)
let current = motion.current();        // Current animated value
let target = motion.target();          // Target value
let velocity = motion.velocity();      // Current velocity
let running = motion.running();        // Animation running state
let elapsed = motion.elapsed();        // Time elapsed
let initial = motion.initial();        // Initial value

// Computed properties (also reactive)
let value = motion.get_value();                    // Same as current()
let is_running = motion.is_running();             // Same as running()
let is_at_target = motion.is_at_target();         // Is animation complete?
let velocity_mag = motion.get_velocity_magnitude(); // Absolute velocity

// Direct manipulation
motion.target().set(200.0);           // Set new target
motion.running().set(true);           // Start animation
motion.current().set(50.0);           // Jump to value
motion.velocity().set(0.0);           // Reset velocity

// Animation control methods (same as before)
motion.animate_to(100.0, config);     // Animate to target
motion.stop();                        // Stop animation
motion.reset();                       // Reset to initial
motion.delay(Duration::from_millis(500)); // Add delay
```

## 🎯 Migration Patterns

### 1. Simple Animation
```rust
// Before
fn simple_animation() -> Element {
    let mut motion = use_motion(0.0f32);
    
    rsx! {
        div {
            style: "transform: translateX({motion.get_value()}px)",
            onclick: move |_| motion.animate_to(100.0, config),
            "Click me"
        }
    }
}

// After
fn simple_animation() -> Element {
    let motion = use_motion_store_f32(0.0);
    let current = motion.current(); // Fine-grained subscription
    
    rsx! {
        div {
            style: "transform: translateX({current()}px)",
            onclick: move |_| motion.animate_to(100.0, config),
            "Click me"
        }
    }
}
```

### 2. Complex UI with Multiple Components
```rust
// After - Each component subscribes only to what it needs
fn complex_animation_ui() -> Element {
    let motion = use_motion_store_f32(0.0);
    
    rsx! {
        div {
            // Only re-renders when animated value changes
            AnimatedElement { motion }
            
            // Only re-renders when control state changes  
            ControlPanel { motion }
            
            // Only re-renders when debug info changes
            DebugPanel { motion }
        }
    }
}

#[component]
fn AnimatedElement(motion: Store<MotionStoreF32>) -> Element {
    let current = motion.current(); // Subscribes only to position
    rsx! {
        div {
            style: "transform: translateX({current()}px); width: 50px; height: 50px; background: blue;",
        }
    }
}

#[component]
fn ControlPanel(motion: Store<MotionStoreF32>) -> Element {
    let is_running = motion.running(); // Subscribes only to running state
    let target = motion.target();      // Subscribes only to target
    
    rsx! {
        div {
            button {
                disabled: is_running(),
                onclick: move |_| motion.animate_to(100.0, config),
                "Animate to 100"
            }
            button {
                disabled: is_running(),
                onclick: move |_| motion.animate_to(0.0, config),
                "Animate to 0"
            }
            p { "Target: {target():.1}" }
        }
    }
}

#[component]
fn DebugPanel(motion: Store<MotionStoreF32>) -> Element {
    let velocity = motion.velocity();     // Subscribes only to velocity
    let elapsed = motion.elapsed();       // Subscribes only to elapsed time
    let current_loop = motion.current_loop(); // Subscribes only to loop count
    
    rsx! {
        div { style: "font-family: monospace; font-size: 12px;",
            p { "Velocity: {velocity():.2}" }
            p { "Elapsed: {elapsed().as_millis()}ms" }
            p { "Loop: {current_loop()}" }
        }
    }
}
```

### 3. Direct Store Manipulation
```rust
// New capability: Direct manipulation without animation
fn direct_control() -> Element {
    let motion = use_motion_store_f32(0.0);
    let current = motion.current();
    
    rsx! {
        div {
            div {
                style: "transform: translateX({current()}px); width: 50px; height: 50px; background: red;",
            }
            
            // Direct value control
            input {
                r#type: "range",
                min: "0",
                max: "200", 
                value: "{current()}",
                oninput: move |e| {
                    if let Ok(value) = e.value().parse::<f32>() {
                        motion.current().set(value); // Direct manipulation
                        motion.running().set(false);  // Stop any running animation
                    }
                }
            }
            
            // Animation controls
            button {
                onclick: move |_| {
                    motion.target().set(200.0);
                    motion.running().set(true);
                },
                "Animate"
            }
            
            button {
                onclick: move |_| motion.stop(),
                "Stop"
            }
        }
    }
}
```

## ⚠️ Important Changes

### 1. Component Props
```rust
// Before
#[component]
fn MyComponent(motion: impl AnimationManager<f32>) -> Element { /* */ }

// After
#[component]
fn MyComponent(motion: Store<MotionStoreF32>) -> Element { /* */ }
```

### 2. Store Extension Trait
Make sure to import the store extension trait:
```rust
use dioxus_motion::prelude::*; // Includes MotionStoreF32StoreExt
// or explicitly:
use dioxus_motion::store::MotionStoreF32StoreExt;
```

### 3. Reactive Values
```rust
// Store fields return reactive values that need to be called
let current = motion.current(); // This is a ReadOnlySignal<f32>
let value = current();          // Call it to get the actual f32 value

// In rsx!, you can call directly
rsx! { div { "{motion.current()}" } }
```

## 🎉 Benefits Summary

✅ **Fine-grained reactivity**: Components only re-render when their subscribed data changes  
✅ **Better performance**: Eliminates unnecessary re-renders in complex UIs  
✅ **Direct manipulation**: Set values directly without going through animation API  
✅ **Same animation API**: All existing animation methods still work  
✅ **Easier debugging**: Access individual animation properties  
✅ **Cleaner code**: Less prop drilling, more targeted subscriptions  

## 🔄 Gradual Migration Strategy

1. **Start simple**: Replace `use_motion(0.0f32)` with `use_motion_store_f32(0.0)`
2. **Identify hot spots**: Find components that re-render too often
3. **Add fine-grained subscriptions**: Use `motion.current()`, `motion.running()`, etc.
4. **Optimize component props**: Pass the store instead of individual values
5. **Use direct manipulation**: Set values directly where appropriate

This migration provides significant performance improvements for animation-heavy UIs while keeping the API familiar and easy to use!
