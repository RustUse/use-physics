#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small scalar helpers for radiation physics calculations.

use core::f64::consts::PI;

pub mod prelude;

/// Speed of light in vacuum, in meters per second.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Planck's constant in joule-seconds.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const PLANCK_CONSTANT: f64 = 6.626_070_15e-34;

/// Elementary charge in coulombs.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const ELEMENTARY_CHARGE: f64 = 1.602_176_634e-19;

/// Joules in one mega-electron-volt.
///
/// Broader physical constants belong in the top-level `use-constants` set.
pub const JOULES_PER_MEV: f64 = 1.602_176_634e-13;

fn is_non_negative_finite(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

fn is_positive_finite(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn normalize_zero(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

fn finite_non_negative(value: f64) -> Option<f64> {
    (value.is_finite() && value >= 0.0).then_some(normalize_zero(value))
}

fn finite_unit_interval(value: f64) -> Option<f64> {
    (value.is_finite() && (0.0..=1.0).contains(&value)).then_some(normalize_zero(value))
}

fn divide_non_negative(numerator: f64, denominator: f64) -> Option<f64> {
    if !is_non_negative_finite(numerator) || !is_positive_finite(denominator) {
        return None;
    }

    finite_non_negative(numerator / denominator)
}

fn multiply_non_negative(left: f64, right: f64) -> Option<f64> {
    if !is_non_negative_finite(left) || !is_non_negative_finite(right) {
        return None;
    }

    finite_non_negative(left * right)
}

/// Computes photon energy from frequency with `E = h * f`.
///
/// Returns joules.
///
/// Broader photon helpers also exist in `use-quantum`.
#[must_use]
pub fn photon_energy_from_frequency(frequency: f64) -> Option<f64> {
    if !is_non_negative_finite(frequency) {
        return None;
    }

    finite_non_negative(PLANCK_CONSTANT * frequency)
}

/// Computes photon energy from wavelength with `E = h * c / lambda`.
///
/// Returns joules.
#[must_use]
pub fn photon_energy_from_wavelength(wavelength: f64) -> Option<f64> {
    if !is_positive_finite(wavelength) {
        return None;
    }

    finite_non_negative((PLANCK_CONSTANT * SPEED_OF_LIGHT) / wavelength)
}

/// Computes photon flux from power with `Phi = P / E_photon`.
///
/// Returns photons per second.
///
/// # Examples
///
/// ```rust
/// use use_radiation::photon_flux_from_power;
///
/// assert_eq!(photon_flux_from_power(10.0, 2.0), Some(5.0));
/// ```
#[must_use]
pub fn photon_flux_from_power(power: f64, photon_energy: f64) -> Option<f64> {
    divide_non_negative(power, photon_energy)
}

/// Computes photon flux density with `phi = Phi / A`.
///
/// Returns photons per square meter per second.
#[must_use]
pub fn photon_flux_density(photon_flux: f64, area: f64) -> Option<f64> {
    divide_non_negative(photon_flux, area)
}

/// Computes intensity from power and area with `I = P / A`.
///
/// Returns watts per square meter.
///
/// # Examples
///
/// ```rust
/// use use_radiation::intensity;
///
/// assert_eq!(intensity(10.0, 2.0), Some(5.0));
/// ```
#[must_use]
pub fn intensity(power: f64, area: f64) -> Option<f64> {
    divide_non_negative(power, area)
}

/// Computes isotropic intensity from power and distance with `I = P / (4 * pi * r^2)`.
///
/// # Examples
///
/// ```rust
/// use use_radiation::isotropic_intensity;
///
/// let Some(value) = isotropic_intensity(4.0 * core::f64::consts::PI, 1.0) else {
///     panic!("expected isotropic intensity");
/// };
///
/// assert!((value - 1.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn isotropic_intensity(power: f64, distance: f64) -> Option<f64> {
    if !is_non_negative_finite(power) || !is_positive_finite(distance) {
        return None;
    }

    intensity(power, 4.0 * PI * distance * distance)
}

/// Computes inverse-square intensity from a reference value.
///
/// Formula: `I2 = I1 * (r1 / r2)^2`.
#[must_use]
pub fn inverse_square_intensity(
    reference_intensity: f64,
    reference_distance: f64,
    target_distance: f64,
) -> Option<f64> {
    if !is_non_negative_finite(reference_intensity)
        || !is_positive_finite(reference_distance)
        || !is_positive_finite(target_distance)
    {
        return None;
    }

    let ratio = reference_distance / target_distance;
    finite_non_negative(reference_intensity * ratio * ratio)
}

/// Computes fluence with `F = N / A`.
///
/// Returns particles per square meter.
#[must_use]
pub fn fluence(particle_count: f64, area: f64) -> Option<f64> {
    divide_non_negative(particle_count, area)
}

/// Computes energy fluence with `Psi = E / A`.
///
/// Returns joules per square meter.
#[must_use]
pub fn energy_fluence(energy: f64, area: f64) -> Option<f64> {
    divide_non_negative(energy, area)
}

/// Computes fluence rate with `fluence_rate = F / t`.
#[must_use]
pub fn fluence_rate(fluence: f64, time: f64) -> Option<f64> {
    divide_non_negative(fluence, time)
}

/// Computes absorbed dose with `D = E / m`.
///
/// Returns gray, equivalent to joules per kilogram.
///
/// # Examples
///
/// ```rust
/// use use_radiation::absorbed_dose;
///
/// assert_eq!(absorbed_dose(20.0, 4.0), Some(5.0));
/// ```
#[must_use]
pub fn absorbed_dose(energy_absorbed: f64, mass: f64) -> Option<f64> {
    divide_non_negative(energy_absorbed, mass)
}

/// Computes absorbed energy from dose with `E = D * m`.
#[must_use]
pub fn absorbed_energy_from_dose(dose: f64, mass: f64) -> Option<f64> {
    multiply_non_negative(dose, mass)
}

/// Computes dose rate with `dose_rate = D / t`.
#[must_use]
pub fn dose_rate(dose: f64, time: f64) -> Option<f64> {
    divide_non_negative(dose, time)
}

/// Computes accumulated dose with `D = dose_rate * t`.
#[must_use]
pub fn accumulated_dose(dose_rate: f64, time: f64) -> Option<f64> {
    multiply_non_negative(dose_rate, time)
}

/// Computes equivalent dose with `H = D * w_R`.
///
/// Returns sieverts.
///
/// # Examples
///
/// ```rust
/// use use_radiation::equivalent_dose;
///
/// assert_eq!(equivalent_dose(2.0, 20.0), Some(40.0));
/// ```
#[must_use]
pub fn equivalent_dose(absorbed_dose: f64, radiation_weighting_factor: f64) -> Option<f64> {
    multiply_non_negative(absorbed_dose, radiation_weighting_factor)
}

/// Computes effective dose with `E = H * w_T`.
///
/// Returns sieverts.
#[must_use]
pub fn effective_dose(equivalent_dose: f64, tissue_weighting_factor: f64) -> Option<f64> {
    multiply_non_negative(equivalent_dose, tissue_weighting_factor)
}

/// Sums pre-weighted equivalent doses into a total effective dose.
#[must_use]
pub fn total_effective_dose(weighted_equivalent_doses: &[f64]) -> Option<f64> {
    weighted_equivalent_doses
        .iter()
        .try_fold(0.0, |sum, value| {
            if !is_non_negative_finite(*value) {
                return None;
            }

            finite_non_negative(sum + *value)
        })
}

/// Computes attenuated intensity with `I = I0 * e^(-mu * x)`.
///
/// # Examples
///
/// ```rust
/// use use_radiation::attenuated_intensity;
///
/// let Some(value) = attenuated_intensity(100.0, core::f64::consts::LN_2, 1.0) else {
///     panic!("expected attenuated intensity");
/// };
///
/// assert!((value - 50.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn attenuated_intensity(
    initial_intensity: f64,
    linear_attenuation_coefficient: f64,
    thickness: f64,
) -> Option<f64> {
    if !is_non_negative_finite(initial_intensity)
        || !is_non_negative_finite(linear_attenuation_coefficient)
        || !is_non_negative_finite(thickness)
    {
        return None;
    }

    finite_non_negative(
        initial_intensity * transmitted_fraction(linear_attenuation_coefficient, thickness)?,
    )
}

/// Computes transmitted fraction with `T = e^(-mu * x)`.
#[must_use]
pub fn transmitted_fraction(linear_attenuation_coefficient: f64, thickness: f64) -> Option<f64> {
    if !is_non_negative_finite(linear_attenuation_coefficient) || !is_non_negative_finite(thickness)
    {
        return None;
    }

    finite_unit_interval((-(linear_attenuation_coefficient * thickness)).exp())
}

/// Computes required shield thickness with `x = ln(I0 / I) / mu`.
#[must_use]
pub fn required_shield_thickness(
    linear_attenuation_coefficient: f64,
    initial_intensity: f64,
    target_intensity: f64,
) -> Option<f64> {
    if !is_positive_finite(linear_attenuation_coefficient)
        || !is_positive_finite(initial_intensity)
        || !is_positive_finite(target_intensity)
        || target_intensity > initial_intensity
    {
        return None;
    }

    finite_non_negative(
        (initial_intensity / target_intensity).ln() / linear_attenuation_coefficient,
    )
}

/// Computes half-value layer with `HVL = ln(2) / mu`.
///
/// # Examples
///
/// ```rust
/// use use_radiation::half_value_layer;
///
/// let Some(value) = half_value_layer(core::f64::consts::LN_2) else {
///     panic!("expected half-value layer");
/// };
///
/// assert!((value - 1.0).abs() < 1.0e-12);
/// ```
#[must_use]
pub fn half_value_layer(linear_attenuation_coefficient: f64) -> Option<f64> {
    if !is_positive_finite(linear_attenuation_coefficient) {
        return None;
    }

    finite_non_negative(core::f64::consts::LN_2 / linear_attenuation_coefficient)
}

/// Computes tenth-value layer with `TVL = ln(10) / mu`.
#[must_use]
pub fn tenth_value_layer(linear_attenuation_coefficient: f64) -> Option<f64> {
    if !is_positive_finite(linear_attenuation_coefficient) {
        return None;
    }

    finite_non_negative(core::f64::consts::LN_10 / linear_attenuation_coefficient)
}

/// Computes linear attenuation coefficient from mass attenuation and density.
///
/// Formula: `mu = (mu / rho) * rho`.
#[must_use]
pub fn linear_attenuation_from_mass_attenuation(
    mass_attenuation_coefficient: f64,
    density: f64,
) -> Option<f64> {
    multiply_non_negative(mass_attenuation_coefficient, density)
}

/// Computes mass attenuation coefficient from linear attenuation and density.
///
/// Formula: `mu / rho = mu / rho`.
#[must_use]
pub fn mass_attenuation_from_linear_attenuation(
    linear_attenuation_coefficient: f64,
    density: f64,
) -> Option<f64> {
    divide_non_negative(linear_attenuation_coefficient, density)
}

/// Simple radiation categories for scalar helper selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RadiationKind {
    /// Alpha-particle radiation.
    Alpha,
    /// Beta-minus radiation.
    BetaMinus,
    /// Beta-plus radiation.
    BetaPlus,
    /// Gamma radiation.
    Gamma,
    /// X-ray radiation.
    XRay,
    /// Neutron radiation.
    Neutron,
    /// Proton radiation.
    Proton,
    /// Electron radiation.
    Electron,
    /// Generic photon radiation.
    Photon,
}

