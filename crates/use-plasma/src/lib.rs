#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small scalar plasma physics helpers.

use core::f64::consts::{PI, TAU};

pub mod prelude;

/// Elementary charge in coulombs.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19;

/// Electron mass in kilograms.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const ELECTRON_MASS: f64 = 9.109_383_701_5e-31;

/// Proton mass in kilograms.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const PROTON_MASS: f64 = 1.672_621_923_69e-27;

/// Vacuum permittivity in farads per meter.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const VACUUM_PERMITTIVITY: f64 = 8.854_187_812_8e-12;

/// Vacuum permeability in henries per meter.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const VACUUM_PERMEABILITY: f64 = 1.256_637_062_12e-6;

/// Boltzmann constant in joules per kelvin.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const BOLTZMANN_CONSTANT: f64 = 1.380_649e-23;

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

fn finite_result(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

/// A simple scalar plasma species description.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlasmaSpecies {
    /// Number density in particles per cubic meter.
    pub number_density: f64,
    /// Temperature in kelvin.
    pub temperature_kelvin: f64,
    /// Signed charge state in elementary-charge units.
    pub charge_state: f64,
    /// Particle mass in kilograms.
    pub mass: f64,
}

impl PlasmaSpecies {
    /// Creates a plasma species when the inputs are finite and physically valid.
    #[must_use]
    pub fn new(
        number_density: f64,
        temperature_kelvin: f64,
        charge_state: f64,
        mass: f64,
    ) -> Option<Self> {
        if !all_finite(&[number_density, temperature_kelvin, charge_state, mass])
            || number_density < 0.0
            || temperature_kelvin < 0.0
            || mass <= 0.0
        {
            return None;
        }

        Some(Self {
            number_density,
            temperature_kelvin,
            charge_state,
            mass,
        })
    }

    /// Computes scalar species pressure using `p = n k_B T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_plasma::{PROTON_MASS, PlasmaSpecies};
    ///
    /// let species = PlasmaSpecies::new(1.0e18, 10_000.0, 1.0, PROTON_MASS);
    ///
    /// assert!(species.and_then(|value| value.pressure()).is_some_and(|value| value > 0.0));
    /// ```
    #[must_use]
    pub fn pressure(&self) -> Option<f64> {
        plasma_pressure(self.number_density, self.temperature_kelvin)
    }

    /// Computes the species thermal speed using `v_th = sqrt(k_B T / m)`.
    #[must_use]
    pub fn thermal_speed(&self) -> Option<f64> {
        particle_thermal_speed(self.temperature_kelvin, self.mass)
    }
}

/// A simple electron-plasma state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElectronPlasma {
    /// Electron number density in particles per cubic meter.
    pub electron_number_density: f64,
    /// Electron temperature in kelvin.
    pub electron_temperature_kelvin: f64,
}

impl ElectronPlasma {
    /// Creates an electron-plasma state when both values are finite and non-negative.
    #[must_use]
    pub fn new(electron_number_density: f64, electron_temperature_kelvin: f64) -> Option<Self> {
        if !all_finite(&[electron_number_density, electron_temperature_kelvin])
            || electron_number_density < 0.0
            || electron_temperature_kelvin < 0.0
        {
            return None;
        }

        Some(Self {
            electron_number_density,
            electron_temperature_kelvin,
        })
    }

    /// Computes the electron plasma angular frequency.
    #[must_use]
    pub fn plasma_angular_frequency(&self) -> Option<f64> {
        electron_plasma_angular_frequency(self.electron_number_density)
    }

    /// Computes the electron plasma frequency.
    #[must_use]
    pub fn plasma_frequency(&self) -> Option<f64> {
        electron_plasma_frequency(self.electron_number_density)
    }

    /// Computes the Debye length for this plasma state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_plasma::ElectronPlasma;
    ///
    /// let plasma = ElectronPlasma::new(1.0e18, 10_000.0);
    ///
    /// assert!(plasma.and_then(|value| value.debye_length()).is_some_and(|value| value > 0.0));
    /// ```
    #[must_use]
    pub fn debye_length(&self) -> Option<f64> {
        debye_length(
            self.electron_temperature_kelvin,
            self.electron_number_density,
        )
    }

    /// Computes the Debye number for this plasma state.
    #[must_use]
    pub fn debye_number(&self) -> Option<f64> {
        debye_number(self.electron_number_density, self.debye_length()?)
    }

