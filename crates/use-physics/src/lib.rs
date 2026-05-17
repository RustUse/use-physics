#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Facade for `RustUse` physics helpers.

#[cfg(feature = "density")]
pub use use_density as density;

#[cfg(feature = "elasticity")]
pub use use_elasticity as elasticity;

#[cfg(feature = "density")]
pub use use_density::{DensityError, density as density_of, mass as mass_from_density, volume};

#[cfg(feature = "elasticity")]
pub use use_elasticity::{
    ElasticBar, ElasticMaterial, axial_deformation, axial_stiffness, bulk_modulus,
    bulk_modulus_from_youngs_and_poisson, change_in_length, change_in_volume,
    elastic_energy_density, elastic_energy_from_force_deformation,
    elastic_energy_from_spring_constant, final_length, force_from_axial_deformation,
    force_from_stress, is_common_poisson_ratio, normal_strain, normal_stress, poisson_ratio,
    pressure_change_from_bulk_modulus, shear_modulus, shear_modulus_from_youngs_and_poisson,
    shear_strain, shear_strain_from_modulus, shear_stress, shear_stress_from_modulus,
    strain_from_youngs_modulus, stress_from_youngs_modulus, transverse_strain_from_poisson_ratio,
    volume_strain, youngs_modulus, youngs_modulus_from_shear_and_poisson,
};

#[cfg(feature = "energy")]
pub use use_energy as energy;

#[cfg(feature = "energy")]
pub use use_energy::{kinetic_energy, potential_energy, work};

#[cfg(feature = "collision")]
pub use use_collision as collision;

#[cfg(all(feature = "collision", not(feature = "energy")))]
pub use use_collision::{
    Collision1D, CollisionBody1D, coefficient_of_restitution, collision_energy_loss_1d,
    collision_energy_loss_fraction_1d, collision_final_velocities_1d, collision_impulse_on_a,
    collision_impulse_on_b, collision_impulses_1d, elastic_collision_final_velocities_1d,
    is_perfectly_elastic, is_perfectly_inelastic, is_valid_restitution, kinetic_energy,
    kinetic_energy_loss, kinetic_energy_loss_fraction,
    perfectly_inelastic_collision_final_velocities_1d, perfectly_inelastic_collision_velocity_1d,
    relative_speed, relative_velocity, separation_speed_from_restitution, total_kinetic_energy_1d,
};

#[cfg(all(feature = "collision", feature = "energy"))]
pub use use_collision::{
    Collision1D, CollisionBody1D, coefficient_of_restitution, collision_energy_loss_1d,
    collision_energy_loss_fraction_1d, collision_final_velocities_1d, collision_impulse_on_a,
    collision_impulse_on_b, collision_impulses_1d, elastic_collision_final_velocities_1d,
    is_perfectly_elastic, is_perfectly_inelastic, is_valid_restitution,
    kinetic_energy as collision_kinetic_energy, kinetic_energy_loss, kinetic_energy_loss_fraction,
    perfectly_inelastic_collision_final_velocities_1d, perfectly_inelastic_collision_velocity_1d,
    relative_speed, relative_velocity, separation_speed_from_restitution, total_kinetic_energy_1d,
};

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

#[cfg(feature = "quantum")]
pub use use_quantum as quantum;

#[cfg(feature = "quantum")]
pub use use_quantum::{
    BOHR_RADIUS, ELECTRON_MASS, ELEMENTARY_CHARGE, MatterWave, PLANCK_CONSTANT, Photon,
    QuantumNumbers, REDUCED_PLANCK_CONSTANT, RYDBERG_ENERGY_EV, angular_frequency_from_energy,
    bohr_orbit_radius, de_broglie_wavelength, de_broglie_wavelength_from_mass_velocity,
    electron_volts_to_joules, energy_from_angular_frequency, frequency_from_photon_energy,
    hydrogen_energy_level_ev, hydrogen_transition_energy_ev, hydrogen_transition_wavelength,
    is_valid_azimuthal_quantum_number, is_valid_magnetic_quantum_number,
    is_valid_principal_quantum_number, is_valid_quantum_numbers, is_valid_spin_twice,
    joules_to_electron_volts, minimum_energy_uncertainty, minimum_momentum_uncertainty,
    minimum_position_uncertainty, minimum_time_uncertainty, momentum_from_de_broglie_wavelength,
    photon_energy_from_frequency, photon_energy_from_wavelength, photon_momentum_from_energy,
    photon_momentum_from_wavelength, wavelength_from_photon_energy,
};

