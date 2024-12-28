# Dioxus Motion 🚀

A lightweight, cross-platform animation library for Dioxus, designed to bring smooth, flexible animations to your Rust web, desktop, and mobile applications.

## ✨ Features

- **Cross-Platform Support**: Works on web (WASM), desktop, and mobile
- **Flexible Animation Configuration**
- **Custom Easing Functions**
- **Modular Feature Setup**
- **Simple, Intuitive API**

## 🛠 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-motion = { version = "0.1.1", optional = true }

[features]
default = ["web"]
web = ["dioxus/web", "dioxus-motion/wasm"]
desktop = ["dioxus/desktop", "dioxus-motion/desktop"]
mobile = ["dioxus/mobile", "dioxus-motion/desktop"]
```

## 🌐 Platform Support

Choose the right feature for your platform:

- `web`: For web applications using WASM
- `desktop`: For desktop and mobile applications
- `default`: Web support (if no feature specified)

## 🚀 Quick Start

### Basic Animation

```rust
use dioxus::prelude::*;
use dioxus_motion::{Motion, use_value_animation};
use instant::Duration;

fn ValueAnimation() -> Element {
    let mut motion = use_value_animation(
        Motion::new(0.0)
            .to(100.0)
            .duration(Duration::from_secs(2))
    );

    rsx! {
        div {
            "Value: {motion.value()}",
            button { onclick: move |_| motion.start(), "Animate" }
        }
    }
}
```

### Transform Animation with Spring

```rust
use dioxus::prelude::*;
use dioxus_motion::{Transform, use_transform_animation};

fn TransformAnimation() -> Element {
    let mut transform = use_transform_animation(
        Transform::default(),
        Transform {
            x: 100.0,
            y: 50.0,
            scale: 1.5,
            rotate: 360.0,
            opacity: 0.8,
        },
        AnimationMode::Spring(Spring {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }),
    );

    rsx! {
        div {
            style: "{transform.style()}",
            onmounted: move |_| transform.loop_animation(),
            "Animated Content"
        }
    }
}
```

### Advanced Value Animation

```rust
use dioxus::prelude::*;
use dioxus_motion::prelude::*;

fn AdvancedValueAnimation() -> Element {
    let mut motion = use_value_animation(
        Motion::new(0.0)
            .to(100.0)
            .duration(Duration::from_secs(1))
            .spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 1.0,
                velocity: 0.0,
            })
            .on_complete(|| println!("Animation complete!"))
    );

    use_effect(move || {
        motion.loop_animation();
    });

    rsx! {
        div {
            "Value: {motion.value()}",
            button { onclick: move |_| motion.stop_loop(), "Stop" }
        }
    }
}
```

### Advanced Transform Animation

```rust
use dioxus::prelude::*;
use dioxus_motion::{Transform, use_transform_animation};

fn AdvancedTransformAnimation() -> Element {
    let mut transform = use_transform_animation(
        Transform::default(),
        Transform {
            x: 200.0,
            y: 100.0,
            scale: 2.0,
            rotate: 720.0,
            opacity: 0.5,
        },
        AnimationMode::Tween(Tween {
            duration: Duration::from_secs(2),
            easing: easer::functions::Bounce::ease_out,
        }),
    );

    rsx! {
        div {
            style: "{transform.style()}",
            onmounted: move |_| transform.start(),
            onmouseenter: move |_| transform.reverse(),
            onmouseleave: move |_| transform.start(),
            "Interactive Animation"
        }
    }
}
```

## 🛠 Configuration Options

### 🎮 Value Animation Methods
#### Core Methods
- 🎯 `.to(value: f32)` - Set target animation value
- ⏱️ `.duration(Duration)` - Set animation duration
- 🌊 `.spring(Spring)` - Configure spring physics
- ✨ `.on_complete(fn)` - Add completion callback

#### Control Methods
- ▶️ `.start()` - Start the animation
- ⏸️ `.stop()` - Pause the animation
- 🔄 `.reset()` - Reset to initial state
- 🔁 `.loop_animation()` - Start continuous loop
- ⏹️ `.stop_loop()` - Stop loop animation

### 🎨 Transform Animation Methods

#### Properties
- 📍 `.x()` - Get horizontal position
- 📐 `.y()` - Get vertical position
- 🔍 `.scale()` - Get scale factor
- 🔄 `.rotate()` - Get rotation angle
- 👻 `.opacity()` - Get opacity value

#### Control Methods
- ▶️ `.start()` - Start transform animation
- ⏸️ `.stop()` - Stop transform animation
- 🔄 `.reset()` - Reset to initial transform
- ⏮️ `.reverse()` - Reverse animation direction
- 🔁 `.loop_animation()` - Start continuous loop
- ⏹️ `.stop_loop()` - Stop loop animation
- 🎨 `.style()` - Get current CSS transform string

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
dioxus = "0.4"
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