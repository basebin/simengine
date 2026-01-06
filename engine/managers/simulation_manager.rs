use crate::physics::physics_engine::PhysicsEngine;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MAX_SIMULATIONS: usize = 5;

#[derive(Clone, Debug, PartialEq)]
pub enum SimulationState {
    Running,
    Paused,
    Stopped,
}

#[derive(Clone, Debug)]
pub struct Simulation {
    id: u32,
    state: SimulationState,
    time_step: f32,
    duration: f32,
    physics_engine: PhysicsEngine,
}

impl Simulation {
    pub fn new(id: u32, time_step: f32, duration: f32) -> Self {
        Simulation {
            id,
            state: SimulationState::Stopped,
            time_step,
            duration,
            physics_engine: PhysicsEngine::new().expect("Failed to create physics engine"),
        }
    }

    pub fn start(&mut self) {
        self.state = SimulationState::Running;
        println!("Simulation {} started", self.id);
        while self.state == SimulationState::Running {
            self.update();
            self.physics_engine
                .simulate(self.time_step)
                .expect("Simulation step failed");
            thread::sleep(Duration::from_millis((self.time_step * 1000.0) as u64));
        }
    }

    pub fn pause(&mut self) {
        self.state = SimulationState::Paused;
    }

    pub fn reset(&mut self) {
        self.state = SimulationState::Stopped;
    }

    pub fn update(&mut self) {
        if self.state == SimulationState::Running {
            self.duration -= self.time_step;
            if self.duration <= 0.0 {
                self.state = SimulationState::Stopped;
            }
        }
    }
}

#[derive(Clone)]
pub struct SimulationManager {
    simulations: Arc<Mutex<Vec<Arc<Mutex<Simulation>>>>>,
}

impl Default for SimulationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationManager {
    pub fn new() -> Self {
        SimulationManager {
            simulations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Starts a new simulation with the given time step and duration.
    /// This method spawns a new thread to run the simulation.
    pub fn start_simulation(&self, time_step: f32, duration: f32) {
        if time_step <= 0.0 || duration <= 0.0 {
            return;
        }
        let mut simulations = self.simulations.lock().unwrap();
        if simulations.len() >= MAX_SIMULATIONS {
            return;
        }
        let new_simulation = Arc::new(Mutex::new(Simulation::new(
            simulations.len() as u32 + 1,
            time_step,
            duration,
        )));
        let _simulation_thread = thread::spawn({
            let new_simulation = Arc::clone(&new_simulation);
            move || {
                let mut simulation = new_simulation.lock().unwrap();
                simulation.start();
            }
        });
        simulations.push(Arc::clone(&new_simulation));
    }

    pub fn stop_simulation(&self) {
        let mut simulations = self.simulations.lock().unwrap();
        for sim_arc in simulations.iter() {
            sim_arc.lock().unwrap().state = SimulationState::Stopped;
        }
        simulations.clear();
    }

    pub fn get_simulations(&self) -> Vec<Simulation> {
        let simulations = self.simulations.lock().unwrap();
        simulations
            .iter()
            .map(|arc| arc.lock().unwrap().clone())
            .collect()
    }

    pub fn pause(&self) {
        let mut simulations = self.simulations.lock().unwrap();
        for simulation in simulations.iter_mut() {
            simulation.lock().unwrap().pause();
        }
    }

    pub fn reset(&self) {
        let mut simulations = self.simulations.lock().unwrap();
        for simulation in simulations.iter_mut() {
            simulation.lock().unwrap().reset();
        }
    }
}
