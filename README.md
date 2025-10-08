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

Run the test suite:
```bash
cargo test
```

Includes 5 integration tests for API endpoints.

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