/// Returns whether the listed radiation kind is ionizing in this simple crate.
#[must_use]
pub const fn is_ionizing(_kind: RadiationKind) -> bool {
    true
}

/// Returns whether the listed radiation kind is photon radiation.
#[must_use]
pub const fn is_photon_radiation(kind: RadiationKind) -> bool {
    matches!(
        kind,
        RadiationKind::Gamma | RadiationKind::XRay | RadiationKind::Photon
    )
}

/// Returns whether the listed radiation kind is particle radiation.
#[must_use]
pub const fn is_particle_radiation(kind: RadiationKind) -> bool {
    !is_photon_radiation(kind)
}

/// Returns a simple conventional radiation weighting factor for the given kind.
///
/// These values are example conveniences, not safety guidance.
/// Neutron weighting depends on energy and is intentionally omitted in v1.
#[must_use]
pub const fn default_radiation_weighting_factor(kind: RadiationKind) -> Option<f64> {
    match kind {
        RadiationKind::Gamma
        | RadiationKind::XRay
        | RadiationKind::BetaMinus
        | RadiationKind::BetaPlus
        | RadiationKind::Electron
        | RadiationKind::Photon => Some(1.0),
        RadiationKind::Proton => Some(2.0),
        RadiationKind::Alpha => Some(20.0),
        RadiationKind::Neutron => None,
    }
}

