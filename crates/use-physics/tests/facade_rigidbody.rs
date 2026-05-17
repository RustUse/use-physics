#![allow(clippy::float_cmp)]

#[cfg(feature = "rigidbody")]
#[test]
fn facade_reexports_rigidbody_module_and_prelude() {
    use use_physics::prelude::{MassProperties, RigidBody1D, center_of_mass_1d};
    use use_physics::rigidbody::{MassProperties as RigidbodyMassProperties, RigidBody1D as Body};

    let props = MassProperties::new(2.0, 4.0).unwrap();
    let body = RigidBody1D::new(props, 10.0, 3.0, 1.0, 5.0).unwrap();
    let module_props = RigidbodyMassProperties::new(2.0, 4.0).unwrap();
    let module_body = Body::new(module_props, 10.0, 3.0, 1.0, 5.0).unwrap();

    assert_eq!(center_of_mass_1d(&[1.0, 1.0], &[0.0, 10.0]), Some(5.0));
    assert_eq!(body.total_kinetic_energy(), Some(59.0));
    assert_eq!(module_body.with_impulse(4.0).unwrap().velocity, 5.0);
}

#[cfg(all(
    feature = "rigidbody",
    not(feature = "rotation"),
    not(feature = "torque")
))]
#[test]
fn facade_exposes_rigidbody_root_exports_without_rotation_or_torque() {
    assert_eq!(use_physics::angular_momentum(4.0, 5.0), Some(20.0));
    assert_eq!(
        use_physics::solid_disk_moment_of_inertia(2.0, 3.0),
        Some(9.0)
    );
    assert_eq!(
        use_physics::point_mass_moment_of_inertia(2.0, 3.0),
        Some(18.0)
    );
}

#[cfg(all(feature = "rigidbody", feature = "rotation"))]
#[test]
fn facade_aliases_rigidbody_rotation_overlaps() {
    assert_eq!(use_physics::angular_momentum(4.0, 5.0), Some(20.0));
    assert_eq!(
        use_physics::rigidbody_angular_momentum(4.0, 5.0),
        Some(20.0)
    );
    assert_eq!(
        use_physics::solid_disk_moment_of_inertia(2.0, 3.0),
        Some(9.0)
    );
    assert_eq!(
        use_physics::rigidbody_solid_disk_moment_of_inertia(2.0, 3.0),
        Some(9.0)
    );
    assert_eq!(
        use_physics::rigidbody_rotational_kinetic_energy(4.0, 5.0),
        Some(50.0)
    );
}

#[cfg(all(feature = "rigidbody", feature = "torque", not(feature = "rotation")))]
#[test]
fn facade_aliases_rigidbody_torque_overlaps_without_rotation() {
    assert_eq!(
        use_physics::point_mass_moment_of_inertia(2.0, 3.0),
        Some(18.0)
    );
    assert_eq!(
        use_physics::rigidbody_point_mass_moment_of_inertia(2.0, 3.0),
        Some(18.0)
    );
    assert_eq!(
        use_physics::rod_moment_of_inertia_about_center(12.0, 2.0),
        Some(4.0)
    );
    assert_eq!(
        use_physics::rigidbody_rod_moment_of_inertia_about_center(12.0, 2.0),
        Some(4.0)
    );
}
