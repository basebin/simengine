# Simulation Engine

This is a high-performance simulation engine built in Rust, designed for real-time simulations and integration with internal modeling frameworks.

## Terminal

```bash
# Commands to run the simulation engine
cargo build
   Compiling simulation_engine v0.1.0 (/Users/niladridas/Desktop/github/simulation_engine)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.75s
```

## Key Features
- Real-time physics simulations
- Multi-threaded architecture
- API for interaction
- Integration capabilities with other frameworks

## Project Version
- **Version**: 0.1.0

## Dependencies
- **tokio**: Version 1 (full features)
- **rayon**: Version 1.5
- **cxx**: Version 1.0 (For C++ interoperability)
- **nalgebra**: Version 0.29 (For high-precision arithmetic)
- **warp**: Version 0.3
- **serde**: Version 1.0
- **serde_json**: Version 1.0
- **serde_derive**: Version 1.0

### Development Dependencies
- **criterion**: Version 0.3 (For benchmarking)

## Getting Started
### Prerequisites
- Rust version: 1.84.1

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/bniladridas/simulation_engine
   cd simulation_engine
   ```

2. Run the project:
   ```bash
   cargo run
   ```

## API Usage

### Starting the Simulation
To start the simulation, send a GET request to the `/api` endpoint with the following query parameters:
- **time_step**: The time step for the simulation (e.g., `0.1`)
- **duration**: The duration of the simulation (e.g., `10.0`)

#### Example Request
You can use `curl` to make the request:
```bash
curl "http://localhost:3030/api?time_step=0.1&duration=10.0"
```

### Expected Response
The API will respond with a message indicating that the simulation is starting:
```
Starting simulation with time_step: 0.1 and duration: 10.0
```

## Usage Examples
- Example API call to start a simulation:
   ```bash
   curl "http://localhost:3030/api?time_step=0.1&duration=10.0"
   ```

## Troubleshooting API Query String Issues

If you encounter an "Invalid query string" error when calling the API, follow these steps to resolve the issue:

1. **Check API Endpoint Implementation**:
   - Ensure the API endpoint in `src/main.rs` correctly handles query parameters. It should look similar to:
   ```rust
   let api = warp::path("api")
       .and(warp::get())
       .and(warp::query::<YourQueryStruct>())
       .map(|params: YourQueryStruct| {
           // Your logic to start the simulation
       });
   ```

2. **Define the Query Struct**:
   - If not already defined, create a struct to represent the query parameters:
   ```rust
   #[derive(Debug, Deserialize)]
   struct SimulationParams {
       time_step: f32,
       duration: f32,
   }
   ```

3. **Update the API Logic**:
   - Ensure the API logic uses the parameters correctly:
   ```rust
   .map(|params: SimulationParams| {
       // Start the simulation using params.time_step and params.duration
   });
   ```

4. **Run the Server Again**:
   - After making changes, save the files and restart the server:
   ```bash
   cargo run
   ```

5. **Test the API Again**:
   - Once the server is running, try making the `curl` request again:
   ```bash
   curl "http://localhost:3030/api?time_step=0.1&duration=10.0"
   ```

6. **Additional Debugging**:
   - If issues persist, add logging statements in your API handler to check received parameters and any errors.

## Warning
> **⚠️ Warning:** Please ensure that you have the latest version of the repository before making any changes. This will help avoid conflicts and ensure that you are working with the most up-to-date code.

## Command Output
```bash
(base) Niladris-MacBook-Air:simulation_engine niladridas$ cargo clean && cargo build
     Removed 2276 files, 625.2MiB total
   Compiling proc-macro2 v1.0.93
   Compiling unicode-ident v1.0.16
   Compiling autocfg v1.4.0
   Compiling libc v0.2.169
   Compiling cfg-if v1.0.0
   Compiling stable_deref_trait v1.2.0
   Compiling version_check v0.9.5
   Compiling bytes v1.10.0
   Compiling smallvec v1.13.2
   Compiling itoa v1.0.14
   Compiling typenum v1.17.0
   Compiling writeable v0.5.5
   Compiling litemap v0.7.4
   Compiling fnv v1.0.7
   Compiling generic-array v0.14.7
   Compiling pin-project-lite v0.2.16
   Compiling lock_api v0.4.12
   Compiling parking_lot_core v0.9.10
   Compiling icu_locid_transform_data v1.5.0
   Compiling slab v0.4.9
   Compiling num-traits v0.2.19
   Compiling icu_properties_data v1.5.0
   Compiling log v0.4.25
   Compiling scopeguard v1.2.0
   Compiling futures-core v0.3.31
   Compiling futures-sink v0.3.31
   Compiling http v0.2.12
   Compiling memchr v2.7.4
   Compiling quote v1.0.38
   Compiling syn v2.0.98
   Compiling utf8_iter v1.0.4
   Compiling write16 v1.0.0
   Compiling httparse v1.10.0
   Compiling utf16_iter v1.0.5
   Compiling shlex v1.3.0
   Compiling byteorder v1.5.0
   Compiling icu_normalizer_data v1.5.0
   Compiling parking_lot v0.12.3
   Compiling cc v1.2.14
   Compiling signal-hook-registry v1.4.2
   Compiling mio v1.0.3
   Compiling getrandom v0.2.15
   Compiling socket2 v0.5.8
   Compiling pin-utils v0.1.0
   Compiling futures-task v0.3.31
   Compiling crossbeam-utils v0.8.21
   Compiling percent-encoding v2.3.1
   Compiling form_urlencoded v1.2.1
   Compiling crypto-common v0.1.6
   Compiling block-buffer v0.10.4
   Compiling futures-util v0.3.31
   Compiling rand_core v0.6.4
   Compiling cpufeatures v0.2.17
   Compiling digest v0.10.7
   Compiling thiserror v1.0.69
   Compiling once_cell v1.20.3
   Compiling serde v1.0.217
   Compiling sha1 v0.10.6
   Compiling tracing-core v0.1.33
   Compiling hashbrown v0.15.2
   Compiling rustversion v1.0.19
   Compiling paste v1.0.15
   Compiling unicase v2.8.1
   Compiling link-cplusplus v1.0.9
   Compiling mime v0.3.17
   Compiling syn v1.0.109
   Compiling bytemuck v1.21.0
   Compiling equivalent v1.0.1
   Compiling mime_guess v2.0.5
   Compiling indexmap v2.7.1
   Compiling safe_arch v0.7.4
   Compiling crossbeam-epoch v0.9.18
   Compiling tracing v0.1.41
   Compiling aho-corasick v1.1.3
   Compiling http v0.3.26
   Compiling matrixmultiply v0.3.9
   Compiling multer v2.1.0
   Compiling httpdate v1.0.3
   Compiling ryu v1.0.19
   Compiling cxxbridge-flags v1.0.140
   Compiling utf-8 v0.7.6
   Compiling try-lock v0.2.5
   Compiling serde_json v1.0.138
   Compiling data-encoding v2.8.0
   Compiling rayon-core v1.12.1
   Compiling regex-syntax v0.8.5
   Compiling want v0.3.1
   Compiling cxx v1.0.140
   Compiling crossbeam-deque v0.8.6
   Compiling wide v0.7.32
   Compiling approx v0.5.1
   Compiling num-integer v0.1.46
   Compiling num-complex v0.4.6
   Compiling synstructure v0.13.1
   Compiling regex-automata v0.4.9
   Compiling headers-core v0.2.0
   Compiling http-body v0.4.6
   Compiling futures-channel v0.3.31
   Compiling encoding_rs v0.8.35
   Compiling rawpointer v0.2.1
   Compiling base64 v0.21.7
   Compiling tower-service v0.3.3
   Compiling spin v0.9.8
   Compiling headers v0.3.9
   Compiling num-rational v0.4.2
   Compiling zerofrom-derive v0.1.5
   Compiling yoke-derive v0.7.5
   Compiling zerovec-derive v0.10.3
   Compiling displaydoc v0.2.5
   Compiling icu_provider_macros v1.5.0
   Compiling zerocopy-derive v0.7.35
   Compiling tokio-macros v2.5.0
   Compiling thiserror-impl v1.0.69
   Compiling zerofrom v0.1.5
   Compiling yoke v0.7.5
   Compiling serde_derive v1.0.217
   Compiling zerocopy v0.7.35
   Compiling tokio v1.43.0
   Compiling pin-project-internal v1.1.9
   Compiling zerovec v0.10.4
   Compiling nalgebra-macros v0.1.0
   Compiling ppv-lite86 v0.2.20
   Compiling regex v1.11.1
   Compiling pin-project v1.1.9
   Compiling rand_chacha v0.3.1
   Compiling cxxbridge-macro v1.0.140
   Compiling tinystr v0.7.6
   Compiling icu_collections v1.5.0
   Compiling rand v0.8.5
   Compiling simba v0.6.0
   Compiling icu_locid v1.5.0
   Compiling icu_provider v1.5.0
   Compiling atty v0.2.14
   Compiling termcolor v1.4.1
   Compiling scoped-tls v1.0.1
   Compiling icu_locid_transform v1.5.0
   Compiling humantime v2.1.0
   Compiling foldhash v0.1.4
   Compiling either v1.13.0
   Compiling env_logger v0.9.3
   Compiling rayon v1.10.0
   Compiling icu_properties v1.5.1
   Compiling tokio-util v0.7.13
   Compiling h2 v0.3.26
   Compiling icu_normalizer v1.5.0
   Compiling serde_urlencoded v0.7.1
   Compiling idna_adapter v1.2.0
   Compiling idna v1.0.3
   Compiling url v2.5.4
   Compiling nalgebra v0.29.0
   Compiling tungstenite v0.21.0
   Compiling tokio-tungstenite v0.21.0
   Compiling hyper v0.14.32
   Compiling warp v0.3.7
   Compiling simulation_engine v0.1.0 (/Users/niladridas/Desktop/github/simulation_engine)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 26.98s
