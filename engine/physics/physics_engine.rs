#[derive(Clone, Debug)]
pub struct Object {
    pub position: f32,
    pub velocity: f32,
    pub mass: f32,
    pub radius: f32,
}

impl Object {
    pub fn new(position: f32, velocity: f32, mass: f32) -> Self {
        Object {
            position,
            velocity,
            mass,
            radius: 0.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PhysicsEngine {
    objects: Vec<Object>,
    gravity: f32,
    bounds: (f32, f32), // min and max position for wall collisions
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine {
            objects: vec![Object::new(0.0, 0.0, 1.0)], // default object
            gravity: -9.81,                            // downward acceleration
            bounds: (-100.0, 100.0),
        }
    }

    #[allow(dead_code)]
    pub fn add_object(&mut self, position: f32, velocity: f32, mass: f32) {
        self.objects.push(Object::new(position, velocity, mass));
    }

    #[allow(dead_code)]
    pub fn set_gravity(&mut self, gravity: f32) {
        self.gravity = gravity;
    }

    fn handle_collisions(&mut self) {
        let len = self.objects.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let pos1 = self.objects[i].position;
                let pos2 = self.objects[j].position;
                let rad1 = self.objects[i].radius;
                let rad2 = self.objects[j].radius;
                let distance = (pos1 - pos2).abs();
                if distance < rad1 + rad2 {
                    // Elastic collision in 1D
                    let m1 = self.objects[i].mass;
                    let m2 = self.objects[j].mass;
                    let v1 = self.objects[i].velocity;
                    let v2 = self.objects[j].velocity;
                    let new_v1 = ((m1 - m2) * v1 + 2.0 * m2 * v2) / (m1 + m2);
                    let new_v2 = ((m2 - m1) * v2 + 2.0 * m1 * v1) / (m1 + m2);
                    self.objects[i].velocity = new_v1;
                    self.objects[j].velocity = new_v2;
                    // Separate them to avoid sticking
                    let overlap = (rad1 + rad2) - distance;
                    let direction = if pos1 > pos2 { 1.0 } else { -1.0 };
                    self.objects[i].position += direction * overlap / 2.0;
                    self.objects[j].position -= direction * overlap / 2.0;
                }
            }
        }
    }

    pub fn simulate(&mut self, time_step: f32) {
        log::info!("Running physics simulation...");
        for obj in &mut self.objects {
            // Apply gravity
            obj.velocity += self.gravity * time_step;
            // Update position
            obj.position += obj.velocity * time_step;

            // Wall collisions
            if obj.position <= self.bounds.0 {
                obj.position = self.bounds.0;
                obj.velocity = -obj.velocity * 0.8; // dampen on bounce
            } else if obj.position >= self.bounds.1 {
                obj.position = self.bounds.1;
                obj.velocity = -obj.velocity * 0.8;
            }
        }

        // Handle object-object collisions
        self.handle_collisions();

        for obj in &self.objects {
            log::info!(
                "Object position: {}, velocity: {}",
                obj.position,
                obj.velocity
            );
        }
    }

    #[allow(dead_code)]
    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity() {
        let mut engine = PhysicsEngine::new();
        let time_step = 0.1;
        let _initial_velocity = 0.0; // Not used, but for clarity
        engine.simulate(time_step);
        // After one step, velocity should be gravity * time_step
        let expected_velocity = -9.81 * time_step;
        assert!((engine.objects[0].velocity - expected_velocity).abs() < 0.01);
        // Position should be velocity * time_step
        let expected_position = expected_velocity * time_step;
        assert!((engine.objects[0].position - expected_position).abs() < 0.01);
    }

    #[test]
    fn test_wall_collision() {
        let mut engine = PhysicsEngine::new();
        engine.objects[0].position = 99.5;
        engine.objects[0].velocity = 10.0; // towards upper bound
        let time_step = 0.1;
        engine.simulate(time_step);
        // Should bounce off upper bound
        assert_eq!(engine.objects[0].position, 100.0);
        assert!(engine.objects[0].velocity < 0.0); // reversed and dampened
    }

    #[test]
    fn test_object_collision() {
        let mut engine = PhysicsEngine::new();
        engine.set_gravity(0.0); // disable gravity for this test
        engine.add_object(1.0, -5.0, 1.0); // second object at position 1, moving left
                                           // First object at 0, velocity 0
        engine.objects[0].velocity = 0.0;
        engine.simulate(0.1);
        // Positions: 0 + 0 = 0, 1 + -5*0.1 = 1 - 0.5 = 0.5
        // Distance 0.5 < 1, collide
        // v1=0, v2=-5, m1=1, m2=1
        // new_v1 = (1-1)*0 + 2*1*(-5) / 2 = -5
        // new_v2 = (1-1)*(-5) + 2*1*0 / 2 = 0
        assert!((engine.objects[0].velocity + 5.0).abs() < 0.01);
        assert!(engine.objects[1].velocity.abs() < 0.01);
    }
}

// Repository URL: https://github.com/bniladridas/simulation_engine
// Directory: engine
