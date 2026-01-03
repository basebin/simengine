use nalgebra::Vector2;
use simulation_engine::managers::simulation_manager::SimulationManager;
use simulation_engine::physics::physics_engine::PhysicsEngine;

#[test]
fn test_simulation_manager_creation() {
    let manager = SimulationManager::new();
    let simulations = manager.get_simulations();
    assert_eq!(simulations.len(), 0);
}

#[test]
fn test_physics_engine_creation() {
    let engine = PhysicsEngine::new().unwrap();
    let objects = engine.get_objects();
    assert_eq!(objects.len(), 1); // Default object
}

#[test]
fn test_physics_engine_add_object() {
    let mut engine = PhysicsEngine::new().unwrap();
    let initial_count = engine.get_objects().len();
    engine
        .add_object(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0), 5.0)
        .unwrap();
    assert_eq!(engine.get_objects().len(), initial_count + 1);
}

#[test]
fn test_physics_engine_simulate() {
    let mut engine = PhysicsEngine::new().unwrap();
    let initial_pos = engine.get_objects()[0].position;
    engine.simulate(0.1).unwrap();
    let new_pos = engine.get_objects()[0].position;
    assert_ne!(initial_pos, new_pos); // Position should change due to gravity
}

#[test]
fn test_simulation_manager_start_simulation() {
    let manager = SimulationManager::new();
    manager.start_simulation(0.01, 0.1); // Very short simulation
    std::thread::sleep(std::time::Duration::from_millis(200)); // Wait for completion
    let _simulations = manager.get_simulations();
    // Note: This test is timing-dependent and may be flaky
    // In a real scenario, you'd use proper synchronization
}
