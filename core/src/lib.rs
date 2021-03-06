extern crate uom;
use uom::typenum;
extern crate pyo3;
use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
extern crate proc_macros;
use proc_macros::impl_pyo3_get;

mod si;

#[pyclass]
#[derive(Clone, Debug)]
#[impl_pyo3_get(
    // optional struct-specific methods
    pub fn orphaned_as_str(&self) -> String {format!("{:?}", self.orphaned)}
    pub fn step_py(&mut self) {self.step(66666.0 * si::W, 66.0 * si::S)}
)]
pub struct FuelConverter {
    #[pyo3(get)]
    pub state: FuelConverterState,
    #[pyo3(get)]
    pub history: Vec<FuelConverterState>,
    pub pwr_max: si::Power,
    pub eta: si::Ratio,
    #[pyo3(get)]
    pub orphaned: bool,
}

impl Default for FuelConverter {
    fn default() -> Self {
        Self {
            state: FuelConverterState::default(),
            history: Vec::new(),
            pwr_max: si::W * 100.0,
            eta: si::R * 0.8,
            orphaned: false,
        }
    }
}

impl FuelConverter {
    pub fn step(&mut self, pwr: si::Power, dt: si::Time) {
        self.state.pwr = pwr;
        self.state.energy += self.state.pwr * dt;
        self.history.push(self.state.clone());
        self.state.i += 1;
    }
}

#[derive(Clone, Debug)]
#[impl_pyo3_get]
#[pyclass]
pub struct FuelConverterState {
    #[pyo3(get)]
    pub i: usize,
    pub pwr: si::Power,
    pub energy: si::Energy,
}

impl Default for FuelConverterState {
    fn default() -> Self {
        Self {
            i: 1,
            pwr: si::W * 0.0,
            energy: si::J * 0.0,
        }
    }
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct LocomotiveConsist {
    pub fc: FuelConverter,
    pub orphaned: bool,
}

#[pymethods]
impl LocomotiveConsist {
    #[getter]
    pub fn get_fc(&self) -> FuelConverter {
        let mut fc = self.fc.clone();
        fc.orphaned = true;
        fc
    }
    #[setter]
    pub fn set_fc(&self, fc: FuelConverter) -> PyResult<()> {
        let mut fc = self.fc.clone();
        fc.orphaned = false;
        Ok(())
    }
}

impl LocomotiveConsist {
    pub fn step(&mut self, pwr: si::Power, dt: si::Time) {
        self.fc.step(pwr, dt);
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct TimeTrace {
    pub time: Vec<si::Time>,
    pub speed: Vec<si::Velocity>,
    pub orphaned: bool,
}

impl Default for TimeTrace {
    fn default() -> Self {
        let speed: Vec<si::Velocity> = (0..100).map(|x| si::MPS * x as f64).collect();
        Self {
            speed: speed.clone(),
            time: (0..speed.len()).map(|x| si::S * x as f64).collect(),
            orphaned: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[pyclass]
pub struct TrainSimulation {
    #[pyo3(get)]
    pub tt: TimeTrace,
    #[pyo3(get)]
    pub loco_con: LocomotiveConsist,
}

#[pymethods]
impl TrainSimulation {
    pub fn step(&mut self) {
        let dt = self.tt.time[self.loco_con.fc.state.i];
        let pwr: si::Power =
            si::KG * self.tt.speed[self.loco_con.fc.state.i].powi(typenum::P2::new()) / dt;
        self.loco_con.step(pwr, dt);
    }

    pub fn walk(&mut self) {
        for _ in 0..self.tt.time.len() - 1 {
            self.step();
        }
    }

    #[new]
    pub fn __new__() -> Self {
        Self::default()
    }
}

#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FuelConverter>()?;
    m.add_class::<FuelConverterState>()?;
    m.add_class::<LocomotiveConsist>()?;
    m.add_class::<TimeTrace>()?;
    m.add_class::<TrainSimulation>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_train_simulation() {
        let mut ts = super::TrainSimulation::default();
        ts.walk();
        assert_eq!(ts.loco_con.fc.state.energy.get::<si::joule>(), 328350.0);
    }

    #[test]
    pub fn test_get_pwr_max() {
        let ts = super::TrainSimulation::default();
        // this method is created by the macro
        assert_eq!(ts.loco_con.fc.get_pwr_max_watts(), 100.0);
    }

    #[test]
    pub fn test_orhpaned_as_str() {
        let fc = FuelConverter::default();
        assert_eq!(fc.orphaned_as_str(), false.to_string());
    }
}
