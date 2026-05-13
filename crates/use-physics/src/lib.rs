#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Facade for `RustUse` physics helpers.

#[cfg(feature = "density")]
pub use use_density as density;

#[cfg(feature = "density")]
pub use use_density::{DensityError, density as density_of, mass as mass_from_density, volume};

#[cfg(feature = "energy")]
pub use use_energy as energy;

#[cfg(feature = "energy")]
pub use use_energy::{kinetic_energy, potential_energy, work};

#[cfg(feature = "force")]
pub use use_force as force;

#[cfg(feature = "force")]
pub use use_force::{STANDARD_GRAVITY, earth_weight, force, impulse, weight};

#[cfg(feature = "motion")]
pub use use_motion as motion;

#[cfg(feature = "motion")]
pub use use_motion::{MotionError, average_speed, displacement, distance, final_velocity};

#[cfg(feature = "power")]
pub use use_power as power;

#[cfg(feature = "power")]
pub use use_power::{PowerError, average_power, electrical_power, mechanical_power};

#[cfg(feature = "pressure")]
pub use use_pressure as pressure;

#[cfg(feature = "pressure")]
pub use use_pressure::{PressureError, gauge_pressure, hydrostatic_pressure, pressure};

#[cfg(feature = "thermodynamics")]
pub use use_thermodynamics as thermodynamics;

#[cfg(feature = "thermodynamics")]
pub use use_thermodynamics::{
    IDEAL_GAS_CONSTANT, ThermodynamicsError, celsius_to_kelvin, heat_energy, ideal_gas_pressure,
};

pub mod prelude;
