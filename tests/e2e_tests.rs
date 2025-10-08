#[cfg(test)]
mod e2e_tests {
    use std::net::TcpListener;
    use std::sync::Arc;
    use tokio::task;
    use reqwest::Client;
    use simulation_engine::managers::simulation_manager::SimulationManager;

    // Import the build_api function from main.rs
    // Since main.rs is not a lib, we need to duplicate or move to lib.
    // For simplicity, duplicate the API building here.

    use warp::Filter;

    fn build_test_api(simulation_manager: Arc<SimulationManager>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct SimulationParams {
            time_step: Option<f32>,
            duration: Option<f32>,
        }

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

    #[tokio::test]
    async fn test_e2e_start_simulation() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener); // Free the port

        let simulation_manager = Arc::new(SimulationManager::new());
        let api = build_test_api(Arc::clone(&simulation_manager));

        let server_future = warp::serve(api).run(([127, 0, 0, 1], port));

        let server_handle = task::spawn(server_future);

        // Give server time to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let client = Client::new();
        let url = format!("http://127.0.0.1:{}/api?time_step=0.1&duration=10.0", port);
        let response = client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        assert_eq!(body, "Simulation started!");

        // Stop the server
        server_handle.abort();
    }

    #[tokio::test]
    async fn test_e2e_stop_simulation() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let simulation_manager = Arc::new(SimulationManager::new());
        let api = build_test_api(Arc::clone(&simulation_manager));

        let server_future = warp::serve(api).run(([127, 0, 0, 1], port));
        let server_handle = task::spawn(server_future);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let client = Client::new();
        let url = format!("http://127.0.0.1:{}/stop", port);
        let response = client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        assert_eq!(body, "Simulation stopped!");

        server_handle.abort();
    }

    #[tokio::test]
    async fn test_e2e_pause_simulation() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let simulation_manager = Arc::new(SimulationManager::new());
        let api = build_test_api(Arc::clone(&simulation_manager));

        let server_future = warp::serve(api).run(([127, 0, 0, 1], port));
        let server_handle = task::spawn(server_future);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let client = Client::new();
        let url = format!("http://127.0.0.1:{}/pause", port);
        let response = client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        assert_eq!(body, "Simulation paused!");

        server_handle.abort();
    }

    #[tokio::test]
    async fn test_e2e_reset_simulation() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let simulation_manager = Arc::new(SimulationManager::new());
        let api = build_test_api(Arc::clone(&simulation_manager));

        let server_future = warp::serve(api).run(([127, 0, 0, 1], port));
        let server_handle = task::spawn(server_future);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let client = Client::new();
        let url = format!("http://127.0.0.1:{}/reset", port);
        let response = client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        assert_eq!(body, "Simulation reset!");

        server_handle.abort();
    }

    #[tokio::test]
    async fn test_e2e_get_simulations() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let simulation_manager = Arc::new(SimulationManager::new());
        let api = build_test_api(Arc::clone(&simulation_manager));

        let server_future = warp::serve(api).run(([127, 0, 0, 1], port));
        let server_handle = task::spawn(server_future);

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let client = Client::new();
        let url = format!("http://127.0.0.1:{}/simulations", port);
        let response = client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 200);
        let body = response.text().await.unwrap();
        assert!(body.starts_with("Current simulations:"));

        server_handle.abort();
    }
}