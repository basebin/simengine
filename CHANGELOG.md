# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-29

### Added
- Initial release of Simulation Engine SDK
- Core physics simulation engine with gravity, collisions, and wall boundaries
- Simulation manager for concurrent simulation handling
- REST API for external control via HTTP endpoints
- Python bindings using PyO3
- C++ bindings using CXX
- Comprehensive documentation and examples
- Cross-platform CI/CD pipeline
- Unit, integration, and end-to-end tests

### Features
- Real-time physics calculations with configurable time steps
- Multi-object physics with elastic collisions
- Customizable gravity and world boundaries
- Thread-safe simulation management
- Web API with endpoints for start/stop/pause/reset operations
- Language bindings for Python and C++

### Technical
- Built with Rust 2021 edition
- Uses nalgebra for high-precision vector mathematics
- Tokio for async runtime
- Warp web framework for API
- Comprehensive test suite with 19 total tests

[0.1.0]: https://github.com/bniladridas/simulation-engine/releases/tag/v0.1.0