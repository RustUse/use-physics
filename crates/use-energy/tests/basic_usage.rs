#![allow(clippy::float_cmp)]

use use_energy::{kinetic_energy, potential_energy};

#[test]
fn energy_helpers_cover_kinetic_and_potential_energy() {
    assert_eq!(kinetic_energy(2.0, 3.0), 9.0);
    assert_eq!(potential_energy(2.0, 10.0, 3.0), 60.0);
}
