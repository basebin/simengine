use nalgebra::Vector2;

#[derive(Clone, Debug)]
pub struct Object {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
}

impl Object {
    pub fn new(position: Vector2<f32>, velocity: Vector2<f32>, mass: f32) -> Self {
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
    gravity: Vector2<f32>,
    bounds: (Vector2<f32>, Vector2<f32>), // min and max position for wall collisions
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine {
            objects: vec![Object::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(0.0, 0.0),
                1.0,
            )], // default object
            gravity: Vector2::new(0.0, -9.81), // downward acceleration
            bounds: (Vector2::new(-100.0, -100.0), Vector2::new(100.0, 100.0)),
        }
    }

    #[allow(dead_code)]
    pub fn add_object(&mut self, position: Vector2<f32>, velocity: Vector2<f32>, mass: f32) {
        self.objects.push(Object::new(position, velocity, mass));
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
                let pos1 = self.objects[i].position;
                let pos2 = self.objects[j].position;
                let rad1 = self.objects[i].radius;
                let rad2 = self.objects[j].radius;
                let diff = pos1 - pos2;
                let distance = diff.norm();
                if distance < rad1 + rad2 && distance > 0.0 {
                    // Elastic collision in 2D
                    let m1 = self.objects[i].mass;
                    let m2 = self.objects[j].mass;
                    let v1 = self.objects[i].velocity;
                    let v2 = self.objects[j].velocity;
                    let normal = diff / distance; // unit vector from 2 to 1
                    let relative_velocity = v1 - v2;
                    let velocity_along_normal = relative_velocity.dot(&normal);
                    if velocity_along_normal > 0.0 {
                        continue; // objects separating
                    }
                    let restitution = 1.0; // elastic
                    let impulse_scalar =
                        -(1.0 + restitution) * velocity_along_normal / (1.0 / m1 + 1.0 / m2);
                    let impulse = impulse_scalar * normal;
                    self.objects[i].velocity += impulse / m1;
                    self.objects[j].velocity -= impulse / m2;
                    // Separate them to avoid sticking
                    let overlap = (rad1 + rad2) - distance;
                    let separation = normal * (overlap / 2.0);
                    self.objects[i].position += separation;
                    self.objects[j].position -= separation;
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

            // Wall collisions (clamp to bounds)
            if obj.position.x < self.bounds.0.x {
                obj.position.x = self.bounds.0.x;
            } else if obj.position.x > self.bounds.1.x {
                obj.position.x = self.bounds.1.x;
            }
            if obj.position.y < self.bounds.0.y {
                obj.position.y = self.bounds.0.y;
            } else if obj.position.y > self.bounds.1.y {
                obj.position.y = self.bounds.1.y;
            }
            // If hit wall, reverse velocity component and dampen
            if obj.position.x <= self.bounds.0.x || obj.position.x >= self.bounds.1.x {
                obj.velocity.x = -obj.velocity.x * 0.8;
            }
            if obj.position.y <= self.bounds.0.y || obj.position.y >= self.bounds.1.y {
                obj.velocity.y = -obj.velocity.y * 0.8;
            }
        }

        // Handle object-object collisions
        // self.handle_collisions();

        for obj in &self.objects {
            log::info!(
                "Object position: {:?}, velocity: {:?}",
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_gravity() {
//         let mut engine = PhysicsEngine::new();
//         let time_step = 0.1;
//         engine.simulate(time_step);
//         // After one step, velocity.y should be gravity.y * time_step
//         let expected_velocity_y = -9.81 * time_step;
//         assert!((engine.objects[0].velocity.y - expected_velocity_y).abs() < 0.01);
//         // Position.y should be velocity.y * time_step
//         let expected_position_y = expected_velocity_y * time_step;
//         assert!((engine.objects[0].position.y - expected_position_y).abs() < 0.01);
//         // x should remain 0
//         assert!((engine.objects[0].position.x).abs() < 0.01);
//     }

//     #[test]
//     fn test_wall_collision() {
//         let mut engine = PhysicsEngine::new();
//         engine.objects[0].position.x = 99.5;
//         engine.objects[0].velocity.x = 10.0; // towards upper bound
//         let time_step = 0.1;
//         engine.simulate(time_step);
//         // Should bounce off upper bound
//         assert!((engine.objects[0].position.x - 100.0).abs() < 0.01);
//         assert!(engine.objects[0].velocity.x < 0.0); // reversed and dampened
//     }

//     #[test]
//     fn test_object_collision() {
//         let mut engine = PhysicsEngine::new();
//         engine.set_gravity(Vector2::zeros()); // disable gravity for this test
//         engine.add_object(Vector2::new(1.0, 0.0), Vector2::new(-5.0, 0.0), 1.0); // second object at position (1,0), moving left
//         // First object at (0,0), velocity (0,0)
//         engine.objects[0].velocity = Vector2::zeros();
//         engine.simulate(0.1);
//         // Positions: (0,0) + (0,0) = (0,0), (1,0) + (-5*0.1,0) = (0.5,0)
//         // Distance 0.5 < 1, collide
//         // v1=(0,0), v2=(-5,0), m1=1, m2=1
//         // new_v1.x = -5, new_v2.x = 0
//         assert!((engine.objects[0].velocity.x + 5.0).abs() < 0.01);
//         assert!(engine.objects[1].velocity.x.abs() < 0.01);
//     }
// }


