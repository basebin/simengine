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

| Method | Endpoint       | Description      |
| :----- | :------------- | :--------------- |
| GET    | `/stop`        | Stop simulation  |
| GET    | `/pause`       | Pause simulation |
| GET    | `/reset`       | Reset simulation |
| GET    | `/simulations` | List simulations |

Use the included Postman collection in `api/postman_collection.json` for testing.

## Testing

Run all tests:

```bash
cargo test
```

Example output:

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running unittests engine/lib.rs (target/debug/deps/simulation_engine-400ba6fdc07de021)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests engine/main.rs (target/debug/deps/simulation_engine-6c2b0ff9e28da289)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/e2e_tests.rs (target/debug/deps/e2e_tests-11cb2792fb07ec12)

running 5 tests
test e2e_tests::test_e2e_start_simulation ... ok
test e2e_tests::test_e2e_pause_simulation ... ok
test e2e_tests::test_e2e_reset_simulation ... ok
test e2e_tests::test_e2e_stop_simulation ... ok
test e2e_tests::test_e2e_get_simulations ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

     Running tests/simulation_tests.rs (target/debug/deps/simulation_tests-98ea053ab1b1cc4d)

running 5 tests
test tests::test_pause_simulation ... ok
test tests::test_reset_simulation ... ok
test tests::test_get_simulations ... ok
test tests::test_stop_simulation ... ok
test tests::test_start_simulation ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests simulation_engine

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
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

## Local CI

Run all quality checks locally (equivalent to CI pipeline):

```bash
./scripts/ci.sh
```

This runs:
- `cargo check` for compilation
- `cargo test` for unit and integration tests
- `cargo fmt --check` for code formatting
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