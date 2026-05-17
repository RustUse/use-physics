#![allow(clippy::float_cmp)]

#[cfg(feature = "electromagnetism")]
#[test]
fn facade_reexports_combined_field_workflow() {
    use use_physics::prelude::{
        ElectromagneticField, lorentz_force_scalar, velocity_selector_speed,
    };

    let field = ElectromagneticField::new(10.0, 2.0).unwrap();

    assert_eq!(velocity_selector_speed(20.0, 4.0), Some(5.0));
    assert_eq!(
        lorentz_force_scalar(1.0, 10.0, 2.0, 3.0, core::f64::consts::FRAC_PI_2),
        Some(16.0)
    );
    assert!(field.energy_density().unwrap() > 0.0);
    assert_eq!(
        use_physics::magnetic_flux_density_from_electric_field_in_vacuum(
            use_physics::electromagnetism::SPEED_OF_LIGHT,
        ),
        Some(1.0)
    );
}
