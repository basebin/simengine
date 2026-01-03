//! Numerical constants for physics and general calculations
//! Centralizes all magic numbers and thresholds used throughout the engine

/// Minimum distance threshold to avoid division by very small values in collision calculations
pub const MIN_DISTANCE_EPSILON: f32 = 1e-6;

/// Maximum velocity magnitude to prevent numerical explosions
pub const MAX_VELOCITY: f32 = 1000.0;

/// Minimum mass threshold (objects below this are considered invalid)
pub const MIN_MASS: f32 = 1e-3;

/// Maximum mass threshold to prevent numerical issues
pub const MAX_MASS: f32 = 1e6;

/// Gravity constant (standard Earth gravity)
pub const EARTH_GRAVITY: f32 = 9.81;

/// Time step epsilon for floating point comparisons
#[allow(dead_code)]
pub const TIME_STEP_EPSILON: f32 = 1e-7;
