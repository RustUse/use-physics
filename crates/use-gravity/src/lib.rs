#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Gravity, orbit, and gravitational energy helpers.

use core::f64::consts::TAU;

pub mod prelude;

/// Conventional standard gravity, in meters per second squared.
///
/// This crate keeps the value locally as a convenience for gravity-specific helpers.
/// Broader physical constants belong in the top-level `use-constants` set.
pub const STANDARD_GRAVITY: f64 = 9.806_65;

/// Newtonian constant of gravitation, in cubic meters per kilogram second squared.
///
/// This crate keeps the value locally as a convenience for gravity-specific helpers.
/// Broader physical constants belong in the top-level `use-constants` set.
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

/// Computes the gravitational attraction between two point masses.
///
/// Formula: `F = G * m1 * m2 / r^2`
///
/// Returns `None` when either mass is negative, when `distance` is less than or equal to zero,
/// or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_gravity::{GRAVITATIONAL_CONSTANT, gravitational_force};
///
/// assert_eq!(gravitational_force(1.0, 1.0, 1.0), Some(GRAVITATIONAL_CONSTANT));
/// ```
#[must_use]
pub fn gravitational_force(mass_a: f64, mass_b: f64, distance: f64) -> Option<f64> {
    if mass_a < 0.0 || mass_b < 0.0 || distance <= 0.0 {
        return None;
    }

    finite(GRAVITATIONAL_CONSTANT * mass_a * mass_b / distance.powi(2))
}

/// Computes the gravitational acceleration caused by a source mass at a distance.
///
/// Formula: `g = G * M / r^2`
///
/// Returns `None` when `source_mass` is negative, when `distance` is less than or equal to
/// zero, or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_gravity::{GRAVITATIONAL_CONSTANT, gravitational_acceleration};
///
/// assert_eq!(gravitational_acceleration(1.0, 1.0), Some(GRAVITATIONAL_CONSTANT));
/// ```
#[must_use]
pub fn gravitational_acceleration(source_mass: f64, distance: f64) -> Option<f64> {
    if source_mass < 0.0 || distance <= 0.0 {
        return None;
    }

    finite(GRAVITATIONAL_CONSTANT * source_mass / distance.powi(2))
}

/// Computes weight from mass and gravitational acceleration.
///
/// Formula: `W = m * g`
///
/// Returns `None` when `mass` is negative, when `gravitational_acceleration` is not finite, or
/// when the computed result is not finite.
#[must_use]
pub fn weight(mass: f64, gravitational_acceleration: f64) -> Option<f64> {
    if mass < 0.0 || !gravitational_acceleration.is_finite() {
        return None;
    }

    finite(mass * gravitational_acceleration)
}

/// Computes weight under conventional standard gravity.
#[must_use]
pub fn standard_weight(mass: f64) -> Option<f64> {
    weight(mass, STANDARD_GRAVITY)
}

/// Computes the circular orbital velocity around a source mass.
///
/// Formula: `v = sqrt(G * M / r)`
///
/// Returns `None` when `source_mass` is negative, when `orbital_radius` is less than or equal to
/// zero, or when the computed result is not finite.
#[must_use]
pub fn circular_orbital_velocity(source_mass: f64, orbital_radius: f64) -> Option<f64> {
    if source_mass < 0.0 || orbital_radius <= 0.0 {
        return None;
    }

    finite((GRAVITATIONAL_CONSTANT * source_mass / orbital_radius).sqrt())
}

/// Computes the escape velocity from a source mass at a distance.
///
/// Formula: `v = sqrt(2 * G * M / r)`
///
/// Returns `None` when `source_mass` is negative, when `distance` is less than or equal to zero,
/// or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_gravity::escape_velocity;
///
/// let velocity = escape_velocity(5.972e24, 6.371e6).unwrap();
///
/// assert!((velocity - 11_186.0).abs() < 2.0);
/// ```
#[must_use]
pub fn escape_velocity(source_mass: f64, distance: f64) -> Option<f64> {
    if source_mass < 0.0 || distance <= 0.0 {
        return None;
    }

    finite((2.0 * GRAVITATIONAL_CONSTANT * source_mass / distance).sqrt())
}

/// Computes the orbital period for a circular orbit.
///
/// Formula: `T = 2π * sqrt(r^3 / (G * M))`
///
/// Returns `None` when `source_mass` is less than or equal to zero, when `orbital_radius` is
/// less than or equal to zero, or when the computed result is not finite.
#[must_use]
pub fn circular_orbital_period(source_mass: f64, orbital_radius: f64) -> Option<f64> {
    if source_mass <= 0.0 || orbital_radius <= 0.0 {
        return None;
    }

    finite(TAU * (orbital_radius.powi(3) / (GRAVITATIONAL_CONSTANT * source_mass)).sqrt())
}

/// Computes gravitational potential energy between two masses.
///
/// Formula: `U = -G * m1 * m2 / r`
///
/// Returns `None` when either mass is negative, when `distance` is less than or equal to zero,
/// or when the computed result is not finite.
#[must_use]
pub fn gravitational_potential_energy(mass_a: f64, mass_b: f64, distance: f64) -> Option<f64> {
    if mass_a < 0.0 || mass_b < 0.0 || distance <= 0.0 {
        return None;
    }

    finite(-GRAVITATIONAL_CONSTANT * mass_a * mass_b / distance)
}

