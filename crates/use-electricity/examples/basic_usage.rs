#![allow(clippy::float_cmp)]

use use_electricity::{ElectricalLoad, parallel_resistance, power_from_voltage_current, voltage};

fn main() {
    assert_eq!(voltage(2.0, 5.0), Some(10.0));
    assert_eq!(power_from_voltage_current(10.0, 2.0), Some(20.0));
    assert_eq!(parallel_resistance(&[2.0, 2.0]), Some(1.0));
    assert_eq!(
        ElectricalLoad::new(10.0, 5.0).and_then(|load| load.current()),
        Some(2.0)
    );
}
