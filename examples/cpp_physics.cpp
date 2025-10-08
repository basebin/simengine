#include "simulation_engine.hpp"
#include <iostream>

int main() {
    std::cout << "Starting C++ physics example..." << std::endl;

    // Create physics engine
    auto engine = new_physics_engine();

    // Add objects
    engine->add_object(0.0f, 0.0f, 1.0f, 0.0f, 1.0f); // x, y, vx, vy, mass
    engine->add_object(5.0f, 0.0f, -0.5f, 0.0f, 1.5f);

    // Set custom gravity
    engine->set_gravity(0.0f, -5.0f);

    // Run simulation for 5 steps
    for (int step = 0; step < 5; ++step) {
        engine->simulate_step(0.1f);
        std::cout << "Step " << step << std::endl;
        // Note: In a real implementation, you'd need to expose object data
    }

    std::cout << "C++ physics example completed." << std::endl;
    return 0;
}