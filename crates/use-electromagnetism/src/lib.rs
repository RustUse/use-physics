#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small scalar helpers for combined electric and magnetic field relations.

use core::f64::consts::TAU;

pub mod prelude;

/// Vacuum permittivity in farads per meter.
///
/// This crate keeps the value locally as a convenience for scalar electromagnetic helpers.
/// Broader physical constants belong in the top-level `use-constants` set.
pub const VACUUM_PERMITTIVITY: f64 = 8.854_187_812_8e-12;

/// Vacuum permeability in henries per meter.
///
/// This crate keeps the value locally as a convenience for scalar electromagnetic helpers.
/// Broader physical constants belong in the top-level `use-constants` set.
pub const VACUUM_PERMEABILITY: f64 = 1.256_637_062_12e-6;

/// Speed of light in vacuum, in meters per second.
///
/// This crate keeps the value locally as a convenience for scalar electromagnetic helpers.
/// Broader physical constants belong in the top-level `use-constants` set.
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

fn is_nonnegative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn nonnegative_finite_result(value: f64) -> Option<f64> {
    (value.is_finite() && value >= 0.0).then_some(value)
}

/// A scalar electric and magnetic field pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElectromagneticField {
    pub electric_field: f64,
    pub magnetic_flux_density: f64,
}

impl ElectromagneticField {
    /// Creates a field pair when both scalar components are finite.
    #[must_use]
    pub const fn new(electric_field: f64, magnetic_flux_density: f64) -> Option<Self> {
        if !electric_field.is_finite() || !magnetic_flux_density.is_finite() {
            return None;
        }

        Some(Self {
            electric_field,
            magnetic_flux_density,
        })
    }

    /// Computes electric force using the field's electric component.
    #[must_use]
    pub fn electric_force_on_charge(&self, charge: f64) -> Option<f64> {
        electric_force_on_charge(charge, self.electric_field)
    }

    /// Computes the scalar Lorentz-force convenience relation for this field pair.
    #[must_use]
    pub fn lorentz_force_scalar(
        &self,
        charge: f64,
        velocity: f64,
        angle_radians: f64,
    ) -> Option<f64> {
        lorentz_force_scalar(
            charge,
            self.electric_field,
            velocity,
            self.magnetic_flux_density,
            angle_radians,
        )
    }

    /// Computes combined electromagnetic energy density for this field pair.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_electromagnetism::ElectromagneticField;
    ///
    /// let field = ElectromagneticField::new(10.0, 2.0).unwrap();
    ///
    /// assert!(field.energy_density().unwrap() > 0.0);
    /// ```
    #[must_use]
    pub fn energy_density(&self) -> Option<f64> {
        electromagnetic_energy_density(self.electric_field, self.magnetic_flux_density)
    }

    /// Computes Poynting magnitude when the stored field values are used as magnitudes.
    #[must_use]
    pub fn poynting_magnitude(&self) -> Option<f64> {
        poynting_magnitude(self.electric_field, self.magnetic_flux_density)
    }
}

/// Computes electric force using `F = qE`.
///
/// Returns `None` when either input is not finite or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_electromagnetism::electric_force_on_charge;
///
/// assert_eq!(electric_force_on_charge(2.0, 3.0), Some(6.0));
/// assert_eq!(electric_force_on_charge(-2.0, 3.0), Some(-6.0));
/// ```
#[must_use]
pub fn electric_force_on_charge(charge: f64, electric_field: f64) -> Option<f64> {
    if !charge.is_finite() || !electric_field.is_finite() {
        return None;
    }

    finite_result(charge * electric_field)
}

/// Computes magnetic force using `F = qvB sin(theta)`.
///
/// Returns `None` when any input is not finite or when the computed result is not finite.
#[must_use]
pub fn magnetic_force_on_moving_charge(
    charge: f64,
    velocity: f64,
    magnetic_flux_density: f64,
    angle_radians: f64,
) -> Option<f64> {
    if !charge.is_finite()
        || !velocity.is_finite()
        || !magnetic_flux_density.is_finite()
        || !angle_radians.is_finite()
    {
        return None;
    }

    finite_result(charge * velocity * magnetic_flux_density * angle_radians.sin())
}