```

## Environment Variables
To run the simulation engine, you may need to set the following environment variables:

- `SIMULATION_ENGINE_LOG_LEVEL`: Sets the log level for the simulation engine. Possible values are `trace`, `debug`, `info`, `warn`, and `error`. Default is `info`.
- `SIMULATION_ENGINE_CONFIG_PATH`: Path to the configuration file for the simulation engine. Default is `./config.toml`.

### Example
To set the environment variables and run the simulation engine:
```bash
export SIMULATION_ENGINE_LOG_LEVEL=debug
export SIMULATION_ENGINE_CONFIG_PATH=/path/to/config.toml
cargo run
```

## Checking with Postman
To check the API with Postman, follow these steps:

1. **Create a new request**:
   - Open Postman and create a new request.
   - Set the request type to `GET`.
   - Enter the URL: `http://localhost:3030/api?time_step=0.1&duration=10.0`

2. **Send the request**:
   - Click the `Send` button to send the request.

3. **Check the response**:
   - The response should indicate that the simulation is starting.

### Importing Postman Collection
To simplify the process of testing the API, you can import the provided Postman collection:

1. **Download the Postman collection file**:
   - [Download postman_collection.json](./postman_collection.json)

2. **Import the collection into Postman**:
   - Open Postman and click on the `Import` button.
   - Select the downloaded `postman_collection.json` file.
   - The collection will be imported, and you can use the predefined requests to test the API.

## Contribution Guidelines
- Contributions are welcome! Please follow the coding standards and submit issues or pull requests.

## License
- This project is licensed under the MIT License.

## Acknowledgments
- Special thanks to the Rust community and libraries that made this project possible.

## Repository Information
- Repository URL: https://github.com/bniladridas/simulation_engine
- Directory: simulation_engine
