#[cfg(test)]
mod tests {
    // use super::*;
    use warp::Filter;
    use std::sync::Arc;
    use tokio::runtime::Runtime;

    #[test]
    fn test_start_simulation() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
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
                });

            let filter = warp::test::request()
                .method("GET")
                .path("/api?time_step=0.1&duration=10.0")
                .reply(&api)
                .await;

            assert_eq!(filter.status(), 200);
            assert_eq!(filter.body(), "Simulation started!");
        });
    }

    #[test]
    fn test_stop_simulation() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let simulation_manager = Arc::new(SimulationManager::new());

            let api = warp::path("stop")
                .and(warp::get())
                .map({
                    let simulation_manager = Arc::clone(&simulation_manager);
                    move || {
                        simulation_manager.stop_simulation();
                        "Simulation stopped!"
                    }
                });

            let filter = warp::test::request()
                .method("GET")
                .path("/stop")
                .reply(&api)
                .await;

            assert_eq!(filter.status(), 200);
            assert_eq!(filter.body(), "Simulation stopped!");
        });
    }

    #[test]
    fn test_pause_simulation() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let simulation_manager = Arc::new(SimulationManager::new());

            let api = warp::path("pause")
                .and(warp::get())
                .map({
                    let simulation_manager = Arc::clone(&simulation_manager);
                    move || {
                        simulation_manager.pause();
                        "Simulation paused!"
                    }
                });

            let filter = warp::test::request()
                .method("GET")
                .path("/pause")
                .reply(&api)
                .await;

            assert_eq!(filter.status(), 200);
            assert_eq!(filter.body(), "Simulation paused!");
        });
    }

    #[test]
    fn test_reset_simulation() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let simulation_manager = Arc::new(SimulationManager::new());

            let api = warp::path("reset")
                .and(warp::get())
                .map({
                    let simulation_manager = Arc::clone(&simulation_manager);
                    move || {
                        simulation_manager.reset();
                        "Simulation reset!"
                    }
                });

            let filter = warp::test::request()
                .method("GET")
                .path("/reset")
                .reply(&api)
                .await;

            assert_eq!(filter.status(), 200);
            assert_eq!(filter.body(), "Simulation reset!");
        });
    }

    #[test]
    fn test_get_simulations() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let simulation_manager = Arc::new(SimulationManager::new());

            let api = warp::path("simulations")
                .and(warp::get())
                .map({
                    let simulation_manager = Arc::clone(&simulation_manager);
                    move || {
                        let current_simulations = simulation_manager.get_simulations();
                        format!("Current simulations: {:?}", current_simulations)
                    }
                });

            let filter = warp::test::request()
                .method("GET")
                .path("/simulations")
                .reply(&api)
                .await;

            assert_eq!(filter.status(), 200);
            assert_eq!(filter.body(), "Current simulations: []");
        });
    }
}

// Repository URL: https://github.com/bniladridas/simulation_engine
// Directory: tests
