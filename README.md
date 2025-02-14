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
