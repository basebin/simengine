use simulation_engine::managers::simulation_manager::SimulationManager;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting basic simulation example...");

    let manager = SimulationManager::new();

    // Start a simulation
    let _ = manager.start_simulation(0.01, 2.0); // 0.01s time step, 2s duration

    // Wait for simulation to complete
    thread::sleep(Duration::from_secs(3));

    // Get final state
    let simulations = manager.get_simulations();
    println!("Final simulation count: {}", simulations.len());

    println!("Basic simulation example completed.");
}
