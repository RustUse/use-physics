#![allow(clippy::float_cmp)]

use use_physics::{force, kinetic_energy};

fn main() {
    let applied_force = force(10.0, 2.0);
    let energy = kinetic_energy(2.0, 3.0);

    assert_eq!(applied_force, 20.0);
    assert_eq!(energy, 9.0);
}
