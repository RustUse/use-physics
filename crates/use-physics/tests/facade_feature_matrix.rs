#![allow(clippy::float_cmp)]

#[cfg(all(
    feature = "motion",
    feature = "force",
    feature = "energy",
    feature = "power",
    feature = "pressure",
    feature = "density",
    feature = "gravity",
    feature = "momentum",
    feature = "thermodynamics"
))]
#[test]
fn facade_exposes_all_namespace_features() {
    use use_physics::{
        density as _, energy as _, force as _, gravity as _, momentum as _, motion as _,
        power as _, pressure as _, thermodynamics as _,
    };
}

#[cfg(all(
    feature = "force",
    not(feature = "motion"),
    not(feature = "energy"),
    not(feature = "power"),
    not(feature = "pressure"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "momentum"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_force_only() {
    assert_eq!(use_physics::force(10.0, 2.0), 20.0);
}

#[cfg(all(
    feature = "momentum",
    not(feature = "motion"),
    not(feature = "force"),
    not(feature = "energy"),
    not(feature = "power"),
    not(feature = "pressure"),
    not(feature = "density"),
    not(feature = "gravity"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_momentum_only() {
    assert_eq!(use_physics::momentum(2.0, 3.0), Some(6.0));
    assert_eq!(use_physics::impulse(10.0, 2.0), Some(20.0));
}

#[cfg(all(feature = "force", feature = "momentum"))]
#[test]
fn facade_renames_conflicting_impulse_exports() {
    assert_eq!(use_physics::impulse(2.0, 1.0, 4.0), 6.0);
    assert_eq!(use_physics::momentum_impulse(10.0, 2.0), Some(20.0));
}
