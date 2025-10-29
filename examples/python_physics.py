#!/usr/bin/env python3

import simulation_engine_py as sim

def main():
    print("Starting Python physics example...")

    # Create physics engine
    engine = sim.PyPhysicsEngine()

    # Add objects
    engine.add_object(0.0, 0.0, 1.0, 0.0, 1.0)  # x, y, vx, vy, mass
    engine.add_object(5.0, 0.0, -0.5, 0.0, 1.5)

    # Set custom gravity
    engine.set_gravity(0.0, -5.0)

    # Run simulation for 5 steps
    for step in range(5):
        engine.simulate(0.1)
        objects = engine.get_objects()
        print(f"Step {step}:")
        for i, obj in enumerate(objects):
            print(f"  Object {i}: pos={obj.position}, vel={obj.velocity}")

    print("Python physics example completed.")

if __name__ == "__main__":
    main()
