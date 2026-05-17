use use_electromagnetism::{
    ElectromagneticField, lorentz_force_scalar_degrees, poynting_magnitude, velocity_selector_speed,
};

#[test]
fn focused_crate_covers_combined_field_workflow() {
    let field = ElectromagneticField::new(10.0, 2.0).unwrap();
    let selector_speed = velocity_selector_speed(20.0, 4.0).unwrap();
    let lorentz_force = lorentz_force_scalar_degrees(1.0, 10.0, 2.0, 3.0, 90.0).unwrap();
    let poynting = poynting_magnitude(10.0, 2.0).unwrap();

    assert_eq!(field.electric_force_on_charge(3.0), Some(30.0));
    assert!((selector_speed - 5.0).abs() < 1.0e-12);
    assert!((lorentz_force - 16.0).abs() < 1.0e-9);
    assert!(poynting > 0.0);
}
