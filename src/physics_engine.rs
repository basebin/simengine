#[derive(Clone, Debug)]
pub struct PhysicsEngine {
    position: f32, 
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine { position: 0.0 }
    }

    pub fn simulate(&mut self) {
        // Example logic for running a physics simulation
        println!("Running physics simulation...");
        self.position += 1.0; // Update position
        println!("Updated position: {}", self.position);
        println!("Current position: {}", self.get_position()); // Call get_position
    }

    pub fn get_position(&self) -> f32 {
        self.position
    }
}
