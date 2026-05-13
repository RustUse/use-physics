#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Average, mechanical, and electrical power helpers.

use core::fmt;

pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerError {
    NonPositiveDuration,
}

impl fmt::Display for PowerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveDuration => formatter.write_str("duration must be greater than zero"),
        }
    }
}

impl std::error::Error for PowerError {}

#[must_use]
pub const fn mechanical_power(force: f64, velocity: f64) -> f64 {
    force * velocity
}

#[must_use]
pub const fn electrical_power(voltage: f64, current: f64) -> f64 {
    voltage * current
}

/// Computes average power from total work and elapsed duration.
///
/// # Errors
///
/// Returns [`PowerError::NonPositiveDuration`] when `duration` is less than or equal to zero.
pub fn average_power(work: f64, duration: f64) -> Result<f64, PowerError> {
    if duration <= 0.0 {
        Err(PowerError::NonPositiveDuration)
    } else {
        Ok(work / duration)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{PowerError, average_power, electrical_power, mechanical_power};

    #[test]
    fn power_helpers_cover_common_calculations() -> Result<(), PowerError> {
        assert_eq!(mechanical_power(10.0, 3.0), 30.0);
        assert_eq!(electrical_power(12.0, 2.0), 24.0);
        assert_eq!(average_power(120.0, 6.0)?, 20.0);
        Ok(())
    }

    #[test]
    fn average_power_requires_positive_duration() {
        assert_eq!(
            average_power(120.0, 0.0),
            Err(PowerError::NonPositiveDuration)
        );
    }
}
