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

#[cfg(feature = "work")]
pub mod work {
    pub use use_work::*;
}

#[cfg(feature = "work")]
pub use use_work::{
    ConstantForceWork, displacement_from_work, final_kinetic_energy_from_work, force_from_work,
    initial_kinetic_energy_from_work, net_work, spring_potential_energy, spring_work,
    work_against_gravity, work_at_angle, work_at_angle_degrees, work_by_friction, work_by_gravity,
    work_from_force_samples, work_from_kinetic_energy_change,
};

#[cfg(feature = "electricity")]
pub use use_electricity as electricity;

#[cfg(feature = "electricity")]
pub use use_electricity::{
    COULOMB_CONSTANT, ElectricalLoad, charge_from_current_time, conductance, coulomb_force,
    current, current_from_charge_time, energy_from_power_time, energy_from_voltage_charge,
    parallel_resistance, power_from_current_resistance, power_from_voltage_current,
    power_from_voltage_resistance, resistance, resistance_from_conductance, series_resistance,
    voltage,
};

#[cfg(feature = "magnetism")]
pub use use_magnetism as magnetism;

#[cfg(feature = "magnetism")]
pub use use_magnetism::{
    MagneticField, VACUUM_PERMEABILITY, magnetic_energy_density,
    magnetic_field_around_long_straight_wire, magnetic_field_at_center_of_loop,
    magnetic_field_inside_solenoid, magnetic_flux, magnetic_flux_degrees,
    magnetic_flux_density_from_flux, magnetic_force_magnitude_on_charge, magnetic_force_on_charge,
    magnetic_force_on_charge_degrees, magnetic_force_on_wire, magnetic_force_on_wire_degrees,
    magnetic_pressure,
};

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

#[cfg(feature = "particle")]
pub use use_particle as particle;

#[cfg(feature = "particle")]
pub use use_particle::{
    ElementaryCharge, Particle, ParticleFamily, ParticleKind, ParticleStatistics, Spin,
    antiparticle, charge, charge_in_elementary_units, charge_thirds, family, has_rest_mass,
    is_antiparticle, is_baryon, is_boson, is_fermion, is_lepton, is_meson, is_quark,
    is_self_antiparticle, rest_mass_mev_c2, spin, statistics,
};

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
