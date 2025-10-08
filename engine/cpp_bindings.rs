#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type SimulationManager;
        type PhysicsEngine;

        fn new_simulation_manager() -> Box<SimulationManager>;
        fn start_simulation(self: &SimulationManager, time_step: f32, duration: f32);
        fn stop_simulation(self: &SimulationManager);
        fn pause_simulations(self: &SimulationManager);
        fn reset_simulations(self: &SimulationManager);

        fn new_physics_engine() -> Box<PhysicsEngine>;
        fn add_object(self: &mut PhysicsEngine, x: f32, y: f32, vx: f32, vy: f32, mass: f32);
        fn set_gravity(self: &mut PhysicsEngine, x: f32, y: f32);
        fn simulate_step(self: &mut PhysicsEngine, time_step: f32);
    }
}

pub struct SimulationManager {
    manager: crate::managers::simulation_manager::SimulationManager,
}

pub fn new_simulation_manager() -> Box<SimulationManager> {
    Box::new(SimulationManager {
        manager: crate::managers::simulation_manager::SimulationManager::new(),
    })
}

impl SimulationManager {
    pub fn start_simulation(&self, time_step: f32, duration: f32) {
        self.manager.start_simulation(time_step, duration);
    }

    pub fn stop_simulation(&self) {
        self.manager.stop_simulation();
    }

    pub fn pause_simulations(&self) {
        self.manager.pause();
    }

    pub fn reset_simulations(&self) {
        self.manager.reset();
    }
}

pub struct PhysicsEngine {
    engine: crate::physics::physics_engine::PhysicsEngine,
}

pub fn new_physics_engine() -> Box<PhysicsEngine> {
    Box::new(PhysicsEngine {
        engine: crate::physics::physics_engine::PhysicsEngine::new(),
    })
}

impl PhysicsEngine {
    pub fn add_object(&mut self, x: f32, y: f32, vx: f32, vy: f32, mass: f32) {
        use nalgebra::Vector2;
        self.engine
            .add_object(Vector2::new(x, y), Vector2::new(vx, vy), mass);
    }

    pub fn set_gravity(&mut self, x: f32, y: f32) {
        use nalgebra::Vector2;
        self.engine.set_gravity(Vector2::new(x, y));
    }

    pub fn simulate_step(&mut self, time_step: f32) {
        self.engine.simulate(time_step);
    }
}
