use nalgebra::Vector2;
use simulation_engine::physics::{data_cleaner::DataCleaner, physics_engine::PhysicsEngine};

fn main() {
    println!("Starting data cleaner example...");

    let mut engine = PhysicsEngine::new().expect("Failed to create physics engine");

    // Add many objects to simulate a busy simulation
    println!("Adding 2000 objects to simulation...");
    for i in 0..2000 {
        // Mix of active and inactive objects
        let position = Vector2::new(i as f32 * 0.5, (i % 100) as f32);
        let velocity = if i % 10 == 0 {
            // Some objects are moving (active)
            Vector2::new(1.0, 0.5)
        } else {
            // Most objects are stationary (inactive)
            Vector2::new(0.0, 0.0)
        };
        engine
            .add_object(position, velocity, 1.0)
            .expect("Failed to add object");
    }

    // Create data cleaner
    let cleaner = DataCleaner::new()
        .with_cleanup_threshold(1500)
        .with_inactive_threshold(30.0);

    // Get initial statistics
    let initial_stats = cleaner.get_stats(&engine);
    println!("Initial state:");
    println!("  Total objects: {}", initial_stats.total_objects);
    println!("  Active objects: {}", initial_stats.active_objects);
    println!("  Inactive objects: {}", initial_stats.inactive_objects);
    println!("  Needs cleanup: {}", initial_stats.needs_cleanup);

    // Run simulation for a bit to let objects move
    println!("\nRunning simulation...");
    for _ in 0..50 {
        engine.simulate(0.1).expect("Simulation failed");
    }

    // Check cleanup status
    if cleaner.needs_cleanup(&engine) {
        println!("\nPerforming cleanup...");

        // Perform cleanup
        let cleanup_result = cleaner.full_cleanup(&mut engine, 5.0);

        println!("Cleanup completed:");
        println!(
            "  Inactive objects removed: {}",
            cleanup_result.inactive_objects_removed
        );
        println!("  Memory optimized: {}", cleanup_result.memory_optimized);

        // Get final statistics
        let final_stats = cleaner.get_stats(&engine);
        println!("\nFinal state:");
        println!("  Total objects: {}", final_stats.total_objects);
        println!("  Active objects: {}", final_stats.active_objects);
        println!("  Inactive objects: {}", final_stats.inactive_objects);
    } else {
        println!("\nCleanup not needed at this time.");
    }

    println!("\nData cleaner example completed.");
}
