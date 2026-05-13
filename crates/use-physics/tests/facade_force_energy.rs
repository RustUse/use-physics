#![allow(clippy::float_cmp)]

#[cfg(all(feature = "force", feature = "energy", feature = "motion"))]
#[test]
fn facade_reexports_core_physics_workflow() {
    use use_physics::prelude::{distance, force, kinetic_energy};

    assert_eq!(force(10.0, 2.0), 20.0);
    assert_eq!(kinetic_energy(2.0, 3.0), 9.0);
    assert_eq!(distance(5.0, 4.0), 20.0);
}
