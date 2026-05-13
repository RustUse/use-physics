#![allow(clippy::float_cmp)]

use use_density::{density, volume};

#[test]
fn density_helpers_cover_density_and_volume() -> Result<(), use_density::DensityError> {
    assert_eq!(density(10.0, 2.0)?, 5.0);
    assert_eq!(volume(10.0, 5.0)?, 2.0);
    Ok(())
}
