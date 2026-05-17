#![allow(clippy::float_cmp)]

#[cfg(all(
    feature = "motion",
    feature = "oscillation",
    feature = "rotation",
    feature = "rigidbody",
    feature = "force",
    feature = "torque",
    feature = "statics",
    feature = "energy",
    feature = "collision",
    feature = "work",
    feature = "power",
    feature = "electricity",
    feature = "magnetism",
    feature = "electromagnetism",
    feature = "plasma",
    feature = "pressure",
    feature = "elasticity",
    feature = "fluid",
    feature = "density",
    feature = "gravity",
    feature = "momentum",
    feature = "relativity",
    feature = "quantum",
    feature = "particle",
    feature = "nuclear",
    feature = "radiation",
    feature = "thermodynamics"
))]
#[test]
fn facade_exposes_all_namespace_features() {
    use use_physics::{
        collision as _, density as _, elasticity as _, electricity as _, electromagnetism as _,
        energy as _, fluid as _, force as _, gravity as _, magnetism as _, momentum as _,
        motion as _, nuclear as _, oscillation as _, particle as _, plasma as _, power as _,
        pressure as _, quantum as _, radiation as _, relativity as _, rigidbody as _,
        rotation as _, statics as _, thermodynamics as _, torque as _,
    };

    let _ = use_physics::work::net_work;
    let _ = use_physics::ELECTROMAGNETISM_SPEED_OF_LIGHT;
    let _ = use_physics::ELECTROMAGNETISM_VACUUM_PERMEABILITY;
    let _ = use_physics::PLASMA_ELEMENTARY_CHARGE;
    let _ = use_physics::PLASMA_ELECTRON_MASS;
    let _ = use_physics::PLASMA_VACUUM_PERMEABILITY;
    let _ = use_physics::PLASMA_VACUUM_PERMITTIVITY;
    let _ = use_physics::RADIATION_SPEED_OF_LIGHT;
    let _ = use_physics::RADIATION_JOULES_PER_MEV;
    let _ = use_physics::RELATIVITY_SPEED_OF_LIGHT;
    let _ = use_physics::QUANTUM_SPEED_OF_LIGHT;
    let _ = use_physics::oscillation_displacement;
    let _ = use_physics::oscillation_spring_potential_energy;
    let _ = use_physics::collision_kinetic_energy;
    let _ = use_physics::rigidbody_angular_momentum;
    let _ = use_physics::rigidbody_point_mass_moment_of_inertia;
    let _ = use_physics::rigidbody_rod_moment_of_inertia_about_center;
    let _ = use_physics::rigidbody_rotational_kinetic_energy;
    let _ = use_physics::rigidbody_solid_disk_moment_of_inertia;
    let _ = use_physics::statics_is_rotational_equilibrium;
}

#[cfg(all(
    feature = "force",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_force_only() {
    assert_eq!(use_physics::force(10.0, 2.0), 20.0);
}

#[cfg(all(
    feature = "momentum",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "relativity"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_momentum_only() {
    assert_eq!(use_physics::momentum(2.0, 3.0), Some(6.0));
    assert_eq!(use_physics::impulse(10.0, 2.0), Some(20.0));
}

#[cfg(all(
    feature = "collision",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "orbit"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "quantum"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_collision_only() {
    assert_eq!(use_physics::relative_velocity(5.0, 2.0), Some(3.0));
    assert_eq!(
        use_physics::coefficient_of_restitution(10.0, 8.0),
        Some(0.8)
    );
    assert_eq!(
        use_physics::perfectly_inelastic_collision_velocity_1d(1.0, 1.0, 1.0, -1.0),
        Some(0.0)
    );
    assert_eq!(use_physics::kinetic_energy(2.0, 3.0), Some(9.0));
}

