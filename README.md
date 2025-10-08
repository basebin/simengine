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

## Contributing

Open an issue or PR.

## License

MIT
