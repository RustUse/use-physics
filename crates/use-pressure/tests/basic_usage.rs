#![allow(clippy::float_cmp)]

use use_pressure::{gauge_pressure, pressure};

#[test]
fn pressure_helpers_cover_pressure_and_gauge_pressure() -> Result<(), use_pressure::PressureError> {
    assert_eq!(pressure(100.0, 4.0)?, 25.0);
    assert_eq!(gauge_pressure(120.0, 101.0), 19.0);
    Ok(())
}