/// A simple beam characterized by total power and illuminated area.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadiationBeam {
    /// Beam power in watts.
    pub power: f64,
    /// Beam cross-sectional area in square meters.
    pub area: f64,
}

impl RadiationBeam {
    /// Creates a beam from non-negative finite power and positive finite area.
    #[must_use]
    pub fn new(power: f64, area: f64) -> Option<Self> {
        if !is_non_negative_finite(power) || !is_positive_finite(area) {
            return None;
        }

        Some(Self { power, area })
    }

    /// Computes beam intensity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_radiation::RadiationBeam;
    ///
    /// # fn main() -> Result<(), &'static str> {
    /// let beam = RadiationBeam::new(10.0, 2.0).ok_or("expected beam")?;
    ///
    /// assert_eq!(beam.intensity(), Some(5.0));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn intensity(&self) -> Option<f64> {
        intensity(self.power, self.area)
    }

    /// Computes photon flux for this beam.
    #[must_use]
    pub fn photon_flux(&self, photon_energy: f64) -> Option<f64> {
        photon_flux_from_power(self.power, photon_energy)
    }

    /// Computes photon flux density for this beam.
    #[must_use]
    pub fn photon_flux_density(&self, photon_energy: f64) -> Option<f64> {
        photon_flux_density(self.photon_flux(photon_energy)?, self.area)
    }
}

