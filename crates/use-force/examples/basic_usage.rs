#![allow(clippy::float_cmp)]

use use_force::{force, impulse};

fn main() {
    assert_eq!(force(10.0, 2.0), 20.0);
    assert_eq!(impulse(2.0, 1.0, 4.0), 6.0);
}
