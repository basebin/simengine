# Simulation Engine Examples

This directory contains example projects demonstrating how to use the Simulation Engine SDK.

## Running Examples

### Rust Examples

```bash
cd examples
cargo run --bin basic_simulation
cargo run --bin custom_physics
```

### Python Examples

First, build with Python feature:

```bash
cargo build --features python
```

Then run:

```bash
python3 python_example.py
python3 python_physics.py
```

### C++ Examples

First, build with C++ feature:

```bash
cargo build --features cxx
```

Then build and run C++ examples:

```bash
cd examples
make
./cpp_example
./cpp_physics
```

## Examples

### Rust

- `basic_simulation.rs`: Demonstrates basic simulation management using SimulationManager.
- `custom_physics.rs`: Shows how to set up custom physics scenarios with multiple objects.
- `data_cleaner.rs`: Demonstrates data cleaning and optimization utilities.

### Python

- `python_example.py`: Basic simulation management in Python.
- `python_physics.py`: Custom physics setup in Python.

### C++

- `cpp_example.cpp`: Basic simulation management in C++.
- `cpp_physics.cpp`: Custom physics setup in C++.