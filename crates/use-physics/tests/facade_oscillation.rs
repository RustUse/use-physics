#[cfg(feature = "oscillation")]
#[test]
fn facade_reexports_oscillation_workflow() {
    use core::f64::consts::PI;

    use use_physics::{
        SimpleHarmonicOscillator, SpringOscillator, damping_ratio, oscillation,
        oscillation_displacement, oscillation_spring_potential_energy,
    };

    let oscillator = SimpleHarmonicOscillator::new(2.0, 1.0, 0.0).unwrap();
    let spring = SpringOscillator::new(8.0, 2.0).unwrap();

    assert_eq!(damping_ratio(4.0, 2.0, 8.0), Some(0.5));
    assert_eq!(oscillation::displacement(2.0, 1.0, 0.0, 0.0), Some(2.0));
    assert_eq!(oscillation_displacement(2.0, 1.0, 0.0, 0.0), Some(2.0));
    assert_eq!(oscillation_spring_potential_energy(100.0, 0.5), Some(12.5));
    assert_eq!(spring.angular_frequency(), Some(2.0));
    assert!((spring.period().unwrap() - PI).abs() < 1.0e-12);
    assert_eq!(oscillator.displacement(0.0), Some(2.0));
}