/// A simple slab shield with a linear attenuation coefficient and thickness.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shield {
    /// Linear attenuation coefficient in inverse meters.
    pub linear_attenuation_coefficient: f64,
    /// Shield thickness in meters.
    pub thickness: f64,
}

impl Shield {
    /// Creates a shield from non-negative finite attenuation and thickness values.
    #[must_use]
    pub fn new(linear_attenuation_coefficient: f64, thickness: f64) -> Option<Self> {
        if !is_non_negative_finite(linear_attenuation_coefficient)
            || !is_non_negative_finite(thickness)
        {
            return None;
        }

        Some(Self {
            linear_attenuation_coefficient,
            thickness,
        })
    }

    /// Computes transmitted fraction for this shield.
    #[must_use]
    pub fn transmitted_fraction(&self) -> Option<f64> {
        transmitted_fraction(self.linear_attenuation_coefficient, self.thickness)
    }

    /// Computes attenuated intensity through this shield.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_radiation::Shield;
    ///
    /// # fn main() -> Result<(), &'static str> {
    /// let shield = Shield::new(core::f64::consts::LN_2, 1.0).ok_or("expected shield")?;
    /// let attenuated = shield
    ///     .attenuated_intensity(100.0)
    ///     .ok_or("expected attenuated intensity")?;
    ///
    /// assert!((attenuated - 50.0).abs() < 1.0e-12);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn attenuated_intensity(&self, initial_intensity: f64) -> Option<f64> {
        attenuated_intensity(
            initial_intensity,
            self.linear_attenuation_coefficient,
            self.thickness,
        )
    }
}

/// A simple absorbed dose wrapper in gray.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dose {
    /// Absorbed dose in gray.
    pub absorbed_dose: f64,
}

impl Dose {
    /// Creates a dose from a non-negative finite absorbed dose.
    #[must_use]
    pub fn new(absorbed_dose: f64) -> Option<Self> {
        if !is_non_negative_finite(absorbed_dose) {
            return None;
        }

        Some(Self { absorbed_dose })
    }