/// Computes near-surface potential energy from mass, height, and gravitational acceleration.
///
/// Formula: `U = m * g * h`
///
/// Returns `None` when `mass` is negative, when `gravitational_acceleration` is not finite, or
/// when the computed result is not finite. Negative heights are allowed.
#[must_use]
pub fn near_surface_potential_energy(
    mass: f64,
    height: f64,
    gravitational_acceleration: f64,
) -> Option<f64> {
    if mass < 0.0 || !gravitational_acceleration.is_finite() {
        return None;
    }

    finite(mass * gravitational_acceleration * height)
}

/// Mass and radius for a body used in gravity calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GravityBody {
    pub mass: f64,
    pub radius: f64,
}

impl GravityBody {
    /// Creates a gravity body from a mass and radius.
    ///
    /// Returns `None` when `mass` is negative, when `radius` is less than or equal to zero, or
    /// when either input is not finite.
    #[must_use]
    pub fn new(mass: f64, radius: f64) -> Option<Self> {
        if !mass.is_finite() || mass < 0.0 || !radius.is_finite() || radius <= 0.0 {
            return None;
        }

        Some(Self { mass, radius })
    }

    /// Computes the surface gravity of the body.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_gravity::GravityBody;
    ///
    /// let earth = GravityBody::new(5.972e24, 6.371e6).unwrap();
    /// let gravity = earth.surface_gravity().unwrap();
    ///
    /// assert!((gravity - 9.82).abs() < 0.05);
    /// ```
    #[must_use]
    pub fn surface_gravity(&self) -> Option<f64> {
        gravitational_acceleration(self.mass, self.radius)
    }

    /// Computes the escape velocity from the body's surface.
    #[must_use]
    pub fn escape_velocity(&self) -> Option<f64> {
        escape_velocity(self.mass, self.radius)
    }

    /// Computes the circular orbital velocity at a given radius from the body's center.
    #[must_use]
    pub fn circular_orbital_velocity_at_radius(&self, orbital_radius: f64) -> Option<f64> {
        circular_orbital_velocity(self.mass, orbital_radius)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GRAVITATIONAL_CONSTANT, GravityBody, STANDARD_GRAVITY, circular_orbital_period,
        circular_orbital_velocity, escape_velocity, gravitational_acceleration,
        gravitational_force, gravitational_potential_energy, near_surface_potential_energy,
        standard_weight, weight,
    };

    fn approx_eq(left: f64, right: f64, tolerance: f64) {
        let delta = (left - right).abs();

        assert!(
            delta <= tolerance,
            "left={left} right={right} delta={delta} tolerance={tolerance}"
        );
    }

    #[test]
    fn gravitational_force_handles_basic_cases() {
        assert_eq!(
            gravitational_force(1.0, 1.0, 1.0),
            Some(GRAVITATIONAL_CONSTANT)
        );
        assert_eq!(gravitational_force(1.0, 1.0, 0.0), None);
        assert_eq!(gravitational_force(-1.0, 1.0, 1.0), None);
    }

    #[test]
    fn gravitational_acceleration_matches_earth_surface() {
        let gravity = gravitational_acceleration(5.972e24, 6.371e6).unwrap();

        approx_eq(gravity, 9.82, 0.05);
    }

    #[test]
    fn weight_helpers_match_standard_gravity() {
        approx_eq(weight(10.0, STANDARD_GRAVITY).unwrap(), 98.066_5, 1.0e-12);
        approx_eq(standard_weight(10.0).unwrap(), 98.066_5, 1.0e-12);
    }

    #[test]
    fn orbital_helpers_match_earth_scale_values() {
        approx_eq(escape_velocity(5.972e24, 6.371e6).unwrap(), 11_186.0, 2.0);
        approx_eq(
            circular_orbital_velocity(5.972e24, 6.371e6).unwrap(),
            7_909.0,
            2.0,
        );

        let period = circular_orbital_period(5.972e24, 6.371e6).unwrap();

        assert!(period.is_finite());
        assert!(period > 0.0);
    }

    #[test]
    fn potential_energy_helpers_match_expected_values() {
        assert_eq!(
            gravitational_potential_energy(1.0, 1.0, 1.0),
            Some(-GRAVITATIONAL_CONSTANT)
        );
        approx_eq(
            near_surface_potential_energy(2.0, 10.0, STANDARD_GRAVITY).unwrap(),
            196.133,
            1.0e-12,
        );
    }

    #[test]
    fn gravity_body_validates_inputs_and_delegates() {
        let earth = GravityBody::new(5.972e24, 6.371e6).unwrap();

        approx_eq(earth.surface_gravity().unwrap(), 9.82, 0.05);
        assert_eq!(GravityBody::new(-1.0, 1.0), None);
        assert_eq!(GravityBody::new(1.0, 0.0), None);
    }
}
