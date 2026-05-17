#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Orbital mechanics helpers.

use core::f64::consts::{PI, TAU};

pub mod prelude;

/// Newtonian constant of gravitation, in cubic meters per kilogram second squared.
///
/// This crate keeps the value locally as a convenience for orbital helpers.
/// Broader physical constants belong in the top-level `use-constants` set.
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

/// Conventional standard gravity, in meters per second squared.
///
/// This crate keeps the value locally as a convenience for orbital helpers.
/// Broader physical constants belong in the top-level `use-constants` set.
pub const STANDARD_GRAVITY: f64 = 9.806_65;

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

/// Computes the standard gravitational parameter from a source mass.
///
/// Formula: `μ = G * M`
///
/// Returns `None` when `source_mass` is negative, when the input is not finite, or when the
/// result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_orbit::{GRAVITATIONAL_CONSTANT, gravitational_parameter};
///
/// assert_eq!(gravitational_parameter(1.0), Some(GRAVITATIONAL_CONSTANT));
/// ```
#[must_use]
pub fn gravitational_parameter(source_mass: f64) -> Option<f64> {
    if source_mass < 0.0 || !source_mass.is_finite() {
        return None;
    }

    finite(GRAVITATIONAL_CONSTANT * source_mass)
}

/// Computes the source mass from a standard gravitational parameter.
///
/// Formula: `M = μ / G`
///
/// Returns `None` when `mu` is negative, when the input is not finite, or when the result is not
/// finite.
#[must_use]
pub fn source_mass_from_gravitational_parameter(mu: f64) -> Option<f64> {
    if mu < 0.0 || !mu.is_finite() {
        return None;
    }

    finite(mu / GRAVITATIONAL_CONSTANT)
}

/// Computes the speed for a circular orbit at a radius around a body with gravitational parameter.
///
/// Formula: `v = sqrt(μ / r)`
///
/// Returns `None` when `mu` is negative, when `orbital_radius` is less than or equal to zero,
/// or when the input or result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_orbit::circular_orbital_speed;
///
/// let speed = circular_orbital_speed(398_600_441_800_000.0, 6_371_000.0);
///
/// assert!(speed.is_some_and(|value| (value - 7_909.8).abs() < 1.0));
/// ```
#[must_use]
pub fn circular_orbital_speed(mu: f64, orbital_radius: f64) -> Option<f64> {
    if mu < 0.0 || orbital_radius <= 0.0 || !all_finite(&[mu, orbital_radius]) {
        return None;
    }

    finite((mu / orbital_radius).sqrt())
}

/// Computes the orbital period for a circular orbit.
///
/// Formula: `T = 2π * sqrt(r³ / μ)`
///
/// Returns `None` when `mu` is less than or equal to zero, when `orbital_radius` is less than or
/// equal to zero, or when the input or result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_orbit::circular_orbital_period;
///
/// let period = circular_orbital_period(398_600_441_800_000.0, 6_371_000.0);
///
/// assert!(period.is_some_and(|value| value.is_finite() && value > 0.0));
/// ```
#[must_use]
pub fn circular_orbital_period(mu: f64, orbital_radius: f64) -> Option<f64> {
    if mu <= 0.0 || orbital_radius <= 0.0 || !all_finite(&[mu, orbital_radius]) {
        return None;
    }

    finite(TAU * (orbital_radius.powi(3) / mu).sqrt())
}

/// Computes the circular orbital radius from an orbital period.
///
/// Formula: `r = cbrt(μ * (T / 2π)²)`
///
/// Returns `None` when `mu` is less than or equal to zero, when `period` is less than or equal
/// to zero, or when the input or result is not finite.
#[must_use]
pub fn orbital_radius_from_period(mu: f64, period: f64) -> Option<f64> {
    if mu <= 0.0 || period <= 0.0 || !all_finite(&[mu, period]) {
        return None;
    }

    let ratio = period / TAU;

    finite((mu * ratio.powi(2)).cbrt())
}

/// Computes the circular orbital radius from circular speed.
///
/// Formula: `r = μ / v²`
///
/// Returns `None` when `mu` is negative, when `speed` is less than or equal to zero, or when the
/// input or result is not finite.
#[must_use]
pub fn orbital_radius_from_circular_speed(mu: f64, speed: f64) -> Option<f64> {
    if mu < 0.0 || speed <= 0.0 || !all_finite(&[mu, speed]) {
        return None;
    }

    finite(mu / speed.powi(2))
}

