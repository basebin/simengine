#!/usr/bin/env python3

import simulation_engine_py as sim
import time

def main():
    print("Starting Python simulation example...")

    # Create simulation manager
    manager = sim.PySimulationManager()

    # Start a simulation
    manager.start_simulation(0.01, 2.0)  # 0.01s time step, 2s duration

    # Wait for simulation to complete
    time.sleep(3)

    print("Python simulation example completed.")

if __name__ == "__main__":
    main()