/// Computes magnetic force using `F = qvB sin(theta)` with the angle in degrees.
#[must_use]
pub fn magnetic_force_on_moving_charge_degrees(
    charge: f64,
    velocity: f64,
    magnetic_flux_density: f64,
    angle_degrees: f64,
) -> Option<f64> {
    magnetic_force_on_moving_charge(
        charge,
        velocity,
        magnetic_flux_density,
        angle_degrees.to_radians(),
    )
}

/// Computes the scalar Lorentz-force convenience relation `F = q(E + vB sin(theta))`.
///
/// This helper is scalar-only and does not model the full vector Lorentz force.
///
/// Returns `None` when any input is not finite or when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_electromagnetism::lorentz_force_scalar;
///
/// assert_eq!(
///     lorentz_force_scalar(1.0, 10.0, 2.0, 3.0, core::f64::consts::FRAC_PI_2),
///     Some(16.0)
/// );
/// ```
#[must_use]
pub fn lorentz_force_scalar(
    charge: f64,
    electric_field: f64,
    velocity: f64,
    magnetic_flux_density: f64,
    angle_radians: f64,
) -> Option<f64> {
    if !charge.is_finite()
        || !electric_field.is_finite()
        || !velocity.is_finite()
        || !magnetic_flux_density.is_finite()
        || !angle_radians.is_finite()
    {
        return None;
    }

    let magnetic_term = velocity * magnetic_flux_density * angle_radians.sin();
    finite_result(charge * (electric_field + magnetic_term))
}

/// Computes the scalar Lorentz-force convenience relation with the angle in degrees.
#[must_use]
pub fn lorentz_force_scalar_degrees(
    charge: f64,
    electric_field: f64,
    velocity: f64,
    magnetic_flux_density: f64,
    angle_degrees: f64,
) -> Option<f64> {
    lorentz_force_scalar(
        charge,
        electric_field,
        velocity,
        magnetic_flux_density,
        angle_degrees.to_radians(),
    )
}

/// Computes `|F| = |q| * |E + vB|` for perpendicular fields along the same scalar direction.
///
/// Returns `None` when `speed` is negative, when either field magnitude is negative, when any
/// input is not finite, or when the computed result is not finite.
#[must_use]
pub fn lorentz_force_magnitude_perpendicular(
    charge: f64,
    electric_field_magnitude: f64,
    speed: f64,
    magnetic_flux_density_magnitude: f64,
) -> Option<f64> {
    if !charge.is_finite()
        || !is_nonnegative_finite(electric_field_magnitude)
        || !is_nonnegative_finite(speed)
        || !is_nonnegative_finite(magnetic_flux_density_magnitude)
    {
        return None;
    }

    let combined_term = speed.mul_add(magnetic_flux_density_magnitude, electric_field_magnitude);
    nonnegative_finite_result(charge.abs() * combined_term.abs())
}

/// Computes selector speed using `v = E / B`.
///
/// Inputs are treated as magnitudes.
///
/// # Examples
///
/// ```rust
/// use use_electromagnetism::velocity_selector_speed;
///
/// assert_eq!(velocity_selector_speed(20.0, 4.0), Some(5.0));
/// assert_eq!(velocity_selector_speed(20.0, 0.0), None);
/// ```
#[must_use]
pub fn velocity_selector_speed(electric_field: f64, magnetic_flux_density: f64) -> Option<f64> {
    if !is_nonnegative_finite(electric_field) || !is_positive_finite(magnetic_flux_density) {
        return None;
    }

    nonnegative_finite_result(electric_field / magnetic_flux_density)
}

