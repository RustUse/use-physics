use use_gravity::{GravityBody, escape_velocity};

#[test]
fn gravity_helpers_cover_surface_and_escape_calculations() {
    let earth = GravityBody::new(5.972e24, 6.371e6).unwrap();
    let gravity = earth.surface_gravity().unwrap();
    let velocity = escape_velocity(5.972e24, 6.371e6).unwrap();

    assert!(gravity > 9.8);
    assert!(velocity > 11_000.0);
}
