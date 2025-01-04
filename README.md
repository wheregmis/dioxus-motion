# Dioxus Motion 🚀

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/wheregmis/dioxus-motion/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/dioxus-motion.svg)](https://crates.io/crates/dioxus-motion)
[![Docs](https://docs.rs/dioxus-motion/badge.svg)](https://docs.rs/dioxus-motion/0.1.4/dioxus_motion/)

A lightweight, cross-platform animation library for Dioxus, designed to bring smooth, flexible animations to your Rust web, desktop, and mobile applications.

## 🎯 Live Examples

Visit our [Example Website](https://wheregmis.github.io/dioxus-motion/) to see these animations in action:

- 🎲 3D Card Flip
- ✨ Particle System
- 📝 Typewriter Effect
- 🔄 Morphing Shapes
- 💫 Spring Animations
- ⚡ Path Animations

### Quick Example

```rust
use dioxus_motion::prelude::*;

#[component]
fn PulseEffect() -> Element {
    let scale = use_motion(1.0f32);
    
    use_effect(move || {
        scale.animate_to(
            1.2,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 5.0,
                mass: 0.5,
                ..Default::default()
            }))
            .with_loop(LoopMode::Infinite)
        );
    });

    rsx! {
        div {
            class: "w-20 h-20 bg-blue-500 rounded-full",
            style: "transform: scale({scale.get_value()})"
        }
    }
}
```

## ✨ Features

- **Cross-Platform Support**: Works on web, desktop, and mobile
- **Flexible Animation Configuration**
- **Custom Easing Functions**
- **Modular Feature Setup**
- **Simple, Intuitive API**

## 🛠 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-motion = { version = "0.1.4", optional = true, default-features = false }

[features]
default = ["web"]
web = ["dioxus/web", "dioxus-motion/web"]
desktop = ["dioxus/desktop", "dioxus-motion/desktop"]
mobile = ["dioxus/mobile", "dioxus-motion/desktop"]
```

## 🌐 Platform Support

Choose the right feature for your platform:

- `web`: For web applications using WASM
- `desktop`: For desktop and mobile applications
- `default`: Web support (if no feature specified)

## 🚀 Quick Start

## 🔄 Migration Guide (v0.2.0)

### Breaking Changes
- Combined `use_value_animation` and `use_transform_animation`  into `use_motion`
- New animation configuration API
- Updated spring physics parameters
- Changed transform property names

### New Animation API
```rust
use dioxus_motion::prelude::*;

// Before (v0.1.x)
let mut motion = use_value_animation(Motion::new(0.0).to(100.0));

// After (v0.2.x)
let mut value = use_motion(0.0f32);
value.animate_to(
    100.0,
    AnimationConfig::new(AnimationMode::Tween(Tween {
        duration: Duration::from_secs(2),
        easing: easer::functions::Linear::ease_in_out,
    }))
);

// Before (v0.1.x)
let mut transform = use_transform_animation(Transform::default());

// After (v0.2.x)
let mut transform = use_motion(Transform::default());
transform.animate_to(
    Transform::new(100.0, 0.0, 1.2, 45.0),
    AnimationConfig::new(AnimationMode::Spring(Spring {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        ..Default::default()
    }))
);
```

## 🆕 New Features
### Loop Modes
```rust
.with_loop(LoopMode::Infinite)
.with_loop(LoopMode::Times(3))
```
### Animation Delays
```rust
.with_delay(Duration::from_secs(1))
```

## 🎓 Advanced Guide: Extending Animations

### Implementing the Animatable Trait

The `Animatable` trait allows you to animate any custom type.

Defination of Animatable Trait
```rust
pub trait Animatable: Copy + 'static {
    fn zero() -> Self;
    fn epsilon() -> f32;
    fn magnitude(&self) -> f32;
    fn scale(&self, factor: f32) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn interpolate(&self, target: &Self, t: f32) -> Self;
}

```
Here's how to implement it:
### Custom Position Type
```rust
#[derive(Debug, Copy, Clone)]
struct Position {
    x: f32,
    y: f32,
}

impl Animatable for Position {
    fn zero() -> Self {
        Position { x: 0.0, y: 0.0 }
    }

    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn scale(&self, factor: f32) -> Self {
        Position {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        Position {
            x: self.x + (target.x - self.x) * t,
            y: self.y + (target.y - self.y) * t,
        }
    }
}
```
### Best Practices
Zero State: Implement zero() as your type's neutral state
Epsilon: Choose a small value (~0.001) for animation completion checks
Magnitude: Return the square root of sum of squares for vector types
Scale: Multiply all components by the factor
Add/Sub: Implement component-wise addition/subtraction
Interpolate: Use linear interpolation for smooth transitions

### Common Patterns
#### Circular Values (e.g., angles)
```rust
fn interpolate(&self, target: &Self, t: f32) -> Self {
    let mut diff = target.angle - self.angle;
    // Ensure shortest path
    if diff > PI { diff -= 2.0 * PI; }
    if diff < -PI { diff += 2.0 * PI; }
    Self { angle: self.angle + diff * t }
}
```
#### Normalized Values (e.g., colors)
```rust
fn scale(&self, factor: f32) -> Self {
    Self {
        value: (self.value * factor).clamp(0.0, 1.0)
    }
}
```

## 🌈 Supported Easing Functions

Leverages the `easer` crate, supporting:
- Linear
- Quadratic
- Cubic
- Quartic
- And more!

## 💻 Example Project Configurations

### Web Project
```toml
[dependencies]
dioxus = "0.6.1"
dioxus-motion = { 
    git = "https://github.com/wheregmis/dioxus-motion.git", 
    features = ["web"] 
}
```

### Desktop and Mobile Project
```toml
[dependencies]
dioxus = "0.6.1"
dioxus-motion = { 
    git = "https://github.com/wheregmis/dioxus-motion.git", 
    features = ["desktop"] 
}
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit changes
4. Push to the branch
5. Create a Pull Request

## 📄 License

MIT License

## 🐞 Reporting Issues

Please report issues on the GitHub repository with:
- Detailed description
- Minimal reproducible example
- Platform and feature configuration used

## 🌟 Motivation

Bringing elegant, performant motion animations to Rust's web and desktop ecosystems with minimal complexity.