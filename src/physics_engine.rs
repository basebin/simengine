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
        log::info!("Running physics simulation...");
        self.position += 1.0; // Update position
        log::info!("Updated position: {}", self.position);
        log::info!("Current position: {}", self.get_position()); // Call get_position
    }

    pub fn get_position(&self) -> f32 {
        self.position
    }
}