    /// Computes the electron thermal speed.
    #[must_use]
    pub fn thermal_speed(&self) -> Option<f64> {
        electron_thermal_speed(self.electron_temperature_kelvin)
    }

    /// Computes the scalar electron pressure using `p = n k_B T`.
    #[must_use]
    pub fn pressure(&self) -> Option<f64> {
        plasma_pressure(
            self.electron_number_density,
            self.electron_temperature_kelvin,
        )
    }
}

/// Computes the electron plasma angular frequency.
///
/// Formula: `ω_pe = sqrt(n_e e^2 / (ε0 m_e))`
///
/// Returns `None` when `electron_number_density` is negative, when the input is not finite, or
/// when the computed result is not finite.
///
/// # Examples
///
/// ```rust
/// use use_plasma::electron_plasma_frequency;
///
/// assert!(electron_plasma_frequency(1.0e18).is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn electron_plasma_angular_frequency(electron_number_density: f64) -> Option<f64> {
    if !electron_number_density.is_finite() || electron_number_density < 0.0 {
        return None;
    }

    let numerator = electron_number_density * ELEMENTARY_CHARGE * ELEMENTARY_CHARGE;
    let denominator = VACUUM_PERMITTIVITY * ELECTRON_MASS;

    finite_result((numerator / denominator).sqrt())
}

/// Computes the electron plasma frequency in hertz.
///
/// Formula: `f_pe = ω_pe / (2π)`
///
/// # Examples
///
/// ```rust
/// use use_plasma::electron_plasma_frequency;
///
/// assert!(electron_plasma_frequency(1.0e18).is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn electron_plasma_frequency(electron_number_density: f64) -> Option<f64> {
    finite_result(electron_plasma_angular_frequency(electron_number_density)? / TAU)
}

/// Computes the ion plasma angular frequency.
///
/// Formula: `ω_pi = sqrt(n_i (Z e)^2 / (ε0 m_i))`
#[must_use]
pub fn ion_plasma_angular_frequency(
    ion_number_density: f64,
    ion_charge_state: f64,
    ion_mass: f64,
) -> Option<f64> {
    if !all_finite(&[ion_number_density, ion_charge_state, ion_mass])
        || ion_number_density < 0.0
        || ion_charge_state < 0.0
        || ion_mass <= 0.0
    {
        return None;
    }

    let ion_charge = ion_charge_state * ELEMENTARY_CHARGE;
    let numerator = ion_number_density * ion_charge * ion_charge;
    let denominator = VACUUM_PERMITTIVITY * ion_mass;

    finite_result((numerator / denominator).sqrt())
}

/// Computes the Debye length.
///
/// Formula: `λ_D = sqrt(ε0 k_B T_e / (n_e e^2))`
///
/// # Examples
///
/// ```rust
/// use use_plasma::debye_length;
///
/// assert!(debye_length(10_000.0, 1.0e18).is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn debye_length(electron_temperature_kelvin: f64, electron_number_density: f64) -> Option<f64> {
    if !all_finite(&[electron_temperature_kelvin, electron_number_density])
        || electron_temperature_kelvin < 0.0
        || electron_number_density <= 0.0
    {
        return None;
    }

    let numerator = VACUUM_PERMITTIVITY * BOLTZMANN_CONSTANT * electron_temperature_kelvin;
    let denominator = electron_number_density * ELEMENTARY_CHARGE * ELEMENTARY_CHARGE;

    finite_result((numerator / denominator).sqrt())
}

/// Computes the Debye sphere volume.
///
/// Formula: `V_D = (4/3)πλ_D^3`
#[must_use]
pub fn debye_sphere_volume(debye_length: f64) -> Option<f64> {
    if !debye_length.is_finite() || debye_length < 0.0 {
        return None;
    }

    finite_result((4.0 / 3.0) * PI * debye_length.powi(3))
}

/// Computes the Debye number.
///
/// Formula: `N_D = n_e (4/3)πλ_D^3`
#[must_use]
pub fn debye_number(electron_number_density: f64, debye_length: f64) -> Option<f64> {
    if !all_finite(&[electron_number_density, debye_length])
        || electron_number_density < 0.0
        || debye_length < 0.0
    {
        return None;
    }

    finite_result(electron_number_density * debye_sphere_volume(debye_length)?)
}