/// Computes electric field magnitude for a selector using `E = vB`.
#[must_use]
pub fn electric_field_for_velocity_selector(speed: f64, magnetic_flux_density: f64) -> Option<f64> {
    if !is_nonnegative_finite(speed) || !is_nonnegative_finite(magnetic_flux_density) {
        return None;
    }

    nonnegative_finite_result(speed * magnetic_flux_density)
}

/// Computes magnetic flux density magnitude for a selector using `B = E / v`.
#[must_use]
pub fn magnetic_flux_density_for_velocity_selector(electric_field: f64, speed: f64) -> Option<f64> {
    if !is_nonnegative_finite(electric_field) || !is_positive_finite(speed) {
        return None;
    }

    nonnegative_finite_result(electric_field / speed)
}

/// Computes cyclotron radius using `r = mv / (|q|B)`.
///
/// # Examples
///
/// ```rust
/// use use_electromagnetism::cyclotron_radius;
///
/// assert_eq!(cyclotron_radius(2.0, 10.0, 1.0, 5.0), Some(4.0));
/// ```
#[must_use]
pub fn cyclotron_radius(
    mass: f64,
    speed: f64,
    charge: f64,
    magnetic_flux_density: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(mass)
        || !is_nonnegative_finite(speed)
        || !charge.is_finite()
        || charge == 0.0
        || !is_positive_finite(magnetic_flux_density)
    {
        return None;
    }

    nonnegative_finite_result(mass * speed / (charge.abs() * magnetic_flux_density))
}

/// Computes cyclotron angular frequency using `ω = |q|B / m`.
#[must_use]
pub fn cyclotron_angular_frequency(
    charge: f64,
    magnetic_flux_density: f64,
    mass: f64,
) -> Option<f64> {
    if !charge.is_finite()
        || charge == 0.0
        || !is_nonnegative_finite(magnetic_flux_density)
        || !is_positive_finite(mass)
    {
        return None;
    }

    nonnegative_finite_result(charge.abs() * magnetic_flux_density / mass)
}

/// Computes cyclotron frequency in cycles per second using `f = |q|B / (2πm)`.
#[must_use]
pub fn cyclotron_frequency(charge: f64, magnetic_flux_density: f64, mass: f64) -> Option<f64> {
    nonnegative_finite_result(
        cyclotron_angular_frequency(charge, magnetic_flux_density, mass)? / TAU,
    )
}

/// Computes electric field energy density using `u_E = 0.5 * ε0 * E²`.
#[must_use]
pub fn electric_field_energy_density(electric_field: f64) -> Option<f64> {
    if !electric_field.is_finite() {
        return None;
    }

    nonnegative_finite_result(0.5 * VACUUM_PERMITTIVITY * electric_field * electric_field)
}

/// Computes magnetic field energy density using `u_B = B² / (2μ0)`.
#[must_use]
pub fn magnetic_field_energy_density(magnetic_flux_density: f64) -> Option<f64> {
    if !magnetic_flux_density.is_finite() {
        return None;
    }

    nonnegative_finite_result(
        magnetic_flux_density * magnetic_flux_density / (2.0 * VACUUM_PERMEABILITY),
    )
}

/// Computes combined electromagnetic energy density.
///
/// # Examples
///
/// ```rust
/// use use_electromagnetism::electromagnetic_energy_density;
///
/// assert!(electromagnetic_energy_density(10.0, 2.0).unwrap() > 0.0);
/// ```
#[must_use]
pub fn electromagnetic_energy_density(
    electric_field: f64,
    magnetic_flux_density: f64,
) -> Option<f64> {
    let electric_density = electric_field_energy_density(electric_field)?;
    let magnetic_density = magnetic_field_energy_density(magnetic_flux_density)?;

    nonnegative_finite_result(electric_density + magnetic_density)
}

