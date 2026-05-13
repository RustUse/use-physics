#![allow(clippy::float_cmp)]

use use_power::{average_power, mechanical_power};

#[test]
fn power_helpers_cover_average_and_mechanical_power() -> Result<(), use_power::PowerError> {
    assert_eq!(average_power(120.0, 6.0)?, 20.0);
    assert_eq!(mechanical_power(10.0, 3.0), 30.0);
    Ok(())
}
