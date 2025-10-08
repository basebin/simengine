# Simulation Engine

A high-performance Rust simulation engine for real-time physics simulations with a web API.  
It provides real-time physics calculations, concurrent simulation management, and a REST API for external control.

## Quick Start

**Prerequisites:**  
Rust 1.90.0 or later. Install from [rustup.rs](https://rustup.rs).

```bash
git clone https://github.com/bniladridas/simulation_engine
cd simulation_engine
cargo run
````

The server starts on `http://localhost:3030`.

## API Usage

### Start a Simulation

```bash
curl "http://localhost:3030/api?time_step=0.1&duration=10.0"
```

### Example (Rust with reqwest)

```rust
use reqwest::blocking;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start simulation
    let resp = blocking::get("http://localhost:3030/api?time_step=0.1&duration=10.0")?;
    println!("{}", resp.text()?); // "Simulation started!"

    // Stop simulation
    let resp = blocking::get("http://localhost:3030/stop")?;
    println!("{}", resp.text()?); // "Simulation stopped!"

    Ok(())
}
```

### Endpoints

| Method | Endpoint       | Description                    |
| :----- | :------------- | :----------------------------- |
| GET    | `/api`         | Start simulation (with params) |
| GET    | `/stop`        | Stop simulation                |
| GET    | `/pause`       | Pause simulation               |
| GET    | `/reset`       | Reset simulation               |
| GET    | `/simulations` | List simulations               |

Use the included Postman collection in `api/postman_collection.json` for testing.

## Testing

Run all tests:

```bash
cargo test
```

Run specific test suites:

```bash
cargo test --test simulation_tests  # Integration tests
cargo test --test e2e_tests         # End-to-end tests
```

Includes 3 unit tests, 5 integration tests, and 5 end-to-end tests for API endpoints.

Run all quality checks locally (equivalent to CI pipeline):

```bash
./scripts/ci.sh
```

This runs:
- `cargo check` for compilation
- `cargo test` for unit and integration tests
- `cargo fmt --check` for code formatting (prompts to auto-fix if issues found)
- `cargo clippy` for linting with warnings as errors

## Conventional Commits

### Setup

```bash
cp scripts/commit-msg .git/hooks/commit-msg
chmod +x .git/hooks/commit-msg
```

### Rules

* Start with type: `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`
* Lowercase, maximum 60 characters

## Contributing

Open issues or pull requests.

## License

MIT