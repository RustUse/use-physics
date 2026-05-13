#![allow(clippy::float_cmp)]

#[cfg(all(
    feature = "motion",
    feature = "force",
    feature = "energy",
    feature = "power",
    feature = "pressure",
    feature = "density",
    feature = "thermodynamics"
))]
#[test]
fn facade_exposes_all_namespace_features() {
    use use_physics::{
        density as _, energy as _, force as _, motion as _, power as _, pressure as _,
        thermodynamics as _,
    };
}

#[cfg(all(
    feature = "force",
    not(feature = "motion"),
    not(feature = "energy"),
    not(feature = "power"),
    not(feature = "pressure"),
    not(feature = "density"),
    not(feature = "thermodynamics")
))]
#[test]
fn facade_supports_force_only() {
    assert_eq!(use_physics::force(10.0, 2.0), 20.0);
}