#[cfg(all(
    feature = "rotation",
    not(feature = "motion"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_rotation_only() {
    assert_eq!(use_physics::angular_velocity(10.0, 2.0), Some(5.0));
    assert_eq!(
        use_physics::solid_disk_moment_of_inertia(2.0, 3.0),
        Some(9.0)
    );
}

#[cfg(all(
    feature = "electromagnetism",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "pressure"),
    not(feature = "plasma"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_electromagnetism_only() {
    assert_eq!(use_physics::electric_force_on_charge(2.0, 3.0), Some(6.0));
    assert_eq!(use_physics::velocity_selector_speed(20.0, 4.0), Some(5.0));
    assert_eq!(use_physics::SPEED_OF_LIGHT, 299_792_458.0);
    assert_eq!(use_physics::VACUUM_PERMEABILITY, 1.256_637_062_12e-6);
}

#[cfg(all(
    feature = "nuclear",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "particle"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_nuclear_only() {
    assert_eq!(use_physics::activity(2.0, 10.0), Some(20.0));
    assert_eq!(use_physics::neutron_count(4, 2), Some(2));
}

#[cfg(all(
    feature = "elasticity",
    not(feature = "motion"),
    not(feature = "oscillation"),
    not(feature = "rotation"),
    not(feature = "rigidbody"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "statics"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "orbit"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "quantum"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_elasticity_only() {
    assert_eq!(use_physics::normal_stress(100.0, 2.0), Some(50.0));
    assert_eq!(
        use_physics::ElasticBar::new(10.0, 2.0, 1_000.0)
            .and_then(|bar| bar.deformation_under_force(100.0)),
        Some(0.5)
    );
}

#[cfg(all(
    feature = "radiation",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "orbit"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "quantum"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_radiation_only() {
    assert_eq!(use_physics::intensity(10.0, 2.0), Some(5.0));
    assert_eq!(
        use_physics::half_value_layer(core::f64::consts::LN_2),
        Some(1.0)
    );
    assert_eq!(use_physics::RADIATION_SPEED_OF_LIGHT, 299_792_458.0);
    assert_eq!(use_physics::RADIATION_JOULES_PER_MEV, 1.602_176_634e-13);
}

#[cfg(all(
    feature = "relativity",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_relativity_only() {
    assert_eq!(use_physics::SPEED_OF_LIGHT, 299_792_458.0);
    assert_eq!(
        use_physics::beta(use_physics::SPEED_OF_LIGHT * 0.5),
        Some(0.5)
    );
    assert!(
        (use_physics::lorentz_factor(use_physics::SPEED_OF_LIGHT * 0.6).unwrap() - 1.25).abs()
            < 1.0e-12
    );
}

#[cfg(all(
    feature = "quantum",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_quantum_only() {
    assert_eq!(
        use_physics::photon_energy_from_frequency(1.0),
        Some(use_physics::PLANCK_CONSTANT)
    );
    assert_eq!(use_physics::SPEED_OF_LIGHT, 299_792_458.0);
}

#[cfg(all(
    feature = "statics",
    not(feature = "motion"),
    not(feature = "oscillation"),
    not(feature = "rotation"),
    not(feature = "rigidbody"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "collision"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "electromagnetism"),
    not(feature = "plasma"),
    not(feature = "pressure"),
    not(feature = "elasticity"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "orbit"),
    not(feature = "momentum"),
    not(feature = "relativity"),
    not(feature = "quantum"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "radiation"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_statics_only() {
    assert_eq!(use_physics::net_force_1d(&[10.0, -4.0, -6.0]), Some(0.0));
    assert_eq!(use_physics::moment_2d(2.0, 0.0, 0.0, 10.0), Some(20.0));
    assert_eq!(
        use_physics::simply_supported_point_load_reactions(10.0, 100.0, 5.0),
        Some((50.0, 50.0))
    );
}

#[cfg(all(feature = "force", feature = "momentum"))]
#[test]
fn facade_renames_conflicting_impulse_exports() {
    assert_eq!(use_physics::impulse(2.0, 1.0, 4.0), 6.0);
    assert_eq!(use_physics::momentum_impulse(10.0, 2.0), Some(20.0));
}

#[cfg(all(feature = "energy", feature = "collision"))]
#[test]
fn facade_renames_conflicting_collision_kinetic_energy_export() {
    assert_eq!(use_physics::kinetic_energy(2.0, 3.0), 9.0);
    assert_eq!(use_physics::collision_kinetic_energy(2.0, 3.0), Some(9.0));
}

#[cfg(all(feature = "rotation", feature = "torque"))]
#[test]
fn facade_renames_conflicting_rotation_exports() {
    assert_eq!(
        use_physics::angular_acceleration_from_torque(20.0, 4.0),
        Some(5.0)
    );
    assert_eq!(
        use_physics::rotation_angular_acceleration_from_torque(20.0, 4.0),
        Some(5.0)
    );
    assert_eq!(
        use_physics::point_mass_moment_of_inertia(2.0, 3.0),
        Some(18.0)
    );
    assert_eq!(
        use_physics::rotation_point_mass_moment_of_inertia(2.0, 3.0),
        Some(18.0)
    );
}

#[cfg(all(feature = "statics", feature = "torque"))]
#[test]
fn facade_renames_conflicting_statics_rotational_equilibrium_export() {
    assert_eq!(
        use_physics::is_rotational_equilibrium(&[10.0, -10.0], 0.0),
        Some(true)
    );
    assert_eq!(
        use_physics::statics_is_rotational_equilibrium(&[10.0, -10.0], 0.0),
        Some(true)
    );
}

#[cfg(all(
    feature = "relativity",
    any(feature = "electromagnetism", feature = "nuclear")
))]
#[test]
fn facade_renames_conflicting_relativity_speed_of_light_export() {
    assert_eq!(use_physics::RELATIVITY_SPEED_OF_LIGHT, 299_792_458.0);
}

#[cfg(all(
    feature = "quantum",
    any(
        feature = "electromagnetism",
        feature = "nuclear",
        feature = "relativity"
    )
))]
#[test]
fn facade_renames_conflicting_quantum_speed_of_light_export() {
    assert_eq!(use_physics::QUANTUM_SPEED_OF_LIGHT, 299_792_458.0);
}
