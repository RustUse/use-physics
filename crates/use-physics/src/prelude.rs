#[cfg(feature = "density")]
pub use crate::{DensityError, density_of, mass_from_density, volume};

#[cfg(feature = "energy")]
pub use crate::{kinetic_energy, potential_energy, work};

#[cfg(feature = "force")]
pub use crate::{STANDARD_GRAVITY, earth_weight, force, impulse, weight};

#[cfg(feature = "gravity")]
pub use crate::{
    GRAVITATIONAL_CONSTANT, GravityBody, circular_orbital_period, circular_orbital_velocity,
    escape_velocity, gravitational_acceleration, gravitational_force,
    gravitational_potential_energy, near_surface_potential_energy, standard_weight,
};

#[cfg(all(feature = "gravity", not(feature = "force")))]
pub use crate::{STANDARD_GRAVITY, weight};

#[cfg(all(feature = "gravity", feature = "force"))]
pub use crate::gravity_weight;

#[cfg(feature = "motion")]
pub use crate::{MotionError, average_speed, displacement, distance, final_velocity};

#[cfg(feature = "power")]
pub use crate::{PowerError, average_power, electrical_power, mechanical_power};

#[cfg(feature = "pressure")]
pub use crate::{PressureError, gauge_pressure, hydrostatic_pressure, pressure};

#[cfg(feature = "thermodynamics")]
pub use crate::{
    IDEAL_GAS_CONSTANT, ThermodynamicsError, celsius_to_kelvin, heat_energy, ideal_gas_pressure,
};
