#![allow(clippy::float_cmp)]

use use_electricity::{ElectricalLoad, parallel_resistance, power_from_voltage_current, voltage};

#[test]
fn electricity_helpers_cover_basic_usage() {
    assert_eq!(voltage(2.0, 5.0), Some(10.0));
    assert_eq!(power_from_voltage_current(10.0, 2.0), Some(20.0));
    assert_eq!(parallel_resistance(&[2.0, 2.0]), Some(1.0));

    let load = ElectricalLoad::new(10.0, 5.0).expect("valid load");
    assert_eq!(load.current(), Some(2.0));
}
