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

#[cfg(feature = "gravity")]
pub use use_gravity as gravity;

#[cfg(feature = "gravity")]
pub use use_gravity::{
    GRAVITATIONAL_CONSTANT, GravityBody, circular_orbital_period, circular_orbital_velocity,
    escape_velocity, gravitational_acceleration, gravitational_force,
    gravitational_potential_energy, near_surface_potential_energy, standard_weight,
};

#[cfg(all(feature = "gravity", not(feature = "force")))]
pub use use_gravity::{STANDARD_GRAVITY, weight};

#[cfg(all(feature = "gravity", feature = "force"))]
pub use use_gravity::weight as gravity_weight;

#[cfg(feature = "momentum")]
pub use use_momentum as momentum;

#[cfg(all(feature = "momentum", not(feature = "force")))]
pub use use_momentum::{
    MovingMass, average_force_from_impulse, elastic_collision_velocities,
    elastic_collision_velocity_a, elastic_collision_velocity_b,
    final_velocity_after_sticking_collision, impulse, impulse_from_momentum_change,
    mass_from_momentum, momentum, recoil_velocity, total_momentum, two_body_total_momentum,
    velocity_from_momentum,
};

#[cfg(all(feature = "momentum", feature = "force"))]
pub use use_momentum::{
    MovingMass, average_force_from_impulse, elastic_collision_velocities,
    elastic_collision_velocity_a, elastic_collision_velocity_b,
    final_velocity_after_sticking_collision, impulse as momentum_impulse,
    impulse_from_momentum_change, mass_from_momentum, momentum, recoil_velocity, total_momentum,
    two_body_total_momentum, velocity_from_momentum,
};

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