#[cfg(all(
    feature = "quantum",
    not(feature = "electromagnetism"),
    not(feature = "nuclear"),
    not(feature = "relativity")
))]
pub use use_quantum::SPEED_OF_LIGHT;

#[cfg(all(
    feature = "quantum",
    any(
        feature = "electromagnetism",
        feature = "nuclear",
        feature = "relativity"
    )
))]
pub use use_quantum::SPEED_OF_LIGHT as QUANTUM_SPEED_OF_LIGHT;

#[cfg(feature = "plasma")]
pub use use_plasma as plasma;

#[cfg(feature = "plasma")]
pub use use_plasma::{
    BOLTZMANN_CONSTANT, ElectronPlasma, PROTON_MASS, PlasmaSpecies, alfven_speed, charge_density,
    debye_length, debye_number, debye_sphere_volume, electron_gyrofrequency, electron_gyroradius,
    electron_plasma_angular_frequency, electron_plasma_frequency, electron_thermal_speed,
    gyro_angular_frequency, gyrofrequency, gyroradius, ion_plasma_angular_frequency,
    is_quasi_neutral, is_valid_coulomb_logarithm, particle_thermal_speed, plasma_beta,
    plasma_pressure, total_plasma_pressure,
};

#[cfg(all(feature = "plasma", not(feature = "quantum")))]
pub use use_plasma::{ELECTRON_MASS, ELEMENTARY_CHARGE};

#[cfg(all(feature = "plasma", feature = "quantum"))]
pub use use_plasma::ELECTRON_MASS as PLASMA_ELECTRON_MASS;

#[cfg(all(feature = "plasma", feature = "quantum"))]
pub use use_plasma::ELEMENTARY_CHARGE as PLASMA_ELEMENTARY_CHARGE;

#[cfg(all(feature = "plasma", not(feature = "electromagnetism")))]
pub use use_plasma::VACUUM_PERMITTIVITY;

#[cfg(all(feature = "plasma", feature = "electromagnetism"))]
pub use use_plasma::VACUUM_PERMITTIVITY as PLASMA_VACUUM_PERMITTIVITY;

#[cfg(all(
    feature = "plasma",
    not(feature = "magnetism"),
    not(feature = "electromagnetism")
))]
pub use use_plasma::VACUUM_PERMEABILITY;

#[cfg(all(
    feature = "plasma",
    any(feature = "magnetism", feature = "electromagnetism")
))]
pub use use_plasma::VACUUM_PERMEABILITY as PLASMA_VACUUM_PERMEABILITY;

#[cfg(all(feature = "plasma", not(feature = "magnetism")))]
pub use use_plasma::magnetic_pressure;

#[cfg(all(feature = "plasma", feature = "magnetism"))]
pub use use_plasma::magnetic_pressure as plasma_magnetic_pressure;

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

#[cfg(feature = "statics")]
pub use use_statics as statics;

