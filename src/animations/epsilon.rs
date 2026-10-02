//! Epsilon utilities for animation precision control
//!
//! With the simplified Animatable trait, most animations use a single default epsilon (0.01).
//! This module provides validation utilities for custom epsilon values.

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
///
/// # Examples
/// ```rust
/// use dioxus_motion::animations::epsilon::validate_epsilon;
///
/// assert!(validate_epsilon(0.01).is_ok());
/// assert!(validate_epsilon(0.0).is_err());
/// assert!(validate_epsilon(0.2).is_err());
/// assert!(validate_epsilon(f32::NAN).is_err());
/// ```
pub fn validate_epsilon(epsilon: f32) -> Result<(), String> {
    if !epsilon.is_finite() {
        return Err("Epsilon must be finite".to_string());
    }

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
    fn epsilon_validation_matches_finite_range() {
        let mut bits = 1u32;
        for _ in 0..4096 {
            bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
            let epsilon = f32::from_bits(bits);
            assert_eq!(
                validate_epsilon(epsilon).is_ok(),
                epsilon > 0.000001 && epsilon <= 0.1,
                "epsilon bits={bits:#x}",
            );
        }
        for epsilon in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(validate_epsilon(epsilon).is_err());
        }
    }

    #[test]
    fn test_validate_epsilon() {
        assert!(validate_epsilon(0.001).is_ok());
        assert!(validate_epsilon(0.01).is_ok());
        assert!(validate_epsilon(0.1).is_ok());

        assert!(validate_epsilon(0.0).is_err());
        assert!(validate_epsilon(-0.001).is_err());
        assert!(validate_epsilon(0.000001).is_err());
        assert!(validate_epsilon(0.2).is_err());
    }
}