/// Computes the semi-major axis from periapsis and apoapsis radii.
///
/// Formula: `a = (r_p + r_a) / 2`
///
/// Returns `None` when either radius is less than or equal to zero, when `apoapsis_radius` is
/// less than `periapsis_radius`, or when the input or result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_orbit::semi_major_axis_from_apsides;
///
/// assert_eq!(semi_major_axis_from_apsides(10.0, 20.0), Some(15.0));
/// ```
#[must_use]
pub fn semi_major_axis_from_apsides(periapsis_radius: f64, apoapsis_radius: f64) -> Option<f64> {
    if periapsis_radius <= 0.0
        || apoapsis_radius <= 0.0
        || apoapsis_radius < periapsis_radius
        || !all_finite(&[periapsis_radius, apoapsis_radius])
    {
        return None;
    }

    finite(periapsis_radius.midpoint(apoapsis_radius))
}

/// Computes eccentricity from periapsis and apoapsis radii.
///
/// Formula: `e = (r_a - r_p) / (r_a + r_p)`
///
/// Returns `None` when either radius is less than or equal to zero, when `apoapsis_radius` is
/// less than `periapsis_radius`, or when the input or result is not finite.
#[must_use]
pub fn eccentricity_from_apsides(periapsis_radius: f64, apoapsis_radius: f64) -> Option<f64> {
    if periapsis_radius <= 0.0
        || apoapsis_radius <= 0.0
        || apoapsis_radius < periapsis_radius
        || !all_finite(&[periapsis_radius, apoapsis_radius])
    {
        return None;
    }

    let eccentricity = (apoapsis_radius - periapsis_radius) / (apoapsis_radius + periapsis_radius);

    if !(0.0..1.0).contains(&eccentricity) {
        return None;
    }

    finite(eccentricity)
}

/// Computes periapsis radius from semi-major axis and eccentricity.
///
/// Formula: `r_p = a * (1 - e)`
///
/// Returns `None` when `semi_major_axis` is less than or equal to zero, when `eccentricity` is
/// outside `[0.0, 1.0)`, or when the input or result is not finite.
#[must_use]
pub fn periapsis_from_semi_major_axis_eccentricity(
    semi_major_axis: f64,
    eccentricity: f64,
) -> Option<f64> {
    if semi_major_axis <= 0.0 || !semi_major_axis.is_finite() || !(0.0..1.0).contains(&eccentricity)
    {
        return None;
    }

    finite(semi_major_axis * (1.0 - eccentricity))
}

/// Computes apoapsis radius from semi-major axis and eccentricity.
///
/// Formula: `r_a = a * (1 + e)`
///
/// Returns `None` when `semi_major_axis` is less than or equal to zero, when `eccentricity` is
/// outside `[0.0, 1.0)`, or when the input or result is not finite.
#[must_use]
pub fn apoapsis_from_semi_major_axis_eccentricity(
    semi_major_axis: f64,
    eccentricity: f64,
) -> Option<f64> {
    if semi_major_axis <= 0.0 || !semi_major_axis.is_finite() || !(0.0..1.0).contains(&eccentricity)
    {
        return None;
    }

    finite(semi_major_axis * (1.0 + eccentricity))
}

/// Computes orbital speed from the vis-viva equation.
///
/// Formula: `v = sqrt(μ * (2/r - 1/a))`
///
/// Returns `None` when `mu` is negative, when `orbital_radius` or `semi_major_axis` is less than
/// or equal to zero, when the value under the square root is negative, or when the input or
/// result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_orbit::vis_viva_speed;
///
/// assert!(vis_viva_speed(100.0, 10.0, 15.0).is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn vis_viva_speed(mu: f64, orbital_radius: f64, semi_major_axis: f64) -> Option<f64> {
    if mu < 0.0
        || orbital_radius <= 0.0
        || semi_major_axis <= 0.0
        || !all_finite(&[mu, orbital_radius, semi_major_axis])
    {
        return None;
    }

    let radicand = mu * ((2.0 / orbital_radius) - (1.0 / semi_major_axis));

    if radicand < 0.0 || !radicand.is_finite() {
        return None;
    }

    finite(radicand.max(0.0).sqrt())
}