/// Computes the electron thermal speed.
///
/// Formula: `v_th,e = sqrt(k_B T_e / m_e)`
///
/// # Examples
///
/// ```rust
/// use use_plasma::electron_thermal_speed;
///
/// assert!(electron_thermal_speed(10_000.0).is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn electron_thermal_speed(electron_temperature_kelvin: f64) -> Option<f64> {
    particle_thermal_speed(electron_temperature_kelvin, ELECTRON_MASS)
}

/// Computes a particle thermal speed.
///
/// Formula: `v_th = sqrt(k_B T / m)`
#[must_use]
pub fn particle_thermal_speed(temperature_kelvin: f64, particle_mass: f64) -> Option<f64> {
    if !all_finite(&[temperature_kelvin, particle_mass])
        || temperature_kelvin < 0.0
        || particle_mass <= 0.0
    {
        return None;
    }

    finite_result((BOLTZMANN_CONSTANT * temperature_kelvin / particle_mass).sqrt())
}

/// Computes the gyro angular frequency.
///
/// Formula: `ω_c = |q| B / m`
#[must_use]
pub fn gyro_angular_frequency(charge: f64, magnetic_flux_density: f64, mass: f64) -> Option<f64> {
    if !all_finite(&[charge, magnetic_flux_density, mass])
        || charge == 0.0
        || magnetic_flux_density < 0.0
        || mass <= 0.0
    {
        return None;
    }

    finite_result(charge.abs() * magnetic_flux_density / mass)
}

/// Computes the gyrofrequency in hertz.
///
/// Formula: `f_c = ω_c / (2π)`
///
/// # Examples
///
/// ```rust
/// use use_plasma::{ELECTRON_MASS, ELEMENTARY_CHARGE, gyrofrequency};
///
/// assert!(gyrofrequency(ELEMENTARY_CHARGE, 1.0, ELECTRON_MASS).is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn gyrofrequency(charge: f64, magnetic_flux_density: f64, mass: f64) -> Option<f64> {
    finite_result(gyro_angular_frequency(charge, magnetic_flux_density, mass)? / TAU)
}

/// Computes the gyroradius.
///
/// Formula: `r_L = m v_perp / (|q| B)`
///
/// # Examples
///
/// ```rust
/// use use_plasma::{ELECTRON_MASS, ELEMENTARY_CHARGE, gyroradius};
///
/// assert!(gyroradius(ELECTRON_MASS, 1_000.0, ELEMENTARY_CHARGE, 1.0)
///     .is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn gyroradius(
    mass: f64,
    perpendicular_speed: f64,
    charge: f64,
    magnetic_flux_density: f64,
) -> Option<f64> {
    if !all_finite(&[mass, perpendicular_speed, charge, magnetic_flux_density])
        || mass <= 0.0
        || perpendicular_speed < 0.0
        || charge == 0.0
        || magnetic_flux_density <= 0.0
    {
        return None;
    }

    finite_result(mass * perpendicular_speed / (charge.abs() * magnetic_flux_density))
}

/// Computes the electron gyrofrequency using electron mass and charge magnitude.
#[must_use]
pub fn electron_gyrofrequency(magnetic_flux_density: f64) -> Option<f64> {
    gyrofrequency(ELEMENTARY_CHARGE, magnetic_flux_density, ELECTRON_MASS)
}

/// Computes the electron gyroradius using electron mass and charge magnitude.
#[must_use]
pub fn electron_gyroradius(perpendicular_speed: f64, magnetic_flux_density: f64) -> Option<f64> {
    gyroradius(
        ELECTRON_MASS,
        perpendicular_speed,
        ELEMENTARY_CHARGE,
        magnetic_flux_density,
    )
}

/// Computes charge density.
///
/// Formula: `ρ_q = e (Z n_i - n_e)`
#[must_use]
pub fn charge_density(
    ion_number_density: f64,
    ion_charge_state: f64,
    electron_number_density: f64,
) -> Option<f64> {
    if !all_finite(&[
        ion_number_density,
        ion_charge_state,
        electron_number_density,
    ]) || ion_number_density < 0.0
        || ion_charge_state < 0.0
        || electron_number_density < 0.0
    {
        return None;
    }

    let charge_imbalance = ion_charge_state.mul_add(ion_number_density, -electron_number_density);
    if !charge_imbalance.is_finite() {
        return None;
    }

    finite_result(ELEMENTARY_CHARGE * charge_imbalance)
}