/// Computes Poynting magnitude in vacuum using `S = EB / μ0`.
///
/// Inputs are treated as magnitudes.
///
/// # Examples
///
/// ```rust
/// use use_electromagnetism::poynting_magnitude;
///
/// assert!(poynting_magnitude(10.0, 2.0).unwrap() > 0.0);
/// assert_eq!(poynting_magnitude(-10.0, 2.0), None);
/// ```
#[must_use]
pub fn poynting_magnitude(electric_field: f64, magnetic_flux_density: f64) -> Option<f64> {
    if !is_nonnegative_finite(electric_field) || !is_nonnegative_finite(magnetic_flux_density) {
        return None;
    }

    nonnegative_finite_result(electric_field * magnetic_flux_density / VACUUM_PERMEABILITY)
}

/// Computes magnetic flux density magnitude in vacuum using `B = E / c`.
///
/// # Examples
///
/// ```rust
/// use use_electromagnetism::{SPEED_OF_LIGHT, magnetic_flux_density_from_electric_field_in_vacuum};
///
/// assert_eq!(
///     magnetic_flux_density_from_electric_field_in_vacuum(SPEED_OF_LIGHT),
///     Some(1.0)
/// );
/// ```
#[must_use]
pub fn magnetic_flux_density_from_electric_field_in_vacuum(electric_field: f64) -> Option<f64> {
    if !is_nonnegative_finite(electric_field) {
        return None;
    }

    nonnegative_finite_result(electric_field / SPEED_OF_LIGHT)
}

/// Computes electric field magnitude in vacuum using `E = cB`.
#[must_use]
pub fn electric_field_from_magnetic_flux_density_in_vacuum(
    magnetic_flux_density: f64,
) -> Option<f64> {
    if !is_nonnegative_finite(magnetic_flux_density) {
        return None;
    }

    nonnegative_finite_result(SPEED_OF_LIGHT * magnetic_flux_density)
}

