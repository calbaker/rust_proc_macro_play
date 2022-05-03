use pyo3::prelude::*;

extern crate core;

#[pyclass]
struct TrainSimulation(core::TrainSimulation);

#[pymethods]
impl TrainSimulation {
    #[new]
    pub fn __new__() -> Self {
        TrainSimulation(core::TrainSimulation::default())
    }

    pub fn walk(&mut self) {
        self.0.walk();
    }

    #[getter]
    pub fn get_time_trace(&self) -> core::TimeTrace {
        let mut tt = self.0.tt.clone();
        tt.orphaned = true;
        tt
    }

    #[setter]
    pub fn set_time_trace(&mut self, tt: core::TimeTrace) {
        let mut tt = tt;
        tt.orphaned = false;
        self.0.tt = tt;
    }

    #[getter]
    pub fn get_loco_con(&self) -> core::LocomotiveConsist {
        let mut loco_con = self.0.loco_con.clone();
        loco_con.orphaned = true;
        loco_con
    }

    #[setter]
    pub fn set_loco_con(&mut self, loco_con: core::LocomotiveConsist) {
        let mut loco_con = loco_con;
        loco_con.orphaned = false;
        self.0.loco_con = loco_con;
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn core_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::FuelConverter>()?;
    m.add_class::<core::FuelConverterState>()?;
    m.add_class::<core::LocomotiveConsist>()?;
    m.add_class::<core::TimeTrace>()?;
    m.add_class::<TrainSimulation>()?;
    Ok(())
}