#[cfg(all(feature = "statics", not(feature = "torque")))]
pub use use_statics::{
    CantileverReaction, Force2D, PointForce2D, StaticSystem2D, can_static_friction_hold,
    cantilever_end_point_load_reaction, downslope_force_incline, force_angle_radians,
    force_magnitude, is_rotational_equilibrium, is_static_equilibrium_2d,
    is_translational_equilibrium_1d, is_translational_equilibrium_2d, maximum_static_friction,
    minimum_static_friction_coefficient_for_incline, moment_2d, moment_from_force_and_arm,
    net_force_1d, net_force_2d, net_moment, net_moment_2d, normal_force_horizontal_surface,
    normal_force_incline, required_static_friction, simply_supported_point_load_reactions,
    simply_supported_uniform_load_reactions,
};

#[cfg(all(feature = "statics", feature = "torque"))]
pub use use_statics::{
    CantileverReaction, Force2D, PointForce2D, StaticSystem2D, can_static_friction_hold,
    cantilever_end_point_load_reaction, downslope_force_incline, force_angle_radians,
    force_magnitude, is_rotational_equilibrium as statics_is_rotational_equilibrium,
    is_static_equilibrium_2d, is_translational_equilibrium_1d, is_translational_equilibrium_2d,
    maximum_static_friction, minimum_static_friction_coefficient_for_incline, moment_2d,
    moment_from_force_and_arm, net_force_1d, net_force_2d, net_moment, net_moment_2d,
    normal_force_horizontal_surface, normal_force_incline, required_static_friction,
    simply_supported_point_load_reactions, simply_supported_uniform_load_reactions,
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

#[cfg(feature = "orbit")]
pub use use_orbit as orbit;

#[cfg(feature = "orbit")]
pub use use_orbit::{
    CentralBody, EllipticalOrbit, altitude_from_orbital_radius,
    apoapsis_from_semi_major_axis_eccentricity, apoapsis_speed, circular_orbital_speed,
    eccentricity_from_apsides, elliptical_orbital_period, escape_speed, gravitational_parameter,
    hohmann_delta_v_1, hohmann_delta_v_2, hohmann_total_delta_v, hohmann_transfer_semi_major_axis,
    hohmann_transfer_time, orbital_radius_from_altitude, orbital_radius_from_circular_speed,
    orbital_radius_from_period, periapsis_from_semi_major_axis_eccentricity, periapsis_speed,
    semi_major_axis_from_apsides, semi_major_axis_from_specific_energy,
    source_mass_from_gravitational_parameter, specific_orbital_energy, vis_viva_speed,
};

#[cfg(feature = "momentum")]
pub use use_momentum as momentum;

#[cfg(all(feature = "momentum", not(feature = "force")))]
pub use use_momentum::{
    MovingMass, average_force_from_impulse, impulse, impulse_from_momentum_change,
    mass_from_momentum, momentum, recoil_velocity, total_momentum, two_body_total_momentum,
    velocity_from_momentum,
};

#[cfg(all(feature = "momentum", feature = "force"))]
pub use use_momentum::{
    MovingMass, average_force_from_impulse, impulse as momentum_impulse,
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

#[cfg(feature = "oscillation")]
pub use use_oscillation as oscillation;

#[cfg(feature = "oscillation")]
pub use use_oscillation::{
    SimpleHarmonicOscillator, SpringOscillator, acceleration, acceleration_from_displacement,
    angular_frequency_from_frequency, angular_frequency_from_period, critical_damping_coefficient,
    damped_angular_frequency, damping_ratio, damping_ratio_from_quality_factor,
    frequency_from_angular_frequency, frequency_from_period, is_critically_damped, is_overdamped,
    is_underdamped, kinetic_energy_from_total_and_potential, mass_from_spring_period,
    max_acceleration, max_speed, oscillator_total_energy, pendulum_length_from_period,
    period_from_angular_frequency, period_from_frequency, quality_factor_from_damping_ratio,
    resonance_angular_frequency_natural, simple_pendulum_angular_frequency,
    simple_pendulum_frequency, simple_pendulum_period, spring_angular_frequency,
    spring_constant_from_period, spring_frequency, spring_period, velocity,
};

#[cfg(feature = "oscillation")]
pub use use_oscillation::{
    displacement as oscillation_displacement,
    spring_potential_energy as oscillation_spring_potential_energy,
};

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

#[cfg(feature = "rigidbody")]
pub use use_rigidbody as rigidbody;

#[cfg(feature = "rigidbody")]
pub use use_rigidbody::{
    MassProperties, RigidBody1D, angular_impulse_from_angular_velocity_change,
    angular_velocity_after_angular_impulse, center_moment_from_parallel_axis, center_of_mass_1d,
    combined_mass, impulse_from_velocity_change, linear_kinetic_energy, linear_momentum,
    parallel_axis_moment_of_inertia, reduced_mass, total_kinetic_energy, velocity_after_impulse,
};

#[cfg(all(
    feature = "rigidbody",
    not(feature = "rotation"),
    not(feature = "torque")
))]
pub use use_rigidbody::{
    angular_momentum, hollow_sphere_moment_of_inertia, point_mass_moment_of_inertia,
    rod_moment_of_inertia_about_center, rod_moment_of_inertia_about_end, rotational_kinetic_energy,
    solid_disk_moment_of_inertia, solid_sphere_moment_of_inertia, thin_ring_moment_of_inertia,
};