    /// Computes equivalent dose from this absorbed dose.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use use_radiation::Dose;
    ///
    /// # fn main() -> Result<(), &'static str> {
    /// let dose = Dose::new(2.0).ok_or("expected dose")?;
    ///
    /// assert_eq!(dose.equivalent(20.0), Some(40.0));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn equivalent(self, radiation_weighting_factor: f64) -> Option<f64> {
        equivalent_dose(self.absorbed_dose, radiation_weighting_factor)
    }

    /// Computes dose rate over the given time.
    #[must_use]
    pub fn rate_over(self, time: f64) -> Option<f64> {
        dose_rate(self.absorbed_dose, time)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{
        Dose, ELEMENTARY_CHARGE, JOULES_PER_MEV, PLANCK_CONSTANT, RadiationBeam, RadiationKind,
        SPEED_OF_LIGHT, Shield, absorbed_dose, absorbed_energy_from_dose, accumulated_dose,
        attenuated_intensity, default_radiation_weighting_factor, dose_rate, effective_dose,
        energy_fluence, equivalent_dose, fluence, fluence_rate, half_value_layer, intensity,
        inverse_square_intensity, is_ionizing, is_particle_radiation, is_photon_radiation,
        isotropic_intensity, linear_attenuation_from_mass_attenuation,
        mass_attenuation_from_linear_attenuation, photon_energy_from_frequency,
        photon_energy_from_wavelength, photon_flux_density, photon_flux_from_power,
        required_shield_thickness, tenth_value_layer, total_effective_dose, transmitted_fraction,
    };

    fn assert_approx_eq(left: f64, right: f64) {
        let tolerance = 1.0e-12 * left.abs().max(right.abs()).max(1.0);
        assert!(
            (left - right).abs() <= tolerance,
            "left={left}, right={right}, tolerance={tolerance}"
        );
    }

    fn assert_some_approx(actual: Option<f64>, expected: f64) {
        let Some(actual) = actual else {
            panic!("expected Some({expected})");
        };

        assert_approx_eq(actual, expected);
    }

    #[test]
    fn photon_energy_helpers_validate_inputs() {
        assert_eq!(photon_energy_from_frequency(1.0), Some(PLANCK_CONSTANT));
        assert_eq!(photon_energy_from_frequency(-1.0), None);
        assert_eq!(photon_energy_from_frequency(f64::NAN), None);

        assert_some_approx(
            photon_energy_from_wavelength(SPEED_OF_LIGHT),
            PLANCK_CONSTANT,
        );
        assert_eq!(photon_energy_from_wavelength(0.0), None);
        assert_eq!(photon_energy_from_wavelength(f64::INFINITY), None);
    }

    #[test]
    fn photon_flux_helpers_cover_common_cases() {
        assert_eq!(photon_flux_from_power(10.0, 2.0), Some(5.0));
        assert_eq!(photon_flux_from_power(10.0, 0.0), None);
        assert_eq!(photon_flux_from_power(-1.0, 2.0), None);

        assert_eq!(photon_flux_density(10.0, 2.0), Some(5.0));
        assert_eq!(photon_flux_density(10.0, 0.0), None);
    }

    #[test]
    fn intensity_helpers_cover_planar_and_inverse_square_cases() {
        assert_eq!(intensity(10.0, 2.0), Some(5.0));
        assert_eq!(intensity(-10.0, 2.0), None);
        assert_eq!(intensity(10.0, 0.0), None);

        assert_some_approx(isotropic_intensity(4.0 * core::f64::consts::PI, 1.0), 1.0);
        assert_eq!(isotropic_intensity(1.0, 0.0), None);

        assert_eq!(inverse_square_intensity(100.0, 1.0, 2.0), Some(25.0));
        assert_eq!(inverse_square_intensity(100.0, 1.0, 0.0), None);
    }

    #[test]
    fn fluence_helpers_validate_inputs() {
        assert_eq!(fluence(100.0, 2.0), Some(50.0));
        assert_eq!(fluence(-100.0, 2.0), None);

        assert_eq!(energy_fluence(100.0, 2.0), Some(50.0));
        assert_eq!(fluence_rate(50.0, 10.0), Some(5.0));
        assert_eq!(fluence_rate(50.0, 0.0), None);
    }

    #[test]
    fn absorbed_dose_helpers_cover_forward_and_inverse_relations() {
        assert_eq!(absorbed_dose(20.0, 4.0), Some(5.0));
        assert_eq!(absorbed_dose(20.0, 0.0), None);

        assert_eq!(absorbed_energy_from_dose(5.0, 4.0), Some(20.0));
        assert_eq!(absorbed_energy_from_dose(-5.0, 4.0), None);

        assert_eq!(dose_rate(10.0, 2.0), Some(5.0));
        assert_eq!(dose_rate(10.0, 0.0), None);

        assert_eq!(accumulated_dose(5.0, 2.0), Some(10.0));
        assert_eq!(accumulated_dose(5.0, -1.0), None);
    }

    #[test]
    fn equivalent_and_effective_dose_helpers_cover_common_cases() {
        assert_eq!(equivalent_dose(2.0, 20.0), Some(40.0));
        assert_eq!(equivalent_dose(-2.0, 20.0), None);

        assert_some_approx(effective_dose(10.0, 0.12), 1.2);
        assert_eq!(effective_dose(10.0, -0.12), None);

        assert_eq!(total_effective_dose(&[1.0, 2.0, 3.0]), Some(6.0));
        assert_eq!(total_effective_dose(&[]), Some(0.0));
        assert_eq!(total_effective_dose(&[1.0, -2.0]), None);
    }

    #[test]
    fn attenuation_helpers_cover_common_shielding_cases() {
        assert_some_approx(
            attenuated_intensity(100.0, core::f64::consts::LN_2, 1.0),
            50.0,
        );
        assert_eq!(attenuated_intensity(100.0, -1.0, 1.0), None);

        assert_some_approx(transmitted_fraction(core::f64::consts::LN_2, 1.0), 0.5);
        assert_eq!(transmitted_fraction(-1.0, 1.0), None);

        assert_some_approx(
            required_shield_thickness(core::f64::consts::LN_2, 100.0, 50.0),
            1.0,
        );
        assert_eq!(
            required_shield_thickness(core::f64::consts::LN_2, 100.0, 200.0),
            None,
        );

        assert_some_approx(half_value_layer(core::f64::consts::LN_2), 1.0);
        assert_eq!(half_value_layer(0.0), None);

        assert_some_approx(tenth_value_layer(core::f64::consts::LN_10), 1.0);
        assert_eq!(tenth_value_layer(0.0), None);
    }

    #[test]
    fn attenuation_conversion_helpers_cover_mass_and_linear_forms() {
        assert_eq!(
            linear_attenuation_from_mass_attenuation(2.0, 3.0),
            Some(6.0)
        );
        assert_eq!(linear_attenuation_from_mass_attenuation(-2.0, 3.0), None);

        assert_eq!(
            mass_attenuation_from_linear_attenuation(6.0, 3.0),
            Some(2.0)
        );
        assert_eq!(mass_attenuation_from_linear_attenuation(6.0, 0.0), None);
    }

    #[test]
    fn radiation_kind_helpers_cover_simple_classification() {
        assert!(is_ionizing(RadiationKind::Alpha));
        assert!(is_photon_radiation(RadiationKind::Gamma));
        assert!(!is_photon_radiation(RadiationKind::Alpha));
        assert!(is_particle_radiation(RadiationKind::Alpha));
        assert!(!is_particle_radiation(RadiationKind::Photon));

        assert_eq!(
            default_radiation_weighting_factor(RadiationKind::Gamma),
            Some(1.0)
        );
        assert_eq!(
            default_radiation_weighting_factor(RadiationKind::Alpha),
            Some(20.0)
        );
        assert_eq!(
            default_radiation_weighting_factor(RadiationKind::Neutron),
            None
        );
    }

    #[test]
    fn simple_types_delegate_to_public_helpers() {
        let Some(beam) = RadiationBeam::new(10.0, 2.0) else {
            panic!("expected valid beam");
        };
        assert_eq!(beam.intensity(), Some(5.0));
        assert_eq!(beam.photon_flux(2.0), Some(5.0));
        assert_eq!(RadiationBeam::new(10.0, 0.0), None);

        let Some(shield) = Shield::new(core::f64::consts::LN_2, 1.0) else {
            panic!("expected valid shield");
        };
        assert_some_approx(shield.transmitted_fraction(), 0.5);
        assert_some_approx(shield.attenuated_intensity(100.0), 50.0);
        assert_eq!(Shield::new(-1.0, 1.0), None);

        let Some(dose) = Dose::new(2.0) else {
            panic!("expected valid dose");
        };
        assert_eq!(dose.equivalent(20.0), Some(40.0));
        assert_eq!(
            Dose::new(10.0).and_then(|value| value.rate_over(2.0)),
            Some(5.0)
        );
        assert_eq!(Dose::new(-1.0), None);
    }

    #[test]
    fn local_constants_match_expected_values() {
        assert_eq!(SPEED_OF_LIGHT, 299_792_458.0);
        assert_eq!(PLANCK_CONSTANT, 6.626_070_15e-34);
        assert_eq!(ELEMENTARY_CHARGE, 1.602_176_634e-19);
        assert_eq!(JOULES_PER_MEV, 1.602_176_634e-13);
    }

    #[test]
    fn non_finite_inputs_return_none() {
        assert_eq!(intensity(f64::NAN, 2.0), None);
        assert_eq!(photon_flux_from_power(1.0, f64::INFINITY), None);
        assert_eq!(required_shield_thickness(1.0, f64::INFINITY, 1.0), None);
    }
}
