#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Small scalar elasticity helpers.

use core::num::FpCategory;

pub mod prelude;

fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

const fn is_zero(value: f64) -> bool {
    matches!(value.classify(), FpCategory::Zero)
}

/// Computes normal stress from applied force and cross-sectional area.
///
/// Formula: `σ = F / A`.
///
/// Returns `None` when `area` is less than or equal to zero or when any input or result is not
/// finite.
///
/// # Examples
///
/// ```
/// use use_elasticity::normal_stress;
///
/// assert_eq!(normal_stress(100.0, 2.0), Some(50.0));
/// ```
#[must_use]
pub fn normal_stress(force: f64, area: f64) -> Option<f64> {
    if !all_finite(&[force, area]) || area <= 0.0 {
        return None;
    }

    finite(force / area)
}

/// Computes shear stress from applied force and loaded area.
///
/// Formula: `τ = F / A`.
///
/// Returns `None` when `area` is less than or equal to zero or when any input or result is not
/// finite.
#[must_use]
pub fn shear_stress(force: f64, area: f64) -> Option<f64> {
    if !all_finite(&[force, area]) || area <= 0.0 {
        return None;
    }

    finite(force / area)
}

/// Computes force from stress and cross-sectional area.
///
/// Formula: `F = σA`.
///
/// Returns `None` when `area` is negative or when any input or result is not finite.
#[must_use]
pub fn force_from_stress(stress: f64, area: f64) -> Option<f64> {
    if !all_finite(&[stress, area]) || area < 0.0 {
        return None;
    }

    finite(stress * area)
}

/// Computes normal strain from change in length and original length.
///
/// Formula: `ε = ΔL / L0`.
///
/// Returns `None` when `original_length` is less than or equal to zero or when any input or
/// result is not finite.
///
/// # Examples
///
/// ```
/// use use_elasticity::normal_strain;
///
/// assert_eq!(normal_strain(2.0, 10.0), Some(0.2));
/// ```
#[must_use]
pub fn normal_strain(change_in_length: f64, original_length: f64) -> Option<f64> {
    if !all_finite(&[change_in_length, original_length]) || original_length <= 0.0 {
        return None;
    }

    finite(change_in_length / original_length)
}

/// Computes engineering shear strain from lateral displacement and height.
///
/// Formula: `γ = x / h`.
///
/// Returns `None` when `height` is less than or equal to zero or when any input or result is not
/// finite.
#[must_use]
pub fn shear_strain(displacement: f64, height: f64) -> Option<f64> {
    if !all_finite(&[displacement, height]) || height <= 0.0 {
        return None;
    }

    finite(displacement / height)
}

/// Computes change in length from strain and original length.
///
/// Formula: `ΔL = εL0`.
///
/// Returns `None` when `original_length` is negative or when any input or result is not finite.
#[must_use]
pub fn change_in_length(strain: f64, original_length: f64) -> Option<f64> {
    if !all_finite(&[strain, original_length]) || original_length < 0.0 {
        return None;
    }

    finite(strain * original_length)
}

