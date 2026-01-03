use nalgebra::Vector2;
use simulation_engine::physics::physics_engine::PhysicsEngine;

fn main() {
    println!("Starting custom physics example...");

    let mut engine = PhysicsEngine::new().expect("Failed to create physics engine");

    // Add multiple objects
    engine
        .add_object(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0), 1.0)
        .expect("Failed to add object");
    engine
        .add_object(Vector2::new(5.0, 0.0), Vector2::new(-0.5, 0.0), 1.5)
        .expect("Failed to add object");

    // Set custom gravity
    engine.set_gravity(Vector2::new(0.0, -5.0));

    // Run simulation for 10 steps
    for step in 0..10 {
        engine.simulate(0.1).expect("Simulation failed");
        let objects = engine.get_objects();
        println!("Step {}:", step);
        for (i, obj) in objects.iter().enumerate() {
            println!(
                "  Object {}: pos={:?}, vel={:?}",
                i, obj.position, obj.velocity
            );
        }
    }

    println!("Custom physics example completed.");
}
