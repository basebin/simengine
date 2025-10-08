# Simulation Engine

A high-performance Rust simulation engine for real-time physics simulations with a web API. It provides real-time physics calculations, concurrent simulation management, and a REST API for external control.

## Features

- Real-time physics simulation
- Concurrent simulation management
- RESTful web API
- Built with Rust

## Quick Start

### Prerequisites

Rust 1.90.0 or later. Install from [rustup.rs](https://rustup.rs).

### Installation

```bash
git clone https://github.com/bniladridas/simulation_engine
cd simulation_engine
cargo run
```

The server starts on `http://localhost:3030`.

## API Usage

Start a simulation:

```bash
curl "http://localhost:3030/api?time_step=0.1&duration=10.0"
```

Example in Rust (using reqwest):

```rust
use reqwest::blocking;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start simulation
    let resp = blocking::get("http://localhost:3030/api?time_step=0.1&duration=10.0")?;
    println!("{}", resp.text()?);  // "Simulation started!"

    // Stop simulation
    let resp = blocking::get("http://localhost:3030/stop")?;
    println!("{}", resp.text()?);  // "Simulation stopped!"

    Ok(())
}
```

Endpoints:

- `GET /stop` - Stop simulation
- `GET /pause` - Pause simulation
- `GET /reset` - Reset simulation
- `GET /simulations` - List simulations

Use the included Postman collection for testing.

## Testing

Run all tests:

```bash
cargo test
```

Run integration tests:

```bash
cargo test --test simulation_tests
```

Run end-to-end tests:

```bash
cargo test --test e2e_tests
```

Includes 5 integration tests and 5 end-to-end tests for API endpoints.

## Conventional Commits

### Setup

```bash
cp scripts/commit-msg .git/hooks/commit-msg
chmod +x .git/hooks/commit-msg
```

### Rules

- Start with type: `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`
- Lowercase, max 60 characters

## Contributing

Open issues or pull requests.

## License

MIT