/// Computes speed at periapsis using the vis-viva equation.
///
/// Returns `None` when the apsides are invalid or when the derived speed is invalid.
#[must_use]
pub fn periapsis_speed(mu: f64, periapsis_radius: f64, apoapsis_radius: f64) -> Option<f64> {
    semi_major_axis_from_apsides(periapsis_radius, apoapsis_radius)
        .and_then(|semi_major_axis| vis_viva_speed(mu, periapsis_radius, semi_major_axis))
}

/// Computes speed at apoapsis using the vis-viva equation.
///
/// Returns `None` when the apsides are invalid or when the derived speed is invalid.
#[must_use]
pub fn apoapsis_speed(mu: f64, periapsis_radius: f64, apoapsis_radius: f64) -> Option<f64> {
    semi_major_axis_from_apsides(periapsis_radius, apoapsis_radius)
        .and_then(|semi_major_axis| vis_viva_speed(mu, apoapsis_radius, semi_major_axis))
}

/// Computes the orbital period for an elliptical orbit.
///
/// Formula: `T = 2π * sqrt(a³ / μ)`
///
/// Returns `None` when `mu` is less than or equal to zero, when `semi_major_axis` is less than
/// or equal to zero, or when the input or result is not finite.
#[must_use]
pub fn elliptical_orbital_period(mu: f64, semi_major_axis: f64) -> Option<f64> {
    if mu <= 0.0 || semi_major_axis <= 0.0 || !all_finite(&[mu, semi_major_axis]) {
        return None;
    }

    finite(TAU * (semi_major_axis.powi(3) / mu).sqrt())
}

/// Computes escape speed from a distance around a body with gravitational parameter.
///
/// Formula: `v_escape = sqrt(2μ / r)`
///
/// Returns `None` when `mu` is negative, when `distance` is less than or equal to zero, or when
/// the input or result is not finite.
#[must_use]
pub fn escape_speed(mu: f64, distance: f64) -> Option<f64> {
    if mu < 0.0 || distance <= 0.0 || !all_finite(&[mu, distance]) {
        return None;
    }

    finite((2.0 * mu / distance).sqrt())
}

/// Computes specific orbital energy.
///
/// Formula: `ε = v² / 2 - μ / r`
///
/// Returns `None` when `mu` is negative, when `distance` is less than or equal to zero, or when
/// the input or result is not finite.
#[must_use]
pub fn specific_orbital_energy(speed: f64, mu: f64, distance: f64) -> Option<f64> {
    if mu < 0.0 || distance <= 0.0 || !all_finite(&[speed, mu, distance]) {
        return None;
    }

    finite((speed.powi(2) / 2.0) - (mu / distance))
}

/// Computes the semi-major axis for a bound orbit from specific orbital energy.
///
/// Formula: `a = -μ / (2ε)`
///
/// Returns `None` when `mu` is less than or equal to zero, when `specific_energy` is greater than
/// or equal to zero, or when the input or result is not finite.
#[must_use]
pub fn semi_major_axis_from_specific_energy(mu: f64, specific_energy: f64) -> Option<f64> {
    if mu <= 0.0 || specific_energy >= 0.0 || !all_finite(&[mu, specific_energy]) {
        return None;
    }

    finite(-mu / (2.0 * specific_energy))
}

/// Computes orbital radius from body radius and altitude.
///
/// Formula: `r = R + h`
///
/// Returns `None` when `body_radius` is less than or equal to zero, when `altitude` is negative,
/// or when the input or result is not finite.
#[must_use]
pub fn orbital_radius_from_altitude(body_radius: f64, altitude: f64) -> Option<f64> {
    if body_radius <= 0.0 || altitude < 0.0 || !all_finite(&[body_radius, altitude]) {
        return None;
    }

    finite(body_radius + altitude)
}

/// Computes altitude from body radius and orbital radius.
///
/// Formula: `h = r - R`
///
/// Returns `None` when `body_radius` is less than or equal to zero, when `orbital_radius` is
/// less than `body_radius`, or when the input or result is not finite.
#[must_use]
pub fn altitude_from_orbital_radius(body_radius: f64, orbital_radius: f64) -> Option<f64> {
    if body_radius <= 0.0
        || orbital_radius < body_radius
        || !all_finite(&[body_radius, orbital_radius])
    {
        return None;
    }

    finite(orbital_radius - body_radius)
}

