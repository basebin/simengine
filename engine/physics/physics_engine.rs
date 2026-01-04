use crate::numerics::*;
use nalgebra::Vector2;

#[derive(Clone, Debug)]
pub struct Object {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
}

impl Object {
    pub fn new(position: Vector2<f32>, velocity: Vector2<f32>, mass: f32) -> Result<Self, String> {
        let mass = validate_mass(mass)?;
        validate_vector(&position)?;
        validate_vector(&velocity)?;

        Ok(Object {
            position,
            velocity,
            mass,
            radius: 0.5,
        })
    }
}

#[derive(Clone, Debug)]
pub struct PhysicsEngine {
    objects: Vec<Object>,
    gravity: Vector2<f32>,
    bounds: (Vector2<f32>, Vector2<f32>), // min and max position for wall collisions
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default PhysicsEngine")
    }
}

impl PhysicsEngine {
    pub fn new() -> Result<Self, String> {
        let default_object = Object::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), 1.0)?;

        Ok(PhysicsEngine {
            objects: vec![default_object],              // default object
            gravity: Vector2::new(0.0, -EARTH_GRAVITY), // downward acceleration
            bounds: (Vector2::new(-100.0, -100.0), Vector2::new(100.0, 100.0)),
        })
    }

    #[allow(dead_code)]
    pub fn add_object(
        &mut self,
        position: Vector2<f32>,
        velocity: Vector2<f32>,
        mass: f32,
    ) -> Result<(), String> {
        let object = Object::new(position, velocity, mass)?;
        self.objects.push(object);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_gravity(&mut self, gravity: Vector2<f32>) {
        self.gravity = gravity;
    }

    #[allow(dead_code)]
    fn handle_collisions(&mut self) {
        let len = self.objects.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if let Some((impulse, separation)) = self.detect_and_resolve_collision(i, j) {
                    // Apply impulse to velocities
                    let m1 = self.objects[i].mass;
                    let m2 = self.objects[j].mass;
                    self.objects[i].velocity += impulse / m1;
                    self.objects[j].velocity -= impulse / m2;

                    // Separate positions to avoid sticking
                    self.objects[i].position += separation;
                    self.objects[j].position -= separation;

                    // Clamp velocities after collision
                    clamp_velocity(&mut self.objects[i].velocity);
                    clamp_velocity(&mut self.objects[j].velocity);
                }
            }
        }
    }

    /// Detect collision between two objects and compute resolution impulse and separation
    fn detect_and_resolve_collision(
        &self,
        i: usize,
        j: usize,
    ) -> Option<(Vector2<f32>, Vector2<f32>)> {
        let pos1 = self.objects[i].position;
        let pos2 = self.objects[j].position;
        let rad1 = self.objects[i].radius;
        let rad2 = self.objects[j].radius;
        let diff = pos1 - pos2;
        let distance = diff.norm();

        // Check if objects are colliding
        if distance >= rad1 + rad2 || distance <= MIN_DISTANCE_EPSILON {
            return None;
        }

        // Unit normal vector from object j to i
        let normal = safe_divide(1.0, distance, 1.0) * diff;

        // Relative velocity
        let v1 = self.objects[i].velocity;
        let v2 = self.objects[j].velocity;
        let relative_velocity = v1 - v2;
        let velocity_along_normal = relative_velocity.dot(&normal);

        // Don't resolve if objects are separating
        if velocity_along_normal > 0.0 {
            return None;
        }

        // Elastic collision impulse calculation
        let m1 = self.objects[i].mass;
        let m2 = self.objects[j].mass;
        let restitution = PERFECTLY_ELASTIC_RESTITUTION;
        let impulse_scalar = -(1.0 + restitution) * velocity_along_normal / (1.0 / m1 + 1.0 / m2);
        let impulse = impulse_scalar * normal;

        // Calculate separation to prevent overlap
        let overlap = (rad1 + rad2) - distance;
        let separation = normal * (overlap / 2.0);

        Some((impulse, separation))
    }

    pub fn simulate(&mut self, time_step: f32) -> Result<(), String> {
        if time_step <= 0.0 || !time_step.is_finite() {
            return Err(format!("Invalid time step: {}", time_step));
        }

        log::info!("Running physics simulation...");

        // Phase 1: Integration (apply forces and update positions)
        self.integrate(time_step)?;

        // Phase 2: Collision Detection and Resolution
        self.handle_wall_collisions();
        self.handle_collisions();

        // Phase 3: Validation and clamping
        self.validate_and_clamp();

        for obj in &self.objects {
            log::info!(
                "Object position: {:?}, velocity: {:?}",
                obj.position,
                obj.velocity
            );
        }

        Ok(())
    }

    /// Phase 1: Integration - apply forces and update positions/velocities
    fn integrate(&mut self, time_step: f32) -> Result<(), String> {
        for obj in &mut self.objects {
            // Apply gravity
            obj.velocity += self.gravity * time_step;

            // Clamp velocity before integration to prevent explosions
            clamp_velocity(&mut obj.velocity);

            // Update position using symplectic Euler integration
            obj.position += obj.velocity * time_step;

            // Validate position is finite
            validate_vector(&obj.position)?;
        }
        Ok(())
    }

    /// Phase 2: Handle wall boundary collisions
    fn handle_wall_collisions(&mut self) {
        for obj in &mut self.objects {
            let mut bounced = false;

            // Clamp position to bounds
            obj.position.x = clamp(obj.position.x, self.bounds.0.x, self.bounds.1.x);
            obj.position.y = clamp(obj.position.y, self.bounds.0.y, self.bounds.1.y);

            // Bounce off walls with damping only if moving towards the boundary
            if (approx_eq(obj.position.x, self.bounds.0.x, MIN_DISTANCE_EPSILON)
                && obj.velocity.x < 0.0)
                || (approx_eq(obj.position.x, self.bounds.1.x, MIN_DISTANCE_EPSILON)
                    && obj.velocity.x > 0.0)
            {
                obj.velocity.x = -obj.velocity.x * WALL_BOUNCE_DAMPING_FACTOR;
                bounced = true;
            }
            if (approx_eq(obj.position.y, self.bounds.0.y, MIN_DISTANCE_EPSILON)
                && obj.velocity.y < 0.0)
                || (approx_eq(obj.position.y, self.bounds.1.y, MIN_DISTANCE_EPSILON)
                    && obj.velocity.y > 0.0)
            {
                obj.velocity.y = -obj.velocity.y * WALL_BOUNCE_DAMPING_FACTOR;
                bounced = true;
            }

            if bounced {
                clamp_velocity(&mut obj.velocity);
            }
        }
    }

    /// Phase 3: Validate and clamp all values to maintain numerical stability
    fn validate_and_clamp(&mut self) {
        for obj in &mut self.objects {
            clamp_velocity(&mut obj.velocity);
            // Ensure position stays within reasonable bounds (expand if needed)
            obj.position.x = clamp(obj.position.x, -MAX_COORDINATE_VALUE, MAX_COORDINATE_VALUE);
            obj.position.y = clamp(obj.position.y, -MAX_COORDINATE_VALUE, MAX_COORDINATE_VALUE);
        }
    }

    #[allow(dead_code)]
    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }

    #[allow(dead_code)]
    pub fn get_objects_mut(&mut self) -> &mut Vec<Object> {
        &mut self.objects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity() {
        let mut engine = PhysicsEngine::new().unwrap();
        let time_step = 0.1;
        engine.simulate(time_step).unwrap();
        // After one step, velocity.y should be gravity.y * time_step
        let expected_velocity_y = -EARTH_GRAVITY * time_step;
        assert!((engine.objects[0].velocity.y - expected_velocity_y).abs() < 0.01);
        // Position.y should be velocity.y * time_step
        let expected_position_y = expected_velocity_y * time_step;
        assert!((engine.objects[0].position.y - expected_position_y).abs() < 0.01);
        // x should remain 0
        assert!((engine.objects[0].position.x).abs() < 0.01);
    }

    #[test]
    fn test_wall_collision() {
        let mut engine = PhysicsEngine::new().unwrap();
        engine.objects[0].position.x = 99.5;
        engine.objects[0].velocity.x = 10.0; // towards upper bound
        let time_step = 0.1;
        engine.simulate(time_step).unwrap();
        // Should bounce off upper bound
        assert!((engine.objects[0].position.x - 100.0).abs() < 0.01);
        assert!(engine.objects[0].velocity.x < 0.0); // reversed and dampened
    }

    #[test]
    fn test_add_object() {
        let mut engine = PhysicsEngine::new().unwrap();
        let initial_count = engine.objects.len();
        engine
            .add_object(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0), 5.0)
            .unwrap();
        assert_eq!(engine.objects.len(), initial_count + 1);
        let obj = &engine.objects[1];
        assert_eq!(obj.position, Vector2::new(1.0, 2.0));
        assert_eq!(obj.velocity, Vector2::new(3.0, 4.0));
        assert_eq!(obj.mass, 5.0);
    }

    #[test]
    fn test_set_gravity() {
        let mut engine = PhysicsEngine::new().unwrap();
        let new_gravity = Vector2::new(0.0, -5.0);
        engine.set_gravity(new_gravity);
        assert_eq!(engine.gravity, new_gravity);
    }

    #[test]
    fn test_invalid_mass() {
        assert!(Object::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), 0.0).is_err());
        assert!(Object::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), -1.0).is_err());
        assert!(Object::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            f32::INFINITY
        )
        .is_err());
    }

    #[test]
    fn test_invalid_time_step() {
        let mut engine = PhysicsEngine::new().unwrap();
        assert!(engine.simulate(0.0).is_err());
        assert!(engine.simulate(-1.0).is_err());
        assert!(engine.simulate(f32::INFINITY).is_err());
    }

    #[test]
    fn test_velocity_clamping() {
        let mut engine = PhysicsEngine::new().unwrap();
        engine.objects[0].velocity = Vector2::new(2000.0, -1500.0);
        engine.simulate(0.1).unwrap();
        // Velocity should be clamped to MAX_VELOCITY
        assert!(engine.objects[0].velocity.x.abs() <= MAX_VELOCITY);
        assert!(engine.objects[0].velocity.y.abs() <= MAX_VELOCITY);
    }

    #[test]
    fn test_stress_large_timestep() {
        let mut engine = PhysicsEngine::new().unwrap();
        // Test with a very large time step
        engine.simulate(10.0).unwrap();
        // Should not have NaN or infinite values
        for obj in &engine.objects {
            assert!(obj.position.x.is_finite());
            assert!(obj.position.y.is_finite());
            assert!(obj.velocity.x.is_finite());
            assert!(obj.velocity.y.is_finite());
        }
    }

    #[test]
    fn test_collision_resolution() {
        let mut engine = PhysicsEngine::new().unwrap();
        // Add two objects that will collide
        engine
            .add_object(Vector2::new(1.0, 0.0), Vector2::new(-1.0, 0.0), 1.0)
            .unwrap();
        // Position them to overlap slightly
        engine.objects[0].position = Vector2::new(0.0, 0.0);
        engine.objects[1].position = Vector2::new(0.9, 0.0); // Close enough to collide

        let initial_vel_0 = engine.objects[0].velocity;
        let initial_vel_1 = engine.objects[1].velocity;

        engine.simulate(0.1).unwrap();

        // Velocities should have changed due to collision
        assert_ne!(engine.objects[0].velocity, initial_vel_0);
        assert_ne!(engine.objects[1].velocity, initial_vel_1);

        // Objects should be separated
        let distance = (engine.objects[0].position - engine.objects[1].position).norm();
        assert!(distance >= engine.objects[0].radius + engine.objects[1].radius - 0.1);
        // Allow small tolerance
    }
}
