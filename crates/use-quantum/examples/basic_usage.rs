use use_quantum::{
    MatterWave, Photon, QuantumNumbers, REDUCED_PLANCK_CONSTANT, RYDBERG_ENERGY_EV,
    hydrogen_energy_level_ev, minimum_position_uncertainty,
};

fn approx_eq(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 1.0e-12 * scale
}

fn main() -> Result<(), &'static str> {
    let photon = Photon::from_wavelength(500.0e-9).ok_or("expected valid photon wavelength")?;
    let matter_wave =
        MatterWave::from_mass_velocity(2.0, 3.0).ok_or("expected valid matter wave")?;
    let quantum_numbers =
        QuantumNumbers::new(2, 1, 0, 1).ok_or("expected valid quantum numbers")?;

    assert!(photon.energy_ev().ok_or("expected photon energy in eV")? > 0.0);
    assert!(
        matter_wave
            .wavelength()
            .ok_or("expected matter wavelength")?
            > 0.0
    );
    assert!(approx_eq(quantum_numbers.spin_projection(), 0.5));
    assert!(approx_eq(
        hydrogen_energy_level_ev(1).ok_or("expected hydrogen energy level")?,
        -RYDBERG_ENERGY_EV,
    ));
    assert!(approx_eq(
        minimum_position_uncertainty(REDUCED_PLANCK_CONSTANT)
            .ok_or("expected minimum uncertainty")?,
        0.5,
    ));

    Ok(())
}
