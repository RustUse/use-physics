use use_electromagnetism::{
    ElectromagneticField, cyclotron_radius, lorentz_force_scalar, velocity_selector_speed,
};

fn main() -> Result<(), &'static str> {
    let field = ElectromagneticField::new(10.0, 2.0).ok_or("invalid field values")?;
    let selector_speed = velocity_selector_speed(20.0, 4.0).ok_or("invalid selector inputs")?;
    let lorentz_force = lorentz_force_scalar(1.0, 10.0, 2.0, 3.0, core::f64::consts::FRAC_PI_2)
        .ok_or("invalid Lorentz-force inputs")?;
    let radius = cyclotron_radius(2.0, 10.0, 1.0, 5.0).ok_or("invalid cyclotron inputs")?;

    assert_eq!(selector_speed, 5.0);
    assert_eq!(lorentz_force, 16.0);
    assert_eq!(radius, 4.0);
    assert!(field.energy_density().ok_or("invalid energy density")? > 0.0);

    Ok(())
}