/// Computes the semi-major axis of a Hohmann transfer ellipse.
///
/// Formula: `a_transfer = (r1 + r2) / 2`
///
/// Returns `None` when either radius is less than or equal to zero, or when the input or result
/// is not finite.
#[must_use]
pub fn hohmann_transfer_semi_major_axis(radius_initial: f64, radius_final: f64) -> Option<f64> {
    if radius_initial <= 0.0 || radius_final <= 0.0 || !all_finite(&[radius_initial, radius_final])
    {
        return None;
    }

    finite(radius_initial.midpoint(radius_final))
}

/// Computes the transfer time for a Hohmann transfer.
///
/// Formula: `t_transfer = π * sqrt(a_transfer³ / μ)`
///
/// Returns `None` when `mu` is less than or equal to zero, when either radius is less than or
/// equal to zero, or when the input or result is not finite.
#[must_use]
pub fn hohmann_transfer_time(mu: f64, radius_initial: f64, radius_final: f64) -> Option<f64> {
    if mu <= 0.0 || !mu.is_finite() {
        return None;
    }

    hohmann_transfer_semi_major_axis(radius_initial, radius_final)
        .and_then(|semi_major_axis| finite(PI * (semi_major_axis.powi(3) / mu).sqrt()))
}

/// Computes the first burn for a Hohmann transfer.
///
/// Formula: `Δv1 = sqrt(μ/r1) * (sqrt(2r2 / (r1 + r2)) - 1)`
///
/// Returns `None` when `mu` is less than or equal to zero, when either radius is less than or
/// equal to zero, or when the input or result is not finite.
#[must_use]
pub fn hohmann_delta_v_1(mu: f64, radius_initial: f64, radius_final: f64) -> Option<f64> {
    if mu <= 0.0 || radius_initial <= 0.0 || radius_final <= 0.0 {
        return None;
    }
    if !all_finite(&[mu, radius_initial, radius_final]) {
        return None;
    }

    let circular_speed = (mu / radius_initial).sqrt();
    let transfer_factor = ((2.0 * radius_final) / (radius_initial + radius_final)).sqrt() - 1.0;

    finite(circular_speed * transfer_factor)
}

/// Computes the second burn for a Hohmann transfer.
///
/// Formula: `Δv2 = sqrt(μ/r2) * (1 - sqrt(2r1 / (r1 + r2)))`
///
/// Returns `None` when `mu` is less than or equal to zero, when either radius is less than or
/// equal to zero, or when the input or result is not finite.
#[must_use]
pub fn hohmann_delta_v_2(mu: f64, radius_initial: f64, radius_final: f64) -> Option<f64> {
    if mu <= 0.0 || radius_initial <= 0.0 || radius_final <= 0.0 {
        return None;
    }
    if !all_finite(&[mu, radius_initial, radius_final]) {
        return None;
    }

    let circular_speed = (mu / radius_final).sqrt();
    let transfer_factor = 1.0 - ((2.0 * radius_initial) / (radius_initial + radius_final)).sqrt();

    finite(circular_speed * transfer_factor)
}

/// Computes the total scalar delta-v magnitude for a Hohmann transfer.
///
/// Returns `None` when either component burn is invalid.
///
/// # Examples
///
/// ```rust
/// use use_orbit::hohmann_total_delta_v;
///
/// assert!(hohmann_total_delta_v(100.0, 10.0, 20.0).is_some_and(|value| value >= 0.0));
/// ```
#[must_use]
pub fn hohmann_total_delta_v(mu: f64, radius_initial: f64, radius_final: f64) -> Option<f64> {
    let delta_v_1 = hohmann_delta_v_1(mu, radius_initial, radius_final)?;
    let delta_v_2 = hohmann_delta_v_2(mu, radius_initial, radius_final)?;

    finite(delta_v_1.abs() + delta_v_2.abs())
}

/// Mass and optional radius for a central body used in orbital calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CentralBody {
    /// Mass in kilograms.
    pub mass: f64,
    /// Optional radius in meters.
    pub radius: Option<f64>,
}

