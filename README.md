<p align="center">
  <img src="https://raw.githubusercontent.com/Coccinella-Labs/simengine/main/.github/assets/thumbnail.png" alt="simengine" width="100%">
</p>

# Simulation Engine

A high-performance Rust engine for real-time physics simulations with a web API.
It supports concurrent simulations, precise physics calculations, and external control via REST.

## Quick Start

**Requires:** Rust 1.90.0+ ([Install](https://rustup.rs))

```bash
git clone https://github.com/bniladridas/simulation-engine
cd simulation-engine
cargo run
````

Server starts on **[http://localhost:3030](http://localhost:3030)**.

## API

### Start a Simulation

```bash
curl "http://localhost:3030/api?time_step=0.1&duration=10.0"
```

### Example (Rust + Reqwest)

```rust
use reqwest::blocking;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = blocking::get("http://localhost:3030/api?time_step=0.1&duration=10.0")?;
    println!("{}", resp.text()?);

    let resp = blocking::get("http://localhost:3030/stop")?;
    println!("{}", resp.text()?);

    Ok(())
}
```

### Endpoints

| Method | Endpoint       | Description      |
| :----- | :------------- | :--------------- |
| GET    | `/api`         | Start simulation |
| GET    | `/stop`        | Stop simulation  |
| GET    | `/pause`       | Pause simulation |
| GET    | `/reset`       | Reset simulation |
| GET    | `/simulations` | List simulations |

📁 Use the Postman collection in `api/postman_collection.json` for quick testing.

## Testing

Run all tests:

```bash
cargo test
```

Specific suites:

```bash
cargo test --test simulation_tests  # Integration
cargo test --test e2e_tests         # End-to-end
```

Includes 3 unit tests, 5 integration tests, and 5 end-to-end API tests.

Run all local checks (same as CI):

```bash
./scripts/ci.sh
```

Runs:

* `cargo check` — compile check
* `cargo test` — all tests
* `cargo fmt --check` — formatting
* `cargo clippy` — linting (treat warnings as errors)

## Conventional Commits

### Setup

```bash
cp scripts/commit-msg .git/hooks/commit-msg
chmod +x .git/hooks/commit-msg
```

### Format

Use:

```
feat: short summary
fix: resolve issue
docs: update docs
refactor: code restructure
chore: misc tasks
```

Lowercase, ≤60 chars.

## SDK Usage

The Simulation Engine can be used as a Rust library for embedding physics simulations in your applications.

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
simulation_engine = "0.1.0"
```

### Quick Start

```rust
use simulation_engine::managers::SimulationManager;

fn main() {
    let manager = SimulationManager::new();
    manager.start_simulation(0.01, 10.0); // Start a 10-second simulation
}
```

See [docs/API.md](docs/API.md) for detailed API reference and [docs/Examples.md](docs/Examples.md) for more examples.

### Python Bindings

For Python usage, build with the `python` feature:

```bash
cargo build --features python
```

Then use the `simulation_engine_py` module.

### C++ Bindings

For C++ usage, build with the `cxx` feature:

```bash
cargo build --features cxx
```

Include the generated header and link against the library.

## Automated Releases

Releases are automated using GitHub Actions and `cargo-release`. On merges to `main` with conventional commits:

- `feat:` commits bump minor version (e.g., 0.1.0 → 0.1.1)
- `fix:` commits bump patch version (e.g., 0.1.0 → 0.1.1)
- `BREAKING CHANGE:` in footer bumps major version

The workflow creates tags, GitHub releases, and attaches built binaries.

## Contributing

Open an issue or PR.

## License

MIT
