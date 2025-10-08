#[cfg(feature = "python")]
use nalgebra::Vector2;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::PyResult;

#[cfg(feature = "python")]
#[pymodule]
fn simulation_engine_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySimulationManager>()?;
    m.add_class::<PyPhysicsEngine>()?;
    m.add_class::<PyObject>()?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyclass]
struct PySimulationManager {
    manager: crate::managers::simulation_manager::SimulationManager,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySimulationManager {
    #[new]
    fn new() -> Self {
        PySimulationManager {
            manager: crate::managers::simulation_manager::SimulationManager::new(),
        }
    }

    fn start_simulation(&self, time_step: f32, duration: f32) {
        self.manager.start_simulation(time_step, duration);
    }

    fn stop_simulation(&self) {
        self.manager.stop_simulation();
    }

    fn pause(&self) {
        self.manager.pause();
    }

    fn reset(&self) {
        self.manager.reset();
    }
}

#[cfg(feature = "python")]
#[pyclass]
struct PyPhysicsEngine {
    engine: crate::physics::physics_engine::PhysicsEngine,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyPhysicsEngine {
    #[new]
    fn new() -> Self {
        PyPhysicsEngine {
            engine: crate::physics::physics_engine::PhysicsEngine::new(),
        }
    }

    fn add_object(&mut self, x: f32, y: f32, vx: f32, vy: f32, mass: f32) {
        self.engine
            .add_object(Vector2::new(x, y), Vector2::new(vx, vy), mass);
    }

    fn set_gravity(&mut self, x: f32, y: f32) {
        self.engine.set_gravity(Vector2::new(x, y));
    }

    fn simulate(&mut self, time_step: f32) {
        self.engine.simulate(time_step);
    }

    fn get_objects(&self) -> Vec<PyObject> {
        self.engine
            .get_objects()
            .iter()
            .map(|obj| PyObject {
                position: (obj.position.x, obj.position.y),
                velocity: (obj.velocity.x, obj.velocity.y),
                mass: obj.mass,
                radius: obj.radius,
            })
            .collect()
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
struct PyObject {
    #[pyo3(get)]
    position: (f32, f32),
    #[pyo3(get)]
    velocity: (f32, f32),
    #[pyo3(get)]
    mass: f32,
    #[pyo3(get)]
    radius: f32,
}
