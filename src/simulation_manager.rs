use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::physics_engine::PhysicsEngine;

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
            physics_engine: PhysicsEngine::new(),
        }
    }

    pub fn start(&mut self) {
        self.state = SimulationState::Running;
        println!("Simulation {} started", self.id);
        while self.state == SimulationState::Running {
            self.update();
            self.physics_engine.simulate();
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
    simulations: Arc<Mutex<Vec<Simulation>>>,
}

impl SimulationManager {
    pub fn new() -> Self {
        SimulationManager {
            simulations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start_simulation(&self, time_step: f32, duration: f32) {
        let mut simulations = self.simulations.lock().unwrap();
        let mut new_simulation = Simulation::new(simulations.len() as u32 + 1, time_step, duration); 
        let simulation_thread = thread::spawn(move || {
            new_simulation.start();
        });
        simulations.push(new_simulation);
        simulation_thread.join().unwrap();
    }

    pub fn stop_simulation(&self) {
        let mut simulations = self.simulations.lock().unwrap();
        if !simulations.is_empty() {
            simulations.pop();
        }
    }

    pub fn get_simulations(&self) -> Vec<Simulation> {
        let simulations = self.simulations.lock().unwrap();
        simulations.clone()
    }

    pub fn pause(&self) {
        let mut simulations = self.simulations.lock().unwrap();
        for simulation in simulations.iter_mut() {
            simulation.pause();
        }
    }

    pub fn reset(&self) {
        let mut simulations = self.simulations.lock().unwrap();
        for simulation in simulations.iter_mut() {
            simulation.reset();
        }
    }
}
