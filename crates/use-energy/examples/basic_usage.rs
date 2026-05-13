#![allow(clippy::float_cmp)]

use use_energy::{kinetic_energy, work};

fn main() {
    assert_eq!(kinetic_energy(2.0, 3.0), 9.0);
    assert_eq!(work(5.0, 10.0), 50.0);
}
