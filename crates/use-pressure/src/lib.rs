#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Pressure and hydrostatic pressure helpers.

use core::fmt;

pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureError {
    NonPositiveArea,
}

impl fmt::Display for PressureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveArea => formatter.write_str("area must be greater than zero"),
        }
    }
}

impl std::error::Error for PressureError {}

#[must_use]
pub const fn hydrostatic_pressure(density: f64, gravity: f64, depth: f64) -> f64 {
    density * gravity * depth
}

#[must_use]
pub const fn gauge_pressure(absolute_pressure: f64, atmospheric_pressure: f64) -> f64 {
    absolute_pressure - atmospheric_pressure
}

/// Computes pressure from applied force and cross-sectional area.
///
/// # Errors
///
/// Returns [`PressureError::NonPositiveArea`] when `area` is less than or equal to zero.
pub fn pressure(force: f64, area: f64) -> Result<f64, PressureError> {
    if area <= 0.0 {
        Err(PressureError::NonPositiveArea)
    } else {
        Ok(force / area)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{PressureError, gauge_pressure, hydrostatic_pressure, pressure};

    #[test]
    fn pressure_helpers_cover_common_calculations() -> Result<(), PressureError> {
        assert_eq!(pressure(100.0, 4.0)?, 25.0);
        assert_eq!(hydrostatic_pressure(1000.0, 10.0, 2.0), 20_000.0);
        assert_eq!(gauge_pressure(120.0, 101.0), 19.0);
        Ok(())
    }

    #[test]
    fn pressure_requires_positive_area() {
        assert_eq!(pressure(100.0, 0.0), Err(PressureError::NonPositiveArea));
    }
}