impl CentralBody {
    /// Creates a central body with mass and no stored radius.
    ///
    /// Returns `None` when `mass` is negative or not finite.
    #[must_use]
    pub fn new(mass: f64) -> Option<Self> {
        if mass < 0.0 || !mass.is_finite() {
            return None;
        }

        Some(Self { mass, radius: None })
    }

    /// Creates a central body with mass and radius.
    ///
    /// Returns `None` when `mass` is negative or not finite, or when `radius` is less than or
    /// equal to zero or not finite.
    #[must_use]
    pub fn with_radius(mass: f64, radius: f64) -> Option<Self> {
        if mass < 0.0 || !mass.is_finite() || radius <= 0.0 || !radius.is_finite() {
            return None;
        }

        Some(Self {
            mass,
            radius: Some(radius),
        })
    }

    /// Computes the body's standard gravitational parameter.
    #[must_use]
    pub fn gravitational_parameter(&self) -> Option<f64> {
        gravitational_parameter(self.mass)
    }

    /// Computes orbital radius from the stored body radius and an altitude.
    ///
    /// Returns `None` when the body has no stored radius or when the inputs are invalid.
    #[must_use]
    pub fn orbital_radius_from_altitude(&self, altitude: f64) -> Option<f64> {
        self.radius
            .and_then(|radius| orbital_radius_from_altitude(radius, altitude))
    }

    /// Computes circular orbital speed at a radius around this body.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_orbit::CentralBody;
    ///
    /// let earth = CentralBody::with_radius(5.972e24, 6.371e6);
    /// let speed = earth.and_then(|body| body.circular_orbital_speed_at_radius(6_771_000.0));
    ///
    /// assert!(speed.is_some_and(|value| value > 7_600.0));
    /// ```
    #[must_use]
    pub fn circular_orbital_speed_at_radius(&self, orbital_radius: f64) -> Option<f64> {
        self.gravitational_parameter()
            .and_then(|mu| circular_orbital_speed(mu, orbital_radius))
    }

    /// Computes circular orbital period at a radius around this body.
    #[must_use]
    pub fn circular_orbital_period_at_radius(&self, orbital_radius: f64) -> Option<f64> {
        self.gravitational_parameter()
            .and_then(|mu| circular_orbital_period(mu, orbital_radius))
    }

    /// Computes escape speed at a radius around this body.
    #[must_use]
    pub fn escape_speed_at_radius(&self, distance: f64) -> Option<f64> {
        self.gravitational_parameter()
            .and_then(|mu| escape_speed(mu, distance))
    }
}

/// Elliptical orbit state described by a gravitational parameter and apsides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticalOrbit {
    /// Central-body gravitational parameter in cubic meters per second squared.
    pub mu: f64,
    /// Periapsis radius in meters.
    pub periapsis_radius: f64,
    /// Apoapsis radius in meters.
    pub apoapsis_radius: f64,
}

impl EllipticalOrbit {
    /// Creates an elliptical orbit from a gravitational parameter and apsides.
    ///
    /// Returns `None` when `mu` is less than or equal to zero or not finite, or when the apsides
    /// are invalid.
    #[must_use]
    pub fn new(mu: f64, periapsis_radius: f64, apoapsis_radius: f64) -> Option<Self> {
        if mu <= 0.0 || !mu.is_finite() {
            return None;
        }

        semi_major_axis_from_apsides(periapsis_radius, apoapsis_radius).map(|_| Self {
            mu,
            periapsis_radius,
            apoapsis_radius,
        })
    }

    /// Computes the semi-major axis.
    #[must_use]
    pub fn semi_major_axis(&self) -> Option<f64> {
        semi_major_axis_from_apsides(self.periapsis_radius, self.apoapsis_radius)
    }

    /// Computes eccentricity.
    #[must_use]
    pub fn eccentricity(&self) -> Option<f64> {
        eccentricity_from_apsides(self.periapsis_radius, self.apoapsis_radius)
    }

