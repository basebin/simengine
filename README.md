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
Output: 10 tests passed (5 integration + 5 e2e)

Run integration tests only:
```bash
cargo test --test simulation_tests
```
Output: 5 tests passed

Run e2e tests only:
```bash
cargo test --test e2e_tests
```
Output: 5 tests passed

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
