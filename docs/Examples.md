# Simulation Engine SDK Examples

This document provides usage examples for the Simulation Engine SDK.

## Basic Simulation

```rust
use simulation_engine::managers::SimulationManager;

fn main() {
    let manager = SimulationManager::new();
    manager.start_simulation(0.01, 5.0); // 0.01s time step, 5s duration
    // Simulation runs in background
}
```

## Custom Physics Setup

```rust
use simulation_engine::physics::PhysicsEngine;
use nalgebra::Vector2;

fn main() {
    let mut engine = PhysicsEngine::new();

    // Add objects
    engine.add_object(Vector2::new(0.0, 0.0), Vector2::new(1.0, 2.0), 1.0);
    engine.add_object(Vector2::new(5.0, 0.0), Vector2::new(-1.0, 0.0), 2.0);

    // Set custom gravity
    engine.set_gravity(Vector2::new(0.0, -5.0));

    // Run simulation steps
    for _ in 0..100 {
        engine.simulate(0.1);
        let objects = engine.get_objects();
        for obj in objects {
            println!("Position: {:?}", obj.position);
        }
    }
}
```

## Multi-Simulation Management

```rust
use simulation_engine::managers::SimulationManager;
use std::thread;
use std::time::Duration;

fn main() {
    let manager = SimulationManager::new();

    // Start multiple simulations
    manager.start_simulation(0.01, 2.0);
    manager.start_simulation(0.02, 3.0);

    // Let them run for a bit
    thread::sleep(Duration::from_secs(1));

    // Pause all
    manager.pause();

    // Check status
    let sims = manager.get_simulations();
    for sim in sims {
        println!("Simulation {} state: {:?}", sim.id, sim.state);
    }

    // Stop all
    manager.reset();
}
```

## Integration with External Systems

```rust
use simulation_engine::physics::PhysicsEngine;
use nalgebra::Vector2;

// Example: Reading data from a file or network
fn load_objects_from_data(data: Vec<(f32, f32, f32, f32, f32)>) -> Vec<Object> {
    data.into_iter()
        .map(|(x, y, vx, vy, mass)| {
            Object::new(Vector2::new(x, y), Vector2::new(vx, vy), mass)
        })
        .collect()
}

fn main() {
    let mut engine = PhysicsEngine::new();

    // Load objects from external source
    let data = vec![(0.0, 0.0, 1.0, 0.0, 1.0), (10.0, 0.0, -1.0, 0.0, 1.5)];
    let objects = load_objects_from_data(data);

    for obj in objects {
        engine.add_object(obj.position, obj.velocity, obj.mass);
    }

    // Simulate
    engine.simulate(0.1);
}
```