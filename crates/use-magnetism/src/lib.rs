#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Magnetism-specific scalar helpers.

use core::f64::consts::TAU;

pub mod prelude;

const TRIG_EPSILON: f64 = 1.0e-12;

/// Vacuum permeability in newtons per ampere squared.
///
/// This crate keeps the value locally as a convenience for magnetism-specific
/// helpers. Broader physical constants belong in the top-level `use-constants`
/// set.
pub const VACUUM_PERMEABILITY: f64 = 1.256_637_062_12e-6;

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

/// Computes magnetic flux through an area.
///
/// Formula: `Φ = B * A * cos(theta)`
///
/// Returns `None` when `area` is negative, when any input is not finite, or
/// when the computed result is not finite.
///
/// # Examples
///
/// ```
/// use use_magnetism::magnetic_flux;
///
/// assert_eq!(magnetic_flux(2.0, 3.0, 0.0), Some(6.0));
/// ```
#[must_use]
pub fn magnetic_flux(magnetic_flux_density: f64, area: f64, angle_radians: f64) -> Option<f64> {
    if !all_finite(&[magnetic_flux_density, area, angle_radians]) || area < 0.0 {
        return None;
    }

    finite_result(magnetic_flux_density * area * angle_radians.cos())
}

/// Computes magnetic flux through an area using an angle in degrees.
#[must_use]
pub fn magnetic_flux_degrees(
    magnetic_flux_density: f64,
    area: f64,
    angle_degrees: f64,
) -> Option<f64> {
    magnetic_flux(magnetic_flux_density, area, angle_degrees.to_radians())
}

/// Computes magnetic flux density from magnetic flux, area, and orientation.
///
/// Formula: `B = Φ / (A * cos(theta))`
///
/// Returns `None` when `area` is less than or equal to zero, when
/// `cos(theta)` is zero or effectively zero, when any input is not finite, or
/// when the computed result is not finite.
#[must_use]
pub fn magnetic_flux_density_from_flux(flux: f64, area: f64, angle_radians: f64) -> Option<f64> {
    if !all_finite(&[flux, area, angle_radians]) || area <= 0.0 {
        return None;
    }

    let angle_factor = angle_radians.cos();
    if !angle_factor.is_finite() || angle_factor.abs() <= TRIG_EPSILON {
        return None;
    }

    finite_result(flux / (area * angle_factor))
}

/// Computes magnetic force on a moving charge.
///
/// Formula: `F = q * v * B * sin(theta)`
///
/// The sign is preserved from the scalar inputs and angle convention.
///
/// # Examples
///
/// ```
/// use std::f64::consts::FRAC_PI_2;
///
/// use use_magnetism::magnetic_force_on_charge;
///
/// assert_eq!(magnetic_force_on_charge(1.0, 2.0, 3.0, FRAC_PI_2), Some(6.0));
/// ```
#[must_use]
pub fn magnetic_force_on_charge(
    charge: f64,
    velocity: f64,
    magnetic_flux_density: f64,
    angle_radians: f64,
) -> Option<f64> {
    if !all_finite(&[charge, velocity, magnetic_flux_density, angle_radians]) {
        return None;
    }

    finite_result(charge * velocity * magnetic_flux_density * angle_radians.sin())
}

/// Computes magnetic force on a moving charge using an angle in degrees.
#[must_use]
pub fn magnetic_force_on_charge_degrees(
    charge: f64,
    velocity: f64,
    magnetic_flux_density: f64,
    angle_degrees: f64,
) -> Option<f64> {
    magnetic_force_on_charge(
        charge,
        velocity,
        magnetic_flux_density,
        angle_degrees.to_radians(),
    )
}

/// Computes the magnitude of magnetic force on a moving charge.
///
/// Formula: `|F| = |q| * speed * |B| * sin(theta)`
#[must_use]
pub fn magnetic_force_magnitude_on_charge(
    charge: f64,
    speed: f64,
    magnetic_flux_density: f64,
    angle_radians: f64,
) -> Option<f64> {
    if !all_finite(&[charge, speed, magnetic_flux_density, angle_radians]) || speed < 0.0 {
        return None;
    }

    finite_result(charge.abs() * speed * magnetic_flux_density.abs() * angle_radians.sin().abs())
}

/// Computes magnetic force on a current-carrying wire.
///
/// Formula: `F = I * L * B * sin(theta)`
///
/// The sign is preserved from the scalar inputs and angle convention.
///
/// # Examples
///
/// ```
/// use std::f64::consts::FRAC_PI_2;
///
/// use use_magnetism::magnetic_force_on_wire;
///
/// assert_eq!(magnetic_force_on_wire(2.0, 3.0, 4.0, FRAC_PI_2), Some(24.0));
/// ```
#[must_use]
pub fn magnetic_force_on_wire(
    current: f64,
    length: f64,
    magnetic_flux_density: f64,
    angle_radians: f64,
) -> Option<f64> {
    if !all_finite(&[current, length, magnetic_flux_density, angle_radians]) || length < 0.0 {
        return None;
    }

    finite_result(current * length * magnetic_flux_density * angle_radians.sin())
}

