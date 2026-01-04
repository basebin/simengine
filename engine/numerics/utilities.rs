//! Numerical utilities and helper functions
//! Provides safe mathematical operations and validation functions

use crate::numerics::constants::*;
use nalgebra::Vector2;

/// Clamps a value between min and max
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Safely divides two floats, returning a default value if denominator is too small
#[inline]
pub fn safe_divide(numerator: f32, denominator: f32, default: f32) -> f32 {
    if denominator.abs() < MIN_DISTANCE_EPSILON {
        default
    } else {
        numerator / denominator
    }
}

/// Clamps velocity components to prevent numerical instability
#[inline]
pub fn clamp_velocity(velocity: &mut Vector2<f32>) {
    velocity.x = clamp(velocity.x, -MAX_VELOCITY, MAX_VELOCITY);
    velocity.y = clamp(velocity.y, -MAX_VELOCITY, MAX_VELOCITY);
}

/// Validates that mass is within acceptable bounds
#[inline]
pub fn validate_mass(mass: f32) -> Result<f32, String> {
    if mass < MIN_MASS {
        Err(format!(
            "Mass {} is below minimum threshold {}",
            mass, MIN_MASS
        ))
    } else if mass > MAX_MASS {
        Err(format!(
            "Mass {} exceeds maximum threshold {}",
            mass, MAX_MASS
        ))
    } else if !mass.is_finite() {
        Err("Mass must be finite".to_string())
    } else {
        Ok(mass)
    }
}

/// Validates that a vector contains only finite values
#[inline]
pub fn validate_vector(vec: &Vector2<f32>) -> Result<(), String> {
    if !vec.x.is_finite() || !vec.y.is_finite() {
        Err(format!("Vector contains non-finite values: {:?}", vec))
    } else {
        Ok(())
    }
}

/// Checks if two floats are approximately equal within epsilon
#[inline]
pub fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}
