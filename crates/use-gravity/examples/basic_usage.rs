use use_gravity::{GRAVITATIONAL_CONSTANT, GravityBody, escape_velocity, gravitational_force};

fn main() -> Result<(), &'static str> {
    let earth = GravityBody::new(5.972e24, 6.371e6).ok_or("invalid Earth body")?;
    let surface_gravity = earth
        .surface_gravity()
        .ok_or("invalid surface gravity calculation")?;
    let force = gravitational_force(1.0, 1.0, 1.0).ok_or("invalid force calculation")?;
    let velocity = escape_velocity(5.972e24, 6.371e6).ok_or("invalid escape calculation")?;

    assert!((force - GRAVITATIONAL_CONSTANT).abs() < f64::EPSILON);
    assert!(surface_gravity > 9.8);
    assert!(velocity > 11_000.0);

    Ok(())
}
