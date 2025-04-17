//! Spring physics implementation for animations
//!
//! Provides a physical spring model for smooth, natural-looking animations.
//! Based on Hooke's law with damping for realistic motion.
//!
//! Includes lazily initialized common spring presets for better performance.

use once_cell::sync::Lazy;

/// Configuration for spring-based animations
///
/// Uses a mass-spring-damper system to create natural motion.
///
/// # Examples
/// ```rust
/// use dioxus_motion::prelude::Spring;
/// let spring = Spring {
///     stiffness: 100.0,  // Higher values = faster snap
///     damping: 10.0,     // Higher values = less bounce
///     mass: 1.0,         // Higher values = more inertia
///     velocity: 0.0,     // Initial velocity
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    /// Spring stiffness constant (default: 100.0)
    /// Higher values make the spring stronger and faster
    pub stiffness: f32,

    /// Damping coefficient (default: 10.0)
    /// Higher values reduce oscillation
    pub damping: f32,

    /// Mass of the object (default: 1.0)
    /// Higher values increase inertia
    pub mass: f32,

    /// Initial velocity (default: 0.0)
    /// Can be set for pre-existing motion
    pub velocity: f32,
}

/// Default spring configuration for general-purpose animations
impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
        }
    }
}

/// Represents the current state of a spring animation
///
/// Used to track whether the spring is still moving or has settled
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SpringState {
    /// Spring is still in motion
    Active,
    /// Spring has settled to its target position
    Completed,
}

// Lazily initialized spring presets
static SPRING_BOUNCY: Lazy<Spring> = Lazy::new(|| Spring {
    stiffness: 120.0,
    damping: 5.0,
    mass: 1.0,
    velocity: 0.0,
});

static SPRING_STIFF: Lazy<Spring> = Lazy::new(|| Spring {
    stiffness: 200.0,
    damping: 20.0,
    mass: 1.0,
    velocity: 0.0,
});

static SPRING_GENTLE: Lazy<Spring> = Lazy::new(|| Spring {
    stiffness: 50.0,
    damping: 10.0,
    mass: 2.0,
    velocity: 0.0,
});

static SPRING_WOBBLY: Lazy<Spring> = Lazy::new(|| Spring {
    stiffness: 180.0,
    damping: 4.0,
    mass: 1.0,
    velocity: 0.0,
});

impl Spring {
    /// Creates a spring with a bouncy configuration
    pub fn bouncy() -> Self {
        *SPRING_BOUNCY
    }

    /// Creates a spring with a stiff configuration (minimal bounce)
    pub fn stiff() -> Self {
        *SPRING_STIFF
    }

    /// Creates a spring with a gentle configuration (slow movement)
    pub fn gentle() -> Self {
        *SPRING_GENTLE
    }

    /// Creates a spring with a wobbly configuration (lots of oscillation)
    pub fn wobbly() -> Self {
        *SPRING_WOBBLY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_default() {
        let spring = Spring::default();
        assert_eq!(spring.stiffness, 100.0);
        assert_eq!(spring.damping, 10.0);
        assert_eq!(spring.mass, 1.0);
        assert_eq!(spring.velocity, 0.0);
    }

    #[test]
    fn test_spring_custom() {
        let spring = Spring {
            stiffness: 200.0,
            damping: 20.0,
            mass: 2.0,
            velocity: 5.0,
        };

        assert_eq!(spring.stiffness, 200.0);
        assert_eq!(spring.damping, 20.0);
        assert_eq!(spring.mass, 2.0);
        assert_eq!(spring.velocity, 5.0);
    }
}