    /// Computes the orbital period.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_orbit::EllipticalOrbit;
    ///
    /// let orbit = EllipticalOrbit::new(100.0, 10.0, 20.0);
    ///
    /// assert!(orbit.and_then(|value| value.period()).is_some_and(|period| period > 0.0));
    /// ```
    #[must_use]
    pub fn period(&self) -> Option<f64> {
        self.semi_major_axis()
            .and_then(|semi_major_axis| elliptical_orbital_period(self.mu, semi_major_axis))
    }

    /// Computes speed at periapsis.
    #[must_use]
    pub fn periapsis_speed(&self) -> Option<f64> {
        periapsis_speed(self.mu, self.periapsis_radius, self.apoapsis_radius)
    }

    /// Computes speed at apoapsis.
    #[must_use]
    pub fn apoapsis_speed(&self) -> Option<f64> {
        apoapsis_speed(self.mu, self.periapsis_radius, self.apoapsis_radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(left: f64, right: f64, tolerance: f64) -> bool {
        (left - right).abs() <= tolerance
    }

    #[test]
    fn gravitational_parameter_matches_constant() {
        assert_eq!(gravitational_parameter(1.0), Some(GRAVITATIONAL_CONSTANT));
        assert_eq!(gravitational_parameter(-1.0), None);
    }

    #[test]
    fn source_mass_from_gravitational_parameter_round_trips() {
        assert!(
            source_mass_from_gravitational_parameter(GRAVITATIONAL_CONSTANT)
                .is_some_and(|value| approx_eq(value, 1.0, 1.0e-12))
        );
        assert_eq!(source_mass_from_gravitational_parameter(-1.0), None);
    }

    #[test]
    fn circular_orbital_speed_handles_valid_and_invalid_inputs() {
        assert!(
            circular_orbital_speed(398_600_441_800_000.0, 6_371_000.0)
                .is_some_and(|value| approx_eq(value, 7_909.8, 1.0))
        );
        assert_eq!(circular_orbital_speed(1.0, 0.0), None);
    }

    #[test]
    fn circular_orbital_period_handles_valid_and_invalid_inputs() {
        assert!(
            circular_orbital_period(398_600_441_800_000.0, 6_371_000.0)
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert_eq!(circular_orbital_period(0.0, 6_371_000.0), None);
    }

    #[test]
    fn orbital_radius_from_period_handles_valid_and_invalid_inputs() {
        assert!(
            orbital_radius_from_period(398_600_441_800_000.0, 5_400.0)
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert_eq!(orbital_radius_from_period(398_600_441_800_000.0, 0.0), None);
    }

    #[test]
    fn orbital_radius_from_circular_speed_handles_valid_and_invalid_inputs() {
        assert_eq!(orbital_radius_from_circular_speed(100.0, 10.0), Some(1.0));
        assert_eq!(orbital_radius_from_circular_speed(100.0, 0.0), None);
    }

    #[test]
    fn semi_major_axis_from_apsides_handles_valid_and_invalid_inputs() {
        assert_eq!(semi_major_axis_from_apsides(10.0, 20.0), Some(15.0));
        assert_eq!(semi_major_axis_from_apsides(20.0, 10.0), None);
    }

    #[test]
    fn eccentricity_from_apsides_handles_circular_and_elliptical_orbits() {
        assert!(
            eccentricity_from_apsides(10.0, 20.0).is_some_and(|value| approx_eq(
                value,
                1.0 / 3.0,
                1.0e-12
            ))
        );
        assert_eq!(eccentricity_from_apsides(10.0, 10.0), Some(0.0));
    }

    #[test]
    fn apsides_from_semi_major_axis_and_eccentricity_round_trip() {
        assert!(
            periapsis_from_semi_major_axis_eccentricity(15.0, 1.0 / 3.0)
                .is_some_and(|value| approx_eq(value, 10.0, 1.0e-12))
        );
        assert!(
            apoapsis_from_semi_major_axis_eccentricity(15.0, 1.0 / 3.0)
                .is_some_and(|value| approx_eq(value, 20.0, 1.0e-12))
        );
        assert_eq!(periapsis_from_semi_major_axis_eccentricity(15.0, 1.0), None);
    }

    #[test]
    fn vis_viva_speed_handles_valid_and_invalid_inputs() {
        assert!(
            vis_viva_speed(100.0, 10.0, 15.0).is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert_eq!(vis_viva_speed(100.0, 0.0, 15.0), None);
    }

    #[test]
    fn apsis_speeds_handle_valid_and_invalid_inputs() {
        assert!(
            periapsis_speed(100.0, 10.0, 20.0)
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert!(
            apoapsis_speed(100.0, 10.0, 20.0).is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert_eq!(periapsis_speed(100.0, 20.0, 10.0), None);
    }

    #[test]
    fn elliptical_orbital_period_handles_valid_and_invalid_inputs() {
        assert!(
            elliptical_orbital_period(100.0, 15.0)
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert_eq!(elliptical_orbital_period(0.0, 15.0), None);
    }

    #[test]
    fn escape_speed_handles_valid_and_invalid_inputs() {
        assert_eq!(escape_speed(100.0, 2.0), Some(10.0));
        assert_eq!(escape_speed(100.0, 0.0), None);
    }

    #[test]
    fn specific_orbital_energy_handles_valid_and_invalid_inputs() {
        assert_eq!(specific_orbital_energy(10.0, 100.0, 10.0), Some(40.0));
        assert_eq!(specific_orbital_energy(10.0, 100.0, 0.0), None);
    }

    #[test]
    fn semi_major_axis_from_specific_energy_handles_valid_and_invalid_inputs() {
        assert_eq!(
            semi_major_axis_from_specific_energy(100.0, -5.0),
            Some(10.0)
        );
        assert_eq!(semi_major_axis_from_specific_energy(100.0, 0.0), None);
    }

    #[test]
    fn altitude_and_radius_helpers_round_trip() {
        assert_eq!(
            orbital_radius_from_altitude(6_371_000.0, 400_000.0),
            Some(6_771_000.0)
        );
        assert_eq!(orbital_radius_from_altitude(6_371_000.0, -1.0), None);
        assert_eq!(
            altitude_from_orbital_radius(6_371_000.0, 6_771_000.0),
            Some(400_000.0)
        );
        assert_eq!(altitude_from_orbital_radius(6_371_000.0, 6_000_000.0), None);
    }

    #[test]
    fn hohmann_helpers_handle_valid_inputs() {
        assert_eq!(hohmann_transfer_semi_major_axis(10.0, 20.0), Some(15.0));
        assert!(
            hohmann_transfer_time(100.0, 10.0, 20.0)
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert!(hohmann_delta_v_1(100.0, 10.0, 20.0).is_some_and(f64::is_finite));
        assert!(hohmann_delta_v_2(100.0, 10.0, 20.0).is_some_and(f64::is_finite));
        assert!(
            hohmann_total_delta_v(100.0, 10.0, 20.0)
                .is_some_and(|value| value.is_finite() && value >= 0.0)
        );
    }

    #[test]
    fn central_body_delegates_to_public_helpers() {
        assert_eq!(
            CentralBody::new(1.0).and_then(|body| body.gravitational_parameter()),
            Some(GRAVITATIONAL_CONSTANT)
        );
        assert!(
            CentralBody::with_radius(5.972e24, 6.371e6)
                .and_then(|body| body.orbital_radius_from_altitude(400_000.0))
                .is_some_and(|value| approx_eq(value, 6.771e6, 1.0e-6))
        );
        assert!(
            CentralBody::with_radius(5.972e24, 6.371e6)
                .and_then(|body| body.circular_orbital_speed_at_radius(6.771e6))
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert_eq!(CentralBody::new(-1.0), None);
        assert_eq!(CentralBody::with_radius(1.0, 0.0), None);
    }

    #[test]
    fn elliptical_orbit_delegates_to_public_helpers() {
        let orbit = EllipticalOrbit::new(100.0, 10.0, 20.0);

        assert_eq!(orbit.and_then(|value| value.semi_major_axis()), Some(15.0));
        assert!(
            orbit
                .and_then(|value| value.eccentricity())
                .is_some_and(|value| approx_eq(value, 1.0 / 3.0, 1.0e-12))
        );
        assert!(
            orbit
                .and_then(|value| value.period())
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert!(
            orbit
                .and_then(|value| value.periapsis_speed())
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert!(
            orbit
                .and_then(|value| value.apoapsis_speed())
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert_eq!(EllipticalOrbit::new(100.0, 20.0, 10.0), None);
    }
}
