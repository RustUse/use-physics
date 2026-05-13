#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Force, weight, and impulse helpers.

pub mod prelude;

pub const STANDARD_GRAVITY: f64 = 9.806_65;

#[must_use]
pub const fn force(mass: f64, acceleration: f64) -> f64 {
    mass * acceleration
}

#[must_use]
pub const fn weight(mass: f64, gravity: f64) -> f64 {
    force(mass, gravity)
}

#[must_use]
pub const fn earth_weight(mass: f64) -> f64 {
    weight(mass, STANDARD_GRAVITY)
}

#[must_use]
pub const fn impulse(mass: f64, initial_velocity: f64, final_velocity: f64) -> f64 {
    mass * (final_velocity - initial_velocity)
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{STANDARD_GRAVITY, earth_weight, force, impulse, weight};

    #[test]
    fn force_helpers_cover_common_relationships() {
        assert_eq!(force(10.0, 2.0), 20.0);
        assert_eq!(weight(2.0, 10.0), 20.0);
        assert_eq!(impulse(2.0, 1.0, 4.0), 6.0);
        assert_eq!(earth_weight(1.0), STANDARD_GRAVITY);
    }
}
