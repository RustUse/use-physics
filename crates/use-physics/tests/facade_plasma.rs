fn approx_eq(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 1.0e-12 * scale
}

#[cfg(feature = "plasma")]
#[test]
fn facade_reexports_plasma_workflow() {
    use use_physics::{ElectronPlasma, PROTON_MASS, PlasmaSpecies, alfven_speed, plasma};

    let plasma_state = ElectronPlasma::new(1.0e18, 10_000.0);
    let ion_species = PlasmaSpecies::new(1.0e18, 10_000.0, 1.0, PROTON_MASS);

    assert!(matches!(
        plasma_state.and_then(|value| value.plasma_frequency()),
        Some(value) if value.is_finite() && value > 0.0
    ));
    assert!(matches!(
        plasma_state.and_then(|value| value.debye_length()),
        Some(value) if value.is_finite() && value > 0.0
    ));
    assert!(matches!(
        ion_species.and_then(|value| value.pressure()),
        Some(value) if value.is_finite() && value > 0.0
    ));
    assert!(matches!(
        alfven_speed(1.0, 1.0e-12),
        Some(value) if value.is_finite() && value > 0.0
    ));
    assert!(matches!(
        plasma::charge_density(1.0e18, 1.0, 1.0e18),
        Some(value) if approx_eq(value, 0.0)
    ));
}

#[cfg(all(
    feature = "plasma",
    feature = "quantum",
    feature = "electromagnetism",
    feature = "magnetism"
))]
#[test]
fn facade_aliases_plasma_conflicting_exports() {
    assert!(approx_eq(
        use_physics::PLASMA_ELEMENTARY_CHARGE,
        use_physics::plasma::ELEMENTARY_CHARGE,
    ));
    assert!(approx_eq(
        use_physics::PLASMA_ELECTRON_MASS,
        use_physics::plasma::ELECTRON_MASS,
    ));
    assert!(approx_eq(
        use_physics::PLASMA_VACUUM_PERMITTIVITY,
        use_physics::plasma::VACUUM_PERMITTIVITY,
    ));
    assert!(approx_eq(
        use_physics::PLASMA_VACUUM_PERMEABILITY,
        use_physics::plasma::VACUUM_PERMEABILITY,
    ));
    assert_eq!(
        use_physics::plasma_magnetic_pressure(1.0),
        use_physics::plasma::magnetic_pressure(1.0)
    );
}
