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

#[cfg(feature = "fluid")]
pub use use_fluid as fluid;

#[cfg(all(feature = "fluid", not(feature = "pressure")))]
pub use use_fluid::{
    Fluid, PipeFlow, absolute_pressure, bernoulli_pressure, buoyant_force, continuity_area,
    continuity_velocity, displaced_volume_from_buoyant_force, drag_force, dynamic_pressure,
    dynamic_viscosity, hydrostatic_pressure, kinematic_viscosity, mass_flow_rate, reynolds_number,
    velocity_from_flow_rate, volumetric_flow_rate,
};

#[cfg(all(feature = "fluid", feature = "pressure"))]
pub use use_fluid::{
    Fluid, PipeFlow, absolute_pressure, bernoulli_pressure, buoyant_force, continuity_area,
    continuity_velocity, displaced_volume_from_buoyant_force, drag_force, dynamic_pressure,
    dynamic_viscosity, hydrostatic_pressure as fluid_hydrostatic_pressure, kinematic_viscosity,
    mass_flow_rate, reynolds_number, velocity_from_flow_rate, volumetric_flow_rate,
};

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

#[cfg(feature = "electromagnetism")]
pub use use_electromagnetism as electromagnetism;

#[cfg(feature = "electromagnetism")]
pub use use_electromagnetism::{
    ElectromagneticField, VACUUM_PERMITTIVITY, cyclotron_angular_frequency, cyclotron_frequency,
    cyclotron_radius, electric_field_energy_density, electric_field_for_velocity_selector,
    electric_field_from_magnetic_flux_density_in_vacuum, electric_force_on_charge,
    electromagnetic_energy_density, lorentz_force_magnitude_perpendicular, lorentz_force_scalar,
    lorentz_force_scalar_degrees, magnetic_field_energy_density,
    magnetic_flux_density_for_velocity_selector,
    magnetic_flux_density_from_electric_field_in_vacuum, magnetic_force_on_moving_charge,
    magnetic_force_on_moving_charge_degrees, poynting_magnitude,
    speed_from_permittivity_permeability, velocity_selector_speed,
};

#[cfg(all(feature = "electromagnetism", not(feature = "magnetism")))]
pub use use_electromagnetism::VACUUM_PERMEABILITY;

#[cfg(all(feature = "electromagnetism", feature = "magnetism"))]
pub use use_electromagnetism::VACUUM_PERMEABILITY as ELECTROMAGNETISM_VACUUM_PERMEABILITY;

#[cfg(all(feature = "electromagnetism", not(feature = "nuclear")))]
pub use use_electromagnetism::SPEED_OF_LIGHT;

#[cfg(all(feature = "electromagnetism", feature = "nuclear"))]
pub use use_electromagnetism::SPEED_OF_LIGHT as ELECTROMAGNETISM_SPEED_OF_LIGHT;

#[cfg(all(
    feature = "relativity",
    not(feature = "electromagnetism"),
    not(feature = "nuclear")
))]
pub use use_relativity::SPEED_OF_LIGHT;

#[cfg(all(
    feature = "relativity",
    any(feature = "electromagnetism", feature = "nuclear")
))]
pub use use_relativity::SPEED_OF_LIGHT as RELATIVITY_SPEED_OF_LIGHT;

#[cfg(feature = "force")]
pub use use_force as force;

#[cfg(feature = "force")]
pub use use_force::{STANDARD_GRAVITY, earth_weight, force, impulse, weight};

#[cfg(feature = "torque")]
pub use use_torque as torque;

#[cfg(feature = "torque")]
pub use use_torque::{
    LeverForce, TorqueSystem, angular_acceleration_from_torque, balancing_force,
    balancing_lever_arm, force_from_torque, is_rotational_equilibrium, lever_arm_from_torque,
    moment_arm, moment_arm_degrees, net_torque, net_torque_from_force_lever_pairs,
    perpendicular_force_component, perpendicular_force_component_degrees,
    point_mass_moment_of_inertia, rod_moment_of_inertia_about_center,
    rod_moment_of_inertia_about_end, torque, torque_at_angle, torque_at_angle_degrees,
    torques_from_force_lever_pairs,
};

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

#[cfg(feature = "relativity")]
pub use use_relativity as relativity;

