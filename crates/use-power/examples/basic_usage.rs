#![allow(clippy::float_cmp)]

use use_power::{average_power, electrical_power};

fn main() -> Result<(), use_power::PowerError> {
    assert_eq!(average_power(120.0, 6.0)?, 20.0);
    assert_eq!(electrical_power(12.0, 2.0), 24.0);
    Ok(())
}
