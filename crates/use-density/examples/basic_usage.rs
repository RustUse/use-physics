#![allow(clippy::float_cmp)]

use use_density::{density, mass, volume};

fn main() -> Result<(), use_density::DensityError> {
    assert_eq!(density(10.0, 2.0)?, 5.0);
    assert_eq!(mass(5.0, 2.0), 10.0);
    assert_eq!(volume(10.0, 5.0)?, 2.0);
    Ok(())
}