#[cfg(feature = "relativity")]
pub use use_relativity::{
    RelativisticBody, beta, beta_from_rapidity, contracted_length, dilated_time,
    doppler_factor_longitudinal_from_beta, energy_momentum_relation, is_subluminal_speed,
    lorentz_factor, lorentz_factor_from_beta, mass_from_rest_energy,
    observed_frequency_longitudinal, proper_length, proper_time, rapidity_from_beta,
    relativistic_kinetic_energy, relativistic_momentum, rest_energy, rest_mass_from_momentum_speed,
    speed_from_beta, speed_from_rapidity, total_energy, velocity_addition,
};

#[cfg(feature = "motion")]
pub use use_motion as motion;

#[cfg(feature = "motion")]
pub use use_motion::{MotionError, average_speed, displacement, distance, final_velocity};

#[cfg(feature = "rotation")]
pub use use_rotation as rotation;

#[cfg(all(feature = "rotation", not(feature = "torque")))]
pub use use_rotation::{
    AngularState, RotatingBody, angular_acceleration, angular_acceleration_from_torque,
    angular_displacement, angular_momentum, angular_velocity,
    angular_velocity_from_angular_momentum, angular_velocity_from_rotational_kinetic_energy,
    angular_velocity_from_tangential_speed, centripetal_acceleration_from_angular_velocity,
    centripetal_acceleration_from_tangential_speed, degrees_from_radians, final_angular_velocity,
    final_angular_velocity_from_displacement, final_angular_velocity_squared,
    hollow_sphere_moment_of_inertia, point_mass_moment_of_inertia, radians_from_degrees,
    radians_from_revolutions, revolutions_from_radians, rod_moment_of_inertia_about_center,
    rod_moment_of_inertia_about_end, rotational_kinetic_energy, solid_disk_moment_of_inertia,
    solid_sphere_moment_of_inertia, tangential_acceleration, tangential_speed,
    thin_ring_moment_of_inertia,
};

#[cfg(all(feature = "rotation", feature = "torque"))]
pub use use_rotation::{
    AngularState, RotatingBody, angular_acceleration,
    angular_acceleration_from_torque as rotation_angular_acceleration_from_torque,
    angular_displacement, angular_momentum, angular_velocity,
    angular_velocity_from_angular_momentum, angular_velocity_from_rotational_kinetic_energy,
    angular_velocity_from_tangential_speed, centripetal_acceleration_from_angular_velocity,
    centripetal_acceleration_from_tangential_speed, degrees_from_radians, final_angular_velocity,
    final_angular_velocity_from_displacement, final_angular_velocity_squared,
    hollow_sphere_moment_of_inertia,
    point_mass_moment_of_inertia as rotation_point_mass_moment_of_inertia, radians_from_degrees,
    radians_from_revolutions, revolutions_from_radians,
    rod_moment_of_inertia_about_center as rotation_rod_moment_of_inertia_about_center,
    rod_moment_of_inertia_about_end as rotation_rod_moment_of_inertia_about_end,
    rotational_kinetic_energy, solid_disk_moment_of_inertia, solid_sphere_moment_of_inertia,
    tangential_acceleration, tangential_speed, thin_ring_moment_of_inertia,
};

#[cfg(feature = "particle")]
pub use use_particle as particle;

#[cfg(feature = "particle")]
pub use use_particle::{
    ElementaryCharge, Particle, ParticleFamily, ParticleKind, ParticleStatistics, Spin,
    antiparticle, charge, charge_in_elementary_units, charge_thirds, family, has_rest_mass,
    is_antiparticle, is_baryon, is_boson, is_fermion, is_lepton, is_meson, is_quark,
    is_self_antiparticle, rest_mass_mev_c2, spin, statistics,
};

#[cfg(feature = "nuclear")]
pub use use_nuclear as nuclear;

#[cfg(feature = "nuclear")]
pub use use_nuclear::{
    ATOMIC_MASS_UNIT_MEV_C2, DecayLaw, JOULES_PER_MEV, LN_2, NuclideNumbers, SPEED_OF_LIGHT,
    activity, binding_energy_mev_from_mass_defect_u, binding_energy_per_nucleon,
    decay_constant_from_half_life, decayed_fraction_from_decay_constant,
    decayed_quantity_from_decay_constant, elapsed_time_from_remaining_fraction,
    energy_from_mass_defect_kg, half_life_from_decay_constant, is_valid_nuclide_numbers,
    joules_to_mev, mass_defect_kg_from_energy, mean_lifetime, mev_to_joules, neutron_count,
    nuclei_from_activity, nucleon_count, remaining_fraction_from_decay_constant,
    remaining_fraction_from_half_life, remaining_quantity_from_decay_constant,
    remaining_quantity_from_half_life, specific_activity,
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
