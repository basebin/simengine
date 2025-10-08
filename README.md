# Simulation Engine

A high-performance simulation engine built in Rust for real-time physics simulations with a web API.

## Getting Started

### Prerequisites
- Rust 1.84.1 or later

### Installation
```bash
git clone https://github.com/bniladridas/simulation_engine
cd simulation_engine
cargo run
```

## API Usage

Start a simulation:
```bash
curl "http://localhost:3030/api?time_step=0.1&duration=10.0"
```

Other endpoints:
- `GET /stop` - Stop simulation
- `GET /pause` - Pause simulation
- `GET /reset` - Reset simulation
- `GET /simulations` - List simulations

## Testing

Run all tests:
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

Run integration tests only:
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

Run e2e tests only:
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

Includes:
- 5 integration tests for API endpoints (using warp::test)
- 5 e2e tests for API endpoints (using real HTTP requests with reqwest)

## Conventional Commits

This project uses conventional commits.

### Setup
```bash
cp scripts/commit-msg .git/hooks/commit-msg
chmod +x .git/hooks/commit-msg
```

### Rules
- Start with type: `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`
- Lowercase, ≤60 chars

## License

MIT
