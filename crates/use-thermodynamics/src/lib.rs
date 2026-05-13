#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Ideal gas and heat-energy helpers.

use core::fmt;

pub mod prelude;

pub const IDEAL_GAS_CONSTANT: f64 = 8.314_462_618_153_24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermodynamicsError {
    NonPositiveTemperature,
    NonPositiveVolume,
}

impl fmt::Display for ThermodynamicsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveTemperature => {
                formatter.write_str("temperature must be greater than zero kelvin")
            },
            Self::NonPositiveVolume => formatter.write_str("volume must be greater than zero"),
        }
    }
}

impl std::error::Error for ThermodynamicsError {}

#[must_use]
pub const fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

#[must_use]
pub const fn heat_energy(mass: f64, specific_heat_capacity: f64, delta_temperature: f64) -> f64 {
    mass * specific_heat_capacity * delta_temperature
}

/// Computes ideal gas pressure from amount of substance, temperature, and volume.
///
/// # Errors
///
/// Returns [`ThermodynamicsError::NonPositiveTemperature`] when `temperature_kelvin` is less
/// than or equal to zero. Returns [`ThermodynamicsError::NonPositiveVolume`] when `volume` is
/// less than or equal to zero.
pub fn ideal_gas_pressure(
    moles: f64,
    temperature_kelvin: f64,
    volume: f64,
) -> Result<f64, ThermodynamicsError> {
    if temperature_kelvin <= 0.0 {
        Err(ThermodynamicsError::NonPositiveTemperature)
    } else if volume <= 0.0 {
        Err(ThermodynamicsError::NonPositiveVolume)
    } else {
        Ok((moles * IDEAL_GAS_CONSTANT * temperature_kelvin) / volume)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{
        IDEAL_GAS_CONSTANT, ThermodynamicsError, celsius_to_kelvin, heat_energy, ideal_gas_pressure,
    };

    #[test]
    fn thermodynamics_helpers_cover_common_calculations() -> Result<(), ThermodynamicsError> {
        let pressure = ideal_gas_pressure(2.0, 300.0, 3.0)?;
        let expected = (2.0 * IDEAL_GAS_CONSTANT * 300.0) / 3.0;

        assert!((pressure - expected).abs() < 1.0e-12);
        assert_eq!(celsius_to_kelvin(0.0), 273.15);
        assert_eq!(heat_energy(2.0, 4.0, 5.0), 40.0);
        Ok(())
    }

    #[test]
    fn ideal_gas_pressure_requires_positive_state() {
        assert_eq!(
            ideal_gas_pressure(1.0, 0.0, 1.0),
            Err(ThermodynamicsError::NonPositiveTemperature)
        );
        assert_eq!(
            ideal_gas_pressure(1.0, 300.0, 0.0),
            Err(ThermodynamicsError::NonPositiveVolume)
        );
    }
}