/// Computes final length after elastic axial strain.
///
/// Formula: `L = L0 * (1 + ε)`.
///
/// Returns `None` when `original_length` is negative, when the result is negative, or when any
/// input or result is not finite.
#[must_use]
pub fn final_length(original_length: f64, strain: f64) -> Option<f64> {
    if !all_finite(&[original_length, strain]) || original_length < 0.0 {
        return None;
    }

    let result = original_length * (1.0 + strain);
    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Computes Young's modulus from stress and strain.
///
/// Formula: `E = σ / ε`.
///
/// Returns `None` when `strain` is zero, when the result is negative, or when any input or result
/// is not finite.
///
/// # Examples
///
/// ```
/// use use_elasticity::youngs_modulus;
///
/// assert_eq!(youngs_modulus(100.0, 0.01), Some(10_000.0));
/// ```
#[must_use]
pub fn youngs_modulus(stress: f64, strain: f64) -> Option<f64> {
    if !all_finite(&[stress, strain]) || is_zero(strain) {
        return None;
    }

    let result = stress / strain;
    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Computes stress from Young's modulus and axial strain.
///
/// Formula: `σ = Eε`.
///
/// Returns `None` when `youngs_modulus` is negative or when any input or result is not finite.
#[must_use]
pub fn stress_from_youngs_modulus(youngs_modulus: f64, strain: f64) -> Option<f64> {
    if !all_finite(&[youngs_modulus, strain]) || youngs_modulus < 0.0 {
        return None;
    }

    finite(youngs_modulus * strain)
}

/// Computes strain from stress and Young's modulus.
///
/// Formula: `ε = σ / E`.
///
/// Returns `None` when `youngs_modulus` is less than or equal to zero or when any input or result
/// is not finite.
#[must_use]
pub fn strain_from_youngs_modulus(stress: f64, youngs_modulus: f64) -> Option<f64> {
    if !all_finite(&[stress, youngs_modulus]) || youngs_modulus <= 0.0 {
        return None;
    }

    finite(stress / youngs_modulus)
}

/// Computes shear modulus from shear stress and shear strain.
///
/// Formula: `G = τ / γ`.
///
/// Returns `None` when `shear_strain` is zero, when the result is negative, or when any input or
/// result is not finite.
#[must_use]
pub fn shear_modulus(shear_stress: f64, shear_strain: f64) -> Option<f64> {
    if !all_finite(&[shear_stress, shear_strain]) || is_zero(shear_strain) {
        return None;
    }

    let result = shear_stress / shear_strain;
    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Computes shear stress from shear modulus and shear strain.
///
/// Formula: `τ = Gγ`.
///
/// Returns `None` when `shear_modulus` is negative or when any input or result is not finite.
#[must_use]
pub fn shear_stress_from_modulus(shear_modulus: f64, shear_strain: f64) -> Option<f64> {
    if !all_finite(&[shear_modulus, shear_strain]) || shear_modulus < 0.0 {
        return None;
    }

    finite(shear_modulus * shear_strain)
}

/// Computes shear strain from shear stress and shear modulus.
///
/// Formula: `γ = τ / G`.
///
/// Returns `None` when `shear_modulus` is less than or equal to zero or when any input or result
/// is not finite.
#[must_use]
pub fn shear_strain_from_modulus(shear_stress: f64, shear_modulus: f64) -> Option<f64> {
    if !all_finite(&[shear_stress, shear_modulus]) || shear_modulus <= 0.0 {
        return None;
    }

    finite(shear_stress / shear_modulus)
}

/// Computes bulk modulus from pressure change and volumetric strain.
///
/// Formula: `K = -ΔP / (ΔV / V)`.
///
/// Returns `None` when `volume_strain` is zero, when the result is negative, or when any input or
/// result is not finite.
#[must_use]
pub fn bulk_modulus(pressure_change: f64, volume_strain: f64) -> Option<f64> {
    if !all_finite(&[pressure_change, volume_strain]) || is_zero(volume_strain) {
        return None;
    }

    let result = -pressure_change / volume_strain;
    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Computes pressure change from bulk modulus and volumetric strain.
///
/// Formula: `ΔP = -K * volume_strain`.
///
/// Returns `None` when `bulk_modulus` is negative or when any input or result is not finite.
#[must_use]
pub fn pressure_change_from_bulk_modulus(bulk_modulus: f64, volume_strain: f64) -> Option<f64> {
    if !all_finite(&[bulk_modulus, volume_strain]) || bulk_modulus < 0.0 {
        return None;
    }

    finite(-bulk_modulus * volume_strain)
}

/// Computes volumetric strain from change in volume and original volume.
///
/// Formula: `ΔV / V0`.
///
/// Returns `None` when `original_volume` is less than or equal to zero or when any input or
/// result is not finite.
#[must_use]
pub fn volume_strain(change_in_volume: f64, original_volume: f64) -> Option<f64> {
    if !all_finite(&[change_in_volume, original_volume]) || original_volume <= 0.0 {
        return None;
    }

    finite(change_in_volume / original_volume)
}

/// Computes change in volume from volumetric strain and original volume.
///
/// Formula: `ΔV = volume_strain * V0`.
///
/// Returns `None` when `original_volume` is negative or when any input or result is not finite.
#[must_use]
pub fn change_in_volume(volume_strain: f64, original_volume: f64) -> Option<f64> {
    if !all_finite(&[volume_strain, original_volume]) || original_volume < 0.0 {
        return None;
    }

    finite(volume_strain * original_volume)
}

/// Computes Poisson's ratio from transverse and axial strain.
///
/// Formula: `ν = -ε_transverse / ε_axial`.
///
/// Returns `None` when `axial_strain` is zero or when any input or result is not finite.
/// Auxetic values are allowed; common stable isotropic materials often fall within a narrower
/// non-negative range.
#[must_use]
pub fn poisson_ratio(transverse_strain: f64, axial_strain: f64) -> Option<f64> {
    if !all_finite(&[transverse_strain, axial_strain]) || is_zero(axial_strain) {
        return None;
    }

    finite(-transverse_strain / axial_strain)
}

/// Computes transverse strain from Poisson's ratio and axial strain.
///
/// Formula: `ε_transverse = -ν * ε_axial`.
///
/// Returns `None` when any input or result is not finite.
#[must_use]
pub fn transverse_strain_from_poisson_ratio(poisson_ratio: f64, axial_strain: f64) -> Option<f64> {
    if !all_finite(&[poisson_ratio, axial_strain]) {
        return None;
    }

    finite(-poisson_ratio * axial_strain)
}

/// Returns `true` when Poisson's ratio is finite and between `0.0` and `0.5`, inclusive.
#[must_use]
pub fn is_common_poisson_ratio(poisson_ratio: f64) -> bool {
    poisson_ratio.is_finite() && (0.0..=0.5).contains(&poisson_ratio)
}

/// Computes shear modulus from Young's modulus and Poisson's ratio.
///
/// Formula: `G = E / (2 * (1 + ν))`.
///
/// Returns `None` when `youngs_modulus` is negative, when the denominator is zero, or when any
/// input or result is not finite.
///
/// # Examples
///
/// ```
/// use use_elasticity::shear_modulus_from_youngs_and_poisson;
///
/// assert_eq!(shear_modulus_from_youngs_and_poisson(260.0, 0.3), Some(100.0));
/// ```
#[must_use]
pub fn shear_modulus_from_youngs_and_poisson(
    youngs_modulus: f64,
    poisson_ratio: f64,
) -> Option<f64> {
    if !all_finite(&[youngs_modulus, poisson_ratio]) || youngs_modulus < 0.0 {
        return None;
    }

    let denominator = 2.0 * (1.0 + poisson_ratio);
    if !denominator.is_finite() || is_zero(denominator) {
        return None;
    }

    finite(youngs_modulus / denominator)
}

/// Computes bulk modulus from Young's modulus and Poisson's ratio.
///
/// Formula: `K = E / (3 * (1 - 2ν))`.
///
/// Returns `None` when `youngs_modulus` is negative, when the denominator is less than or equal
/// to zero, or when any input or result is not finite.
#[must_use]
pub fn bulk_modulus_from_youngs_and_poisson(
    youngs_modulus: f64,
    poisson_ratio: f64,
) -> Option<f64> {
    if !all_finite(&[youngs_modulus, poisson_ratio]) || youngs_modulus < 0.0 {
        return None;
    }

    let denominator = 3.0 * poisson_ratio.mul_add(-2.0, 1.0);
    if !denominator.is_finite() || denominator <= 0.0 {
        return None;
    }

    finite(youngs_modulus / denominator)
}

/// Computes Young's modulus from shear modulus and Poisson's ratio.
///
/// Formula: `E = 2G(1 + ν)`.
///
/// Returns `None` when `shear_modulus` is negative, when the result is negative, or when any
/// input or result is not finite.
#[must_use]
pub fn youngs_modulus_from_shear_and_poisson(
    shear_modulus: f64,
    poisson_ratio: f64,
) -> Option<f64> {
    if !all_finite(&[shear_modulus, poisson_ratio]) || shear_modulus < 0.0 {
        return None;
    }

    let result = 2.0 * shear_modulus * (1.0 + poisson_ratio);
    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Computes axial deformation of a prismatic bar under linear elastic loading.
///
/// Formula: `δ = FL / AE`.
///
/// Returns `None` when `length` is negative, `area` is less than or equal to zero,
/// `youngs_modulus` is less than or equal to zero, or when any input or result is not finite.
///
/// # Examples
///
/// ```
/// use use_elasticity::axial_deformation;
///
/// assert_eq!(axial_deformation(100.0, 10.0, 2.0, 1_000.0), Some(0.5));
/// ```
#[must_use]
pub fn axial_deformation(force: f64, length: f64, area: f64, youngs_modulus: f64) -> Option<f64> {
    if !all_finite(&[force, length, area, youngs_modulus])
        || length < 0.0
        || area <= 0.0
        || youngs_modulus <= 0.0
    {
        return None;
    }

    finite(force * length / (area * youngs_modulus))
}

/// Computes axial stiffness of a uniform elastic bar.
///
/// Formula: `k = AE / L`.
///
/// Returns `None` when `area` is negative, `youngs_modulus` is negative, `length` is less than or
/// equal to zero, or when any input or result is not finite.
#[must_use]
pub fn axial_stiffness(area: f64, youngs_modulus: f64, length: f64) -> Option<f64> {
    if !all_finite(&[area, youngs_modulus, length])
        || area < 0.0
        || youngs_modulus < 0.0
        || length <= 0.0
    {
        return None;
    }

    finite(area * youngs_modulus / length)
}

/// Computes force from axial deformation of a uniform elastic bar.
///
/// Formula: `F = δAE / L`.
///
/// Returns `None` when `length` is less than or equal to zero, `area` is negative,
/// `youngs_modulus` is negative, or when any input or result is not finite.
#[must_use]
pub fn force_from_axial_deformation(
    deformation: f64,
    length: f64,
    area: f64,
    youngs_modulus: f64,
) -> Option<f64> {
    if !all_finite(&[deformation, length, area, youngs_modulus])
        || length <= 0.0
        || area < 0.0
        || youngs_modulus < 0.0
    {
        return None;
    }

    finite(deformation * area * youngs_modulus / length)
}

/// Computes elastic strain-energy density.
///
/// Formula: `u = 0.5 * σ * ε`.
///
/// Returns `None` when the result is negative or when any input or result is not finite.
///
/// # Examples
///
/// ```
/// use use_elasticity::elastic_energy_density;
///
/// assert_eq!(elastic_energy_density(100.0, 0.01), Some(0.5));
/// ```
#[must_use]
pub fn elastic_energy_density(stress: f64, strain: f64) -> Option<f64> {
    if !all_finite(&[stress, strain]) {
        return None;
    }

    let result = 0.5 * stress * strain;
    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Computes elastic energy stored in a linear spring from stiffness and deformation.
///
/// Formula: `U = 0.5 * k * x²`.
///
/// Returns `None` when `spring_constant` is negative or when any input or result is not finite.
#[must_use]
pub fn elastic_energy_from_spring_constant(spring_constant: f64, deformation: f64) -> Option<f64> {
    if !all_finite(&[spring_constant, deformation]) || spring_constant < 0.0 {
        return None;
    }

    finite(0.5 * spring_constant * deformation * deformation)
}

/// Computes elastic energy from force and deformation for a linear loading path.
///
/// Formula: `U = 0.5 * F * x`.
///
/// Returns `None` when the result is negative or when any input or result is not finite.
#[must_use]
pub fn elastic_energy_from_force_deformation(force: f64, deformation: f64) -> Option<f64> {
    if !all_finite(&[force, deformation]) {
        return None;
    }

    let result = 0.5 * force * deformation;
    if result < 0.0 {
        return None;
    }

    finite(result)
}

/// Simple elastic material parameters for linear isotropic summaries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElasticMaterial {
    /// Young's modulus in pascals.
    pub youngs_modulus: f64,
    /// Poisson's ratio when known.
    pub poisson_ratio: Option<f64>,
}

impl ElasticMaterial {
    /// Creates a material summary with Young's modulus only.
    #[must_use]
    pub fn new(youngs_modulus: f64) -> Option<Self> {
        if !youngs_modulus.is_finite() || youngs_modulus < 0.0 {
            return None;
        }

        Some(Self {
            youngs_modulus,
            poisson_ratio: None,
        })
    }

    /// Creates a material summary with Young's modulus and Poisson's ratio.
    #[must_use]
    pub fn with_poisson_ratio(youngs_modulus: f64, poisson_ratio: f64) -> Option<Self> {
        if !poisson_ratio.is_finite() {
            return None;
        }

        Self::new(youngs_modulus).map(|material| Self {
            poisson_ratio: Some(poisson_ratio),
            ..material
        })
    }

    /// Computes stress from strain using this material's Young's modulus.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_elasticity::ElasticMaterial;
    ///
    /// let Some(material) = ElasticMaterial::new(200.0) else {
    ///     unreachable!();
    /// };
    ///
    /// assert_eq!(material.stress_from_strain(0.01), Some(2.0));
    /// ```
    #[must_use]
    pub fn stress_from_strain(&self, strain: f64) -> Option<f64> {
        stress_from_youngs_modulus(self.youngs_modulus, strain)
    }

    /// Computes strain from stress using this material's Young's modulus.
    #[must_use]
    pub fn strain_from_stress(&self, stress: f64) -> Option<f64> {
        strain_from_youngs_modulus(stress, self.youngs_modulus)
    }

    /// Computes shear modulus when Poisson's ratio is available.
    #[must_use]
    pub fn shear_modulus(&self) -> Option<f64> {
        self.poisson_ratio
            .and_then(|ratio| shear_modulus_from_youngs_and_poisson(self.youngs_modulus, ratio))
    }

    /// Computes bulk modulus when Poisson's ratio is available.
    #[must_use]
    pub fn bulk_modulus(&self) -> Option<f64> {
        self.poisson_ratio
            .and_then(|ratio| bulk_modulus_from_youngs_and_poisson(self.youngs_modulus, ratio))
    }
}

/// Simple uniform elastic bar properties for axial loading summaries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElasticBar {
    /// Original bar length in meters.
    pub length: f64,
    /// Cross-sectional area in square meters.
    pub area: f64,
    /// Young's modulus in pascals.
    pub youngs_modulus: f64,
}

impl ElasticBar {
    /// Creates a uniform elastic bar summary.
    #[must_use]
    pub fn new(length: f64, area: f64, youngs_modulus: f64) -> Option<Self> {
        if !all_finite(&[length, area, youngs_modulus])
            || length <= 0.0
            || area <= 0.0
            || youngs_modulus <= 0.0
        {
            return None;
        }

        Some(Self {
            length,
            area,
            youngs_modulus,
        })
    }

    /// Computes axial stiffness for this bar.
    #[must_use]
    pub fn axial_stiffness(&self) -> Option<f64> {
        axial_stiffness(self.area, self.youngs_modulus, self.length)
    }

    /// Computes axial deformation under the given force.
    ///
    /// # Examples
    ///
    /// ```
    /// use use_elasticity::ElasticBar;
    ///
    /// let Some(bar) = ElasticBar::new(10.0, 2.0, 1_000.0) else {
    ///     unreachable!();
    /// };
    ///
    /// assert_eq!(bar.deformation_under_force(100.0), Some(0.5));
    /// ```
    #[must_use]
    pub fn deformation_under_force(&self, force: f64) -> Option<f64> {
        axial_deformation(force, self.length, self.area, self.youngs_modulus)
    }

    /// Computes force required to cause the given deformation.
    #[must_use]
    pub fn force_for_deformation(&self, deformation: f64) -> Option<f64> {
        force_from_axial_deformation(deformation, self.length, self.area, self.youngs_modulus)
    }

    /// Computes normal stress under the given force.
    #[must_use]
    pub fn stress_under_force(&self, force: f64) -> Option<f64> {
        normal_stress(force, self.area)
    }

    /// Computes normal strain under the given force using Young's modulus.
    #[must_use]
    pub fn strain_under_force(&self, force: f64) -> Option<f64> {
        self.stress_under_force(force)
            .and_then(|stress| strain_from_youngs_modulus(stress, self.youngs_modulus))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ElasticBar, ElasticMaterial, axial_deformation, axial_stiffness, bulk_modulus,
        bulk_modulus_from_youngs_and_poisson, change_in_length, change_in_volume,
        elastic_energy_density, elastic_energy_from_force_deformation,
        elastic_energy_from_spring_constant, final_length, force_from_axial_deformation,
        force_from_stress, is_common_poisson_ratio, normal_strain, normal_stress, poisson_ratio,
        pressure_change_from_bulk_modulus, shear_modulus, shear_modulus_from_youngs_and_poisson,
        shear_strain, shear_strain_from_modulus, shear_stress, shear_stress_from_modulus,
        strain_from_youngs_modulus, stress_from_youngs_modulus,
        transverse_strain_from_poisson_ratio, volume_strain, youngs_modulus,
        youngs_modulus_from_shear_and_poisson,
    };

    fn assert_option_approx_eq(actual: Option<f64>, expected: f64) {
        let Some(actual) = actual else {
            panic!("expected Some({expected}), got None");
        };

        assert!(
            (actual - expected).abs() < 1.0e-12,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn stress_helpers_cover_expected_cases() {
        assert_eq!(normal_stress(100.0, 2.0), Some(50.0));
        assert_eq!(normal_stress(100.0, 0.0), None);

        assert_eq!(shear_stress(100.0, 2.0), Some(50.0));
        assert_eq!(shear_stress(100.0, 0.0), None);

        assert_eq!(force_from_stress(50.0, 2.0), Some(100.0));
        assert_eq!(force_from_stress(50.0, -2.0), None);
    }

    #[test]
    fn strain_helpers_cover_expected_cases() {
        assert_option_approx_eq(normal_strain(2.0, 10.0), 0.2);
        assert_option_approx_eq(normal_strain(-2.0, 10.0), -0.2);
        assert_eq!(normal_strain(2.0, 0.0), None);

        assert_option_approx_eq(shear_strain(2.0, 10.0), 0.2);
        assert_eq!(shear_strain(2.0, 0.0), None);

        assert_option_approx_eq(change_in_length(0.2, 10.0), 2.0);
        assert_option_approx_eq(final_length(10.0, 0.2), 12.0);
        assert_eq!(final_length(10.0, -1.2), None);
    }

    #[test]
    fn youngs_modulus_helpers_cover_expected_cases() {
        assert_option_approx_eq(youngs_modulus(100.0, 0.01), 10_000.0);
        assert_eq!(youngs_modulus(100.0, 0.0), None);
        assert_eq!(youngs_modulus(-100.0, 0.01), None);

        assert_option_approx_eq(stress_from_youngs_modulus(10_000.0, 0.01), 100.0);
        assert_option_approx_eq(strain_from_youngs_modulus(100.0, 10_000.0), 0.01);
    }

    #[test]
    fn shear_helpers_cover_expected_cases() {
        assert_option_approx_eq(shear_modulus(50.0, 0.01), 5_000.0);
        assert_eq!(shear_modulus(50.0, 0.0), None);

        assert_option_approx_eq(shear_stress_from_modulus(5_000.0, 0.01), 50.0);
        assert_option_approx_eq(shear_strain_from_modulus(50.0, 5_000.0), 0.01);
    }

    #[test]
    fn bulk_helpers_cover_expected_cases() {
        assert_option_approx_eq(volume_strain(-2.0, 10.0), -0.2);
        assert_eq!(volume_strain(-2.0, 0.0), None);

        assert_option_approx_eq(bulk_modulus(100.0, -0.01), 10_000.0);
        assert_eq!(bulk_modulus(100.0, 0.01), None);

        assert_option_approx_eq(pressure_change_from_bulk_modulus(10_000.0, -0.01), 100.0);
        assert_option_approx_eq(change_in_volume(-0.2, 10.0), -2.0);
    }

    #[test]
    fn poisson_helpers_cover_expected_cases() {
        assert_option_approx_eq(poisson_ratio(-0.003, 0.01), 0.3);
        assert_eq!(poisson_ratio(-0.003, 0.0), None);

        assert_option_approx_eq(transverse_strain_from_poisson_ratio(0.3, 0.01), -0.003);
        assert!(is_common_poisson_ratio(0.3));
        assert!(!is_common_poisson_ratio(-0.1));
        assert!(!is_common_poisson_ratio(0.6));
    }

    #[test]
    fn modulus_relationships_cover_expected_cases() {
        assert_option_approx_eq(shear_modulus_from_youngs_and_poisson(260.0, 0.3), 100.0);
        assert_option_approx_eq(bulk_modulus_from_youngs_and_poisson(300.0, 0.25), 200.0);
        assert_option_approx_eq(youngs_modulus_from_shear_and_poisson(100.0, 0.3), 260.0);
    }

    #[test]
    fn axial_helpers_cover_expected_cases() {
        assert_option_approx_eq(axial_deformation(100.0, 10.0, 2.0, 1_000.0), 0.5);
        assert_eq!(axial_deformation(100.0, 10.0, 0.0, 1_000.0), None);

        assert_option_approx_eq(axial_stiffness(2.0, 1_000.0, 10.0), 200.0);
        assert_eq!(axial_stiffness(2.0, 1_000.0, 0.0), None);

        assert_option_approx_eq(force_from_axial_deformation(0.5, 10.0, 2.0, 1_000.0), 100.0);
    }

    #[test]
    fn elastic_energy_helpers_cover_expected_cases() {
        assert_option_approx_eq(elastic_energy_density(100.0, 0.01), 0.5);
        assert_eq!(elastic_energy_density(-100.0, 0.01), None);

        assert_option_approx_eq(elastic_energy_from_spring_constant(100.0, 0.5), 12.5);
        assert_eq!(elastic_energy_from_spring_constant(-100.0, 0.5), None);

        assert_option_approx_eq(elastic_energy_from_force_deformation(100.0, 0.5), 25.0);
        assert_eq!(elastic_energy_from_force_deformation(-100.0, 0.5), None);
    }

    #[test]
    fn elastic_material_methods_cover_expected_cases() {
        let Some(material) = ElasticMaterial::with_poisson_ratio(260.0, 0.3) else {
            panic!("expected valid ElasticMaterial");
        };

        assert_option_approx_eq(material.stress_from_strain(0.01), 2.6);
        assert_option_approx_eq(material.strain_from_stress(2.6), 0.01);
        assert_option_approx_eq(material.shear_modulus(), 100.0);

        assert_eq!(ElasticMaterial::new(-1.0), None);
        assert_eq!(ElasticMaterial::with_poisson_ratio(260.0, f64::NAN), None);
    }

    #[test]
    fn elastic_bar_methods_cover_expected_cases() {
        let Some(bar) = ElasticBar::new(10.0, 2.0, 1_000.0) else {
            panic!("expected valid ElasticBar");
        };

        assert_option_approx_eq(bar.axial_stiffness(), 200.0);
        assert_option_approx_eq(bar.deformation_under_force(100.0), 0.5);
        assert_option_approx_eq(bar.force_for_deformation(0.5), 100.0);
        assert_option_approx_eq(bar.stress_under_force(100.0), 50.0);
        assert_option_approx_eq(bar.strain_under_force(100.0), 0.05);

        assert_eq!(ElasticBar::new(0.0, 2.0, 1_000.0), None);
        assert_eq!(ElasticBar::new(10.0, 0.0, 1_000.0), None);
        assert_eq!(ElasticBar::new(10.0, 2.0, 0.0), None);
    }
}
