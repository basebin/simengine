#include "simulation_engine.hpp"
#include <iostream>
#include <thread>
#include <chrono>

int main() {
    std::cout << "Starting C++ simulation example..." << std::endl;

    // Create simulation manager
    auto manager = new_simulation_manager();

    // Start a simulation
    manager->start_simulation(0.01f, 2.0f); // 0.01s time step, 2s duration

    // Wait for simulation to complete
    std::this_thread::sleep_for(std::chrono::seconds(3));

    std::cout << "C++ simulation example completed." << std::endl;
    return 0;
}