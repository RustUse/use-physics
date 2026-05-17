#![allow(clippy::float_cmp)]

#[cfg(all(
    feature = "motion",
    feature = "rotation",
    feature = "force",
    feature = "torque",
    feature = "energy",
    feature = "work",
    feature = "power",
    feature = "electricity",
    feature = "magnetism",
    feature = "pressure",
    feature = "fluid",
    feature = "density",
    feature = "gravity",
    feature = "momentum",
    feature = "particle",
    feature = "nuclear",
    feature = "thermodynamics"
))]
#[test]
fn facade_exposes_all_namespace_features() {
    use use_physics::{
        density as _, electricity as _, energy as _, fluid as _, force as _, gravity as _,
        magnetism as _, momentum as _, motion as _, nuclear as _, particle as _, power as _,
        pressure as _, rotation as _, thermodynamics as _, torque as _,
    };

    let _ = use_physics::work::net_work;
}

#[cfg(all(
    feature = "force",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "energy"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "pressure"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "particle"),
    not(feature = "nuclear"),
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
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "pressure"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "particle"),
    not(feature = "nuclear"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_momentum_only() {
    assert_eq!(use_physics::momentum(2.0, 3.0), Some(6.0));
    assert_eq!(use_physics::impulse(10.0, 2.0), Some(20.0));
}

#[cfg(all(
    feature = "rotation",
    not(feature = "motion"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "pressure"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "particle"),
    not(feature = "nuclear"),
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
    feature = "nuclear",
    not(feature = "motion"),
    not(feature = "rotation"),
    not(feature = "force"),
    not(feature = "torque"),
    not(feature = "energy"),
    not(feature = "work"),
    not(feature = "power"),
    not(feature = "electricity"),
    not(feature = "magnetism"),
    not(feature = "pressure"),
    not(feature = "fluid"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "particle"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_nuclear_only() {
    assert_eq!(use_physics::activity(2.0, 10.0), Some(20.0));
    assert_eq!(use_physics::neutron_count(4, 2), Some(2));
}

#[cfg(all(feature = "force", feature = "momentum"))]
#[test]
fn facade_renames_conflicting_impulse_exports() {
    assert_eq!(use_physics::impulse(2.0, 1.0, 4.0), 6.0);
    assert_eq!(use_physics::momentum_impulse(10.0, 2.0), Some(20.0));
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
