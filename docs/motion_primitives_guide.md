# Dioxus Motion Primitives Guide

This guide explains how to use the motion primitives in the `dioxus-motion` library to create fluid, interactive animations in your Dioxus applications.

## Introduction

The `dioxus-motion` library provides a set of motion primitives that make it easy to add animations to your Dioxus components. These primitives are inspired by Framer Motion and provide a declarative way to define animations.

```rust
use dioxus::prelude::*;
use dioxus_motion::prelude::*;
```

## Basic Motion Components

The `motion::` namespace provides motion-enabled versions of standard HTML elements. These components accept animation properties in addition to the standard properties.

### Example: Basic Animation

```rust
#[component]
fn FadeInHeading() -> Element {
    rsx! {
        motion::h1 {
            class: "text-2xl font-bold",
            initial: Some(AnimationTarget::new().opacity(0.0).y(-20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
            ),
            "Hello, Motion!"
        }
    }
}
```

## Core Animation Properties

### `initial`

The `initial` property defines the starting state of the animation. It accepts an `AnimationTarget` that can specify properties like opacity, position, scale, and more.

```rust
initial: Some(AnimationTarget::new().opacity(0.0).y(-20.0))
```

### `animate`

The `animate` property defines the end state of the animation. When the component mounts, it will animate from the `initial` state to the `animate` state.

```rust
animate: Some(AnimationTarget::new().opacity(1.0).y(0.0))
```

### `transition`

The `transition` property defines how the animation should be performed. It accepts a `TransitionConfig` that can specify the type of animation, stiffness, damping, and delay.

```rust
transition: Some(
    TransitionConfig::new(TransitionType::Spring)
        .stiffness(100.0)
        .damping(15.0)
        .delay(0.2)
)
```

## Transition Types

### Spring Animation

Spring animations provide a natural, physics-based motion. They're great for creating realistic, fluid animations.

```rust
TransitionConfig::new(TransitionType::Spring)
    .stiffness(100.0)  // Controls the "tightness" of the spring
    .damping(15.0)     // Controls how quickly the spring comes to rest
```

### Tween Animation

Tween animations provide a more traditional, keyframe-based animation. They're useful for precise, controlled animations.

```rust
TransitionConfig::new(TransitionType::Tween)
    .duration(0.5)     // Duration in seconds
    .ease(EasingFunction::EaseInOut)
```

## Interactive Animations

### `while_hover`

The `while_hover` property defines the state of the element when it's being hovered over. When the user hovers over the element, it will animate to this state.

```rust
motion::button {
    class: "px-4 py-2 bg-blue-500 text-white rounded",
    while_hover: Some(AnimationTarget::new().scale(1.05)),
    "Hover Me"
}
```

### `while_tap`

The `while_tap` property defines the state of the element when it's being pressed. When the user presses the element, it will animate to this state.

```rust
motion::button {
    class: "px-4 py-2 bg-blue-500 text-white rounded",
    while_tap: Some(AnimationTarget::new().scale(0.95)),
    "Press Me"
}
```

## Animation Target Properties

The `AnimationTarget` struct provides methods to define various animation properties:

### Position

```rust
// Move 20 pixels down from the initial position
AnimationTarget::new().y(20.0)

// Move 20 pixels right from the initial position
AnimationTarget::new().x(20.0)
```

### Scale

```rust
// Scale to 1.5 times the original size
AnimationTarget::new().scale(1.5)

// Scale only in the X direction
AnimationTarget::new().scale_x(1.5)

// Scale only in the Y direction
AnimationTarget::new().scale_y(1.5)
```

### Opacity

```rust
// Fade to 50% opacity
AnimationTarget::new().opacity(0.5)
```

### Rotation

```rust
// Rotate 45 degrees
AnimationTarget::new().rotate(45.0)
```

### Combining Properties

You can chain multiple properties together to create complex animations:

```rust
AnimationTarget::new()
    .opacity(0.0)
    .y(-20.0)
    .scale(0.8)
    .rotate(10.0)
```

## Practical Examples

### Fade-In Card

```rust
#[component]
fn FadeInCard() -> Element {
    rsx! {
        motion::div {
            class: "bg-white p-6 rounded-lg shadow-md",
            initial: Some(AnimationTarget::new().opacity(0.0).y(20.0)),
            animate: Some(AnimationTarget::new().opacity(1.0).y(0.0)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(100.0)
                    .damping(15.0)
            ),
            
            h2 { class: "text-xl font-bold mb-4", "Card Title" }
            p { class: "text-gray-600", "This card fades in and slides up when it appears." }
        }
    }
}
```

### Interactive Button

```rust
#[component]
fn AnimatedButton() -> Element {
    rsx! {
        motion::button {
            class: "px-6 py-3 bg-blue-600 text-white font-medium rounded-md shadow-sm",
            initial: Some(AnimationTarget::new().scale(1.0)),
            while_hover: Some(AnimationTarget::new().scale(1.05)),
            while_tap: Some(AnimationTarget::new().scale(0.95)),
            transition: Some(
                TransitionConfig::new(TransitionType::Spring)
                    .stiffness(300.0)
                    .damping(20.0)
            ),
            
            "Click Me"
        }
    }
}
```

### Staggered List Animation

```rust
#[component]
fn StaggeredList() -> Element {
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];
    
    rsx! {
        ul { class: "space-y-2",
            {items.iter().enumerate().map(|(index, item)| {
                rsx! {
                    motion::li {
                        key: "{index}",
                        class: "bg-gray-100 p-4 rounded",
                        initial: Some(AnimationTarget::new().opacity(0.0).x(-20.0)),
                        animate: Some(AnimationTarget::new().opacity(1.0).x(0.0)),
                        transition: Some(
                            TransitionConfig::new(TransitionType::Spring)
                                .stiffness(100.0)
                                .damping(15.0)
                                .delay(0.1 * index as f64)  // Staggered delay
                        ),
                        
                        "{item}"
                    }
                }
            })}
        }
    }
}
```

## Advanced Techniques

### Drag and Drop

The `dioxus-motion` library supports drag and drop functionality, allowing you to create interactive interfaces where elements can be dragged and reordered.

```rust
#[component]
fn DraggableItem() -> Element {
    let mut is_dragging = use_signal(|| false);
    
    let handle_drag_start = move |_| {
        is_dragging.set(true);
    };
    
    let handle_drag_end = move |_| {
        is_dragging.set(false);
    };
    
    rsx! {
        motion::div {
            class: if is_dragging() {
                "p-4 bg-blue-100 rounded shadow-lg scale-105 z-10"
            } else {
                "p-4 bg-gray-100 rounded"
            },
            onmousedown: handle_drag_start,
            onmouseup: handle_drag_end,
            
            "Drag Me"
        }
    }
}
```

## Performance Considerations

- Use `scale` instead of changing width/height for smoother animations
- Prefer animating `transform` and `opacity` properties for better performance
- For complex animations, consider using staggered delays to reduce the load on the browser

## Conclusion

The `dioxus-motion` library provides a powerful set of primitives for creating fluid, interactive animations in your Dioxus applications. By combining these primitives with Dioxus's reactive programming model, you can create rich, engaging user experiences with minimal code.
