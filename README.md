# Simulation Engine

Welcome to the Simulation Engine! 🚀

This high-performance Rust-based project lets you run real-time physics simulations with an easy-to-use web API. Perfect for developers wanting to add simulation capabilities to their apps.

## What it does

- ⚡ Runs physics simulations in real-time
- 🔄 Manages multiple simulations at once
- 🌐 Provides a simple REST API for control
- 🦀 Built with Rust for speed and reliability

## Quick Start

## Quick Start

### Prerequisites
- Rust 1.90.0 or later (install from [rustup.rs](https://rustup.rs))

### Installation
```bash
git clone https://github.com/bniladridas/simulation_engine
cd simulation_engine
cargo run
```
The server will start on `http://localhost:3030`. You're ready to simulate! 🎉

## API Usage

Start your first simulation:
```bash
curl "http://localhost:3030/api?time_step=0.1&duration=10.0"
```

Control your simulations:
- `GET /stop` - Stop the current simulation
- `GET /pause` - Pause it
- `GET /reset` - Reset to start
- `GET /simulations` - See all running simulations

Try it out with the included Postman collection! 📡

## Testing

We take testing seriously! Run the full test suite:
```bash
cargo test
```
```
running 5 tests
... (tests pass)
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
running 5 tests
... (tests pass)
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

For integration tests only:
```bash
cargo test --test simulation_tests
```
```
running 5 tests
test tests::test_get_simulations ... ok
test tests::test_pause_simulation ... ok
test tests::test_reset_simulation ... ok
test tests::test_start_simulation ... ok
test tests::test_stop_simulation ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

For end-to-end tests:
```bash
cargo test --test e2e_tests
```
```
running 5 tests
test e2e_tests::test_e2e_reset_simulation ... ok
test e2e_tests::test_e2e_pause_simulation ... ok
test e2e_tests::test_e2e_start_simulation ... ok
test e2e_tests::test_e2e_get_simulations ... ok
test e2e_tests::test_e2e_stop_simulation ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Covers:
- 5 integration tests for API endpoints (using warp::test)
- 5 e2e tests for API endpoints (using real HTTP requests with reqwest)

All tests passing means everything works! ✅

## Conventional Commits

We follow conventional commits for clean git history.

### Setup
```bash
cp scripts/commit-msg .git/hooks/commit-msg
chmod +x .git/hooks/commit-msg
```

### Rules
- Start with type: `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`
- All lowercase, max 60 characters

## Contributing

Got ideas? Open an issue or PR! Let's build something amazing together. 🤝

## License

MIT - Free to use, modify, and distribute! 📄
