//! Spring physics implementation for animations
//!
//! Provides a physical spring model for smooth, natural-looking animations.
//! Based on Hooke's law with damping for realistic motion.

#[cfg(feature = "dioxus")]
use dioxus::prelude::Store;

/// Configuration for spring-based animations
///
/// Uses a mass-spring-damper system to create natural motion.
/// Velocity belongs to the animated value type; call `set_velocity` after starting playback.
///
/// ```compile_fail,E0560
/// use dioxus_motion::prelude::Spring;
/// let spring = Spring { velocity: 2.0, ..Spring::default() };
/// ```
///
/// # Examples
/// ```rust
/// use dioxus_motion::prelude::Spring;
/// let spring = Spring {
///     stiffness: 100.0,  // Higher values = faster snap
///     damping: 10.0,     // Higher values = less bounce
///     mass: 1.0,         // Higher values = more inertia
/// };
/// ```
#[cfg_attr(feature = "dioxus", derive(Store))]
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
}

/// Default spring configuration for general-purpose animations
impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
        }
    }
}

/// Exact transition for x'' + (damping/mass)x' + (stiffness/mass)x = 0.
/// Coefficients use f64 so finite f32 parameters do not overflow intermediate ratios.
/// See https://www.ryanjuckett.com/damped-springs/ for the oscillator derivation.
#[derive(Clone, Copy)]
pub(crate) struct SpringStep {
    pub displacement: f32,
    pub position_velocity: f32,
    pub velocity_position: f32,
    pub velocity: f32,
}

#[cfg(test)]
std::thread_local! {
    pub(crate) static STEP_CALCULATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl Spring {
    pub(crate) fn step(self, dt: f32) -> SpringStep {
        #[cfg(test)]
        STEP_CALCULATIONS.set(STEP_CALCULATIONS.get().saturating_add(1));
        let t = f64::from(dt);
        let a = f64::from(self.damping) / (2.0 * f64::from(self.mass));
        let b = f64::from(self.stiffness) / f64::from(self.mass);
        let discriminant = a * a - b;
        let (displacement, position_velocity, velocity) = if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let fast = -a - root;
            // Avoid cancellation in the slow root and differences of near-equal exponentials.
            let slow = -b / (a + root);
            let gap = 2.0 * root;
            let slow_decay = (slow * t).exp();
            let position_velocity = slow_decay * -(-gap * t).exp_m1() / gap;
            (
                -(slow * t).exp_m1() + slow * position_velocity,
                position_velocity,
                (fast * t).exp() + slow * position_velocity,
            )
        } else if discriminant < 0.0 {
            let frequency = (-discriminant).sqrt();
            let angle = frequency * t;
            let decay = (-a * t).exp();
            let position_velocity = decay * angle.sin() / frequency;
            (
                -(-a * t).exp_m1() + decay * (2.0 * (angle * 0.5).sin().powi(2))
                    - a * position_velocity,
                position_velocity,
                decay * angle.cos() - a * position_velocity,
            )
        } else {
            let decay = (-a * t).exp();
            let position_velocity = decay * t;
            (
                -(-a * t).exp_m1() - a * position_velocity,
                position_velocity,
                decay * (1.0 - a * t),
            )
        };
        SpringStep {
            displacement: displacement.max(0.0) as f32,
            position_velocity: position_velocity as f32,
            velocity_position: (-b * position_velocity) as f32,
            velocity: velocity as f32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzz_finite_spring_coefficients_keep_transition_finite() {
        let mut bits = 0x51f0_2a9du32;
        let mut next = || {
            bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
            f32::from_bits(bits & 0x7fff_ffff)
        };
        for _ in 0..4096 {
            let spring = Spring {
                stiffness: next(),
                damping: next(),
                mass: next(),
            };
            if crate::animations::core::AnimationConfig::spring(spring)
                .validate()
                .is_err()
            {
                continue;
            }
            let step = spring.step(next().min(0.1));
            assert!(
                [
                    step.displacement,
                    step.position_velocity,
                    step.velocity_position,
                    step.velocity
                ]
                .into_iter()
                .all(f32::is_finite),
                "spring={spring:?}"
            );
        }
    }

    #[test]
    fn exact_step_covers_damping_regimes_and_extreme_coefficients() {
        for (stiffness, damping, mass) in [
            (0.0, 0.0, 1.0),
            (100.0, 0.0, 1.0),
            (100.0, 10.0, 1.0),
            (100.0, 20.0, 1.0),
            (100.0, 40.0, 1.0),
            (1e6, 10.0, 1.0),
            (100.0, 10.0, 1e-30),
            (100.0, 1e30, 1.0),
        ] {
            let spring = Spring {
                stiffness,
                damping,
                mass,
            };
            let advance = |x: f32, v: f32, dt| {
                let step = spring.step(dt);
                (
                    x * (1.0 - step.displacement) + v * step.position_velocity,
                    x * step.velocity_position + v * step.velocity,
                )
            };
            let (x, v) = advance(1.0, 0.5, 0.02);
            let (half_x, half_v) = advance(1.0, 0.5, 0.01);
            let (split_x, split_v) = advance(half_x, half_v, 0.01);
            assert!((x - split_x).abs() <= 1e-5 * x.abs().max(1.0));
            assert!((v - split_v).abs() <= 1e-5 * v.abs().max(1.0));
            for dt in [0.0, f32::from_bits(1), 0.001, 0.1] {
                let step = spring.step(dt);
                assert!(
                    [
                        step.displacement,
                        step.position_velocity,
                        step.velocity_position,
                        step.velocity
                    ]
                    .into_iter()
                    .all(f32::is_finite)
                );
            }
        }
        let critical = Spring {
            stiffness: 100.0,
            damping: 20.0,
            ..Spring::default()
        }
        .step(0.1);
        assert!((critical.displacement - (1.0 - 2.0 / std::f32::consts::E)).abs() < 1e-7);
        assert!((critical.position_velocity - (0.1 / std::f32::consts::E)).abs() < 1e-7);
        assert!(critical.velocity.abs() < 1e-7);
        let overdamped = Spring {
            stiffness: 100.0,
            damping: 40.0,
            ..Spring::default()
        }
        .step(0.02);
        // Independent two-real-root solution, x(0)=1 and v(0)=0.5.
        assert!(
            (1.0 - overdamped.displacement + 0.5 * overdamped.position_velocity - 0.991_304).abs()
                < 1e-6
        );
        assert!(
            (overdamped.velocity_position + 0.5 * overdamped.velocity + 1.148_904_1).abs() < 1e-6
        );
    }

    #[test]
    fn test_spring_default() {
        let spring = Spring::default();
        assert_eq!(spring.stiffness, 100.0);
        assert_eq!(spring.damping, 10.0);
        assert_eq!(spring.mass, 1.0);
    }

    #[test]
    fn test_spring_custom() {
        let spring = Spring {
            stiffness: 200.0,
            damping: 20.0,
            mass: 2.0,
        };

        assert_eq!(spring.stiffness, 200.0);
        assert_eq!(spring.damping, 20.0);
        assert_eq!(spring.mass, 2.0);
    }
}
