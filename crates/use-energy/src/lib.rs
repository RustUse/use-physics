#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Work and mechanical energy helpers.

pub mod prelude;

#[must_use]
pub const fn kinetic_energy(mass: f64, velocity: f64) -> f64 {
    0.5 * mass * velocity * velocity
}

#[must_use]
pub const fn potential_energy(mass: f64, gravity: f64, height: f64) -> f64 {
    mass * gravity * height
}

#[must_use]
pub const fn work(force: f64, displacement: f64) -> f64 {
    force * displacement
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{kinetic_energy, potential_energy, work};

    #[test]
    fn energy_helpers_cover_common_calculations() {
        assert_eq!(kinetic_energy(2.0, 3.0), 9.0);
        assert_eq!(potential_energy(2.0, 10.0, 3.0), 60.0);
        assert_eq!(work(5.0, 10.0), 50.0);
    }
}
