extern crate log;
extern crate env_logger;

use warp::Filter;
use std::sync::Arc;
use serde::Deserialize;

mod physics;
mod managers;

use managers::simulation_manager::SimulationManager;

#[derive(Deserialize)]
pub struct SimulationParams {
    pub time_step: Option<f32>,
    pub duration: Option<f32>,
}

fn build_api(simulation_manager: Arc<SimulationManager>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(warp::get())
        .and(warp::query::<SimulationParams>())
        .map({
            let simulation_manager = Arc::clone(&simulation_manager);
            move |params: SimulationParams| {
                let time_step = params.time_step.unwrap_or(0.1);
                let duration = params.duration.unwrap_or(10.0);
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
            }))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let simulation_manager = Arc::new(SimulationManager::new());

    let api = build_api(Arc::clone(&simulation_manager));

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}
