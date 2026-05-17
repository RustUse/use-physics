#[cfg(feature = "density")]
pub use crate::{DensityError, density_of, mass_from_density, volume};

#[cfg(feature = "energy")]
pub use crate::{kinetic_energy, potential_energy, work};

#[cfg(all(feature = "fluid", not(feature = "pressure")))]
pub use crate::{
    Fluid, PipeFlow, absolute_pressure, bernoulli_pressure, buoyant_force, continuity_area,
    continuity_velocity, displaced_volume_from_buoyant_force, drag_force, dynamic_pressure,
    dynamic_viscosity, hydrostatic_pressure, kinematic_viscosity, mass_flow_rate, reynolds_number,
    velocity_from_flow_rate, volumetric_flow_rate,
};

#[cfg(all(feature = "fluid", feature = "pressure"))]
pub use crate::{
    Fluid, PipeFlow, absolute_pressure, bernoulli_pressure, buoyant_force, continuity_area,
    continuity_velocity, displaced_volume_from_buoyant_force, drag_force, dynamic_pressure,
    dynamic_viscosity, fluid_hydrostatic_pressure, kinematic_viscosity, mass_flow_rate,
    reynolds_number, velocity_from_flow_rate, volumetric_flow_rate,
};

#[cfg(feature = "work")]
pub use crate::{
    ConstantForceWork, displacement_from_work, final_kinetic_energy_from_work, force_from_work,
    initial_kinetic_energy_from_work, net_work, spring_potential_energy, spring_work,
    work_against_gravity, work_at_angle, work_at_angle_degrees, work_by_friction, work_by_gravity,
    work_from_force_samples, work_from_kinetic_energy_change,
};

#[cfg(feature = "electricity")]
pub use crate::{
    COULOMB_CONSTANT, ElectricalLoad, charge_from_current_time, conductance, coulomb_force,
    current, current_from_charge_time, energy_from_power_time, energy_from_voltage_charge,
    parallel_resistance, power_from_current_resistance, power_from_voltage_current,
    power_from_voltage_resistance, resistance, resistance_from_conductance, series_resistance,
    voltage,
};

#[cfg(feature = "magnetism")]
pub use crate::{
    MagneticField, VACUUM_PERMEABILITY, magnetic_energy_density,
    magnetic_field_around_long_straight_wire, magnetic_field_at_center_of_loop,
    magnetic_field_inside_solenoid, magnetic_flux, magnetic_flux_degrees,
    magnetic_flux_density_from_flux, magnetic_force_magnitude_on_charge, magnetic_force_on_charge,
    magnetic_force_on_charge_degrees, magnetic_force_on_wire, magnetic_force_on_wire_degrees,
    magnetic_pressure,
};

#[cfg(feature = "force")]
pub use crate::{STANDARD_GRAVITY, earth_weight, force, impulse, weight};

#[cfg(feature = "torque")]
pub use crate::{
    LeverForce, TorqueSystem, angular_acceleration_from_torque, balancing_force,
    balancing_lever_arm, force_from_torque, is_rotational_equilibrium, lever_arm_from_torque,
    moment_arm, moment_arm_degrees, net_torque, net_torque_from_force_lever_pairs,
    perpendicular_force_component, perpendicular_force_component_degrees,
    point_mass_moment_of_inertia, rod_moment_of_inertia_about_center,
    rod_moment_of_inertia_about_end, torque, torque_at_angle, torque_at_angle_degrees,
    torques_from_force_lever_pairs,
};

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

#[cfg(all(feature = "momentum", not(feature = "force")))]
pub use crate::{
    MovingMass, average_force_from_impulse, elastic_collision_velocities,
    elastic_collision_velocity_a, elastic_collision_velocity_b,
    final_velocity_after_sticking_collision, impulse, impulse_from_momentum_change,
    mass_from_momentum, momentum, recoil_velocity, total_momentum, two_body_total_momentum,
    velocity_from_momentum,
};

#[cfg(all(feature = "momentum", feature = "force"))]
pub use crate::{
    MovingMass, average_force_from_impulse, elastic_collision_velocities,
    elastic_collision_velocity_a, elastic_collision_velocity_b,
    final_velocity_after_sticking_collision, impulse_from_momentum_change, mass_from_momentum,
    momentum, momentum_impulse, recoil_velocity, total_momentum, two_body_total_momentum,
    velocity_from_momentum,
};

#[cfg(feature = "motion")]
pub use crate::{MotionError, average_speed, displacement, distance, final_velocity};

#[cfg(feature = "particle")]
pub use crate::{
    ElementaryCharge, Particle, ParticleFamily, ParticleKind, ParticleStatistics, Spin,
    antiparticle, charge, charge_in_elementary_units, charge_thirds, family, has_rest_mass,
    is_antiparticle, is_baryon, is_boson, is_fermion, is_lepton, is_meson, is_quark,
    is_self_antiparticle, rest_mass_mev_c2, spin, statistics,
};

#[cfg(feature = "power")]
pub use crate::{PowerError, average_power, electrical_power, mechanical_power};

#[cfg(feature = "pressure")]
pub use crate::{PressureError, gauge_pressure, hydrostatic_pressure, pressure};

#[cfg(feature = "thermodynamics")]
pub use crate::{
    IDEAL_GAS_CONSTANT, ThermodynamicsError, celsius_to_kelvin, heat_energy, ideal_gas_pressure,
};
