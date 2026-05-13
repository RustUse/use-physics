#![allow(clippy::float_cmp)]

use use_pressure::{hydrostatic_pressure, pressure};

fn main() -> Result<(), use_pressure::PressureError> {
    assert_eq!(pressure(100.0, 4.0)?, 25.0);
    assert_eq!(hydrostatic_pressure(1000.0, 10.0, 2.0), 20_000.0);
    Ok(())
}
