# Simulation Engine SDK API Reference

This document provides detailed API reference for the Simulation Engine SDK.

## Overview

The Simulation Engine is a Rust library for running real-time physics simulations. It provides components for managing simulations and handling physics calculations.

## Core Components

### SimulationManager

Manages multiple simulation instances.

#### Methods

- `new() -> SimulationManager`: Creates a new simulation manager.
- `start_simulation(time_step: f32, duration: f32)`: Starts a new simulation with the given time step and duration. Runs in a separate thread.
- `stop_simulation()`: Stops the last started simulation.
- `pause()`: Pauses all running simulations.
- `reset()`: Resets all simulations to stopped state.
- `get_simulations() -> Vec<Simulation>`: Returns a list of all simulations.

### Simulation

Represents a single simulation instance.

#### Fields

- `id: u32`: Unique simulation identifier.
- `state: SimulationState`: Current state (Running, Paused, Stopped).
- `time_step: f32`: Time step for physics updates.
- `duration: f32`: Total simulation duration.
- `physics_engine: PhysicsEngine`: The physics engine for this simulation.

#### Methods

- `new(id: u32, time_step: f32, duration: f32) -> Simulation`: Creates a new simulation.
- `start()`: Starts the simulation loop.
- `pause()`: Pauses the simulation.
- `reset()`: Resets the simulation to stopped state.
- `update()`: Updates the simulation state.

### PhysicsEngine

Handles physics calculations and object interactions.

#### Fields

- `objects: Vec<Object>`: List of physics objects.
- `gravity: Vector2<f32>`: Gravity vector.
- `bounds: (Vector2<f32>, Vector2<f32>)`: World bounds for wall collisions.

#### Methods

- `new() -> PhysicsEngine`: Creates a new physics engine with default settings.
- `add_object(position: Vector2<f32>, velocity: Vector2<f32>, mass: f32)`: Adds a new object to the simulation.
- `set_gravity(gravity: Vector2<f32>)`: Sets the gravity vector.
- `simulate(time_step: f32)`: Advances the simulation by one time step.
- `get_objects() -> &Vec<Object>`: Returns a reference to the objects list.

### Object

Represents a physics object.

#### Fields

- `position: Vector2<f32>`: Current position.
- `velocity: Vector2<f32>`: Current velocity.
- `mass: f32`: Object mass.
- `radius: f32`: Object radius (for collision detection).

#### Methods

- `new(position: Vector2<f32>, velocity: Vector2<f32>, mass: f32) -> Object`: Creates a new object.

### DataCleaner

Utility for optimizing simulation performance by cleaning up inactive objects and managing memory.

#### Methods

- `new() -> DataCleaner`: Creates a new data cleaner with default settings.
- `with_cleanup_threshold(threshold: usize) -> DataCleaner`: Configure cleanup threshold.
- `with_inactive_threshold(threshold: f32) -> DataCleaner`: Configure inactive time threshold.
- `with_memory_limit(limit_mb: usize) -> DataCleaner`: Configure memory limit.
- `cleanup_inactive_objects(engine: &mut PhysicsEngine, current_time: f32) -> usize`: Remove inactive objects.
- `optimize_memory(engine: &mut PhysicsEngine) -> bool`: Optimize memory usage.
- `needs_cleanup(engine: &PhysicsEngine) -> bool`: Check if cleanup is needed.
- `get_stats(engine: &PhysicsEngine) -> CleanupStats`: Get cleanup statistics.
- `full_cleanup(engine: &mut PhysicsEngine, current_time: f32) -> CleanupResult`: Perform comprehensive cleanup.

## Usage Example

```rust
use simulation_engine::{managers::SimulationManager, physics::PhysicsEngine};
use nalgebra::Vector2;

fn main() {
    // Create a simulation manager
    let manager = SimulationManager::new();

    // Start a simulation
    manager.start_simulation(0.01, 10.0);

    // Get current simulations
    let sims = manager.get_simulations();
    println!("Running simulations: {}", sims.len());

    // Create a physics engine directly
    let mut engine = PhysicsEngine::new();
    engine.add_object(Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0), 1.0);
    engine.simulate(0.1);
}
```

## Error Handling

All methods are designed to be safe and will not panic under normal operation. However, concurrent access to shared state (e.g., in multi-threaded scenarios) should be handled carefully using the provided Arc<Mutex<>> wrappers.

## Performance Notes

- Simulations run in separate threads for concurrency.
- Physics calculations use nalgebra for efficient vector operations.
- The engine is optimized for real-time performance with configurable time steps.
