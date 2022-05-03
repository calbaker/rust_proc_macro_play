//! Import uom si system and add unit constants
//! Zero values should be created using standard uom syntax ($Quantity::ZERO) after adding "use uom::ConstZero"
//! Non-zero values should be created using standard uom syntax ($Quantity::new::<$unit>($value)) or multiplication syntax ($value * $UNIT_CONSTANT)

use uom::si::{self, Quantity};

pub use si::acceleration::meter_per_second_squared;
pub use si::area::square_meter;
pub use si::energy::joule;
pub use si::f64::{
    Acceleration, Area, Energy, Force, Length, Mass, MassDensity, Power, PowerRate, Ratio, Time,
    Velocity,
};
pub use si::force::newton;
pub use si::length::meter;
pub use si::mass::kilogram;
pub use si::mass_density::kilogram_per_cubic_meter;
pub use si::power::watt;
pub use si::power_rate::watt_per_second;
pub use si::ratio::ratio;
pub use si::time::second;
pub use si::velocity::meter_per_second;
use uom::lib::marker::PhantomData;

pub const W: Power = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};
pub const S: Time = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};
pub const J: Energy = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};
pub const R: Ratio = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};
pub const M: Length = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};

pub const WPS: PowerRate = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};

pub const MPS: Velocity = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};

pub const MPS2: Acceleration = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};

pub const M2: Area = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};

pub const KG: Mass = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};

pub const KGPM3: MassDensity = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1.0,
};

pub fn accel_grav_mps() -> Acceleration {
    MPS2 * 9.81
} // acceleration due to gravity
  // TODO: make this variable
pub fn rho_air() -> MassDensity {
    KGPM3 * 1.225
} // air density at sea level
