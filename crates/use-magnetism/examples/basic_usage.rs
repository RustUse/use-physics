#![allow(clippy::float_cmp)]

use std::f64::consts::FRAC_PI_2;

use use_magnetism::{MagneticField, magnetic_field_inside_solenoid, magnetic_flux};

fn main() {
    assert_eq!(magnetic_flux(2.0, 3.0, 0.0), Some(6.0));
    assert_eq!(
        magnetic_field_inside_solenoid(1_000.0, 2.0, 0.5).map(|value| value > 0.0),
        Some(true)
    );
    assert_eq!(
        MagneticField::new(3.0).and_then(|field| field.force_on_charge(1.0, 2.0, FRAC_PI_2)),
        Some(6.0)
    );
}
