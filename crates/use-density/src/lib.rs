#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Density, mass, and volume helpers.

use core::fmt;

pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DensityError {
    NonPositiveVolume,
    NonPositiveDensity,
}

impl fmt::Display for DensityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveVolume => formatter.write_str("volume must be greater than zero"),
            Self::NonPositiveDensity => formatter.write_str("density must be greater than zero"),
        }
    }
}

impl std::error::Error for DensityError {}

#[must_use]
pub const fn mass(density: f64, volume: f64) -> f64 {
    density * volume
}

/// Computes density from mass and occupied volume.
///
/// # Errors
///
/// Returns [`DensityError::NonPositiveVolume`] when `volume` is less than or equal to zero.
pub fn density(mass: f64, volume: f64) -> Result<f64, DensityError> {
    if volume <= 0.0 {
        Err(DensityError::NonPositiveVolume)
    } else {
        Ok(mass / volume)
    }
}

/// Computes occupied volume from mass and density.
///
/// # Errors
///
/// Returns [`DensityError::NonPositiveDensity`] when `density` is less than or equal to zero.
pub fn volume(mass: f64, density: f64) -> Result<f64, DensityError> {
    if density <= 0.0 {
        Err(DensityError::NonPositiveDensity)
    } else {
        Ok(mass / density)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{DensityError, density, mass, volume};

    #[test]
    fn density_helpers_cover_mass_volume_relationships() -> Result<(), DensityError> {
        assert_eq!(density(10.0, 2.0)?, 5.0);
        assert_eq!(mass(5.0, 2.0), 10.0);
        assert_eq!(volume(10.0, 5.0)?, 2.0);
        Ok(())
    }

    #[test]
    fn density_validation_requires_positive_inputs() {
        assert_eq!(density(10.0, 0.0), Err(DensityError::NonPositiveVolume));
        assert_eq!(volume(10.0, 0.0), Err(DensityError::NonPositiveDensity));
    }
}
