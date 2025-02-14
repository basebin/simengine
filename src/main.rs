use warp::Filter;
use std::sync::Arc;

mod simulation_manager;
mod physics_engine;

use simulation_manager::SimulationManager;

#[tokio::main]
async fn main() {
    let simulation_manager = Arc::new(SimulationManager::new());

    let api = warp::path("api")
        .and(warp::get())
        .and(warp::query::<(Option<f32>, Option<f32>)>())
        .map({
            let simulation_manager = Arc::clone(&simulation_manager);
            move |(time_step, duration): (Option<f32>, Option<f32>)| {
                let time_step = time_step.unwrap_or(0.1);
                let duration = duration.unwrap_or(10.0);
                simulation_manager.start_simulation(time_step, duration);
                "Simulation started!"
            }
        })
        .or(warp::path("stop")
            .and(warp::get())
            .map({
                let simulation_manager = Arc::clone(&simulation_manager);
                move || {
                    simulation_manager.stop_simulation();
                    "Simulation stopped!"
                }
            }))
        .or(warp::path("pause")
            .and(warp::get())
            .map({
                let simulation_manager = Arc::clone(&simulation_manager);
                move || {
                    simulation_manager.pause();
                    "Simulation paused!"
                }
            }))
        .or(warp::path("reset")
            .and(warp::get())
            .map({
                let simulation_manager = Arc::clone(&simulation_manager);
                move || {
                    simulation_manager.reset();
                    "Simulation reset!"
                }
            }))
        .or(warp::path("simulations")
            .and(warp::get())
            .map({
                let simulation_manager = Arc::clone(&simulation_manager);
                move || {
                    let current_simulations = simulation_manager.get_simulations();
                    format!("Current simulations: {:?}", current_simulations)
                }
            }));

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;

    println!("Server running on http://127.0.0.1:3030");
}
