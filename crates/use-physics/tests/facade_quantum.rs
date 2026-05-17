#[cfg(feature = "quantum")]
#[test]
fn facade_reexports_quantum_helpers() {
    use use_physics::{Photon, QuantumNumbers, hydrogen_transition_wavelength, quantum};

    fn approx_eq(left: f64, right: f64) -> bool {
        let scale = left.abs().max(right.abs()).max(1.0);
        (left - right).abs() <= 1.0e-12 * scale
    }

    let photon = Photon::from_frequency(1.0).expect("valid photon frequency");
    let quantum_numbers = QuantumNumbers::new(2, 1, 0, 1).expect("valid quantum numbers");

    assert!(approx_eq(
        photon.energy_joules(),
        use_physics::PLANCK_CONSTANT
    ));
    assert!(approx_eq(quantum_numbers.spin_projection(), 0.5));
    assert!(hydrogen_transition_wavelength(2, 1).is_some_and(|value| value > 0.0));
    assert_eq!(
        quantum::photon_energy_from_frequency(1.0),
        Some(use_physics::PLANCK_CONSTANT)
    );
}
