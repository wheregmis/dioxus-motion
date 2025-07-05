//! Epsilon constants for animation precision control
//!
//! This module provides standardized epsilon values for different animation types.
//! Using consistent epsilon values ensures that animations complete synchronously
//! and provides predictable timing behavior across all animation systems.

/// High precision epsilon for color animations
///
/// Colors require high precision due to their visual sensitivity.
/// Small color differences can be perceptible, so we use a tight epsilon
/// to ensure smooth, high-quality color transitions.
pub const COLOR_EPSILON: f32 = 0.0001;

/// Standard precision epsilon for basic numeric animations
///
/// This is the default epsilon for most animation types including
/// basic numeric values, coordinates, and simple transformations.
/// Provides a good balance between precision and performance.
pub const DEFAULT_EPSILON: f32 = 0.001;

/// Medium precision epsilon for transform animations
///
/// Transform animations (translation, rotation, scale) can tolerate
/// slightly less precision since small pixel differences are often
/// imperceptible at typical screen resolutions.
pub const TRANSFORM_EPSILON: f32 = 0.005;

/// Low precision epsilon for page transitions
///
/// Page transitions involve larger movements and can use lower precision
/// for better performance. The visual impact of small differences is
/// minimal when transitioning between full pages.
pub const PAGE_TRANSITION_EPSILON: f32 = 0.01;

/// Ultra-high precision epsilon for specialized animations
///
/// Reserved for animations that require maximum precision,
/// such as mathematical visualizations or high-precision graphics.
pub const ULTRA_HIGH_PRECISION_EPSILON: f32 = 0.00001;

/// Gets the appropriate epsilon for an animation type
///
/// This function can be used to select the right epsilon value
/// based on the animation context or type.
///
/// # Examples
/// ```rust
/// use dioxus_motion::animations::epsilon::*;
///
/// // For color animations
/// let color_eps = get_epsilon_for_type(AnimationTypeHint::Color);
/// assert_eq!(color_eps, COLOR_EPSILON);
///
/// // For transform animations  
/// let transform_eps = get_epsilon_for_type(AnimationTypeHint::Transform);
/// assert_eq!(transform_eps, TRANSFORM_EPSILON);
/// ```
pub fn get_epsilon_for_type(animation_type: AnimationTypeHint) -> f32 {
    match animation_type {
        AnimationTypeHint::Color => COLOR_EPSILON,
        AnimationTypeHint::Transform => TRANSFORM_EPSILON,
        AnimationTypeHint::PageTransition => PAGE_TRANSITION_EPSILON,
        AnimationTypeHint::UltraHighPrecision => ULTRA_HIGH_PRECISION_EPSILON,
        AnimationTypeHint::Default => DEFAULT_EPSILON,
    }
}

/// Hint for selecting appropriate epsilon values
///
/// This enum helps select the right epsilon value for different
/// animation contexts, ensuring consistent precision across the system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationTypeHint {
    /// Default numeric animations
    Default,
    /// Color animations requiring high precision
    Color,
    /// Transform animations (translation, rotation, scale)
    Transform,
    /// Page transition animations
    PageTransition,
    /// Ultra-high precision animations for specialized use cases
    UltraHighPrecision,
}

/// Validates that an epsilon value is within reasonable bounds
///
/// Ensures epsilon values are neither too small (causing infinite loops)
/// nor too large (causing jerky animations).
///
/// # Arguments
/// * `epsilon` - The epsilon value to validate
///
/// # Returns
/// * `Ok(())` if the epsilon is valid
/// * `Err(String)` with an error message if invalid
pub fn validate_epsilon(epsilon: f32) -> Result<(), String> {
    if epsilon <= 0.0 {
        return Err("Epsilon must be positive".to_string());
    }

    if epsilon <= 0.000001 {
        return Err("Epsilon too small, may cause infinite animation loops".to_string());
    }

    if epsilon > 0.1 {
        return Err("Epsilon too large, may cause jerky animations".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epsilon_constants() {
        assert!(COLOR_EPSILON < DEFAULT_EPSILON);
        assert!(DEFAULT_EPSILON < TRANSFORM_EPSILON);
        assert!(TRANSFORM_EPSILON < PAGE_TRANSITION_EPSILON);
        assert!(ULTRA_HIGH_PRECISION_EPSILON < COLOR_EPSILON);
    }

    #[test]
    fn test_get_epsilon_for_type() {
        assert_eq!(
            get_epsilon_for_type(AnimationTypeHint::Color),
            COLOR_EPSILON
        );
        assert_eq!(
            get_epsilon_for_type(AnimationTypeHint::Transform),
            TRANSFORM_EPSILON
        );
        assert_eq!(
            get_epsilon_for_type(AnimationTypeHint::PageTransition),
            PAGE_TRANSITION_EPSILON
        );
        assert_eq!(
            get_epsilon_for_type(AnimationTypeHint::Default),
            DEFAULT_EPSILON
        );
        assert_eq!(
            get_epsilon_for_type(AnimationTypeHint::UltraHighPrecision),
            ULTRA_HIGH_PRECISION_EPSILON
        );
    }

    #[test]
    fn test_validate_epsilon() {
        assert!(validate_epsilon(0.001).is_ok());
        assert!(validate_epsilon(0.0).is_err());
        assert!(validate_epsilon(-0.001).is_err());
        assert!(validate_epsilon(0.000001).is_err());
        assert!(validate_epsilon(0.2).is_err());
    }
}