/// Checks whether a plasma is quasi-neutral within a relative tolerance.
///
/// # Examples
///
/// ```rust
/// use use_plasma::is_quasi_neutral;
///
/// assert_eq!(is_quasi_neutral(1.0e18, 1.0, 1.0e18, 1.0e-9), Some(true));
/// ```
#[must_use]
pub fn is_quasi_neutral(
    ion_number_density: f64,
    ion_charge_state: f64,
    electron_number_density: f64,
    relative_tolerance: f64,
) -> Option<bool> {
    if !all_finite(&[
        ion_number_density,
        ion_charge_state,
        electron_number_density,
        relative_tolerance,
    ]) || ion_number_density < 0.0
        || ion_charge_state < 0.0
        || electron_number_density < 0.0
        || relative_tolerance < 0.0
    {
        return None;
    }

    let ion_equivalent_density = ion_charge_state * ion_number_density;
    if !ion_equivalent_density.is_finite() {
        return None;
    }

    if ion_equivalent_density == 0.0 && electron_number_density == 0.0 {
        return Some(true);
    }

    let scale = ion_equivalent_density.max(electron_number_density);
    if scale == 0.0 || !scale.is_finite() {
        return None;
    }

    let relative_difference = (ion_equivalent_density - electron_number_density).abs() / scale;
    relative_difference
        .is_finite()
        .then_some(relative_difference <= relative_tolerance)
}

/// Computes scalar plasma pressure.
///
/// Formula: `p = n k_B T`
#[must_use]
pub fn plasma_pressure(number_density: f64, temperature_kelvin: f64) -> Option<f64> {
    if !all_finite(&[number_density, temperature_kelvin])
        || number_density < 0.0
        || temperature_kelvin < 0.0
    {
        return None;
    }

    finite_result(number_density * BOLTZMANN_CONSTANT * temperature_kelvin)
}

/// Computes total scalar plasma pressure.
///
/// Formula: `p_total = n_e k_B T_e + n_i k_B T_i`
#[must_use]
pub fn total_plasma_pressure(
    electron_number_density: f64,
    electron_temperature_kelvin: f64,
    ion_number_density: f64,
    ion_temperature_kelvin: f64,
) -> Option<f64> {
    let electron_pressure = plasma_pressure(electron_number_density, electron_temperature_kelvin)?;
    let ion_pressure = plasma_pressure(ion_number_density, ion_temperature_kelvin)?;

    finite_result(electron_pressure + ion_pressure)
}

/// Computes magnetic pressure.
///
/// Formula: `p_B = B^2 / (2μ0)`
#[must_use]
pub fn magnetic_pressure(magnetic_flux_density: f64) -> Option<f64> {
    if !magnetic_flux_density.is_finite() || magnetic_flux_density < 0.0 {
        return None;
    }

    finite_result((magnetic_flux_density * magnetic_flux_density) / (2.0 * VACUUM_PERMEABILITY))
}

/// Computes plasma beta.
///
/// Formula: `β = p / p_B`
///
/// # Examples
///
/// ```rust
/// use use_plasma::plasma_beta;
///
/// assert!(plasma_beta(1.0, 1.0).is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn plasma_beta(plasma_pressure: f64, magnetic_flux_density: f64) -> Option<f64> {
    if !all_finite(&[plasma_pressure, magnetic_flux_density])
        || plasma_pressure < 0.0
        || magnetic_flux_density <= 0.0
    {
        return None;
    }

    finite_result(plasma_pressure / magnetic_pressure(magnetic_flux_density)?)
}

/// Computes the Alfven speed.
///
/// Formula: `v_A = B / sqrt(μ0 ρ)`
///
/// # Examples
///
/// ```rust
/// use use_plasma::alfven_speed;
///
/// assert!(alfven_speed(1.0, 1.0e-12).is_some_and(|value| value > 0.0));
/// ```
#[must_use]
pub fn alfven_speed(magnetic_flux_density: f64, mass_density: f64) -> Option<f64> {
    if !all_finite(&[magnetic_flux_density, mass_density])
        || magnetic_flux_density < 0.0
        || mass_density <= 0.0
    {
        return None;
    }

    finite_result(magnetic_flux_density / (VACUUM_PERMEABILITY * mass_density).sqrt())
}

