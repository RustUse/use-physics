use use_quantum::{MatterWave, Photon, QuantumNumbers, hydrogen_transition_wavelength};

fn approx_eq(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 1.0e-12 * scale
}

#[test]
fn quantum_helpers_cover_photons_matter_waves_and_quantum_numbers() -> Result<(), &'static str> {
    let photon = Photon::from_frequency(1.0).ok_or("expected valid photon")?;
    let matter_wave =
        MatterWave::from_mass_velocity(2.0, 3.0).ok_or("expected valid matter wave")?;
    let quantum_numbers =
        QuantumNumbers::new(2, 1, 0, 1).ok_or("expected valid quantum numbers")?;
    let wavelength =
        hydrogen_transition_wavelength(2, 1).ok_or("expected transition wavelength")?;

    assert!(approx_eq(
        photon.energy_joules(),
        use_quantum::PLANCK_CONSTANT
    ));
    assert!(approx_eq(
        matter_wave
            .wavelength()
            .ok_or("expected matter-wave wavelength")?,
        use_quantum::PLANCK_CONSTANT / 6.0,
    ));
    assert!(approx_eq(quantum_numbers.spin_projection(), 0.5));
    assert!(wavelength.is_finite() && wavelength > 0.0);

    Ok(())
}