#[cfg(all(feature = "rigidbody", not(feature = "rotation"), feature = "torque"))]
pub use use_rigidbody::{
    angular_momentum, hollow_sphere_moment_of_inertia,
    point_mass_moment_of_inertia as rigidbody_point_mass_moment_of_inertia,
    rod_moment_of_inertia_about_center as rigidbody_rod_moment_of_inertia_about_center,
    rod_moment_of_inertia_about_end as rigidbody_rod_moment_of_inertia_about_end,
    rotational_kinetic_energy, solid_disk_moment_of_inertia, solid_sphere_moment_of_inertia,
    thin_ring_moment_of_inertia,
};

#[cfg(all(feature = "rigidbody", feature = "rotation"))]
pub use use_rigidbody::{
    angular_momentum as rigidbody_angular_momentum,
    hollow_sphere_moment_of_inertia as rigidbody_hollow_sphere_moment_of_inertia,
    point_mass_moment_of_inertia as rigidbody_point_mass_moment_of_inertia,
    rod_moment_of_inertia_about_center as rigidbody_rod_moment_of_inertia_about_center,
    rod_moment_of_inertia_about_end as rigidbody_rod_moment_of_inertia_about_end,
    rotational_kinetic_energy as rigidbody_rotational_kinetic_energy,
    solid_disk_moment_of_inertia as rigidbody_solid_disk_moment_of_inertia,
    solid_sphere_moment_of_inertia as rigidbody_solid_sphere_moment_of_inertia,
    thin_ring_moment_of_inertia as rigidbody_thin_ring_moment_of_inertia,
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

#[cfg(feature = "radiation")]
pub use use_radiation as radiation;

#[cfg(feature = "radiation")]
pub use use_radiation::{
    Dose, RadiationBeam, RadiationKind, Shield, absorbed_dose, absorbed_energy_from_dose,
    accumulated_dose, attenuated_intensity, default_radiation_weighting_factor, dose_rate,
    effective_dose, energy_fluence, equivalent_dose, fluence, fluence_rate, half_value_layer,
    intensity, inverse_square_intensity, is_ionizing, is_particle_radiation, is_photon_radiation,
    isotropic_intensity, linear_attenuation_from_mass_attenuation,
    mass_attenuation_from_linear_attenuation, photon_flux_density, photon_flux_from_power,
    required_shield_thickness, tenth_value_layer, total_effective_dose, transmitted_fraction,
};

#[cfg(feature = "radiation")]
pub use use_radiation::JOULES_PER_MEV as RADIATION_JOULES_PER_MEV;

#[cfg(feature = "radiation")]
pub use use_radiation::SPEED_OF_LIGHT as RADIATION_SPEED_OF_LIGHT;

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