/// Computes magnetic force on a current-carrying wire using an angle in degrees.
#[must_use]
pub fn magnetic_force_on_wire_degrees(
    current: f64,
    length: f64,
    magnetic_flux_density: f64,
    angle_degrees: f64,
) -> Option<f64> {
    magnetic_force_on_wire(
        current,
        length,
        magnetic_flux_density,
        angle_degrees.to_radians(),
    )
}

/// Computes magnetic flux density around a long straight wire.
///
/// Formula: `B = μ0 * I / (2πr)`
///
/// # Examples
///
/// ```
/// use use_magnetism::magnetic_field_around_long_straight_wire;
///
/// let field = magnetic_field_around_long_straight_wire(10.0, 0.5).unwrap();
/// assert!(field.is_sign_positive());
/// ```
#[must_use]
pub fn magnetic_field_around_long_straight_wire(current: f64, distance: f64) -> Option<f64> {
    if !all_finite(&[current, distance]) || distance <= 0.0 {
        return None;
    }

    finite_result(VACUUM_PERMEABILITY * current / (TAU * distance))
}

/// Computes magnetic flux density inside an ideal long solenoid.
///
/// Formula: `B = μ0 * (N / L) * I`
///
/// # Examples
///
/// ```
/// use use_magnetism::magnetic_field_inside_solenoid;
///
/// let field = magnetic_field_inside_solenoid(1_000.0, 2.0, 0.5).unwrap();
/// assert!(field.is_sign_positive());
/// ```
#[must_use]
pub fn magnetic_field_inside_solenoid(turns: f64, current: f64, length: f64) -> Option<f64> {
    if !all_finite(&[turns, current, length]) || turns < 0.0 || length <= 0.0 {
        return None;
    }

    finite_result(VACUUM_PERMEABILITY * (turns / length) * current)
}

/// Computes magnetic flux density at the center of a circular current loop.
///
/// Formula: `B = μ0 * I / (2r)`
#[must_use]
pub fn magnetic_field_at_center_of_loop(current: f64, radius: f64) -> Option<f64> {
    if !all_finite(&[current, radius]) || radius <= 0.0 {
        return None;
    }

    finite_result(VACUUM_PERMEABILITY * current / (2.0 * radius))
}

/// Computes magnetic energy density.
///
/// Formula: `u = B² / (2μ0)`
///
/// # Examples
///
/// ```
/// use use_magnetism::magnetic_energy_density;
///
/// let energy_density = magnetic_energy_density(2.0).unwrap();
/// assert!(energy_density.is_sign_positive());
/// ```
#[must_use]
pub fn magnetic_energy_density(magnetic_flux_density: f64) -> Option<f64> {
    if !magnetic_flux_density.is_finite() {
        return None;
    }

    finite_result((magnetic_flux_density * magnetic_flux_density) / (2.0 * VACUUM_PERMEABILITY))
}

/// Computes magnetic pressure.
///
/// Magnetic pressure and magnetic energy density share the same numeric
/// expression in SI units.
#[must_use]
pub fn magnetic_pressure(magnetic_flux_density: f64) -> Option<f64> {
    magnetic_energy_density(magnetic_flux_density)
}

/// A simple magnetic field described by flux density.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagneticField {
    pub flux_density: f64,
}

impl MagneticField {
    /// Creates a new magnetic field from flux density.
    #[must_use]
    pub fn new(flux_density: f64) -> Option<Self> {
        flux_density.is_finite().then_some(Self { flux_density })
    }

    /// Computes the magnetic flux through an area in this field.
    #[must_use]
    pub fn flux_through_area(&self, area: f64, angle_radians: f64) -> Option<f64> {
        magnetic_flux(self.flux_density, area, angle_radians)
    }

    /// Computes the magnetic force on a moving charge in this field.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::consts::FRAC_PI_2;
    ///
    /// use use_magnetism::MagneticField;
    ///
    /// let field = MagneticField::new(3.0).unwrap();
    /// assert_eq!(field.force_on_charge(1.0, 2.0, FRAC_PI_2), Some(6.0));
    /// ```
    #[must_use]
    pub fn force_on_charge(&self, charge: f64, velocity: f64, angle_radians: f64) -> Option<f64> {
        magnetic_force_on_charge(charge, velocity, self.flux_density, angle_radians)
    }

    /// Computes the magnetic force on a current-carrying wire in this field.
    #[must_use]
    pub fn force_on_wire(&self, current: f64, length: f64, angle_radians: f64) -> Option<f64> {
        magnetic_force_on_wire(current, length, self.flux_density, angle_radians)
    }

