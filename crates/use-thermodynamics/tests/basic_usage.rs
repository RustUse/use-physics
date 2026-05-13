#![allow(clippy::float_cmp)]

use use_thermodynamics::{celsius_to_kelvin, ideal_gas_pressure};

#[test]
fn thermodynamics_helpers_cover_ideal_gas_and_temperature()
-> Result<(), use_thermodynamics::ThermodynamicsError> {
    let pressure = ideal_gas_pressure(2.0, 300.0, 3.0)?;

    assert!((pressure - 1_662.892_523_630_648).abs() < 1.0e-12);
    assert_eq!(celsius_to_kelvin(0.0), 273.15);

    Ok(())
}
