use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the API routes
    let api = warp::path("api")
        .and(warp::get())
        .and(warp::query::<SimulationParams>())
        .map(|params: SimulationParams| {
            // Logic to start the simulation using params.time_step and params.duration
            format!("Starting simulation with time_step: {} and duration: {}", params.time_step, params.duration)
        });

    // Start the server
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Debug, serde::Deserialize)]
struct SimulationParams {
    time_step: f32,
    duration: f32,
}