/// Returns whether a Coulomb logarithm value is finite and positive.
///
/// Detailed collisional plasma models and collision-frequency formulas are intentionally out of
/// scope for this crate.
#[must_use]
pub fn is_valid_coulomb_logarithm(coulomb_logarithm: f64) -> bool {
    coulomb_logarithm.is_finite() && coulomb_logarithm > 0.0
}

#[cfg(test)]
mod tests {
    use super::{
        BOLTZMANN_CONSTANT, ELECTRON_MASS, ELEMENTARY_CHARGE, ElectronPlasma, PI, PROTON_MASS,
        PlasmaSpecies, VACUUM_PERMEABILITY, alfven_speed, charge_density, debye_length,
        debye_number, debye_sphere_volume, electron_gyrofrequency, electron_gyroradius,
        electron_plasma_angular_frequency, electron_plasma_frequency, electron_thermal_speed,
        gyro_angular_frequency, gyrofrequency, gyroradius, ion_plasma_angular_frequency,
        is_quasi_neutral, is_valid_coulomb_logarithm, magnetic_pressure, particle_thermal_speed,
        plasma_beta, plasma_pressure, total_plasma_pressure,
    };

    fn approx_eq(left: f64, right: f64) -> bool {
        let scale = left.abs().max(right.abs()).max(1.0);
        (left - right).abs() <= 1.0e-12 * scale
    }

