use use_orbit::{CentralBody, EllipticalOrbit, hohmann_total_delta_v};

fn main() {
    let earth = CentralBody::with_radius(5.972e24, 6.371e6);
    let low_orbit_radius = earth.and_then(|body| body.orbital_radius_from_altitude(400_000.0));
    let low_orbit_speed = earth.and_then(|body| {
        low_orbit_radius.and_then(|radius| body.circular_orbital_speed_at_radius(radius))
    });
    let transfer = hohmann_total_delta_v(398_600_441_800_000.0, 6_771_000.0, 42_164_000.0);

    assert_eq!(low_orbit_radius, Some(6_771_000.0));
    assert!(low_orbit_speed.is_some_and(|speed| speed > 7_600.0));
    assert!(transfer.is_some_and(|delta_v| delta_v > 0.0));
    assert!(
        EllipticalOrbit::new(100.0, 10.0, 20.0)
            .and_then(|orbit| orbit.period())
            .is_some_and(|period| period > 0.0)
    );
}