    /// Computes the magnetic energy density for this field.
    #[must_use]
    pub fn energy_density(&self) -> Option<f64> {
        magnetic_energy_density(self.flux_density)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use core::f64::consts::FRAC_PI_2;

    use super::{
        MagneticField, magnetic_energy_density, magnetic_field_around_long_straight_wire,
        magnetic_field_at_center_of_loop, magnetic_field_inside_solenoid, magnetic_flux,
        magnetic_flux_degrees, magnetic_flux_density_from_flux, magnetic_force_magnitude_on_charge,
        magnetic_force_on_charge, magnetic_force_on_charge_degrees, magnetic_force_on_wire,
        magnetic_force_on_wire_degrees, magnetic_pressure,
    };

    const EPSILON: f64 = 1.0e-12;

    fn assert_close(actual: f64, expected: f64) {
        let scale = actual.abs().max(expected.abs()).max(1.0);
        let delta = (actual - expected).abs();

        assert!(
            delta <= EPSILON * scale,
            "actual={actual} expected={expected} delta={delta} tolerance={}",
            EPSILON * scale
        );
    }

    fn assert_some_close(actual: Option<f64>, expected: f64) {
        assert_close(actual.expect("expected Some value"), expected);
    }

    #[test]
    fn magnetic_flux_handles_requested_cases() {
        assert_eq!(magnetic_flux(2.0, 3.0, 0.0), Some(6.0));
        assert_some_close(magnetic_flux(2.0, 3.0, FRAC_PI_2), 0.0);
        assert_eq!(magnetic_flux(2.0, -3.0, 0.0), None);
        assert_some_close(magnetic_flux_degrees(2.0, 3.0, 60.0), 3.0);
    }

    #[test]
    fn magnetic_flux_density_requires_valid_geometry() {
        assert_eq!(magnetic_flux_density_from_flux(6.0, 3.0, 0.0), Some(2.0));
        assert_eq!(magnetic_flux_density_from_flux(6.0, 0.0, 0.0), None);
        assert_eq!(magnetic_flux_density_from_flux(6.0, 3.0, FRAC_PI_2), None);
    }

    #[test]
    fn magnetic_force_on_charge_handles_sign_and_units() {
        assert_some_close(magnetic_force_on_charge(1.0, 2.0, 3.0, FRAC_PI_2), 6.0);
        assert_some_close(magnetic_force_on_charge(-1.0, 2.0, 3.0, FRAC_PI_2), -6.0);
        assert_some_close(magnetic_force_on_charge_degrees(1.0, 2.0, 3.0, 90.0), 6.0);
    }

    #[test]
    fn magnetic_force_magnitude_requires_non_negative_speed() {
        assert_some_close(
            magnetic_force_magnitude_on_charge(-1.0, 2.0, -3.0, FRAC_PI_2),
            6.0,
        );
        assert_eq!(
            magnetic_force_magnitude_on_charge(1.0, -2.0, 3.0, FRAC_PI_2),
            None
        );
    }

    #[test]
    fn magnetic_force_on_wire_handles_requested_cases() {
        assert_some_close(magnetic_force_on_wire(2.0, 3.0, 4.0, FRAC_PI_2), 24.0);
        assert_some_close(magnetic_force_on_wire_degrees(2.0, 3.0, 4.0, 90.0), 24.0);
        assert_eq!(magnetic_force_on_wire(2.0, -3.0, 4.0, FRAC_PI_2), None);
    }

    #[test]
    fn magnetic_field_helpers_require_positive_lengths() {
        let wire_field = magnetic_field_around_long_straight_wire(10.0, 0.5)
            .expect("expected finite wire field");
        assert!(wire_field.is_finite());
        assert!(wire_field > 0.0);
        assert_eq!(magnetic_field_around_long_straight_wire(10.0, 0.0), None);

        let solenoid_field =
            magnetic_field_inside_solenoid(1_000.0, 2.0, 0.5).expect("expected finite solenoid");
        assert!(solenoid_field.is_finite());
        assert!(solenoid_field > 0.0);
        assert_eq!(magnetic_field_inside_solenoid(-1_000.0, 2.0, 0.5), None);
        assert_eq!(magnetic_field_inside_solenoid(1_000.0, 2.0, 0.0), None);

        let loop_field =
            magnetic_field_at_center_of_loop(10.0, 0.5).expect("expected finite loop field");
        assert!(loop_field.is_finite());
        assert!(loop_field > 0.0);
        assert_eq!(magnetic_field_at_center_of_loop(10.0, 0.0), None);
    }

    #[test]
    fn magnetic_pressure_matches_energy_density() {
        let energy_density = magnetic_energy_density(2.0).expect("expected finite energy density");
        assert!(energy_density.is_finite());
        assert!(energy_density > 0.0);
        assert_eq!(magnetic_pressure(2.0), magnetic_energy_density(2.0));
    }

    #[test]
    fn magnetic_field_struct_delegates_to_free_functions() {
        let field = MagneticField::new(3.0).expect("valid field");

        assert_eq!(field.flux_through_area(2.0, 0.0), Some(6.0));
        assert_some_close(field.force_on_charge(1.0, 2.0, FRAC_PI_2), 6.0);
        assert_eq!(MagneticField::new(f64::NAN), None);
    }
}
