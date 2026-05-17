use use_physics::{Photon, QuantumNumbers, hydrogen_transition_wavelength};

fn approx_eq(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 1.0e-12 * scale
}

fn main() -> Result<(), &'static str> {
    let photon = Photon::from_frequency(1.0).ok_or("expected valid photon")?;
    let quantum_numbers =
        QuantumNumbers::new(2, 1, 0, 1).ok_or("expected valid quantum numbers")?;
    let wavelength =
        hydrogen_transition_wavelength(2, 1).ok_or("expected transition wavelength")?;

    assert!(approx_eq(
        photon.energy_joules(),
        use_physics::PLANCK_CONSTANT
    ));
    assert!(approx_eq(quantum_numbers.spin_projection(), 0.5));
    assert!(wavelength > 0.0);

    Ok(())
}