    #[test]
    fn plasma_frequency_helpers_cover_common_inputs() {
        assert!(matches!(
            electron_plasma_angular_frequency(1.0e18),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert!(matches!(
            electron_plasma_frequency(1.0e18),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(electron_plasma_angular_frequency(-1.0), None);

        assert!(matches!(
            ion_plasma_angular_frequency(1.0e18, 1.0, PROTON_MASS),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(
            ion_plasma_angular_frequency(1.0e18, -1.0, PROTON_MASS),
            None
        );
        assert_eq!(ion_plasma_angular_frequency(1.0e18, 1.0, 0.0), None);
    }

    #[test]
    fn debye_helpers_cover_common_inputs() {
        assert!(matches!(
            debye_length(10_000.0, 1.0e18),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(debye_length(-1.0, 1.0e18), None);
        assert_eq!(debye_length(10_000.0, 0.0), None);

        let expected_volume = (4.0 / 3.0) * PI * 8.0;
        assert!(matches!(
            debye_sphere_volume(2.0),
            Some(value) if approx_eq(value, expected_volume)
        ));
        assert_eq!(debye_sphere_volume(-1.0), None);

        assert!(matches!(
            debye_number(1.0e18, 1.0e-4),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(debye_number(-1.0, 1.0e-4), None);
    }

    #[test]
    fn thermal_speed_helpers_cover_common_inputs() {
        assert!(matches!(
            electron_thermal_speed(10_000.0),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(electron_thermal_speed(-1.0), None);

        assert!(matches!(
            particle_thermal_speed(10_000.0, PROTON_MASS),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(particle_thermal_speed(10_000.0, 0.0), None);
    }

    #[test]
    fn gyro_helpers_cover_common_inputs() {
        assert!(matches!(
            gyro_angular_frequency(ELEMENTARY_CHARGE, 1.0, ELECTRON_MASS),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(gyro_angular_frequency(0.0, 1.0, ELECTRON_MASS), None);
        assert_eq!(
            gyro_angular_frequency(ELEMENTARY_CHARGE, -1.0, ELECTRON_MASS),
            None
        );

        assert!(matches!(
            gyrofrequency(ELEMENTARY_CHARGE, 1.0, ELECTRON_MASS),
            Some(value) if value.is_finite() && value > 0.0
        ));

        assert!(matches!(
            gyroradius(ELECTRON_MASS, 1_000.0, ELEMENTARY_CHARGE, 1.0),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(
            gyroradius(ELECTRON_MASS, -1_000.0, ELEMENTARY_CHARGE, 1.0),
            None
        );
        assert_eq!(gyroradius(ELECTRON_MASS, 1_000.0, 0.0, 1.0), None);
        assert_eq!(
            gyroradius(ELECTRON_MASS, 1_000.0, ELEMENTARY_CHARGE, 0.0),
            None
        );

        assert!(matches!(
            electron_gyrofrequency(1.0),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert!(matches!(
            electron_gyroradius(1_000.0, 1.0),
            Some(value) if value.is_finite() && value > 0.0
        ));
    }

    #[test]
    fn charge_density_and_quasi_neutrality_cover_common_inputs() {
        assert!(matches!(
            charge_density(1.0e18, 1.0, 1.0e18),
            Some(value) if approx_eq(value, 0.0)
        ));
        assert!(matches!(
            charge_density(1.0e18, 1.0, 0.5e18),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(charge_density(-1.0, 1.0, 1.0e18), None);

        assert_eq!(is_quasi_neutral(1.0e18, 1.0, 1.0e18, 1.0e-9), Some(true));
        assert_eq!(is_quasi_neutral(1.0e18, 1.0, 0.5e18, 1.0e-9), Some(false));
        assert_eq!(is_quasi_neutral(0.0, 1.0, 0.0, 0.0), Some(true));
        assert_eq!(is_quasi_neutral(1.0e18, 1.0, 1.0e18, -1.0), None);
    }

    #[test]
    fn pressure_and_field_helpers_cover_common_inputs() {
        assert!(matches!(
            plasma_pressure(1.0e18, 10_000.0),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(plasma_pressure(-1.0, 10_000.0), None);
        assert_eq!(plasma_pressure(1.0e18, -1.0), None);

        assert!(matches!(
            total_plasma_pressure(1.0e18, 10_000.0, 1.0e18, 10_000.0),
            Some(value) if value.is_finite() && value > 0.0
        ));

        assert!(matches!(
            magnetic_pressure(1.0),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(magnetic_pressure(-1.0), None);

        assert!(matches!(
            plasma_beta(1.0, 1.0),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(plasma_beta(-1.0, 1.0), None);
        assert_eq!(plasma_beta(1.0, 0.0), None);

        assert!(matches!(
            alfven_speed(1.0, 1.0e-12),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(alfven_speed(-1.0, 1.0e-12), None);
        assert_eq!(alfven_speed(1.0, 0.0), None);
    }

    #[test]
    fn simple_validators_cover_common_inputs() {
        assert!(is_valid_coulomb_logarithm(10.0));
        assert!(!is_valid_coulomb_logarithm(0.0));
        assert!(!is_valid_coulomb_logarithm(f64::NAN));
    }

    #[test]
    fn simple_types_delegate_to_public_helpers() {
        let proton_species = PlasmaSpecies::new(1.0e18, 10_000.0, 1.0, PROTON_MASS);
        assert!(matches!(
            proton_species.and_then(|species| species.pressure()),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert!(matches!(
            proton_species.and_then(|species| species.thermal_speed()),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(PlasmaSpecies::new(-1.0, 10_000.0, 1.0, PROTON_MASS), None);
        assert_eq!(PlasmaSpecies::new(1.0e18, 10_000.0, 1.0, 0.0), None);

        let electron_plasma = ElectronPlasma::new(1.0e18, 10_000.0);
        assert!(matches!(
            electron_plasma.and_then(|plasma| plasma.plasma_frequency()),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert!(matches!(
            electron_plasma.and_then(|plasma| plasma.debye_length()),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert!(matches!(
            electron_plasma.and_then(|plasma| plasma.debye_number()),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert!(matches!(
            electron_plasma.and_then(|plasma| plasma.thermal_speed()),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert!(matches!(
            electron_plasma.and_then(|plasma| plasma.pressure()),
            Some(value) if value.is_finite() && value > 0.0
        ));
        assert_eq!(ElectronPlasma::new(-1.0, 10_000.0), None);
    }

    #[test]
    fn formulas_match_expected_scalar_relations() {
        let expected_pressure = 1.0e18 * BOLTZMANN_CONSTANT * 10_000.0;
        assert!(matches!(
            plasma_pressure(1.0e18, 10_000.0),
            Some(value) if approx_eq(value, expected_pressure)
        ));

        let expected_magnetic_pressure = 1.0 / (2.0 * VACUUM_PERMEABILITY);
        assert!(matches!(
            magnetic_pressure(1.0),
            Some(value) if approx_eq(value, expected_magnetic_pressure)
        ));
    }
}