/// Computes propagation speed from permittivity and permeability using `v = 1 / sqrt(εμ)`.
#[must_use]
pub fn speed_from_permittivity_permeability(permittivity: f64, permeability: f64) -> Option<f64> {
    if !is_positive_finite(permittivity) || !is_positive_finite(permeability) {
        return None;
    }

    let product = permittivity * permeability;
    if !is_positive_finite(product) {
        return None;
    }

    nonnegative_finite_result(product.sqrt().recip())
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    fn approx_eq(left: f64, right: f64) -> bool {
        let scale = left.abs().max(right.abs()).max(1.0);
        (left - right).abs() <= 1.0e-9 * scale
    }

    #[test]
    fn electric_force_helpers_cover_sign() {
        assert_eq!(electric_force_on_charge(2.0, 3.0), Some(6.0));
        assert_eq!(electric_force_on_charge(-2.0, 3.0), Some(-6.0));
    }

    #[test]
    fn magnetic_force_helpers_cover_radians_and_degrees() {
        let radians_force =
            magnetic_force_on_moving_charge(1.0, 2.0, 3.0, core::f64::consts::FRAC_PI_2).unwrap();
        let degrees_force = magnetic_force_on_moving_charge_degrees(1.0, 2.0, 3.0, 90.0).unwrap();

        assert!(approx_eq(radians_force, 6.0));
        assert!(approx_eq(degrees_force, 6.0));
    }

    #[test]
    fn lorentz_force_helpers_cover_sign_and_magnitude() {
        let positive_force =
            lorentz_force_scalar(1.0, 10.0, 2.0, 3.0, core::f64::consts::FRAC_PI_2).unwrap();
        let degrees_force = lorentz_force_scalar_degrees(1.0, 10.0, 2.0, 3.0, 90.0).unwrap();
        let negative_force =
            lorentz_force_scalar(-1.0, 10.0, 2.0, 3.0, core::f64::consts::FRAC_PI_2).unwrap();

        assert!(approx_eq(positive_force, 16.0));
        assert!(approx_eq(degrees_force, 16.0));
        assert!(approx_eq(negative_force, -16.0));
        assert_eq!(
            lorentz_force_magnitude_perpendicular(1.0, 10.0, 2.0, 3.0),
            Some(16.0)
        );
        assert_eq!(
            lorentz_force_magnitude_perpendicular(1.0, -10.0, 2.0, 3.0),
            None
        );
        assert_eq!(
            lorentz_force_magnitude_perpendicular(1.0, 10.0, -2.0, 3.0),
            None
        );
    }

    #[test]
    fn velocity_selector_helpers_cover_common_relations() {
        assert_eq!(velocity_selector_speed(20.0, 4.0), Some(5.0));
        assert_eq!(velocity_selector_speed(20.0, 0.0), None);
        assert_eq!(electric_field_for_velocity_selector(5.0, 4.0), Some(20.0));
        assert_eq!(electric_field_for_velocity_selector(-5.0, 4.0), None);
        assert_eq!(
            magnetic_flux_density_for_velocity_selector(20.0, 5.0),
            Some(4.0)
        );
        assert_eq!(magnetic_flux_density_for_velocity_selector(20.0, 0.0), None);
    }

    #[test]
    fn cyclotron_helpers_cover_radius_and_frequency() {
        assert_eq!(cyclotron_radius(2.0, 10.0, 1.0, 5.0), Some(4.0));
        assert_eq!(cyclotron_radius(2.0, 10.0, 0.0, 5.0), None);
        assert_eq!(cyclotron_radius(2.0, 10.0, 1.0, 0.0), None);
        assert_eq!(cyclotron_angular_frequency(2.0, 5.0, 10.0), Some(1.0));

        let frequency = cyclotron_frequency(2.0, 5.0, 10.0).unwrap();
        assert!(approx_eq(frequency, 1.0 / (2.0 * core::f64::consts::PI)));
    }

    #[test]
    fn energy_density_helpers_return_positive_results() {
        let electric_density = electric_field_energy_density(10.0).unwrap();
        let magnetic_density = magnetic_field_energy_density(2.0).unwrap();
        let combined_density = electromagnetic_energy_density(10.0, 2.0).unwrap();

        assert!(electric_density.is_finite() && electric_density > 0.0);
        assert!(magnetic_density.is_finite() && magnetic_density > 0.0);
        assert!(combined_density.is_finite() && combined_density > 0.0);
    }

    #[test]
    fn poynting_magnitude_requires_nonnegative_inputs() {
        let poynting = poynting_magnitude(10.0, 2.0).unwrap();

        assert!(poynting.is_finite() && poynting > 0.0);
        assert_eq!(poynting_magnitude(-10.0, 2.0), None);
    }

    #[test]
    fn plane_wave_and_speed_relations_cover_vacuum_helpers() {
        let magnetic_flux_density =
            magnetic_flux_density_from_electric_field_in_vacuum(SPEED_OF_LIGHT).unwrap();
        let electric_field = electric_field_from_magnetic_flux_density_in_vacuum(1.0).unwrap();
        let speed =
            speed_from_permittivity_permeability(VACUUM_PERMITTIVITY, VACUUM_PERMEABILITY).unwrap();

        assert!(approx_eq(magnetic_flux_density, 1.0));
        assert!(approx_eq(electric_field, SPEED_OF_LIGHT));
        assert!(approx_eq(speed, SPEED_OF_LIGHT));
        assert_eq!(
            speed_from_permittivity_permeability(0.0, VACUUM_PERMEABILITY),
            None
        );
    }

    #[test]
    fn electromagnetic_field_methods_delegate_to_free_functions() {
        let field = ElectromagneticField::new(10.0, 2.0).unwrap();
        let lorentz_force = field
            .lorentz_force_scalar(1.0, 2.0, core::f64::consts::FRAC_PI_2)
            .unwrap();
        let energy_density = field.energy_density().unwrap();

        assert_eq!(field.electric_force_on_charge(3.0), Some(30.0));
        assert!(approx_eq(lorentz_force, 14.0));
        assert!(energy_density.is_finite() && energy_density > 0.0);
        assert_eq!(ElectromagneticField::new(f64::NAN, 2.0), None);
    }
}
