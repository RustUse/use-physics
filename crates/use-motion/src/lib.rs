#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Basic kinematics helpers.

use core::fmt;

pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionError {
    NonPositiveDuration,
}

impl fmt::Display for MotionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveDuration => formatter.write_str("duration must be greater than zero"),
        }
    }
}

impl std::error::Error for MotionError {}

#[must_use]
pub const fn distance(speed: f64, time: f64) -> f64 {
    speed * time
}

#[must_use]
pub const fn displacement(initial_position: f64, final_position: f64) -> f64 {
    final_position - initial_position
}

#[must_use]
pub const fn final_velocity(initial_velocity: f64, acceleration: f64, time: f64) -> f64 {
    initial_velocity + acceleration * time
}

/// Computes average speed from traveled distance and elapsed duration.
///
/// # Errors
///
/// Returns [`MotionError::NonPositiveDuration`] when `duration` is less than or equal to zero.
pub fn average_speed(distance: f64, duration: f64) -> Result<f64, MotionError> {
    if duration <= 0.0 {
        Err(MotionError::NonPositiveDuration)
    } else {
        Ok(distance / duration)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{MotionError, average_speed, displacement, distance, final_velocity};

    #[test]
    fn motion_helpers_cover_common_kinematics() -> Result<(), MotionError> {
        assert_eq!(distance(5.0, 4.0), 20.0);
        assert_eq!(displacement(3.0, 9.0), 6.0);
        assert_eq!(final_velocity(2.0, 3.0, 4.0), 14.0);
        assert_eq!(average_speed(100.0, 10.0)?, 10.0);
        Ok(())
    }

    #[test]
    fn average_speed_requires_positive_duration() {
        assert_eq!(
            average_speed(100.0, 0.0),
            Err(MotionError::NonPositiveDuration)
        );
    }
}
