/// Physics constraints and boundary conditions
/// Handles various types of constraints that can be applied to physics objects
use nalgebra::Vector2;

/// Represents different types of constraints that can be applied to physics objects
#[derive(Debug, Clone)]
pub enum Constraint {
    /// Fixed position constraint - object cannot move from a specific point
    FixedPosition(Vector2<f32>),
    /// Distance constraint - maintains a fixed distance between two objects
    Distance { target_distance: f32 },
    /// Axis constraint - restricts movement to a specific axis
    Axis { axis: Vector2<f32>, position: f32 },
    /// Boundary constraint - keeps object within rectangular bounds
    Boundary {
        min: Vector2<f32>,
        max: Vector2<f32>,
    },
}

/// A constraint that can be applied to a physics object
#[derive(Debug, Clone)]
pub struct PhysicsConstraint {
    pub constraint_type: Constraint,
    pub stiffness: f32, // How strongly the constraint is enforced (0.0 to 1.0)
    pub damping: f32,   // Damping factor for constraint forces
}

impl Default for PhysicsConstraint {
    fn default() -> Self {
        Self {
            constraint_type: Constraint::Boundary {
                min: Vector2::new(-100.0, -100.0),
                max: Vector2::new(100.0, 100.0),
            },
            stiffness: 1.0,
            damping: 0.1,
        }
    }
}

impl PhysicsConstraint {
    /// Creates a new boundary constraint with the given bounds
    pub fn boundary(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        Self {
            constraint_type: Constraint::Boundary { min, max },
            stiffness: 1.0,
            damping: 0.1,
        }
    }

    /// Applies the constraint to a position and velocity, returning the constrained values
    pub fn apply(&self, position: &mut Vector2<f32>, velocity: &mut Vector2<f32>, delta_time: f32) {
        match &self.constraint_type {
            Constraint::Boundary { min, max } => {
                self.apply_boundary_constraint(position, velocity, *min, *max, delta_time);
            }
            Constraint::FixedPosition(fixed_pos) => {
                *position = *fixed_pos;
                *velocity = Vector2::zeros();
            }
            _ => {
                // Other constraint types not yet implemented
            }
        }
    }

    fn apply_boundary_constraint(
        &self,
        position: &mut Vector2<f32>,
        velocity: &mut Vector2<f32>,
        min: Vector2<f32>,
        max: Vector2<f32>,
        _delta_time: f32,
    ) {
        // Clamp position to bounds
        position.x = position.x.max(min.x).min(max.x);
        position.y = position.y.max(min.y).min(max.y);

        // Apply damping when hitting boundaries
        if position.x <= min.x || position.x >= max.x {
            velocity.x *= 1.0 - self.damping;
        }
        if position.y <= min.y || position.y >= max.y {
            velocity.y *= 1.0 - self.damping;
        }
    }
}
