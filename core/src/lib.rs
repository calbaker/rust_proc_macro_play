extern crate si_api;
use si_api as si;
extern crate uom;
use uom::typenum;

struct FuelConverter {
    state: FuelConverterState,
    history: Vec<FuelConverterState>,
    pwr_max: si::Power,
    eta: si::Ratio,
}

impl Default for FuelConverter {
    fn default() -> Self {
        Self {
            state: FuelConverterState::default(),
            history: Vec::new(),
            pwr_max: si::W * 100.0,
            eta: si::R * 0.8,
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
struct FuelConverterState {
    i: usize,
    pwr: si::Power,
    energy: si::Energy,
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

struct LocomotiveConsist {
    fc: FuelConverter,
}

impl Default for LocomotiveConsist {
    fn default() -> Self {
        Self {
            fc: FuelConverter::default(),
        }
    }
}

impl LocomotiveConsist {
    pub fn step(&mut self, pwr: si::Power, dt: si::Time) {
        self.fc.step(pwr, dt);
    }
}

struct TimeTrace {
    time: Vec<si::Time>,
    speed: Vec<si::Velocity>,
}

impl Default for TimeTrace {
    fn default() -> Self {
        let speed: Vec<si::Velocity> = (0..100).map(|x| si::MPS * x as f64).collect();
        Self {
            speed: speed.clone(),
            time: (0..speed.len()).map(|x| si::S * x as f64).collect(),
        }
    }
}

struct TrainSimulation {
    tt: TimeTrace,
    loco_con: LocomotiveConsist,
}

impl Default for TrainSimulation {
    fn default() -> Self {
        Self {
            tt: TimeTrace::default(),
            loco_con: LocomotiveConsist::default(),
        }
    }
}

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
}
