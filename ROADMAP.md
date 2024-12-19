# Dioxus Motion - Simple Animation Roadmap If i dont get bored?

## Phase 1: Core Animation Primitives
### Animate Component
- [ ] Basic `animate` prop support
  ```rust
  rsx! {
      div {
          animate: Motion::new()
              .to(100.0)
              .duration(300),
      }
  }
  ```
- [ ] Common animation properties
  - [ ] x, y (transform)
  - [ ] scale
  - [ ] rotate
  - [ ] opacity

### Basic Transitions
- [ ] Duration
- [ ] Easing functions
- [ ] Delay
- [ ] Basic spring animations

## Phase 2: Gesture Animations
### Hover Animations
- [ ] hover state detection
- [ ] whileHover animations
  ```rust
  rsx! {
      div {
          on_hover: move |_| Motion::new()
              .scale(1.1)
              .duration(200),
      }
  }
  ```

### Tap/Click Animations
- [ ] tap/click state
- [ ] whileTap animations
- [ ] Basic gesture feedback

## Phase 3: Animation Controls
### Basic Controls
- [ ] start/stop
- [ ] pause/resume
- [ ] reset

### Variants
- [ ] Named animation states
  ```rust
  let variants = AnimationVariants::new()
      .add("open", Motion::new().scale(1).opacity(1))
      .add("closed", Motion::new().scale(0).opacity(0));
  
  rsx! {
      div {
          variants: variants,
          animate: "open",
      }
  }
  ```

## Implementation Priorities

1. Week 1-2: Basic Animations
   - Implement `Motion` struct
   - Basic value interpolation
   - Duration and easing

2. Week 3-4: Transform Support
   - x, y transforms
   - scale and rotate
   - opacity handling

3. Week 5-6: Gesture Integration
   - Hover detection
   - Click/tap handling
   - Basic state management

## Example API (Target)
```rust
use dioxus_motion::prelude::*;

fn AnimatedComponent() -> Element {
    rsx! {
        // Simple animation
        div {
            animate: Motion::new()
                .x(100)
                .opacity(0.5)
                .duration(300),
            "Animated div"
        }

        // Hover animation
        button {
            whileHover: Motion::new()
                .scale(1.1)
                .duration(200),
            "Hover me"
        }

        // Variants
        div {
            variants: variants,
            animate: is_open.then(|| "open").unwrap_or("closed"),
            "Toggle me"
        }
    }
}
```

## Success Goals
- Simple, intuitive API
- Smooth animations
- Small bundle size
- Easy to understand documentation
- Basic but powerful feature set